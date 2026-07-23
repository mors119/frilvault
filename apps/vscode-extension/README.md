# FrilVault VS Code Extension

VS Code integration for FrilVault.

## Knowledge Layer Foundation

Epic #123 introduces shared current-file note state for the VS Code extension.
The sidebar, gutter decorations, and hover preview all read from one
`CurrentFileNotesStore` cache instead of issuing separate CLI queries.

Note mutations call a single invalidation path so every affected view refreshes
together after add, sync, or repair operations.

## Multi-root Workspaces

FrilVault currently targets one workspace root at a time:

- By default it uses the first folder in a multi-root workspace.
- Set `frilvault.workspaceRoot` to pin FrilVault to a specific folder when
  several roots are open.
- Files outside the selected root are ignored safely and do not trigger vault
  creation.
- Each workspace root keeps its own enable/disable preference.

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
├── current-file
├── decorations
├── enablement
├── hover
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
- notes view registration idempotency and disposal

## Extension Development Host

When debugging this extension, disable or uninstall the marketplace FrilVault
extension first. VS Code cannot register the same contributed view id
(`frilvault.notes`) twice, so running the development extension alongside an
installed copy produces repeated errors such as:

```text
Cannot register multiple views with same id `frilvault.notes`
```

After changing extension code, reload the Extension Development Host window so
activation and disposables run cleanly. Shared view and command identifiers
live in `src/constants/ids.ts` and must stay aligned with `package.json`.

## Notes

- line-note UX is the most complete path today
- symbol anchors exist in the shared model, but editor UX around them is still limited

## Hover and Clipboard

FrilVault renders note hovers through a single registered hover provider. Gutter
and inline decorations do not attach duplicate hover messages, so each note
appears once when you hover editor content.

Note content and interactive action links are built as separate Markdown
sections. FrilVault copy commands (`Copy Link`, `Copy Content`, `Copy Markdown`)
write only the intended payload and never include hover action labels.

VS Code controls manual text selection inside the native hover. If you select
and copy the entire hover yourself, action link text may be included. FrilVault
does not patch that native behavior; use the provided copy commands for clean
clipboard output.
