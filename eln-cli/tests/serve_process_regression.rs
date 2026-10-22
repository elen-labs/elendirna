//! N0089 / N0090 process suicide regression test.
//!
//! 시나리오: `elf serve --mcp`를 cwd가 vault 아닌 위치에서 spawn + fake `USERPROFILE`/`HOME`에
//! 기존 vault 박혀 있는 상태. v0.5.4까지는 auto-init이 `AlreadyInitialized`로 process suicide →
//! Desktop transport close → re-spawn 무한 루프 발현. v0.6.0(N0090)부터는 Fallback init이
//! 기존 vault를 채택하고 stderr warning 후 Ok → process 살아있음.
//!
//! codex 사전 검토 권고 (D 항목):
//! - USERPROFILE + HOME 둘 다 fake set (cross-platform 안전)
//! - ELF_VAULT 반드시 env_remove (serve가 ELF_VAULT를 home fallback보다 먼저 봄)
//! - cwd는 fake home **밖** (그렇지 않으면 find_local_vault_root가 home vault hit → CwdSearchHome 분기 → fallback init 자체가 트리거 안 됨)
//! - child.stdin은 `Stdio::piped()`로 열고 핸들 유지 (stdin 닫히면 stdio server 정상 종료 → false failure)
//! - kill + wait 모두 호출

use assert_cmd::cargo::CommandCargoExt;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// CI/Windows 호환: fake home에 vault init은 binary로 한 번 호출 (`elf init --global` 류 대신
/// `elf init <path>`로 fake home에 박기). 다음 spawn에서 그 vault를 발견하게 함.
fn init_vault_at(path: &std::path::Path) {
    let mut cmd = Command::cargo_bin("elf").expect("cargo binary 'elf' 찾기 실패");
    cmd.arg("init")
        .arg(path)
        .arg("--name")
        .arg("regression-fake-home")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let status = cmd.status().expect("init 실행 실패");
    assert!(status.success(), "fake home vault init 실패");
}

