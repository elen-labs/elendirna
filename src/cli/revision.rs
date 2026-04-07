use clap::Args;
use crate::error::ElfError;
use crate::vault::{self, id::EntryId};
use crate::vault::entry::Entry;
use crate::vault::revision::Revision;

#[derive(Debug, Args)]
pub struct RevisionArgs {
    #[command(subcommand)]
    pub command: RevisionCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum RevisionCommand {
    /// revision 추가
    Add(AddArgs),
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// entry ID (예: N0001)
    pub id: String,

    /// delta 내용 (생략 시 stdin에서 읽음, Q2)
    #[arg(long)]
    pub delta: Option<String>,

    /// dry-run (fix-003)
    #[arg(long)]
    pub dry_run: bool,

    /// JSON 출력
    #[arg(long)]
    pub json: bool,
}

pub fn run(args: RevisionArgs) -> Result<(), ElfError> {
    match args.command {
        RevisionCommand::Add(a) => run_add(a),
    }
}

fn run_add(args: AddArgs) -> Result<(), ElfError> {
    let cwd = std::env::current_dir()?;
    let vault_root = vault::find_vault_root(&cwd)?;

    // entry 존재 확인
    let entry_id = EntryId::from_str(&args.id).ok_or_else(|| ElfError::InvalidInput {
        message: format!("'{}' 는 유효한 entry ID가 아닙니다 (예: N0001)", args.id),
    })?;
    let mut entry = Entry::find_by_id(&vault_root, &entry_id)
        .ok_or_else(|| ElfError::NotFound { id: args.id.clone() })?;

    // delta 수집: --delta 플래그 → stdin (Q2)
    let delta = match args.delta {
        Some(d) => d,
        None => {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf.trim_end().to_string()
        }
    };

    // 빈 delta 거부
    if delta.trim().is_empty() {
        return Err(ElfError::InvalidInput {
            message: "--delta 또는 stdin으로 delta 내용을 제공하세요".to_string(),
        });
    }

    // dry-run
    if args.dry_run {
        let rev_dir = Revision::rev_dir(&vault_root, &entry_id);
        let next_id = crate::vault::id::RevisionId::next(&rev_dir)?;
        println!("-- dry-run: 실제로 생성되지 않습니다 --");
        println!("  [create] revisions/{}/{next_id}.md", entry_id);
        println!("  [update] entries/…/manifest.toml  (updated 갱신)");
        println!("  [append] .elendirna/sync.jsonl");
        return Ok(());
    }

    let rev = Revision::create(&vault_root, &entry_id, &delta)?;

    // manifest updated 갱신 (Q3: revision.add)
    entry.manifest.touch_and_write(&entry.dir)?;
    crate::vault::util::append_sync_event(&vault_root, "revision.add", Some(&entry_id.to_string()))?;

    if args.json {
        let out = serde_json::json!({
            "command": "revision.add",
            "ok": true,
            "data": {
                "entry_id": entry_id.to_string(),
                "rev_id":   rev.rev_id.to_string(),
                "baseline": rev.baseline.to_string(),
                "created":  rev.created.to_rfc3339(),
            }
        });
        println!("{out}");
    } else {
        println!("✓ revision 추가: {} / {}", entry_id, rev.rev_id);
        println!("  baseline: {}", rev.baseline);
    }

    Ok(())
}

