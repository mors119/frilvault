# FrilVault VS Code Extension Quickstart

## Run the Extension

1. Install Rust and Node.js.
2. Open `apps/vscode-extension` in VS Code.
3. Run `npm run compile`.
4. Press `F5` to launch an Extension Development Host.
5. Run `FrilVault: Turn On` inside the test workspace.

## Configure the CLI

Marketplace and target-specific VSIX builds bundle `flvt` automatically.

Set `frilvault.cliPath` only when you want to override the bundled binary:

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
- all extension flows use the bundled or overridden `flvt` CLI through `CliClient`
