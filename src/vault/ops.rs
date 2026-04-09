/// MCP 서버와 CLI가 공유하는 고수준 vault 조작 함수.
/// 출력 로직 없음 — 호출자(CLI 핸들러 또는 MCP tool)가 결과를 직렬화한다.
use std::path::Path;
use crate::error::ElfError;
use crate::vault::entry::Entry;
use crate::vault::id::EntryId;
use crate::vault::revision::Revision;
use crate::vault::util::append_sync_event;

// ─── entry ───────────────────────────────

pub struct EntryNewResult {
    pub entry: Entry,
}

/// entry 생성. baseline 존재 확인 + slug 충돌 확인 포함.
pub fn entry_new(
    vault_root: &Path,
    title: &str,
    baseline: Option<&str>,
    tags: Vec<String>,
) -> Result<EntryNewResult, ElfError> {
    // baseline 존재 확인
    if let Some(b) = baseline {
        let entry_part = b.split('@').next().unwrap_or(b);
        let bid = EntryId::from_str(entry_part).ok_or_else(|| ElfError::InvalidInput {
            message: format!("baseline '{b}'의 entry ID 형식이 잘못됐습니다"),
        })?;
        if Entry::find_by_id(vault_root, &bid).is_none() {
            return Err(ElfError::NotFound { id: bid.to_string() });
        }
    }

    // slug 충돌 확인 (멱등성)
    if let Some(existing) = Entry::find_by_slug(vault_root, title) {
        return Err(ElfError::AlreadyExists { id: existing.manifest.id });
    }

    let entries_dir = vault_root.join("entries");
    let next_id = EntryId::next(&entries_dir)?;
    let entry = Entry::create(
        vault_root,
        next_id,
        title.to_string(),
        baseline.map(String::from),
        tags,
    )?;

    Ok(EntryNewResult { entry })
}

pub struct EntryShowResult {
    pub entry: Entry,
    pub note_body: String,
}

/// entry 조회 (manifest + note body).
pub fn entry_show(vault_root: &Path, id_str: &str) -> Result<EntryShowResult, ElfError> {
    let id = EntryId::from_str(id_str).ok_or_else(|| ElfError::InvalidInput {
        message: format!("'{id_str}' 는 유효한 entry ID가 아닙니다 (예: N0001)"),
    })?;
    let entry = Entry::find_by_id(vault_root, &id)
        .ok_or_else(|| ElfError::NotFound { id: id_str.to_string() })?;
    let note_body = entry.note_body()?;
    Ok(EntryShowResult { entry, note_body })
}

/// 전체 entry 목록 조회. 필터는 호출자가 적용.
pub fn entry_list(vault_root: &Path) -> Vec<Entry> {
    Entry::find_all(vault_root)
}

// ─── revision ────────────────────────────

pub struct RevisionAddResult {
    pub revision: Revision,
}

/// revision 추가. manifest updated 갱신 + sync.jsonl 기록 포함.
pub fn revision_add(
    vault_root: &Path,
    entry_id_str: &str,
    delta: &str,
) -> Result<RevisionAddResult, ElfError> {
    if delta.trim().is_empty() {
        return Err(ElfError::InvalidInput {
            message: "delta 내용이 비어 있습니다".to_string(),
        });
    }

    let entry_id = EntryId::from_str(entry_id_str).ok_or_else(|| ElfError::InvalidInput {
        message: format!("'{entry_id_str}' 는 유효한 entry ID가 아닙니다"),
    })?;
    let mut entry = Entry::find_by_id(vault_root, &entry_id)
        .ok_or_else(|| ElfError::NotFound { id: entry_id_str.to_string() })?;

    let revision = Revision::create(vault_root, &entry_id, delta)?;
    entry.manifest.touch_and_write(&entry.dir)?;
    append_sync_event(vault_root, "revision.add", Some(entry_id_str))?;

    Ok(RevisionAddResult { revision })
}

