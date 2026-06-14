# FrilVault VS Code Extension Quickstart

## Run the Extension

1. Install Rust and Node.js.
2. Open `apps/vscode-extension` in VS Code.
3. Run `npm run compile`.
4. Press `F5` to launch an Extension Development Host.

## Configure the CLI

CLI-backed features need `flvt`.

If `flvt` is not on `PATH`, set:

```json
{
  "frilvault.cliPath": "/absolute/path/to/flvt"
}
```

## Useful Commands

- `FrilVault: Add Note`
- `FrilVault: Search Notes`
- `FrilVault: Show Workspace Stats`
- `FrilVault: Show Workspace Health`
- `FrilVault: Apply Repairs`

## Test

```bash
npm test
```

## Current Notes

- the side panel tracks the current active editor
- gutter decorations refresh when notes change or when the active editor changes
- some extension flows are still CLI-backed while others use `frilvault-node`
