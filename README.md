# Elendirna

> Keep your thinking continuous across models and services.
>
> 모델이 바뀌어도, 탐구의 기록과 작업 맥락을 이어갑니다.

[![crates.io](https://img.shields.io/crates/v/eln-cli.svg)](https://crates.io/crates/eln-cli) [![Rust](https://github.com/elen-labs/elendirna/actions/workflows/rust.yml/badge.svg)](https://github.com/elen-labs/elendirna/actions/workflows/rust.yml)

Elendirna is a knowledge vault for carrying the history behind your ideas into AI conversations. It records a stable base and meaningful deltas, so an LLM can recover not only what you think now, but how that thinking developed. The same vault works across models and services; the history remains yours.

Elendirna는 아이디어를 탐구하며 쌓은 기록을 보관하는 vault입니다. 바탕이 되는 내용을 적고, 생각이 달라지거나 더해질 때마다 revision으로 남깁니다. 그래서 모델이나 서비스를 바꿔도 지금의 결론뿐 아니라 여기까지 온 과정까지 다시 살펴볼 수 있습니다.

Elendirna is an independent project inspired by [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF), not its official implementation or successor.

Elendirna는 [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF)에서 영감을 받은 독립 프로젝트이며, 공식 구현체나 후속 프로젝트는 아닙니다.

## Why keep a history of thinking? / 왜 생각의 이력을 남기나요?

Chats stay with the model or service where they happened. Notes save conclusions, but can lose the reasoning and changes behind them. Elendirna keeps entries and their revisions in your vault, so you and your AI tools can pick up the work later.

대화는 그때 사용한 모델이나 서비스에 남습니다. 노트에는 결론만 적고, 그 결론에 이른 과정은 빠뜨리기 쉽습니다. Elendirna는 생각과 변화의 기록을 vault에 모아 두어, 나중에 사람이나 AI 도구가 이어서 살펴볼 수 있게 합니다.

Think of it as a history for ideas—not a replacement for Git or chat history. Git tracks file changes; Elendirna records meaningful context and links between entries. You choose what to capture and what to share.

아이디어의 변화 과정을 기록하지만 Git이나 채팅 기록을 대신하지는 않습니다. Git은 파일의 변경을 추적하고, Elendirna는 생각과 결정의 맥락 및 entry 사이의 관계를 기록합니다. 무엇을 기록하고 공유할지는 사용자가 정합니다.

## Install / 설치

### npm (prebuilt binary; no Rust toolchain) / npm (사전 빌드 바이너리, Rust 불필요)

```bash
npx -y elendirna --help       # one-off
npm install -g elendirna      # installs the `elendirna` command
elendirna --help
```

The npm package bundles prebuilt `elf` binaries for Linux, macOS, and Windows on x64 and arm64. It has no `postinstall` script and does not need network access at install or run time. Linux binaries require glibc 2.35 or newer; on musl/Alpine, install with Cargo instead.

npm 패키지에는 Linux, macOS, Windows용 `elf` 실행 파일이 x64와 arm64 버전으로 포함되어 있습니다. 설치 후 실행 스크립트를 돌리지 않으며, 설치하거나 실행할 때 네트워크에 연결하지 않습니다. Linux 바이너리는 glibc 2.35 이상이 필요합니다. musl/Alpine에서는 Cargo로 설치하세요.

### Cargo

```bash
cargo install eln-cli
```

This installs the `elf` executable. The Rust workspace contains `eln-cli` (CLI), `eln-core` (vault and MCP runtime), and the separately published `eln-plugin-sdk` (Rust plugin interface). The older `elendirna` crate is for the pre-split 0.6.x line.

설치되는 실행 파일 이름은 `elf`입니다. Rust workspace는 CLI를 제공하는 `eln-cli`, vault와 MCP 서버를 담은 `eln-core`, 별도 배포되는 Rust plugin interface `eln-plugin-sdk`로 구성됩니다. `elendirna` crate는 패키지를 분리하기 전 0.6.x 버전입니다.

## Quick start / 빠른 시작

These commands use the `elf` executable installed by Cargo. With npm, replace `elf` with `elendirna`, or prefix each command with `npx -y elendirna`.

아래 명령은 Cargo로 설치한 `elf` 실행 파일 기준입니다. npm으로 설치했다면 `elf`를 `elendirna`로 바꾸거나, 각 명령 앞에 `npx -y elendirna`를 붙이세요.

```bash
elf init my-vault
cd my-vault
elf entry new "Rust ownership"
elf entry new "Borrow checker"
elf revision add N0001 --delta "Added notes on the borrow checker"
elf link N0001 N0002
elf entry status N0001 stable
elf entry status N0002 stable
elf validate
```

Use `elf entry show N0001` to read an entry. Edit its note with `elf entry edit N0001` when an editor is configured. `elf guide` prints the current command surface and AI workflow; `elf <command> --help` shows command options.

`elf entry show N0001`로 entry 내용을 확인하세요. 편집기가 설정되어 있다면 `elf entry edit N0001`로 note를 열어 수정할 수 있습니다. `elf guide`는 전체 명령과 AI 작업 흐름을 보여 주며, `elf <command> --help`에서는 각 명령의 옵션을 확인할 수 있습니다.

## Work with an AI client / AI 클라이언트 연결

Run the MCP server over stdio and point it at the vault you want to share:

공유할 vault를 지정해 stdio 방식으로 MCP 서버를 실행합니다.

```bash
elf serve --mcp --vault /path/to/vault
```

`elf serve --snippet` prints an MCP client configuration snippet. The vault selection precedence is `--vault`, `--global`, `ELF_VAULT`, then working-directory discovery and the configured global fallback. Only connect an AI client to a vault whose contents you intend to share with it.

`elf serve --snippet`은 MCP 클라이언트 설정 예시를 출력합니다. vault는 `--vault`, `--global`, `ELF_VAULT`, 현재 작업 디렉터리 탐색, 전역 vault 순으로 선택됩니다. AI 클라이언트에는 공유해도 되는 vault만 연결하세요.

## What is stored / 무엇을 저장하나요?

A vault keeps entries (structured metadata and human-readable notes), revisions (meaningful changes), links between entries, and optional attachments. SQLite indexes are derived and can be rebuilt by `elf validate`; the vault also records session handoffs in `sync.jsonl`.

vault에는 entry(메타데이터와 본문), revision(변경 기록), entry 간 링크, 선택적 첨부 파일이 들어갑니다. SQLite 인덱스는 다시 만들 수 있는 파생 데이터이며 `elf validate`가 재생성합니다. 세션 인계 기록은 `sync.jsonl`에 남습니다.

The repository includes a small demo vault at [`eln-core/tests/fixtures/demo_vault`](eln-core/tests/fixtures/demo_vault/README.md). Do not use a private working vault as public repository data: promote only reviewed material into public code, documentation, fixtures, or releases.

작은 데모 vault([`eln-core/tests/fixtures/demo_vault`](eln-core/tests/fixtures/demo_vault/README.md))가 저장소에 포함되어 있습니다. 비공개 작업 vault를 공개 저장소에 복사하지 말고, 검토를 마친 내용만 코드, 문서, fixture, 릴리스에 반영하세요.

## License and attribution / 라이선스 및 저작자 표시

The software is MIT-licensed; see [LICENSE](LICENSE).

소프트웨어는 MIT 라이선스로 제공됩니다. 자세한 내용은 [LICENSE](LICENSE)를 참고하세요.

Protocol concepts (Base-Delta, SSoT, and Session-Trial identifiers) are derived from [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF) by Eli (projectschnee@gmail.com), licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).

Base-Delta, SSoT, Session-Trial 식별자 개념은 Eli (projectschnee@gmail.com)의 [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF)에서 유래했으며 [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) 라이선스를 따릅니다.
