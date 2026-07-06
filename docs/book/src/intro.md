# Introduction

FrilVault is a local-first knowledge layer for source code.

It lets developers attach personal notes to files, lines, and symbols without modifying the source code itself. The system stores note metadata under `.vault/` and keeps application code clean.

## Vision

FrilVault treats documentation and personal code context as separate concerns:

- source code remains production-oriented
- developer knowledge remains persistent and local
- tooling surfaces reuse the same core rules

## Local-First Principle

FrilVault does not depend on a remote backend. Notes, workspace indexes, and runtime metadata live inside the workspace-local `.vault` directory.

This gives the project three useful properties:

- offline-first usage
- predictable storage ownership
- simple backup and versioning strategies

## Source Code Integrity Rule

FrilVault must not rewrite or annotate user source files.

Instead, it stores external note files and workspace metadata in a parallel storage model:

```text
.vault/
├── notes/
├── index/
└── workspace.json
```

## Current Product Shape

The current repository has three main surfaces:

- `frilvault-core`: shared Rust domain and storage logic
- `frilvault-cli`: the `flvt` command-line interface
- `apps/vscode-extension`: the current editor MVP

The VS Code extension is not fully native to the Rust core yet. Some features still go through the CLI, while others use a Node bridge.

## Documentation As Code

This documentation is maintained as an `mdBook` project:

- source files live under `docs/book/src`
- diagrams are declared as Mermaid inside Markdown
- GitHub Actions builds and deploys the rendered site