#[test]
fn serve_mcp_survives_when_home_already_initialized_and_cwd_is_non_vault() {
    let fake_home = tempfile::tempdir().expect("fake home tempdir");
    let fake_cwd = tempfile::tempdir().expect("fake cwd tempdir");

    // 1. fake home에 미리 vault 박음 (config.toml 존재)
    init_vault_at(fake_home.path());
    assert!(fake_home.path().join(".elendirna/config.toml").exists());

    // 2. fake cwd는 vault 아님 (init 안 함)
    assert!(!fake_cwd.path().join(".elendirna").exists());

    // 3. serve --mcp spawn — fake home + non-vault cwd
    let mut cmd = Command::cargo_bin("elf").expect("cargo binary 'elf' 찾기 실패");
    cmd.arg("serve")
        .arg("--mcp")
        .env_remove("ELF_VAULT")
        .env("USERPROFILE", fake_home.path())
        .env("HOME", fake_home.path())
        .current_dir(fake_cwd.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("elf serve --mcp spawn 실패");
    // stdin 핸들은 drop하지 않고 유지 (drop되면 stdio server가 정상 종료 → false failure).
    // child가 scope 안에 살아있는 동안 stdin도 함께 유지된다.
    let _stdin_keep_alive = child.stdin.take();

    // 4. 3초 poll loop — process가 살아있는지 확인. 더 안정적인 timing.
    let start = Instant::now();
    let timeout = Duration::from_secs(3);
    let poll_interval = Duration::from_millis(200);
    let mut early_exit: Option<std::process::ExitStatus> = None;
    while start.elapsed() < timeout {
        match child.try_wait() {
            Ok(Some(status)) => {
                early_exit = Some(status);
                break;
            }
            Ok(None) => {
                // 아직 살아있음 — 계속 poll
            }
            Err(e) => panic!("try_wait 실패: {e}"),
        }
        std::thread::sleep(poll_interval);
    }

    // 5. cleanup — 어떤 결과든 kill + wait
    let _ = child.kill();
    let _ = child.wait();

    // 6. assertion — N0089 회귀였다면 3초 안에 exit 했을 것 (이전 v0.5.4 동작).
    //    본 PR fix로 살아있어야 함.
    assert!(
        early_exit.is_none(),
        "N0089 regression: elf serve --mcp exited within 3s (status={early_exit:?}). \
         Expected stdio server to stay alive after Fallback init adopts existing vault."
    );
}

/// `--no-fallback`: vault를 못 찾으면 글로벌로 넘어가지 않고 즉시 종료해야 한다 (주제 vault 격리).
/// 홈 vault 미존재 → 자동 생성도 하지 않아야 하고, cwd 상위 탐색이 홈 vault에 닿는 경우도 거부.
fn serve_no_fallback(fake_home: &std::path::Path, cwd: &std::path::Path) -> std::process::Output {
    let mut cmd = Command::cargo_bin("elf").expect("cargo binary 'elf' 찾기 실패");
    cmd.arg("serve")
        .arg("--mcp")
        .arg("--no-fallback")
        .env_remove("ELF_VAULT")
        .env("USERPROFILE", fake_home)
        .env("HOME", fake_home)
        .current_dir(cwd)
        // 거부 경로는 stdin을 읽기 전에 끝나야 한다 — 서버가 떴다면 null stdin으로 곧 정상 종료.
        .stdin(Stdio::null());
    cmd.output()
        .expect("elf serve --mcp --no-fallback 실행 실패")
}

/// tempdir 상위에 실제 vault가 있으면(예: 시스템 temp가 실제 홈 vault 아래) walk-up이 그걸 잡으므로
/// "vault 없음" 시나리오 자체가 성립하지 않는다.
fn has_ancestor_vault(path: &std::path::Path) -> bool {
    path.ancestors()
        .any(|p| p.join(".elendirna").join("config.toml").exists())
}

#[test]
fn serve_no_fallback_rejects_when_no_vault_found() {
    let fake_home = tempfile::tempdir().expect("fake home tempdir");
    let fake_cwd = tempfile::tempdir().expect("fake cwd tempdir");
    if has_ancestor_vault(fake_cwd.path()) {
        eprintln!(
            "skip: tempdir 상위에 실제 vault 존재 — {}",
            fake_cwd.path().display()
        );
        return;
    }

    let out = serve_no_fallback(fake_home.path(), fake_cwd.path());
    assert!(!out.status.success(), "--no-fallback인데 성공 종료함");
    assert!(String::from_utf8_lossy(&out.stderr).contains("--no-fallback"));
    assert!(
        !fake_home.path().join(".elendirna").exists(),
        "--no-fallback인데 글로벌 vault를 자동 생성함"
    );
}

#[test]
fn serve_no_fallback_rejects_cwd_walkup_hitting_home_vault() {
    let fake_home = tempfile::tempdir().expect("fake home tempdir");
    init_vault_at(fake_home.path());
    let topic_dir = fake_home.path().join("topic-without-vault");
    std::fs::create_dir_all(&topic_dir).unwrap();

    let out = serve_no_fallback(fake_home.path(), &topic_dir);
    assert!(
        !out.status.success(),
        "홈 vault walk-up을 --no-fallback이 막지 못함"
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("홈(글로벌) vault"));
}

#[test]
fn serve_no_fallback_allows_topic_vault_under_home() {
    let fake_home = tempfile::tempdir().expect("fake home tempdir");
    init_vault_at(fake_home.path());
    let topic_dir = fake_home.path().join("gaming");
    std::fs::create_dir_all(&topic_dir).unwrap();
    init_vault_at(&topic_dir);

    // 주제 vault가 있으면 정상 기동 — stdin을 열어 둔 채 살아 있는지만 본다.
    let mut cmd = Command::cargo_bin("elf").expect("cargo binary 'elf' 찾기 실패");
    cmd.arg("serve")
        .arg("--mcp")
        .arg("--no-fallback")
        .env_remove("ELF_VAULT")
        .env("USERPROFILE", fake_home.path())
        .env("HOME", fake_home.path())
        .current_dir(&topic_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("elf serve --mcp spawn 실패");
    let _stdin_keep_alive = child.stdin.take();
    std::thread::sleep(Duration::from_millis(1500));
    let early_exit = child.try_wait().expect("try_wait 실패");
    let _ = child.kill();
    let _ = child.wait();
    assert!(
        early_exit.is_none(),
        "주제 vault cwd에서 --no-fallback이 기동을 막음 (status={early_exit:?})"
    );
}
