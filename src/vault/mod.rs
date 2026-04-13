pub mod config;
pub mod entry;
pub mod id;
pub mod index;
pub mod ops;
pub mod revision;
pub mod util;

#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};
use crate::error::ElfError;

/// 현재 디렉터리에서 상위로 올라가며 `.elendirna/config.toml`을 탐색.
/// 탐색 실패 시 글로벌 vault(`~/.elendirna/config.toml`)로 폴백.
pub fn find_vault_root(start: &Path) -> Result<PathBuf, ElfError> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".elendirna").join("config.toml").exists() {
            return Ok(current);
        }
        if !current.pop() {
            break;
        }
    }
    // 글로벌 vault 폴백: %USERPROFILE% 또는 $HOME
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .ok();
    if let Some(home) = home {
        if home.join(".elendirna").join("config.toml").exists() {
            return Ok(home);
        }
    }
    Err(ElfError::NotAVault)
}

/// v2(compact) 레이아웃 여부를 filesystem으로 감지하여 데이터 루트를 반환.
/// `.elendirna/entries/` 존재 → v2: `.elendirna/` 반환
/// 없음 → v1: `vault_root` 그대로 반환 (기존 vault, test temp dir 호환)
pub fn data_root(vault_root: &Path) -> PathBuf {
    let meta = metadata_root(vault_root);
    if meta.join("entries").exists() {
        meta
    } else {
        vault_root.to_path_buf()
    }
}

/// 메타데이터 디렉터리(.elendirna) 경로 반환.
/// vault_root가 이미 .elendirna를 가리키고 있으면 그대로 반환, 아니면 join(".elendirna").
pub fn metadata_root(vault_root: &Path) -> PathBuf {
    if vault_root.file_name().and_then(|n| n.to_str()) == Some(".elendirna") {
        vault_root.to_path_buf()
    } else {
        vault_root.join(".elendirna")
    }
}

/// 제공된 경로가 .elendirna 내부라면 부모를, 아니면 그대로 반환하여 vault_root를 정규화.
pub fn normalize_vault_root(path: PathBuf) -> PathBuf {
    if path.file_name().and_then(|n| n.to_str()) == Some(".elendirna") {
        path.parent().map(|p| p.to_path_buf()).unwrap_or(path)
    } else {
        path
    }
}
