# Architecture

FrilVault is organized around a shared Rust core and thin integration surfaces.

## Repository Structure

```text
frilvault/
├── crates/
│   └── frilvault-core
└── apps/
    ├── frilvault-cli
    └── vscode-extension
```

## Responsibility Boundaries

### `frilvault-core`

`frilvault-core` contains the application rules and data model:

- note CRUD and search
- workspace statistics and health checks
- repair suggestions and repair application
- repository access to `.vault`
- runtime coordination through `VaultContext`

### `frilvault-cli`

The CLI is an application surface over the core:

- parses commands
- opens the workspace
- calls `FrilVault`
- prints human or JSON output

It should not reimplement business rules from the core crate.

### `apps/vscode-extension`

The VS Code extension is a UI integration layer:

- commands
- notes panel
- hover and gutter decorations
- CLI invocation through `CliClient`
- native calls through `NodeBridge`

The extension is currently hybrid. CLI-backed and Node-bridge-backed paths coexist in the MVP.

## Core Modules

The current core is centered around these areas:

- `app`: `FrilVault` facade that opens a workspace and constructs services
- `note`: note entities, DTOs, repository, and service
- `workspace`: workspace entities, path resolution, indexing, diffing, repository, and service
- `runtime`: `VaultContext` and `NoteCache`
- `parser`, `error`, `constants`: support modules

## Current Boundaries

The runtime boundary exists, but the architecture is not fully centralized yet.

- `NoteService` uses `VaultContext` for cache-aware note loading, but still writes through the repository stored inside the context.
- `WorkspaceService` rebuilds indexes through both `VaultContext` and a direct `WorkspaceIndexRepository` field.
- Search reads currently enumerate persisted note files instead of reusing the in-memory note cache.

That means the project already has a clear runtime container, but not every flow has been fully routed through it yet.

## VaultContext

`VaultContext` currently owns:

- `NoteRepository`
- `WorkspaceIndexRepository`
- `NoteCache`

Its main role today is to keep note-loading cache behavior and workspace-level helpers in one place.

## Architectural Rules

- all business logic lives in `frilvault-core`
- source files are never modified by FrilVault
- the `.vault` directory is the storage boundary
- caches are runtime concerns, not client concerns
- editor integrations should stay thin and reuse shared core behavior

See the diagram pages for concrete system views:

- [System Diagram](diagrams/system.md)
- [Runtime Diagram](diagrams/runtime.md)
- [Editor Diagram](diagrams/editor.md)
