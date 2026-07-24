# FrilVault

Personal notes for source code, without modifying source files.

FrilVault is a local-first workspace knowledge layer. Notes live under `.vault/` beside the project, while application code stays untouched.

## What It Supports

- line-anchored and symbol-anchored notes
- note attachments
- workspace search
- workspace explorer output
- workspace stats and health checks
- note URI resolution
- workspace sync and gitignore helper flows
- note-file repair after file moves or renames
- VS Code integration with notes panel, gutter markers, hover preview, inline editing, and CodeLens

## Repository Layout

```text
.
├── AGENTS.md
├── apps/
│   ├── frilvault-cli
│   └── vscode-extension
├── crates/
│   └── frilvault-core
└── docs/
```

- `crates/frilvault-core` contains the domain logic, persistence boundaries, symbol helpers, URI helpers, and workspace services.
- `apps/frilvault-cli` is the main executable surface for local workflows and JSON consumers.
- `apps/vscode-extension` contains the current editor integration.
- `docs/` contains architecture, workflow, testing, and release guidance.

The current checkout contains a CLI and a VS Code extension. Repository guidance still reserves room for a future desktop application, but there is no desktop application source tree in the current layout.

## Current Release Status

As of July 23, 2026, the Rust workspace is passing its format, lint, test, and release-build checks.

The repository is not yet release-ready as a whole because the VS Code extension still has open release blockers:

- a post-save `.gitignore` prompt failure can be surfaced to the user as if note persistence failed
- malformed FrilVault note URIs can still escape the extension's normal error handling path
- CodeLens path matching is brittle for nested configured roots and Windows-style paths
- `npm test` currently aborts with `SIGABRT` under `vscode-test`

## Storage Model

```text
.vault/
├── notes/
├── index/
└── workspace.json
```

FrilVault does not rewrite or annotate source files.

## Quick Commands

```bash
flvt add --file src/main.rs --line 10 --column 5 --content "parser needs cleanup"
flvt explorer --format json
flvt list --file src/main.rs
flvt search parser
flvt resolve-uri "frilvault://note/..."
flvt stats
flvt doctor
flvt repair --apply
```

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Contributing](docs/CONTRIBUTING.md)
- [Repository Rules](AGENTS.md)
- [GitHub Workflow](docs/github-workflow.md)
- [Testing](docs/testing.md)
- [Release Documents](docs/RELEASES/README.md)

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace --release
```

```bash
cd apps/vscode-extension
npm install
npm run check-types
npm run lint
npm run compile
npm test
```

Use `npm run check-types`, not `npm run typecheck`; that script does not exist in the current extension package.
