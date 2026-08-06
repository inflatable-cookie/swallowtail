# 130 v0.1.0 Tag Authorization Handoff

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../043-v0-1-0-source-release-readiness.md`
Depends on: card 129

## Goal

Present the exact accepted candidate for a separate operator decision on local
annotated-tag creation and tag push.

## Scope

1. Name the exact commit, branch, remote, tag, and annotation.
2. State that crates.io and GitHub Release creation are excluded.
3. Check tag absence and remote commit reachability read-only.
4. Stop for explicit authorization before any tag mutation.

## Validation

- candidate evidence remains exact and unchanged
- tag and remote state are read-only observations before approval

## Auto-Continuation

No. External tag mutation cannot auto-continue.

## Completion Evidence

- accepted candidate: `0ef25a8c4f8bb9ee5c7c71b27cb0c4df0f608b01`
- release commit: `a8bef72b718d3d9e503da48b3af05da4b674d4ec`
- annotated tag object: `630d33a0d1ff285d20787ee038147dc3493f8b88`
- tag and annotation: `v0.1.0`
- canonical branch and remote: `main`,
  `git@github.com:inflatable-cookie/swallowtail.git`
- remote branch and peeled tag both resolve to the release commit
- crates.io publication and GitHub Release creation did not run

The first manually dispatched tag CI run exposed a deterministic-under-load
Anthropic cancellation/deadline race. Card 131 owns the repair. The published
tag remains immutable.
