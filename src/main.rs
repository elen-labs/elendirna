use elendirna::cli;
use elendirna::error::ElfError;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "elf",
    about = "Elendirna vault CLI",
    version = env!("CARGO_PKG_VERSION"),
)]
struct Cli {
    /// 모든 출력을 JSON으로 (fix-015: 에러는 --json 무관 항상 JSON/stderr)
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// vault 초기화
    Init(cli::init::InitArgs),

    /// entry 관리 (new / show / edit)
    Entry(cli::entry::EntryArgs),

    /// revision 추가
    Revision(cli::revision::RevisionArgs),

    /// entry 간 링크 생성
    Link(cli::link::LinkArgs),

    /// vault 무결성 검사
    Validate(cli::validate::ValidateArgs),
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Init(args)     => cli::init::run(args),
        Commands::Entry(args)    => run_entry(args),
        Commands::Revision(args) => cli::revision::run(args),
        Commands::Link(args)     => cli::link::run(args),
        Commands::Validate(args) => cli::validate::run(args),
    };

    if let Err(e) = result {
        // fix-015: 에러는 항상 JSON으로 stderr
        e.emit_json();
        std::process::exit(e.exit_code());
    }
}

fn run_entry(args: cli::entry::EntryArgs) -> Result<(), ElfError> {
    match args.command {
        cli::entry::EntryCommand::New(a)  => cli::entry::run_new(a),
        cli::entry::EntryCommand::Show(a) => cli::entry::run_show(a),
        cli::entry::EntryCommand::Edit(a) => cli::entry::run_edit(a),
    }
}
