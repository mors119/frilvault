# FrilVault Docs Book

## Local Usage

Run these commands from the repository root, not from `docs/` or `docs/book/`.

Install the required tools:

```bash
cargo install mdbook --locked
cargo install mdbook-mermaid --locked
```

Install Mermaid assets into the book once:

```bash
mdbook-mermaid install docs/book
```

If you are already inside `docs/book/`, use:

```bash
mdbook-mermaid install .
```

Build the site:

```bash
mdbook build docs/book
```

If you are already inside `docs/book/`, use:

```bash
mdbook build .
```

Serve the site locally:

```bash
mdbook serve docs/book --open
```

If you are already inside `docs/book/`, use:

```bash
mdbook serve . --open
```

The generated HTML output is written to `docs/book/book/`.
