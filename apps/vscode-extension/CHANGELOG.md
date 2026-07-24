# Changelog

All notable changes to the FrilVault VS Code extension are documented here.

## 0.0.1 - 2026-07-23

### Added

- Initial VS Code Marketplace release
- Line-anchored and symbol-anchored notes
- Inline note creation and editing
- Current-file notes view
- Workspace note search
- Gutter actions for viewing, editing, deleting, and copying note links
- Local JSON-based persistence through the FrilVault CLI

### Fixed

- Auto-save race conditions during active typing
- Save serialization for overlapping inline editor writes
- IME-aware auto-save behavior for in-progress composition
- Stale save completion handling for newer drafts
- Bundle the `flvt` CLI into platform-specific VSIX packages
- Prefer the bundled CLI by default with a custom `frilvault.cliPath` override
- Replace generic CLI startup failures with actionable runtime errors and output-channel diagnostics

### Known limitations

- Targets one workspace root at a time in multi-root workspaces
- Early preview release
