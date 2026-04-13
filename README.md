# Elendirna

> Agent-friendly knowledge base CLI. Inspired by [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF).

![crates.io](https://img.shields.io/crates/v/elendirna.svg) [![Rust](https://github.com/RainyLens/elendirna/actions/workflows/rust.yml/badge.svg)](https://github.com/RainyLens/elendirna/actions/workflows/rust.yml)

개인 지식을 **Base-Delta** 구조로 보존합니다. AI 에이전트가 컨벤션을 기억하지 않아도 되도록, CLI가 규칙을 강제합니다.

## Install

```bash
cargo install elendirna
```

## Quick start

```bash
elf init my-vault
cd my-vault
elf entry new "Rust ownership"
elf entry edit N0001
elf revision add N0001 --delta "borrow checker 관련 내용 추가"
elf link N0001 N0002
elf validate
```

## Vault layout

```
vault/
├── .elendirna/
│   ├── config.toml          # vault 설정, schema_version
│   ├── index.sqlite         # 파생 인덱스 (재생성 가능, .gitignore)
│   └── sync.jsonl           # AI handoff append-only log
├── entries/
│   └── N0042_rust_ownership/
│       ├── manifest.toml    # 구조화 메타데이터
│       ├── note.md          # 사람용 narrative (frontmatter 포함)
│       └── attachments/
├── assets/                  # immutable 원본 (CLI가 mutation 거부)
├── revisions/
│   └── N0042/
│       ├── r001.md          # delta-only
│       └── r002.md
├── CLAUDE.md                # Claude Code agent manifest
├── GEMINI.md                # Gemini CLI agent manifest
├── AGENTS.md                # Codex / OpenAI Agents manifest
└── README.md
```

`demo_vault/`에서 실제 사용 예시를 확인할 수 있습니다.

## Commands

| 명령 | 역할 |
|------|------|
| `elf init [path]` | vault 스캐폴드 생성 |
| `elf entry new <title>` | entry 생성 (ID 자동 채번) |
| `elf entry edit <id>` | `$EDITOR`로 note.md 편집 |
| `elf entry show <id>` | manifest + note 출력 |
| `elf revision add <id> --delta <text>` | base-delta 로깅 |
| `elf link <from> <to>` | 양방향 cross-ref 추가 |
| `elf bundle <id>` | baseline → revision 체인 export (AI 컨텍스트용) |
| `elf validate` | 스키마, dangling link, cycle 검증 |
| `elf graph [--format dot\|mermaid\|json]` | 의존 그래프 export |
| `elf query <expr>` | sqlite 인덱스 기반 검색 |
| `elf sync record --summary <text>` | AI 세션 요약 기록 |
| `elf sync log [--tail N]` | AI handoff 로그 조회 |
| `elf serve --mcp` | MCP 서버 구동 (stdio, Claude Desktop 등) |

전역 플래그: `--json`, `--dry-run`

MCP 서버 vault 경로 우선순위: `--vault` 플래그 → `ELF_VAULT` 환경변수 → CWD walk-up

## Dogfooding

Elendirna는 **자기 자신을 Elendirna로 관리**합니다.

이 프로젝트의 설계 결정, 제안(Proposal), 시나리오, 철학적 논의는 모두 `.elendirna/` vault에 Base-Delta 구조로 기록되어 있습니다. `elf bundle`로 꺼내보면 "왜 이 기능이 이렇게 만들어졌는지"의 계보를 따라갈 수 있습니다.

## Try it with AI

MCP 서버를 세팅한 뒤, AI에게 이렇게 말해보세요:

> "이 vault의 유지보수 이력을 알려줘"

```bash
# Claude Desktop / 다른 MCP 클라이언트 설정
elf serve --mcp --vault /path/to/elendirna
```

AI는 `sync_record` 로그와 revision chain을 통해 프로젝트가 어떤 결정을 거쳐 지금 모습이 됐는지 컨텍스트를 복원합니다.

## License

MIT — see [LICENSE](LICENSE).

Protocol concepts (Base-Delta, SSoT, Session-Trial identifiers) are derived from
[ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF) by Eli (projectschnee@gmail.com),
licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
