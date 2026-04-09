/// `elf serve --mcp` — MCP 서버 진입점 (Phase 6에서 구현)
use clap::Args;
use crate::error::ElfError;

#[derive(Debug, Args)]
pub struct ServeArgs {
    /// MCP 프로토콜로 서버 구동 (stdio transport)
    #[arg(long)]
    pub mcp: bool,

    /// vault 경로 (기본: 현재 디렉터리에서 탐색)
    #[arg(long)]
    pub vault: Option<std::path::PathBuf>,
}

pub fn run(args: ServeArgs) -> Result<(), ElfError> {
    if !args.mcp {
        return Err(ElfError::InvalidInput {
            message: "현재는 --mcp 플래그만 지원합니다".to_string(),
        });
    }

    // Phase 6에서 구현: MCP 서버 시작
    eprintln!("MCP 서버는 Phase 6에서 구현 예정입니다.");
    Ok(())
}
