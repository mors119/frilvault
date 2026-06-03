# FrilVault

> Your Personal Knowledge Vault for Code

🚧 Early development stage

FrilVault is a developer-focused personal knowledge vault that allows you to attach private notes to source code without modifying the code itself.

Instead of adding temporary comments, TODOs, research notes, debugging records, or learning materials directly into source files, FrilVault stores them separately inside a local `.vault` directory.

This keeps source code clean while preserving valuable knowledge.

---

## Why?

When studying large codebases, contributing to open source projects, or maintaining long-term software, developers often create personal notes such as:

- Architecture analysis
- Research findings
- Debugging records
- TODO items
- Learning notes
- AI context
- Reverse engineering notes

Adding these notes directly into source files creates several problems:

- Pollutes the codebase
- Causes merge conflicts
- Makes upstream synchronization difficult
- Mixes personal knowledge with production code

FrilVault solves this by storing notes outside of the source code.

---

## Quick Start

Add a note:

```bash
flv add \
  --file src/main.rs \
  --line 10 \
  --column 5 \
  --content "parser 개선 필요"
```

List notes:

```bash
flv list \
  --file src/main.rs
```

Search notes:

```bash
flv search parser
```

Update a note:

```bash
flv update \
  --file src/main.rs \
  --id <NOTE_ID> \
  --content "parser 구조 재설계 필요"
```

Delete a note:

```bash
flv delete \
  --file src/main.rs \
  --id <NOTE_ID>
```

---

## Features

### Private Notes

Store personal notes without modifying source code.

### Line Anchors

Attach notes to specific locations inside source files.

```yaml
anchor:
  type: Line
  line: 10
  column: 5
```

### Search

Search notes using keywords.

```bash
flv search parser
```

### Local First

All data is stored locally.

No external services are required.

### Clean Repository

Keep repositories free from temporary comments and personal annotations.

### Developer Knowledge Base

Build a personal knowledge layer on top of any codebase.

---

## Example

Source code:

```rust
pub fn calculate_damage() {
    // production code
}
```

Personal note:

```bash
flv add \
  --file src/combat.rs \
  --line 1 \
  --column 1 \
  --content "Consider SIMD optimization in the future"
```

Stored note:

```yaml
notes:
  - id: '15b7c4b3-f4a6-4cc1-accb-428f344cc597'

    source_file: src/combat.rs

    anchor:
      type: Line
      line: 1
      column: 1

    content: Consider SIMD optimization in the future

    created_at: '2026-06-03T17:42:17.853037Z'
    updated_at: '2026-06-03T17:42:17.853037Z'
```

---

## Storage Structure

Current storage layout:

```text
project/
├── src/
│
└── .vault/
    └── src/
        ├── main.rs.yml
        ├── lib.rs.yml
        └── service.rs.yml
```

Example:

```yaml
notes:
  - id: '15b7c4b3-f4a6-4cc1-accb-428f344cc597'

    source_file: src/lib.rs

    anchor:
      type: Line
      line: 3
      column: 1

    content: Parser trait 검토

    created_at: '2026-06-03T17:42:17.853037Z'
    updated_at: '2026-06-03T17:42:17.853037Z'
```

---

## Current Status

### Core

- YAML note storage
- Line anchors
- CRUD operations
- Keyword search

### CLI

- add
- list
- update
- delete
- search

---

## Use Cases

### Open Source Analysis

Study libraries and frameworks without modifying upstream code.

### Reverse Engineering

Document control flow and implementation details.

### Personal Documentation

Store architecture notes and design decisions.

### Learning Notes

Record discoveries while exploring unfamiliar codebases.

### AI-Assisted Development

Build project-specific context for future AI workflows.

---

## Roadmap

### Phase 1 (Current)

- Rust core library
- YAML storage
- CLI support
- CRUD operations
- Keyword search

### Phase 2

- Symbol anchors
- JSON output
- VSCode extension

### Phase 3

- Project indexing
- Workspace explorer
- Cached search

### Phase 4

- AI Context Engine
- Semantic search
- RAG integration

### Phase 5

- JetBrains plugin
- Desktop application
- Team knowledge sharing

---

## Philosophy

Source code should remain clean.

Knowledge belongs in the vault.
