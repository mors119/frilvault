# FrilVault Architecture

## Vision

FrilVault is a personal knowledge vault for developers.

The system should let a developer attach private notes to code without changing the code itself.

## Core Principles

### Local First

All data is stored locally.

### Source Code Integrity

FrilVault must not modify source files.

### Shared Core Logic

Editor integrations should reuse the same core behavior instead of reimplementing note logic.

### Editor-Agnostic Design

VS Code is one integration surface, not the product boundary.

## Current Repository Architecture

```text
frilvault
├── crates
│   ├── frilvault-core
│   └── frilvault-node
└── apps
    ├── frilvault-cli
    └── vscode-extension
```

## `frilvault-core`

`frilvault-core` owns domain behavior.

```text
frilvault-core
├── note
│   ├── entity
│   ├── repository
│   └── service
├── workspace
│   ├── entity
│   ├── path
│   ├── repository
│   └── service
├── storage
├── parser
└── cache
```

Responsibilities:

- note CRUD
- search
- YAML serialization
- workspace metadata
- workspace indexing
- workspace health checks
- repair suggestion and application

## `frilvault-cli`

`frilvault-cli` is the command-line surface over `frilvault-core`.

Responsibilities:

- parse user input
- call core services
- print text or JSON output

Current notable interface:

- `flvt list --format json`

## `frilvault-node`

`frilvault-node` is a Node-API bridge around `frilvault-core`.

Purpose:

- expose core operations to Node-based editor runtimes
- avoid duplicating domain behavior in TypeScript

Current exposed operations are still selective rather than complete.

## VS Code Extension

The VS Code extension is the UI layer.

Current feature structure:

```text
src/features
├── add-note
├── decorations
└── notes-panel
```

Current behavior split:

- CLI-backed flows:
  - add note
  - active-file notes panel
  - gutter decorations
- Node-bridge-backed flows:
  - edit note
  - delete note
  - search
  - stats
  - health
  - repair

This split works for the MVP, but it is transitional.

## Storage Model

```text
.vault
├── notes
├── cache
├── index
└── workspace.yml
```

## Repair Flow

```text
health_check
↓
repair_suggestions
↓
apply_repairs
```

Current repair implementation:

- filename-based candidate matching

Planned improvements:

- interactive candidate selection
- stronger similarity heuristics
- symbol-aware repair

## Target Direction

Long-term direction:

```text
Editor UI
   │
   ├── CLI integration
   │
   └── Native bridge integration
            │
            ▼
      frilvault-core
```

Desired properties:

- one source of truth for note behavior
- reusable integration boundary for multiple editors
- minimal UI-specific logic outside the editor surface
