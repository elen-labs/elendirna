# Changelog

## [0.3.2] — 2026-04-13

### 기능
- MCP 서버 시작 시 vault가 없으면 `~/.elendirna/` 전역 vault 자동 초기화
- MCP 서버 시작 시 v1 vault를 v2 compact layout으로 자동 마이그레이션

### 버그 수정
- `os error 3`: vault_root가 `.elendirna`를 직접 가리킬 때 경로 정규화 누락 수정
- `atomic_write` PID 기반 임시 파일로 동시 쓰기 충돌 방지
- SQLite WAL 모드 및 `busy_timeout` 설정으로 동시 접근 안정성 향상

### 문서
- Proposal 003: MCP 서버 자동 설정
- Proposal 004: 첨부파일 지원 및 무결성 검사
- README에 dogfooding 섹션 추가

---

## [0.3.1] — 2026-04-13

> v0.3.0 릴리스 직후 Cargo.toml 버전 조정 (0.3.0 → 0.3.1). 기능 변경 없음.

---

## [0.3.0] — 2026-04-13

### 주요 변경 (breaking)
- **Compact Layout (schema v2)**: `entries/`, `revisions/`, `assets/` 디렉터리를 `.elendirna/` 하위로 이동
  - 기존 v1 vault는 폴백으로 그대로 동작 (하위 호환 유지)
  - `data_root()`: `.elendirna/entries/` 존재 여부로 v1/v2 자동 판단
- `CURRENT_SCHEMA_VERSION`: 1 → 2

### 기능
- `elf migrate`: v1 → v2 compact layout 이관 커맨드 (`--dry-run` 지원)
- `elf init --global`: 홈 디렉터리(`~/.elendirna/`)에 전역 vault 초기화
- `find_vault_root`: cwd 상위 탐색 실패 시 `~/.elendirna/` 폴백

### 기타
- 전체 테스트 63개 통과
- 프로젝트 자체 vault도 v2로 migrate 완료

---

## [0.2.4] — 2026-04-10

### 변경
- `thiserror` 1 → 2 업그레이드 반영

### 버그 수정
- Windows CRLF 관련 파싱 버그 수정 (v0.3 브레인스토밍 세션 중 발견)

### 내부
- v0.3 설계 결정 사항 브레인스토밍 및 dogfooding 세션
- MCP 서버 자동 설정 Proposal 초안 작성
- 통합 테스트 추가

---

## [0.2.3] — 2026-04-09

### 버그 수정
- MCP 응답의 `outputSchema`에 `type: "object"` 강제 지정하여 MCP spec 준수

---

## [0.2.0] — 2026-04-09

### 주요 기능
- **MCP 서버** (`elf serve --mcp`): AI 에이전트가 vault를 직접 조작할 수 있는 JSON-RPC over stdio 서버
  - 제공 tool: `entry_list`, `entry_show`, `entry_new`, `revision_add`, `bundle`, `query`, `sync_record`, `validate`
- **SQLite 인덱스** + **`elf query`**: tag, status, title_contains, baseline 기반 전문 검색
- **`elf bundle`**: entry + revision delta chain + 링크된 entry 전체를 하나의 컨텍스트로 수집
- **sync record** (`sync.jsonl`): 세션 요약을 append-only로 기록하는 세션 로그
- **`ELF_VAULT` 환경변수**: 전역/폴더별 vault 경로 명시적 지정 지원
- Gemini CLI / Codex 대응 agent 안내 파일 (`demo_vault`)

### 인프라
- GitHub Actions CI 워크플로우 추가 (Rust build + test)
- v0.2 설계 문서 작성 (`DESIGN.md`, MCP 서버 통합 원칙)
- v0.2 milestone 문서 `done/`으로 이동

---

## [0.1.0] — 2026-04-07

### 초기 구현

**CLI 커맨드 (Phase 0~8 완전 구현)**

| 커맨드 | 설명 |
|---|---|
| `elf init` | vault 초기화 (`--dry-run`, `CLAUDE.md` 자동 생성, `git add -f`) |
| `elf entry new` | entry 생성 (slug 충돌 멱등성, `--baseline`, stdin 지원) |
| `elf entry show` | manifest + note 출력 (`--json`: 본문만) |
| `elf entry edit` | `$EDITOR` 호출 + frontmatter → manifest 역반영 |
| `elf revision add` | revision 추가 (`--delta` 또는 stdin 파이프) |
| `elf link` | 양방향 링크 (원자적 쓰기, 정렬 유지) |
| `elf validate` | 7단계 검사 (Naming / Schema / Consistency / Dangling / Cycle / Orphan / Asset) |

**테스트 구조**
- 단위 테스트: `src/cli/tests.rs`, `src/vault/tests.rs`, `src/schema/tests.rs`
- 통합 테스트: `tests/integration.rs` (45 tests, all pass)
