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
    if vault_root.join(".elendirna").join("entries").exists() {
        vault_root.join(".elendirna")
    } else {
        vault_root.to_path_buf()
    }
}
