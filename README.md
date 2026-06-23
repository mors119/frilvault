# FrilVault

> Personal notes for source code, without modifying source files

FrilVault is a local-first knowledge layer for developers. It stores personal notes in a `.vault` directory instead of writing them into production code.

## What It Does

- Attach notes to source files with line or symbol anchors
- Keep personal context out of the repository code
- Search notes across a workspace
- Inspect workspace stats and health
- Repair note files after file moves or renames
- Surface notes inside VS Code

## Repository Layout

```text
apps/
├── frilvault-cli
└── vscode-extension

crates/
├── frilvault-core
└── frilvault-node
```

- `frilvault-core`: domain logic, repositories, workspace services
- `frilvault-cli`: command-line interface for note and workspace operations
- `frilvault-node`: Node-API bridge for editor integrations
- `apps/vscode-extension`: VS Code UI layer

## Storage Model

```text
.vault/
├── notes/
├── cache/
├── index/
└── workspace.json
```

Notes are stored separately from source files. FrilVault does not modify the source code it annotates.

## CLI Examples

Add a note:

```bash
flvt add \
  --file src/main.rs \
  --line 10 \
  --column 5 \
  --content "parser needs cleanup"
```

List notes for one file:

```bash
flvt list --file src/main.rs
```

List notes as JSON:

```bash
flvt list --file src/main.rs --format json
```

Search notes:

```bash
flvt search parser
```

Workspace commands:

```bash
flvt stats
flvt doctor
flvt repair
flvt repair --apply
```

## VS Code Extension

Current extension scope:

- `FrilVault: Add Note` command
- `FrilVault Notes` side panel for the active editor
- Gutter decorations for line notes
- Edit and delete note flows
- Workspace search, stats, health, and repair commands

Current integration model:

- `Add Note`, notes panel, and gutter decorations use the CLI
- edit/delete/search/stats/health/repair still use the Node bridge

This is an in-progress architecture and will continue moving toward shared `frilvault-core` behavior across surfaces.

## Build

Rust:

```bash
cargo test -p frilvault-core
cargo test -p frilvault-node
```

VS Code extension:

```bash
cd apps/vscode-extension
npm run compile
npm test
```

## Status

Implemented:

- Core note CRUD
- Search
- Line and symbol anchors
- Workspace index, stats, health, repair
- Node bridge MVP
- VS Code extension MVP

Planned:

- Watcher and cache invalidation
- Richer symbol workflows
- Semantic search
- IntelliJ integration

See [docs/CURRENT_STATE.md](docs/CURRENT_STATE.md), [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), and [docs/ROADMAP.md](docs/ROADMAP.md).
