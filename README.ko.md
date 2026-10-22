# Elendirna

> 모델이 바뀌어도, 탐구의 기록과 작업 맥락을 이어갑니다.

[![crates.io](https://img.shields.io/crates/v/eln-cli.svg)](https://crates.io/crates/eln-cli) [![Rust](https://github.com/elen-labs/elendirna/actions/workflows/rust.yml/badge.svg)](https://github.com/elen-labs/elendirna/actions/workflows/rust.yml)

[English](README.md) | [한국어](README.ko.md)

Elendirna는 아이디어를 탐구하며 쌓은 기록을 보관하는 vault입니다. 바탕이 되는 내용을 적고, 생각이 달라지거나 더해질 때마다 revision으로 남깁니다. 그래서 모델이나 서비스를 바꿔도 지금의 결론뿐 아니라 여기까지 온 과정까지 다시 살펴볼 수 있습니다.

Elendirna는 [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF)에서 영감을 받은 독립 프로젝트이며, ELF의 공식 구현체나 후속 프로젝트는 아닙니다.

## 왜 생각의 이력을 남기나요?

대화는 그때 쓴 모델이나 서비스에 남습니다. 노트에 결론을 적어 두더라도 그 결론에 이른 과정은 빠뜨리기 쉽습니다. Elendirna는 entry와 revision을 사용자의 vault에 모아 두어, 나중에 사람이든 AI 도구든 이어서 살펴볼 수 있게 합니다.

Elendirna가 Git이나 채팅 기록을 대신하지는 않습니다. Git은 파일이 어떻게 바뀌었는지를 추적하고, Elendirna는 왜 그렇게 결정했는지, 생각이 어떻게 바뀌었는지, 어떤 entry와 이어지는지를 기록합니다. 무엇을 기록하고 무엇을 공유할지는 사용자가 정합니다.

## 설치

### npm (사전 빌드 바이너리, Rust 툴체인 불필요)

```bash
npx -y elendirna --help       # 일회 실행
npm install -g elendirna      # `elendirna` 명령 설치
elendirna --help
```

npm 패키지에는 Linux, macOS, Windows용 `elf` 실행 파일이 x64와 arm64 버전으로 들어 있습니다. `postinstall` 스크립트가 없고, 설치할 때나 실행할 때 네트워크에 연결하지 않습니다. Linux 바이너리는 glibc 2.35 이상이 필요하며, musl/Alpine에서는 Cargo로 설치하세요.

### Cargo

```bash
cargo install eln-cli
```

설치되는 실행 파일 이름은 `elf`입니다. Rust workspace는 crate 세 개로 되어 있습니다. CLI인 `eln-cli`, vault와 MCP 서버를 담은 `eln-core`, 그리고 따로 배포되는 Rust plugin interface `eln-plugin-sdk`입니다. `elendirna` crate는 패키지를 나누기 전의 0.6.x 버전입니다.

## 빠른 시작

아래 명령은 Cargo로 설치한 `elf` 기준입니다. npm으로 설치했다면 `elf`를 `elendirna`로 바꾸거나, 각 명령 앞에 `npx -y elendirna`를 붙이세요.

```bash
elf init my-vault
cd my-vault
elf entry new "Rust ownership"
elf entry new "Borrow checker"
elf revision add N0001 --delta "문법보다 데이터의 소유자를 누구로 둘지 정하는 쪽이 어려웠다"
elf link N0001 N0002
elf entry status N0001 stable
elf entry status N0002 stable
elf validate
```

revision에는 생각이 바뀐 지점을 적습니다. "무엇을 추가했다"는 변경 로그를 넣는 자리는 아닙니다. 위 delta는 이 entry의 초점이 어디로 옮겨 갔는지를 한 줄로 남기는데, 나중에 읽는 사람이나 AI에게 필요한 것도 바로 그 한 줄입니다.

`elf entry show N0001`로 entry를 읽을 수 있습니다. 편집기가 설정되어 있다면 `elf entry edit N0001`이 note를 편집기로 엽니다. `elf guide`는 전체 명령과 AI 작업 흐름을 보여 주고, `elf <command> --help`는 명령 하나의 옵션을 보여 줍니다.

## AI 클라이언트 연결

공유할 vault를 지정해 MCP 서버를 stdio로 띄웁니다.

```bash
elf serve --mcp --vault /path/to/vault
```

`elf serve --snippet`은 MCP 클라이언트에 넣을 설정 예시를 출력합니다. `--vault`를 생략하면 `--global`, `ELF_VAULT`, 현재 디렉터리에서 찾은 vault, 설정된 전역 vault 순으로 고릅니다. AI 클라이언트에는 공유해도 되는 vault만 연결하세요.

연결이 되면 결론 대신 과정을 물어보세요.

> Rust ownership entry에서 초점이 문법에서 데이터 소유권으로 옮겨 간 게 언제였고, 이유가 뭐였지?

클라이언트는 vault의 `bundle` 도구로 entry와 revision chain, 연결된 entry를 받아 와서 기록된 내용을 근거로 답합니다. 내일 다른 모델을 연결해도 같은 이력을 읽습니다.

## 무엇을 저장하나요?

vault는 평범한 파일로 된 디렉터리입니다. entry마다 폴더가 하나씩 있고, 그 안에 구조화된 메타데이터를 담은 `manifest.toml`, 사람이 읽는 `note.md`, 첨부 파일용 `attachments/` 폴더가 들어갑니다. revision은 `revisions/<entry id>/` 아래에 delta 하나당 Markdown 파일 하나로 쌓이고, entry 간 링크는 manifest에 기록됩니다. SQLite 인덱스는 파생 데이터라서 `elf validate`가 다시 만듭니다. AI 에이전트 사이의 세션 인계 기록은 `sync.jsonl`에 쌓입니다.

저장소에는 작은 데모 vault([`eln-core/tests/fixtures/demo_vault`](eln-core/tests/fixtures/demo_vault/README.md))가 들어 있습니다. 이 프로젝트 자체는 비공개 작업 vault로 개발하고 있고, 그 vault는 저장소에 포함하지 않습니다. 비공개 vault의 내용은 검토를 거친 뒤에야 공개 코드, 문서, fixture, 릴리스로 옮깁니다. 작업 vault를 직접 운영한다면 같은 경계를 두는 편이 좋습니다.

## 라이선스와 저작자 표시

소프트웨어는 MIT 라이선스입니다. [LICENSE](LICENSE)를 참고하세요.

Base-Delta, SSoT, Session-Trial 식별자 개념은 Eli (projectschnee@gmail.com)의 [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF)에서 왔으며, [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) 라이선스를 따릅니다.
