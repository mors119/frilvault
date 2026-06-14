# FrilVault Current State

## Implemented

### Core

- Add note
- Update note
- Delete note
- List notes
- Search notes
- Line anchors
- Symbol anchors
- YAML note storage
- Workspace metadata
- Workspace index
- Workspace statistics
- Workspace health check
- Repair suggestions
- Automatic repair apply

### CLI

Available commands:

```bash
flvt add
flvt list
flvt update
flvt delete
flvt search
flvt stats
flvt doctor
flvt repair
```

Notable current behavior:

- `flvt list` supports `--format json`
- repair works with filename-based matching

### Node Bridge

`frilvault-node` is implemented as a Node-API wrapper around `frilvault-core`.

Currently exposed operations:

- add line note
- list notes
- update note
- delete note
- search notes
- workspace stats
- workspace health
- repair suggestions
- apply repairs

### VS Code Extension

Status:

MVP implemented.

Current scope:

- `FrilVault: Add Note`
- `FrilVault Notes` side panel
- gutter decorations for line notes
- note open from TreeView
- note edit flow
- note delete flow
- search notes
- workspace stats
- workspace health
- repair apply

Current implementation split:

- CLI-backed:
  - add note
  - notes side panel
  - gutter decorations
- Node-bridge-backed:
  - edit note
  - delete note
  - search
  - stats
  - health
  - repair

### Tests

Implemented test coverage:

- `frilvault-core` unit tests
- `frilvault-node` compile-level test pass
- VS Code integration tests for:
  - add note command
  - notes panel
  - CLI JSON parsing path

## Refactoring Completed

### Core Domain Split

Separated:

- note entities
- note repository
- note service
- workspace entities
- workspace repositories
- workspace service

### VS Code Feature Split

Separated:

- `features/add-note`
- `features/notes-panel`
- `features/decorations`

## In Progress

### Editor Integration Cleanup

Goal:

- reduce mixed CLI and Node bridge usage
- move toward a more consistent integration boundary

## Not Yet Implemented

### Watcher

Status:

Not started.

### Rich Symbol Workflows

Status:

Not started.

Current limitation:

- symbol anchors exist in the core model
- the VS Code UI is still primarily line-note oriented

### Semantic Search

Status:

Not started.

### AI Context Engine

Status:

Not started.

### IntelliJ Integration

Status:

Not started.
