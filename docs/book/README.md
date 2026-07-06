# FrilVault Docs Book

Run these commands from the repository root.

## Install Tools

```bash
cargo install mdbook --locked
cargo install mdbook-mermaid --locked
```

## Prepare Mermaid Assets

```bash
mdbook-mermaid install docs/book
```

## Build

```bash
mdbook build docs/book
```

## Serve Locally

```bash
mdbook serve docs/book --open
```