/// entry의 revision 목록 (시간순).
pub fn revision_list(vault_root: &Path, entry_id_str: &str) -> Result<Vec<Revision>, ElfError> {
    let entry_id = EntryId::from_str(entry_id_str).ok_or_else(|| ElfError::InvalidInput {
        message: format!("'{entry_id_str}' 는 유효한 entry ID가 아닙니다"),
    })?;
    Entry::find_by_id(vault_root, &entry_id)
        .ok_or_else(|| ElfError::NotFound { id: entry_id_str.to_string() })?;
    Ok(Revision::list(vault_root, &entry_id))
}

// ─── bundle ──────────────────────────────

pub struct LinkedEntry {
    pub entry: Entry,
    pub note_body: String,
}

pub struct BundleOutput {
    pub entry: Entry,
    pub note_body: String,
    pub revisions: Vec<Revision>,
    pub linked: Vec<LinkedEntry>, // depth=1 linked entries
}

/// entry + revision chain + linked entries(depth=1) 수집.
/// readable 합성은 호출자(CLI 출력 or MCP tool)가 담당.
pub fn bundle(vault_root: &Path, id_str: &str) -> Result<BundleOutput, ElfError> {
    let id = EntryId::from_str(id_str).ok_or_else(|| ElfError::InvalidInput {
        message: format!("'{id_str}' 는 유효한 entry ID가 아닙니다"),
    })?;
    let entry = Entry::find_by_id(vault_root, &id)
        .ok_or_else(|| ElfError::NotFound { id: id_str.to_string() })?;

    let note_body = entry.note_body().unwrap_or_default();
    let revisions = Revision::list(vault_root, &id);

    let mut linked = vec![];
    for link_id_str in &entry.manifest.links {
        if let Some(lid) = EntryId::from_str(link_id_str) {
            if let Some(le) = Entry::find_by_id(vault_root, &lid) {
                let lb = le.note_body().unwrap_or_default();
                linked.push(LinkedEntry { entry: le, note_body: lb });
            }
        }
    }

    Ok(BundleOutput { entry, note_body, revisions, linked })
}

// ─── link ────────────────────────────────

/// 양방향 링크 추가. 이미 존재하면 no-op.
pub fn link_add(
    vault_root: &Path,
    from_str: &str,
    to_str: &str,
) -> Result<bool, ElfError> { // true = 새로 추가됨, false = no-op
    if from_str == to_str {
        return Err(ElfError::InvalidInput {
            message: format!("자기 자신({from_str})에게 링크할 수 없습니다"),
        });
    }

    let from_id = EntryId::from_str(from_str).ok_or_else(|| ElfError::InvalidInput {
        message: format!("'{from_str}' 는 유효한 entry ID가 아닙니다"),
    })?;
    let to_id = EntryId::from_str(to_str).ok_or_else(|| ElfError::InvalidInput {
        message: format!("'{to_str}' 는 유효한 entry ID가 아닙니다"),
    })?;

    let mut from_entry = Entry::find_by_id(vault_root, &from_id)
        .ok_or_else(|| ElfError::NotFound { id: from_str.to_string() })?;
    let mut to_entry = Entry::find_by_id(vault_root, &to_id)
        .ok_or_else(|| ElfError::NotFound { id: to_str.to_string() })?;

    if from_entry.manifest.links.contains(&to_str.to_string()) {
        return Ok(false); // no-op
    }

    insert_sorted(&mut from_entry.manifest.links, to_str.to_string());
    insert_sorted(&mut to_entry.manifest.links, from_str.to_string());

    from_entry.manifest.touch_and_write(&from_entry.dir)?;
    to_entry.manifest.touch_and_write(&to_entry.dir)?;

    let event = format!("link.{from_str}.{to_str}");
    append_sync_event(vault_root, &event, None)?;

    Ok(true)
}

fn insert_sorted(vec: &mut Vec<String>, item: String) {
    if vec.contains(&item) { return; }
    let pos = vec.partition_point(|x| x < &item);
    vec.insert(pos, item);
}
