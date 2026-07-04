# 📘 FrilVault Architecture

## Version

v0.1 (Current Core + VaultContext + Cache Transition)

---

# 1. Vision

FrilVault is a developer-focused personal knowledge vault.

It allows developers to attach structured, persistent notes to source code without modifying the code itself.

The system acts as a **knowledge layer on top of codebases**, not a code annotation tool.

---

# 2. Core Principles

## 2.1 Local First

All data is stored locally inside the `.vault` directory.

No external service dependency exists.

---

## 2.2 Source Code Integrity

FrilVault must never modify source files.

All metadata and notes are stored externally.

---

## 2.3 Shared Core Logic

All clients must reuse `frilvault-core` as the single source of truth.

No business logic duplication in:

- CLI
- VSCode extension
- Node bridge

---

## 2.4 Runtime-Centric Design

The system introduces a runtime container:

> VaultContext

This is responsible for:

- caching
- repository coordination
- index access
- runtime optimization

---

## 2.5 Editor-Agnostic Design

VSCode is an integration layer, not the system boundary.

Future editors must be able to integrate without modifying core logic.

---

# 3. Repository Architecture

```text
frilvault
├── crates
│   └── frilvault-core
│
└── apps
    ├── frilvault-cli
    └── vscode-extension
```

## 3.1 Current Structure (Mermaid)

```mermaid
flowchart TD
    subgraph Clients
        CLI["apps/frilvault-cli"]
        VSCode["apps/vscode-extension"]
    end

    subgraph Core["crates/frilvault-core"]
        FV["FrilVault facade"]
        NS["NoteService"]
        WS["WorkspaceService"]
        VC["VaultContext"]
        NR["NoteRepository"]
        WIR["WorkspaceIndexRepository"]
        WR["WorkspaceRepository"]
        NC["NoteCache"]
        ND["note"]
        WD["workspace"]
        RT["runtime"]
    end

    CLI --> FV
    VSCode --> CLI
    FV --> NS
    FV --> WS
    NS --> VC
    WS --> VC
    WS --> WIR
    VC --> NR
    VC --> WIR
    VC --> NC
    NR --> ND
    WIR --> WD
    WR --> WD
    VC --> RT
```

## 3.2 Enterprise Architecture (PlantUML)

```plantuml
@startuml
title FrilVault Enterprise Architecture

top to bottom direction
skinparam shadowing false
skinparam packageStyle rectangle
skinparam componentStyle rectangle
skinparam defaultTextAlignment center

actor Developer

node "Editor Layer" as editor_layer {
  component "VSCode Extension\n(apps/vscode-extension)" as vscode
}

node "Application Layer" as app_layer {
  component "FrilVault CLI\n(apps/frilvault-cli)\nBinary: flvt" as cli
}

node "Core Domain Layer" as core_layer {
  package "frilvault-core" {
    component "FrilVault\nFacade" as facade

    package "Application Services" {
      component "NoteService" as note_service
      component "WorkspaceService" as workspace_service
    }

    package "Runtime" {
      component "VaultContext" as vault_context
      component "NoteCache" as note_cache
    }

    package "Repositories" {
      component "NoteRepository" as note_repository
      component "WorkspaceIndexRepository" as index_repository
      component "WorkspaceRepository" as workspace_repository
    }

    package "Domain Modules" {
      component "note" as note_domain
      component "workspace" as workspace_domain
      component "parser / error / constants" as support_modules
    }
  }
}

node "Workspace Storage" as storage_layer {
  folder "Source Workspace" as source_workspace
  folder ".vault/notes" as vault_notes
  folder ".vault/index" as vault_index
  file ".vault/workspace.json" as workspace_meta
}

Developer --> vscode : add/search/list notes
vscode --> cli : execFile()
cli --> facade : open(workspace_root)
facade --> note_service : notes()
facade --> workspace_service : workspace()

note_service --> vault_context : read/write notes
workspace_service --> vault_context : stats / health / repair
workspace_service --> index_repository : rebuild()

vault_context --> note_cache : cache hit / invalidate
vault_context --> note_repository : load / append / replace
vault_context --> index_repository : rebuild / scan

note_repository --> note_domain
index_repository --> workspace_domain
workspace_repository --> workspace_domain
note_service --> note_domain
workspace_service --> workspace_domain
facade --> support_modules

note_repository --> vault_notes
index_repository --> vault_index
workspace_repository --> workspace_meta
index_repository --> source_workspace : scan files
workspace_service --> source_workspace : repair candidates

note right of vscode
Current integration model:
VSCode calls the CLI instead of
linking directly to frilvault-core.
end note

note bottom of vault_context
VaultContext is the runtime boundary.
Services should not access repositories
outside this container.
end note
@enduml
```

