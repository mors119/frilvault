# Release Process

This document defines the release workflow for FrilVault.

The first objective is to publish a small, usable release. Do not delay a release solely to include optional infrastructure or future-facing architecture.

## Release Principles

- Release a coherent minimum product before implementing every open issue.
- Separate release infrastructure from unrelated feature work.
- Keep `main` releasable.
- Build releases from reviewed and merged commits.
- Use semantic version tags.
- Never publish a release without explicit maintainer approval.
- Prefer draft releases until artifacts and notes have been verified.
- Do not treat automatic application updates as a prerequisite for the first release.

## Versioning

Use semantic version tags:

```text
v0.0.1
v0.0.2
v0.1.0
```

General interpretation before `v1.0.0`:

- patch: fixes and small compatible improvements
- minor: meaningful feature or architecture milestone
- major: stable public contract after the project is ready for `v1.0.0`

The maintainer decides the final version.

## Release Scope

Before release work begins, create or confirm a release checklist containing:

```text
Target version:
Target date:
Required issues:
Optional issues:
Deferred issues:
Supported platforms:
Release artifacts:
Known limitations:
```

Only required issues block the release.

Move optional or deferred work to a later milestone instead of silently expanding the release.

## Release Readiness Checklist

Confirm:

- required release issues are closed
- no blocking Pull Request remains open
- `main` is synchronized and clean
- workspace formatting passes
- workspace linting passes
- workspace tests pass
- release build succeeds
- desktop package configuration is correct
- version numbers are consistent
- lockfiles are current
- user-facing documentation is current
- release workflow exists and is valid
- artifact names are predictable
- checksums or signatures are produced when configured
- known limitations are documented

## Version Update

Search for all version declarations before changing them.

Typical locations may include:

```text
Cargo.toml
Cargo.lock
package.json
package-lock.json
tauri.conf.json
tauri.conf.json5
tauri.conf.toml
CHANGELOG.md
```

Do not assume every listed file exists.

Keep the same version across all components that are released together.

## Changelog

Maintain user-focused entries.

Recommended structure:

```markdown
# Changelog

## [Unreleased]

### Added

### Changed

### Fixed

## [0.0.1] - YYYY-MM-DD

### Added

- First desktop release.
- Local FrilVault workspace support.
- Note display for the current source file.

### Known limitations

- Automatic updates are not available.
- Some advanced symbol repair features are deferred.
```

Describe outcomes, not internal commit history.

## Release Branch

For a small project, release directly from a short-lived branch:

```text
release/<version>
```

Example:

```text
release/0.0.1
```

The release branch should contain only release preparation:

- version updates
- changelog updates
- release metadata
- packaging fixes required for release
- documentation corrections required for installation

Do not add unrelated product features to the release branch.

## Release Validation

Run the full project validation appropriate to the release.

Rust baseline:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
```

Run relevant frontend, extension, and desktop commands defined by their actual package files.

Record unsupported platform validation explicitly.

## Release Pull Request

Open a dedicated Pull Request.

Suggested title:

```text
release: prepare v0.0.1
```

Suggested body:

```markdown
## Release

`v0.0.1`

## Included

- List the user-visible features.
- List important fixes.
- List supported platforms.

## Deferred

- List non-blocking work moved to later releases.

## Validation

- [x] Formatting
- [x] Linting
- [x] Tests
- [x] Release build
- [ ] Platform-specific manual checks

## Known limitations

- Document current limitations.
```

Merge using the repository's normal reviewed workflow.

## Tagging

Create the tag from the verified commit on the canonical default branch.

Before tagging:

```bash
git switch main
git fetch upstream
git pull --ff-only upstream main
git status --short
```

Confirm the target commit:

```bash
git log -1 --oneline
```

Do not tag an uncommitted or unreviewed working tree.

Tag format:

```bash
git tag -a v0.0.1 -m "FrilVault v0.0.1"
git push upstream v0.0.1
```

Tagging and pushing to the canonical repository require explicit maintainer approval.

When the GitHub Actions release workflow creates tags automatically, follow the workflow's documented trigger instead of creating a duplicate tag manually.

## GitHub Release

Create a draft first:

```bash
gh release create v0.0.1 \
  --repo FrilLab/frilvault \
  --draft \
  --generate-notes \
  --title "FrilVault v0.0.1"
```

Upload artifacts when they are not attached automatically:

```bash
gh release upload v0.0.1 \
  <ARTIFACTS> \
  --repo FrilLab/frilvault \
  --clobber
```

Use `--clobber` only when intentionally replacing incorrect draft artifacts.

Verify:

- release points to the expected tag
- artifact names include platform and architecture
- artifacts open or execute as expected
- checksums match
- release notes explain installation
- known limitations are included

Publish only after explicit maintainer approval.

## Suggested Artifact Naming

Use predictable names:

```text
frilvault-v0.0.1-macos-aarch64.dmg
frilvault-v0.0.1-macos-x86_64.dmg
frilvault-v0.0.1-windows-x86_64.msi
frilvault-v0.0.1-linux-x86_64.AppImage
frilvault-v0.0.1-checksums.txt
```

Only publish platforms that were actually built and validated.

## Release Notes

Release notes should answer:

- what FrilVault does
- what is included in this version
- how to install it
- which platforms are supported
- what limitations remain
- where to report problems

Avoid copying raw commit messages without editing.

## Post-Release Verification

After publication:

- download the public artifacts
- verify installation from the public release page
- start the application
- create or open a local vault
- verify the primary release path
- confirm the tag and release are visible
- confirm release notes are correct
- open issues for newly discovered defects
- update the next milestone

Do not silently replace a published release to hide a defect. Document the problem and issue a corrected release when necessary.

## Rollback and Correction

If a draft release is wrong:

- keep it as a draft
- replace the incorrect artifacts
- correct the release notes
- rerun validation

If a published release is defective:

- document the defect
- avoid deleting the release unless absolutely necessary
- prepare a patch version
- publish corrected artifacts under the new version

Destructive release or tag operations require explicit maintainer approval.

## First Release Recommendation

For `v0.0.1`, prioritize:

- a working desktop application
- basic local vault behavior
- current-file note display
- stable storage behavior
- clear installation instructions
- GitHub Release artifacts

Defer when necessary:

- automatic application updates
- advanced cache and indexing
- file watchers
- rename and repair automation
- image attachments
- advanced symbol resolution
