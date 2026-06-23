# 📘 FrilVault Roadmap

## Version

v0.1 → v1.0 (Core → Runtime → IDE Transition)

---

# Phase 1 — Core Foundation (COMPLETED)

## Goal

Local note system with structured storage and basic workspace intelligence.

## Completed Features

### Note System

- JSON-based note storage
- Note CRUD (create / update / delete / list)
- Line-based anchors
- Symbol-based anchors
- Note search (content + symbol)

---

### Workspace System

- Workspace indexing
- Workspace statistics
- Workspace health check
- Repair suggestion system
- Repair execution (file rename/move support)

---

### Storage Layer

- `.vault` directory structure
- JSON persistence model
- structured note file format

---

### CLI Layer (frilvault-cli)

- add note
- list notes
- update note
- delete note
- search notes
- stats
- health check
- repair

---

# Phase 2 — Runtime Foundation (IN PROGRESS)

## Goal

Introduce long-running runtime abstraction and remove stateless execution limitations.

---

## Completed / In Progress

### VaultContext (Runtime Container)

- Central runtime state object introduced
- Owns repositories
- Owns cache
- Provides unified access layer

---

### Cache System (Partial)

- NoteCache implemented
- Cache-aware note loading introduced
- Cache invalidation hooks added
- Repository bypass eliminated in read path

---

## Remaining Tasks

- full service migration to VaultContext-only access
- unify read/write paths through VaultContext
- ensure cache consistency across all operations

---

# Phase 3 — Service Unification (CURRENT FOCUS)

## Goal

Remove repository leakage from service layer.

---

## Tasks

### Service Layer Refactor

- NoteService → VaultContext-only access
- WorkspaceService → VaultContext-only access
- Remove direct repository usage from services

---

### Cache Completion

- ensure cache used in:
  - list notes
  - search notes
  - symbol queries (future)

- ensure invalidation on:
  - add note
  - update note
  - delete note
  - repair operations

---

### API Stabilization

- finalize VaultContext API surface
- define stable internal service contracts

---

# Phase 4 — Watcher System (NEXT CORE MILESTONE)

## Goal

Enable automatic system updates in long-running environments (VSCode, Node runtime).

---

## Tasks

### File System Watcher

- watch `.vault` changes
- watch source file changes

---

### Reactive Cache System

- automatic cache invalidation
- automatic index rebuild
- incremental updates (future optimization)

---

### Runtime Synchronization

- ensure VSCode stays in sync with core state

---

# Phase 5 — Symbol Intelligence Layer

## Goal

Move from “note system” → “code-aware knowledge system”

---

## Tasks

### Symbol Query System

- search notes by symbol name
- partial symbol matching
- namespace-style queries

---

### Symbol Resolution Engine (critical)

- track function rename
- track symbol movement across files
- maintain note attachment consistency

---

### Symbol Index

- build structured symbol graph
- connect notes ↔ symbols ↔ files

---

# Phase 6 — VSCode Full Integration

## Goal

Replace CLI dependency with native editor runtime integration.

---

## Tasks

### UI Features

- gutter decorations
- hover previews
- sidebar notes panel
- symbol tree view

---

### Integration Layer

- Node bridge full adoption
- remove CLI dependency in runtime path
- unify extension → VaultContext flow

---

# Phase 7 — Knowledge Layer Expansion

## Goal

Transform FrilVault into a structured developer memory system.

---

## Tasks

### Semantic Search

- vector-based search (future)
- context-aware retrieval

---

### AI Context Engine

- build context from:
  - notes
  - workspace index
  - symbol graph

---

### Project Memory Graph

- file ↔ symbol ↔ note relationships
- dependency-aware context building

---

# Phase 8 — Multi-Editor Support

## Goal

Make FrilVault editor-agnostic.

---

## Tasks

- IntelliJ plugin
- Neovim integration (optional future)
- shared core runtime usage

---

# 🚀 Current Position Summary

```text
FrilVault is currently transitioning from:

Phase 2 → Phase 3

Meaning:
Core is stable
Runtime layer is being introduced
Cache is becoming central
Services are being unified
```

---

# 🎯 Key Strategic Direction

FrilVault is evolving into:

> A runtime knowledge layer for codebases, not a note tool

Core abstraction enabling this:

```text
VaultContext + Cache + Workspace Index + Symbol Engine
```
