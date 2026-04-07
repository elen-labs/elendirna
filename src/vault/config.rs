use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::ElfError;

pub const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    pub schema_version: u32,
    pub vault_name: String,
    pub created: DateTime<Utc>,
    /// 편집기 명령. 기본값 "$EDITOR" (환경 변수 참조)
    pub editor: String,
}

impl VaultConfig {
    pub fn new(vault_name: impl Into<String>) -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            vault_name: vault_name.into(),
            created: Utc::now(),
            editor: "$EDITOR".to_string(),
        }
    }

    pub fn read(vault_root: &Path) -> Result<Self, ElfError> {
        let path = vault_root.join(".elendirna").join("config.toml");
        let raw = std::fs::read_to_string(&path)?;
        toml::from_str(&raw).map_err(|e| ElfError::ParseError {
            message: format!("config.toml 파싱 실패: {e}"),
        })
    }

    pub fn write(&self, vault_root: &Path) -> Result<(), ElfError> {
        let dir = vault_root.join(".elendirna");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("config.toml");
        let content = toml::to_string_pretty(self).map_err(|e| ElfError::ParseError {
            message: format!("config.toml 직렬화 실패: {e}"),
        })?;
        crate::vault::util::atomic_write(&path, content.as_bytes())?;
        Ok(())
    }

    /// 편집기 실행 명령 반환 ($EDITOR 또는 config 필드)
    pub fn resolve_editor(&self) -> Option<String> {
        if self.editor == "$EDITOR" {
            std::env::var("EDITOR").ok()
        } else {
            Some(self.editor.clone())
        }
    }
}
