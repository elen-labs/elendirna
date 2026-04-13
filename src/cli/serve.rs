/// `elf serve --mcp` — MCP 서버 진입점
use clap::Args;
use crate::error::ElfError;
use crate::vault;

#[derive(Debug, Args)]
pub struct ServeArgs {
    /// MCP 프로토콜로 서버 구동 (stdio transport)
    #[arg(long)]
    pub mcp: bool,

    /// vault 경로 (기본: 현재 디렉터리에서 탐색 → 없으면 글로벌 vault 자동 생성)
    #[arg(long)]
    pub vault: Option<std::path::PathBuf>,
}

pub fn run(args: ServeArgs) -> Result<(), ElfError> {
    if !args.mcp {
        return Err(ElfError::InvalidInput {
            message: "현재는 --mcp 플래그만 지원합니다".to_string(),
        });
    }

    let vault_root = match args.vault {
        Some(path) => path,
        None => match std::env::var("ELF_VAULT") {
            Ok(env_path) => std::path::PathBuf::from(env_path),
            Err(_) => {
                let cwd = std::env::current_dir()?;
                match vault::find_vault_root(&cwd) {
                    Ok(root) => root,
                    Err(ElfError::NotAVault) => {
                        // vault를 찾지 못하면 글로벌 vault(~/.elendirna/)를 자동 생성
                        let home = std::env::var("USERPROFILE")
                            .or_else(|_| std::env::var("HOME"))
                            .map(std::path::PathBuf::from)
                            .map_err(|_| ElfError::InvalidInput {
                                message: "홈 디렉터리를 결정할 수 없습니다".to_string(),
                            })?;
                        eprintln!("[elf] vault 없음 — 글로벌 vault 자동 생성: {}", home.display());
                        crate::cli::init::run(crate::cli::init::InitArgs {
                            path: home.clone(),
                            dry_run: false,
                            name: Some("global".to_string()),
                            global: true,
                        })?;
                        home
                    }
                    Err(e) => return Err(e),
                }
            }
        },
    };

    // v1 vault 자동 이관 (MCP stdio 보호: stderr만 사용)
    crate::cli::migrate::auto_migrate_silent(&vault_root);

    crate::mcp::run_stdio(vault_root)
        .map_err(|e| ElfError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))
}
