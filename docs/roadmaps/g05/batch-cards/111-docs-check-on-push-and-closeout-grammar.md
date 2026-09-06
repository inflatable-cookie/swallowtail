# 111 Docs Check On Push And Closeout Grammar

Status: planned; after the `v0.4.3` tag
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../034-release-lane-simplification.md`
Depends on: the g05.034 measured causes table

## Goal

Stop closeouts from landing index drift on `main`: three of four `v0.4.2` prepare attempts failed on it.

## Scope

1. A git pre-push hook, installed by a script wired into `effigy bootstrap` or `effigy doctor`, runs `effigy qa:docs` when the push touches `docs/**`, `PAPERCUTS.md`, or `CHANGELOG.md`; it refuses the push on failure. Workers and coordinators install it once.
2. `scripts/check-roadmap-status-drift.py`: document the status grammar in the script header (first recognised token wins; the recognised set; how batch-card indexes are matched) and make the error messages name the exact line to fix.
3. Contract 001 closeout line: a closeout commit runs the docs check before push; the hook is the enforcement, the rule is the authority.

## Acceptance Criteria

- [ ] a deliberately broken index is rejected at push time
- [ ] the checker's grammar is documented in its header and every failure names a line
- [ ] Contract 001 updated

## Validation

- hook proof on a throwaway branch; `effigy qa:docs`; `git diff --check`

## Review Oracle

See the g05.034 manifest row.

## Auto-Continuation

No. Stop for exact-head review.
