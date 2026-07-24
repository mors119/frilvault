# FrilVault

FrilVault is a local-first developer knowledge layer that attaches persistent
notes to source code without modifying the source file.

## Features

- Add notes to source lines and symbols
- View notes directly inside VS Code
- Edit and delete notes from the editor
- Navigate between code and notes
- Search notes across the current workspace
- Store all note data locally as JSON
- Keep project knowledge inside `.vault`

## Requirements

FrilVault ships with a bundled `flvt` CLI inside each platform-specific VSIX.

Supported packaged targets:

- `darwin-arm64`
- `darwin-x64`
- `linux-x64`
- `win32-x64`

`frilvault.cliPath` is now an advanced override for custom builds.

## Getting Started

1. Install the FrilVault extension.
2. Open a project in VS Code.
3. Run `FrilVault: Turn On` for the workspace.
4. Run `FrilVault: Add Note`.
5. Enter a note in the inline editor.

## Commands

| Command | Description |
| --- | --- |
| `FrilVault: Add Note` | Add a note at the current line or symbol |
| `FrilVault: Show Notes for Current File` | Show notes for the active file |
| `FrilVault: Search Notes` | Search notes in the current workspace |
| `FrilVault: Show Workspace Stats` | Show workspace note statistics |
| `FrilVault: Show Workspace Health` | Show missing-file health information |
| `FrilVault: Apply Repairs` | Apply note repair suggestions for renamed or moved files |

## Data Storage

FrilVault stores project data locally under:

```text
.vault/
```

No cloud account is required.

## Known Limitations

- FrilVault targets one workspace root at a time, so multi-root workspace support is limited
- This is an early preview release

## Roadmap

- Bundle or simplify CLI installation for extension users
- Improve multi-root workspace behavior
- Expand editor UX for symbol-anchored notes

## Privacy

FrilVault does not upload source code or note content to an external service.

## Issues

Report bugs and feature requests through the GitHub issue tracker:

https://github.com/FrilLab/frilvault/issues

## License

MIT
