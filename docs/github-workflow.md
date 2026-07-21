# GitHub Development Workflow

This document defines the issue, branch, commit, Pull Request, and merge workflow for FrilVault.

## Repository Model

The expected Git remote layout is:

```text
origin   -> contributor fork
upstream -> canonical FrilVault repository
```

Default project values:

```bash
UPSTREAM_REPO="FrilLab/frilvault"
BASE_BRANCH="main"
```

The fork repository may differ by contributor. Determine it from `origin` instead of assuming an account name.

```bash
git remote -v
git remote get-url origin
git remote get-url upstream
```

GitHub Issues and Pull Requests belong to the upstream repository.

Working branches are pushed to the contributor fork.

## Core Workflow

Use this lifecycle for issue work:

```text
Issue
  -> Scope
  -> Branch
  -> Implementation
  -> Validation
  -> Commit
  -> Push
  -> Pull Request
  -> CI
  -> Merge
  -> Issue closure
```

Do not start several unrelated issues merely to reduce the open issue count. Prioritize changes that move the next release toward completion.

## Pull Request

When creating a Pull Request:

- Read `.github/PULL_REQUEST_TEMPLATE.md`
- Preserve its structure.
- Fill every section.
- Do not remove sections.
- Add "Closes #xx" only when the issue is completely resolved.

## 1. Inspect the Issue

Read the complete issue before changing code.

```bash
gh issue view <ISSUE_NUMBER> \
  --repo "$UPSTREAM_REPO" \
  --comments
```

Confirm:

- the issue is still open
- no existing Pull Request already solves it
- the latest comments have not changed the requirements
- required predecessor issues are complete
- the acceptance criteria are understandable
- the work belongs in the current release

Summarize the intended scope before implementation:

```text
Issue:
Goal:
Included:
Excluded:
Affected modules:
Validation:
```

Do not expand the scope to nearby cleanup or architectural work unless it is required for the issue to function correctly.

## 2. Check the Working Tree

Before switching branches:

```bash
git status --short
```

If uncommitted changes exist:

- do not discard them
- do not automatically stash them
- do not reset the working tree
- report the conflict and preserve the changes

## 3. Synchronize the Base Branch

Use the canonical repository as the source of truth.

```bash
git fetch upstream
git switch "$BASE_BRANCH"
git pull --ff-only upstream "$BASE_BRANCH"
```

Do not push directly to `main`.

Do not use `git reset --hard` as a normal synchronization step.

## 4. Create an Issue Branch

Branch format:

```text
<type>/<issue-number>-<short-description>
```

Allowed types:

```text
feat
fix
refactor
test
docs
ci
chore
release
```

Examples:

```text
feat/62-tag-support
fix/39-remove-duplicate-notes
refactor/71-note-query
test/75-separate-test-helpers
ci/93-github-releases
```

Rules:

- use lowercase
- use hyphens between words
- include the issue number
- keep the description short and specific
- do not include a date or developer name

Prefer creating a branch linked to the issue:

```bash
gh issue develop <ISSUE_NUMBER> \
  --repo "$UPSTREAM_REPO" \
  --base "$BASE_BRANCH" \
  --name "<BRANCH_NAME>" \
  --checkout
```

When the linked branch must be created in a fork:

```bash
gh issue develop <ISSUE_NUMBER> \
  --repo "$UPSTREAM_REPO" \
  --branch-repo "<FORK_OWNER>/frilvault" \
  --base "$BASE_BRANCH" \
  --name "<BRANCH_NAME>" \
  --checkout
```

If `gh issue develop` cannot support the repository arrangement, create the branch with Git:

```bash
git switch -c "<BRANCH_NAME>" "upstream/$BASE_BRANCH"
```

Report why the fallback was required.

## 5. Implement the Smallest Complete Change

During implementation:

- follow existing module boundaries and naming conventions
- put shared behavior in `frilvault-core`
- keep integration layers thin
- add a regression test before or with a bug fix
- add tests for new observable behavior
- preserve backward compatibility unless the issue explicitly changes it
- avoid speculative abstraction
- avoid unrelated formatting or file movement
- remove temporary logging and debugging code
- validate paths and file-system operations
- return useful errors instead of hiding failures

When an unrelated problem is discovered, record it as a follow-up issue candidate rather than adding it to the current branch.

## 6. Review the Diff

Before staging:

```bash
git status --short
git diff --check
git diff
```

Check for:

- accidental files
- debug output
- generated artifacts
- credentials
- unrelated changes
- missing tests
- unexpected API changes

Stage intentionally:

```bash
git add <FILES>
git diff --cached
```

Avoid `git add .` when unrelated local files may be present.

## 7. Commit

Use Conventional Commits:

```text
<type>: <imperative description>
```

Examples:

```text
feat: add note search by source file
fix: remove notes with duplicate anchors
refactor: introduce note query abstraction
test: separate shared test helpers
ci: add GitHub release workflow
```

Optional issue reference:

```text
feat: add tag support (#62)
```

Commit rules:

- use an imperative description
- do not end the subject with a period
- avoid vague subjects such as `update`, `changes`, or `fix stuff`
- keep unrelated changes in separate commits
- do not rewrite published history without approval

