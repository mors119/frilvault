# Agents

This file defines the repository-wide rules that coding agents must follow.

## Project Priorities

- Preserve local-first behavior.
- Never rewrite source files as part of the note model.
- Keep reusable domain logic in `crates/frilvault-core`.
- Keep the CLI and editor integrations thin.
- Prefer incremental changes over broad architectural rewrites.
- Avoid infrastructure that increases maintenance cost without a clear release benefit.
- Keep the repository in a releasable state.

## Repository Guidance

### Core crate

Add or change shared behavior in `crates/frilvault-core` first.

The CLI, desktop application, and editor integrations should call the core crate instead of duplicating business logic.

### CLI

Keep `apps/frilvault-cli` focused on:

- parsing command-line input
- opening the workspace
- invoking core behavior
- formatting output
- mapping failures to useful exit codes

Do not place persistent domain behavior directly in CLI command handlers.

### VS Code extension

The extension currently contains both CLI-based and native Node integration paths.

Changes should make that boundary clearer and should not introduce a third integration path without an explicit architectural decision.

### Desktop application

The desktop application is the primary release target.

Keep platform-specific behavior outside the core crate unless it represents a reusable domain capability.

## Change Rules

- Work from a GitHub Issue whenever an issue exists.
- Keep one clear purpose per branch and Pull Request.
- Implement the smallest complete change that satisfies the issue.
- Add or update tests when behavior changes.
- Do not delete tests merely to make a build pass.
- Do not include unrelated refactoring in an issue branch.
- Do not silently change public behavior.
- Report architectural mismatches when discovered.
- Record out-of-scope findings as follow-up issue candidates.

## Safety Rules

Do not perform destructive or history-rewriting actions without explicit user approval.

This includes:

- `git reset --hard`
- `git clean -fd`
- `git clean -fdx`
- `git push --force`
- `git push --force-with-lease`
- `git branch -D`
- deleting tags, releases, issues, repositories, or Pull Requests

Do not commit:

- access tokens
- credentials
- local environment files
- build outputs
- editor-specific temporary files
- generated files that are not intentionally versioned

## Required References

Follow the detailed repository workflow in:

- [`docs/github-workflow.md`](docs/github-workflow.md)
- [`docs/testing.md`](docs/testing.md)
- [`docs/RELEASES/PROCESS.md`](docs/RELEASES/PROCESS.md)

When instructions conflict, use this priority:

1. explicit user instruction
2. this `AGENTS.md`
3. the referenced workflow documents
4. existing repository conventions
