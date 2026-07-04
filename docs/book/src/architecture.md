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

The extension should remain thin and delegate behavior to shared core logic through the CLI or a future bridge.

## Core Modules

The current core is centered around these areas:

- `app`: `FrilVault` facade that opens a workspace and constructs services
- `note`: note entities, DTOs, repository, and service
- `workspace`: workspace entities, path resolution, indexing, diffing, repository, and service
- `runtime`: `VaultContext` and `NoteCache`
- `parser`, `error`, `constants`: support modules

## VaultContext

`VaultContext` is the runtime boundary inside `frilvault-core`.

It owns:

- `NoteRepository`
- `WorkspaceIndexRepository`
- `NoteCache`

Services should use `VaultContext` instead of reaching into repositories directly. That keeps cache behavior, repository access, and future runtime policy centralized.

## Architectural Rules

- all business logic lives in `frilvault-core`
- source files are never modified by FrilVault
- the `.vault` directory is the storage boundary
- caches are runtime concerns, not client concerns

See the diagram pages for concrete system views:

- [System Diagram](diagrams/system.md)
- [Runtime Diagram](diagrams/runtime.md)
- [Editor Diagram](diagrams/editor.md)