## 8. Push

Push the working branch to the fork:

```bash
git push -u origin HEAD
```

Never push directly to `main`.

Do not force-push without explicit approval.

## 9. Open a Pull Request

Create the Pull Request against the canonical repository:

```bash
gh pr create \
  --repo "$UPSTREAM_REPO" \
  --base "$BASE_BRANCH" \
  --head "<FORK_OWNER>:<BRANCH_NAME>" \
  --title "<TYPE>: <DESCRIPTION>" \
  --body-file .github/PULL_REQUEST_TEMPLATE.md
```

For the common fork workflow, prefer an explicit command like:

```bash
gh pr create \
  --repo "FrilLab/frilvault" \
  --base "main" \
  --head "<FORK_OWNER>:<BRANCH_NAME>" \
  --draft \
  --fill
```

Use `--draft` by default while the branch is still under review or while CI has not finished.

If the template must be filled manually, write the PR body to a temporary file with real newlines and then pass it to `--body-file`.

When generating the body directly, use this structure:

```markdown
## Summary

- Describe the primary change.
- Describe the user-visible or architectural result.
- Mention important compatibility details.

## Motivation

Explain the problem addressed by the issue.

## Implementation

- Describe the important implementation decisions.
- Mention the main modules changed.
- Explain meaningful tradeoffs.

## Validation

- [x] Formatting
- [x] Linting
- [x] Unit and integration tests
- [ ] Manual validation, when required

## Scope

### Included

- Items completed by this Pull Request.

### Excluded

- Explicit follow-up work.

Closes #<ISSUE_NUMBER>
```

Use an automatic closing keyword only when the Pull Request fully resolves the issue:

```text
Closes #62
Fixes #39
Resolves #75
```

For partial work, use:

```text
Related to #66
Part of #27
```

## 10. Verify the Pull Request

Inspect the created Pull Request:

```bash
gh pr view \
  --repo "$UPSTREAM_REPO"
```

Watch checks:

```bash
gh pr checks \
  --repo "$UPSTREAM_REPO" \
  --watch
```

When CI fails:

```bash
gh run list \
  --repo "$UPSTREAM_REPO" \
  --branch "<BRANCH_NAME>"

gh run view <RUN_ID> \
  --repo "$UPSTREAM_REPO" \
  --log-failed
```

Identify whether the failure:

- was introduced by the branch
- already exists on the base branch
- is environmental or flaky
- represents a missing requirement

Do not disable checks merely to merge the Pull Request.

## 11. Address Review Feedback

For each review cycle:

1. inspect unresolved review threads
2. separate required changes from questions
3. apply only the relevant changes
4. rerun affected validation
5. commit and push
6. summarize what changed
7. resolve only completed threads

Do not perform a broad rewrite in response to a narrow review comment.

## 12. Merge

The default merge strategy is squash merge.

Merge only when:

- required checks pass
- required reviews are complete
- merge conflicts are resolved
- acceptance criteria are satisfied
- validation results are recorded
- the issue closing reference is correct

Command:

```bash
gh pr merge \
  --repo "$UPSTREAM_REPO" \
  --squash \
  --delete-branch
```

Use automatic merge only when explicitly authorized:

```bash
gh pr merge \
  --repo "$UPSTREAM_REPO" \
  --squash \
  --delete-branch \
  --auto
```

If the user requested only implementation or Pull Request creation, do not merge.

## 13. Verify Closure and Clean Up

After merge:

```bash
git switch "$BASE_BRANCH"
git fetch upstream
git pull --ff-only upstream "$BASE_BRANCH"
```

Confirm the issue state:

```bash
gh issue view <ISSUE_NUMBER> \
  --repo "$UPSTREAM_REPO"
```

Delete a local branch only after confirming it is merged:

```bash
git branch --merged "$BASE_BRANCH"
git branch -d "<BRANCH_NAME>"
```

Never use `git branch -D` as routine cleanup.

## Worktrees

Use `git worktree` when independent issues must be developed concurrently:

```bash
git fetch upstream

git worktree add \
  "../frilvault-issue-<ISSUE_NUMBER>" \
  -b "<BRANCH_NAME>" \
  "upstream/$BASE_BRANCH"
```

Rules:

- one issue per worktree
- one branch per worktree
- do not mix files between worktrees
- limit active worktrees to two or three
- do not parallelize strongly dependent issues
- do not force-remove a worktree with uncommitted changes

Clean up after merge:

```bash
git worktree remove "../frilvault-issue-<ISSUE_NUMBER>"
git worktree prune
```

## Dependent Issues

Process dependent issues sequentially unless a stacked Pull Request workflow was explicitly requested.

Example:

```text
In-memory cache
  -> Cache invalidation
  -> File preloading
  -> Index warm-up
  -> Cache integration
```

Do not create all dependent branches from `main` at the same time.

## Completion Report

At the end of an issue task, report:

```text
Issue:
Branch:
Commit:
Pull Request:
Status:

Implemented:
- ...

Validation:
- PASS: ...
- FAIL: ...
- SKIPPED: ...

Remaining:
- ...

Risks:
- ...
```

Never report incomplete or failing work as complete.
