# FrilVault

> Personal Knowledge Vault for Developers

🚧 Early development stage

FrilVault is a developer-focused knowledge vault that allows you to attach private notes to source code without modifying the code itself.

Instead of adding temporary comments, TODOs, research notes, or learning records directly into a repository, FrilVault stores them separately in a local `.vault` directory.

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

Adding these notes directly into source files creates several problems:

- Pollutes the codebase
- Causes merge conflicts
- Makes upstream synchronization difficult
- Mixes personal knowledge with production code

FrilVault solves this by storing notes outside of the source code.

---

## Features

### Private Notes

Store personal notes without modifying source files.

### Symbol-Based Notes

Attach notes to functions, methods, structs, classes, or modules.

### Local First

All data is stored locally.

### Search

Search notes by symbol, file, tag, or keyword.

### AI Ready

Use personal notes as context for AI-assisted development workflows.

### Clean Repository

Keep repositories free from temporary comments and personal annotations.

---

## Example

Source code:

```rust
pub fn calculate_damage() {
    // production code
}
```

Stored note:

```yaml
notes:
  - symbol: calculate_damage

    tags:
      - analysis

    comment: |
      Consider SIMD optimization in the future.
```

---

## Project Structure

```text
frilvault/
├── crates/
│   └── frilvault-core/
│
├── apps/
│   └── vscode-extension/
│
└── .vault/
    ├── notes/
    ├── cache/
    └── index/
```

---

## Roadmap

### Phase 1

- Rust core library
- VSCode extension
- YAML note storage
- Symbol-based notes

### Phase 2

- Project indexing
- Search engine
- Workspace explorer

### Phase 3

- Semantic search
- AI context integration
- RAG support

### Phase 4

- JetBrains plugin
- Desktop application
- Team knowledge sharing

---

## Philosophy

Source code should remain clean.

Knowledge belongs in the vault.
