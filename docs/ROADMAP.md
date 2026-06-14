# FrilVault Roadmap

## Phase 1

Completed:

- YAML note storage
- note CRUD
- search
- line anchors
- symbol anchors
- workspace index
- workspace stats
- workspace doctor
- repair

## Phase 2

Completed or mostly completed:

- Node bridge MVP
- VS Code extension MVP
- active-file notes panel
- add note command
- gutter decorations
- VS Code integration tests

## Phase 3

Current focus:

- reduce mixed integration paths in the VS Code extension
- expand editor workflows around symbols
- improve extension UX consistency

Tasks:

- align CLI and Node bridge usage
- improve note edit and delete UX
- add richer note previews
- strengthen extension test coverage

## Phase 4

Workspace runtime improvements:

- watcher support
- cache invalidation
- long-running editor session optimization

Tasks:

- monitor `.vault` changes
- monitor source file changes
- invalidate cached note data

## Phase 5

Symbol and search improvements:

- symbol resolution
- rename tracking
- indexed search improvements
- symbol-aware repair

## Phase 6

Additional editor integrations:

- IntelliJ plugin
- shared editor integration strategy around `frilvault-core`

## Phase 7

Knowledge-layer expansion:

- AI context engine
- semantic search
- project-level context assembly
