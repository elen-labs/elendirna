# Elendirna

> The history of your thinking stays with you when the model changes.

[![crates.io](https://img.shields.io/crates/v/eln-cli.svg)](https://crates.io/crates/eln-cli) [![Rust](https://github.com/elen-labs/elendirna/actions/workflows/rust.yml/badge.svg)](https://github.com/elen-labs/elendirna/actions/workflows/rust.yml)

[English](README.md) | [한국어](README.ko.md)

Elendirna is a knowledge vault for the history behind your ideas. You write down a base, and each time your thinking changes or grows you add a revision. An AI client connected to the vault sees not just your current conclusion but the path that led to it, whichever model or service you happen to be using that day.

Elendirna is an independent project inspired by [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF). It is not ELF's official implementation or successor.

## Why keep a history of thinking?

A chat stays with the model or service where it happened. Notes keep conclusions, and the reasoning behind them tends to fall away. Elendirna keeps entries and their revisions in a vault you own, so you or an AI tool can pick the work up later.

It is not a replacement for Git or for chat logs. Git tracks changes to files. Elendirna records why a decision was made, how it changed, and which other entries it relates to. What goes in, and what gets shared, is up to you.

## Install

### npm (prebuilt binary, no Rust toolchain)

```bash
npx -y elendirna --help       # one-off
npm install -g elendirna      # installs the `elendirna` command
elendirna --help
```

The npm package bundles prebuilt `elf` binaries for Linux, macOS, and Windows on x64 and arm64. There is no `postinstall` script, and nothing is fetched from the network at install or run time. Linux binaries need glibc 2.35 or newer. On musl/Alpine, install with Cargo instead.

### Cargo

```bash
cargo install eln-cli
```

This installs the `elf` executable. The Rust workspace has three crates: `eln-cli` (the CLI), `eln-core` (vault and MCP runtime), and `eln-plugin-sdk` (the Rust plugin interface, published separately). The older `elendirna` crate is the pre-split 0.6.x line.

## Quick start

The commands below use the `elf` executable installed by Cargo. With npm, replace `elf` with `elendirna`, or prefix each command with `npx -y elendirna`.

```bash
elf init my-vault
cd my-vault
elf entry new "Rust ownership"
elf entry new "Borrow checker"
elf revision add N0001 --delta "The hard part was not the syntax. It was deciding who owns the data."
elf link N0001 N0002
elf entry status N0001 stable
elf entry status N0002 stable
elf validate
```

A revision records a change in thinking. It is not the place for a changelog line such as "added notes". The delta above captures where the focus of the entry moved, and that one line is what a later reader, human or AI, actually needs.

`elf entry show N0001` prints an entry. `elf entry edit N0001` opens its note in your editor when one is configured. `elf guide` prints the full command surface and the intended AI workflow, and `elf <command> --help` shows the options of a single command.

## Work with an AI client

Run the MCP server over stdio and point it at the vault you want to share:

```bash
elf serve --mcp --vault /path/to/vault
```

`elf serve --snippet` prints a configuration snippet for your MCP client. Without `--vault`, the vault is chosen in this order: `--global`, `ELF_VAULT`, a vault found from the working directory, then the configured global vault. Connect an AI client only to a vault whose contents you intend to share with it.

Once connected, ask the client something that needs the history rather than the conclusion:

> In the Rust ownership entry, when did the focus move from syntax to data ownership, and why?

The client fetches the entry, its revision chain, and the linked entries through the vault's `bundle` tool and answers from what was recorded. Connect a different model tomorrow and it reads the same history.

## What is stored

A vault is a directory of plain files. Each entry is a folder holding a `manifest.toml` with structured metadata, a `note.md` with the human-readable text, and an `attachments/` folder. Revisions live under `revisions/<entry id>/` as one Markdown file per delta, and links between entries are recorded in the manifest. The SQLite index is derived data, and `elf validate` rebuilds it. Session handoffs between AI agents are appended to `sync.jsonl`.

The repository ships a small demo vault at [`eln-core/tests/fixtures/demo_vault`](eln-core/tests/fixtures/demo_vault/README.md). The project itself is developed with a private working vault, and that vault is not part of this repository. Material moves from the private vault into public code, documentation, fixtures, or releases only after it has been reviewed. If you run a working vault of your own, the same boundary is worth keeping.

## License and attribution

The software is MIT-licensed. See [LICENSE](LICENSE).

Protocol concepts (Base-Delta, SSoT, and Session-Trial identifiers) come from [ELF (Eli's Lab Framework)](https://github.com/ProjectEli/ELF) by Eli (projectschnee@gmail.com), licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