---

# 4. Core Architecture (frilvault-core)

## 4.1 Module Structure

```text
frilvault-core
├── note
│   ├── dto
│   ├── entity
│   ├── note_repository
│   └── note_service
│
├── workspace
│   ├── diff
│   ├── entity
│   ├── path
│   ├── repository
│   └── service
│
├── parser
├── runtime
├── app
├── constants
└── error
```

---

## 4.1.1 Core Runtime View (Mermaid)

```mermaid
flowchart LR
    A["FrilVault::open(workspace_root)"] --> B["PathResolver"]
    B --> C["WorkspaceRepository"]
    B --> D["WorkspaceIndexRepository"]
    B --> E["NoteRepository"]
    E --> F["VaultContext"]
    D --> F
    F --> G["NoteCache"]
    F --> H["NoteService"]
    F --> I["WorkspaceService"]
    D --> I
    C --> J[".vault/workspace.json"]
    D --> K[".vault/index"]
    E --> L[".vault/notes"]
```

---

## 4.2 Responsibilities

### Note Domain

Responsible for:

- CRUD operations
- line-based anchors
- symbol-based anchors
- note search
- JSON persistence

---

### Workspace Domain

Responsible for:

- workspace indexing
- statistics
- health checks
- repair suggestions
- repair execution

---

### VaultContext (Runtime Core)

VaultContext is the runtime container of FrilVault.

It owns:

- NoteRepository
- WorkspaceIndexRepository
- NoteCache

Responsibilities:

- cache-aware note loading
- cache invalidation
- unified access layer for services

---

### Cache Layer

In-memory optimization layer used only in long-running processes.

Current responsibilities:

- note caching
- future: index cache, symbol cache

Important:

CLI usage is short-lived; cache is primarily useful for VSCode and Node runtime.

---

# 5. Service Layer

## 5.1 NoteService

Responsible for:

- note CRUD orchestration
- search coordination
- interaction with VaultContext

Important:

- Must not directly access repositories
- Must go through VaultContext

---

## 5.2 WorkspaceService

Responsible for:

- workspace statistics
- health checking
- repair system
- file scanning

Important:

- Uses both index data and note data via VaultContext

---

# 6. Storage Model

```text
.vault
├── notes
├── cache
├── index
└── workspace.json
```

---

# 7. Repair System

## Flow

```text
health_check
↓
repair_suggestions
↓
apply_repairs
```

---

## Current Implementation

- filename-based matching
- heuristic candidate selection

---

## Future Improvements

- symbol-aware repair
- interactive selection
- semantic matching

---

# 8. Runtime Data Flow

## 8.1 Read Path

```text
Client (CLI / VSCode)
↓
Service
↓
VaultContext
↓
Cache (hit/miss)
↓
Repository (fallback)
↓
Filesystem (JSON)
```

---

## 8.2 Write Path

```text
Client
↓
Service
↓
Repository
↓
Filesystem
↓
Cache invalidation
```

---

# 9. Editor Integration Model

## 9.1 VSCode Architecture (Current)

```mermaid
flowchart TD
    Editor["VSCode UI
    - gutter
    - hover
    - notes panel
    - commands"] --> Client["CliClient"]
    Client --> Binary["flvt CLI"]
    Binary --> Facade["FrilVault"]
    Facade --> Services["NoteService / WorkspaceService"]
    Services --> Context["VaultContext"]
    Context --> Vault[".vault"]
```

---

## 9.2 Target Architecture

```mermaid
flowchart TD
    Editor["VSCode Extension"] --> Bridge["Node bridge or native binding"]
    Bridge --> Core["frilvault-core"]
    Core --> Context["VaultContext"]
    Context --> Vault[".vault"]
```

---

## 9.3 Features Owned by VSCode Layer

- gutter decorations
- hover previews
- sidebar panels
- commands UI

No business logic allowed here.

---

# 10. Key Design Constraints

## 10.1 Single Source of Truth

All logic must live in:

> frilvault-core

---

## 10.2 No Repository Leakage

Services must not directly depend on repositories.

All access goes through VaultContext.

---

## 10.3 Cache Transparency

Cache must be invisible to clients.

Clients should not know whether data is cached or loaded from disk.

---

# 11. Target Evolution

## Current State

- Core fully functional
- Cache introduced
- VaultContext introduced
- CLI stable
- Repair system functional

---

## Next State

- full VaultContext adoption
- unified service layer
- cache-driven reads
- VSCode integration stabilization

---

## Future State

- symbol resolution engine
- watcher system
- semantic search
- AI context layer
