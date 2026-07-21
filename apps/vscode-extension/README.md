# FrilVault VS Code Extension

VS Code integration for FrilVault.

## Current Scope

- `FrilVault: Add Note`
- `FrilVault Notes` side panel for the active editor
- gutter decorations for line notes
- note open from the side panel
- note edit and delete flows
- search notes
- workspace stats
- workspace health
- repair apply

## Current Integration Model

The extension currently uses the `flvt` CLI as its active backend:

- add note
- notes panel
- gutter decorations
- search
- stats
- health
- repair

Native bridge scaffolding still exists in the repo, but the current shipped command path is CLI-backed.

## Feature Structure

```text
src/features
├── add-note
├── decorations
└── notes-panel
```

## Requirements

- Rust toolchain with `cargo`
- Node.js
- VS Code
- an accessible `flvt` binary for CLI-backed features

If `flvt` is not on `PATH`, set `frilvault.cliPath` in VS Code settings.

## Build

```bash
npm run compile
```

This builds:

1. the extension bundle at `dist/extension.js`
2. `dist/frilvault.node` only when the optional `frilvault-node` crate exists

## Test

```bash
npm test
```

Current integration tests cover:

- CLI JSON parsing
- active-editor notes panel behavior
- add note command execution and refresh behavior

## Notes

- line-note UX is the most complete path today
- symbol anchors exist in the shared model, but editor UX around them is still limited
