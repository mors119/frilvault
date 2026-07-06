# Getting Started

This book documents the current FrilVault repository and MVP behavior.

## Repository Areas

```text
apps/
├── frilvault-cli
└── vscode-extension

crates/
└── frilvault-core
```

- `crates/frilvault-core`: note model, repositories, services, runtime helpers
- `apps/frilvault-cli`: `flvt` binary
- `apps/vscode-extension`: current editor integration

## Local Development

### Rust

```bash
cargo test -p frilvault-core
cargo test -p frilvault-cli
```

### VS Code Extension

```bash
cd apps/vscode-extension
npm install
npm run compile
npm test
```

The extension currently depends on both the CLI and a native Node bridge.

## CLI Examples

Add a line note:

```bash
flvt add \
  --file src/main.rs \
  --line 10 \
  --column 5 \
  --content "parser needs cleanup"
```

Add a symbol note:

```bash
flvt add \
  --file src/main.rs \
  --symbol parse_config \
  --kind function \
  --content "shared entry point"
```

Read and search notes:

```bash
flvt list --file src/main.rs
flvt search parser
flvt search --file src/main.rs --format json
```

Workspace operations:

```bash
flvt stats
flvt doctor
flvt repair
flvt repair --apply
```

## Building This Book

Run these commands from the repository root:

```bash
cargo install mdbook --locked
cargo install mdbook-mermaid --locked
mdbook-mermaid install docs/book
mdbook build docs/book
```

To preview locally:

```bash
mdbook serve docs/book --open
```
