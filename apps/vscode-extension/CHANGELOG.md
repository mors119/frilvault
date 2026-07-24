# Changelog

All notable changes to the FrilVault VS Code extension are documented here.

## 0.0.2 - 2026-07-24

### Added

- Platform-specific VSIX packages for `darwin-arm64`, `darwin-x64`, `linux-x64`, and `win32-x64`
- GitHub Release assets for manual VSIX installation

### Changed

- Bundle the `flvt` CLI into each platform-specific VSIX package
- Prefer the bundled CLI by default with a custom `frilvault.cliPath` override
- Publish GitHub Release assets and Marketplace packages through separate workflows

### Fixed

- Note creation on fresh installs that did not already have `flvt` on the system path
- Generic CLI startup failures now surface actionable runtime errors and output-channel diagnostics

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

### Known limitations

- Targets one workspace root at a time in multi-root workspaces
- Early preview release
