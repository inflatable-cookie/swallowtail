# 111 Docs Check On Push And Closeout Grammar

Status: complete; PR 250 merged at `f51575c9`
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

- [x] a deliberately broken index is rejected at push time
- [x] the checker's grammar is documented in its header and every failure names a line
- [x] Contract 001 updated

## Validation

- hook proof on a throwaway branch; `effigy qa:docs`; `git diff --check`

## Review Oracle

See the g05.034 manifest row.

## Auto-Continuation

No. Stop for exact-head review.

## Result

Pre-push `effigy qa:docs` when the push changes `docs/**`, `PAPERCUTS.md`, or
`CHANGELOG.md`. `scripts/install-git-hooks.sh` sets local `core.hooksPath` to
`scripts/git-hooks`. `effigy hooks:install` is wired into `[bootstrap]` and
`health`, so `effigy bootstrap` and `effigy doctor` install it once.

The status checker header now owns first-token grammar: split on the first `;`,
recognised set, batch-card section matching. Annotation primaries no longer
scan later detail for `stopped`/`blocked`. Every failure names `path:line`.

Contract 001 closeout: the docs check is the authority; the hook is
enforcement.

Validation: `effigy qa:docs` passed; `git diff --check` clean. Throwaway clone
push of card 111 listed under `## Ready` was refused:
`docs/roadmaps/g05/batch-cards/README.md:5`. A planned annotation containing
later `stopped` now passes. The hook unsets `GIT_DIR` and related vars before
`qa:docs` so nested git in the number-collision tests is not the pushed repo.
Path collection failures refuse the push. `qa:docs` runs only when the pushed
commit is HEAD and `git status --porcelain` is empty, so an untracked card
cannot hide missing-file drift. Tag refs are skipped so a Contract 036
annotated tag at a non-HEAD SHA can push. `qa:docs` now runs
`scripts/tests/release-version-identity.sh` so the Card 110 strict-older
fixture runs on every docs push; the live-tree `0.4.2` assertion is gone so
the next release lane does not fail the docs hook. Rebase onto Card 110
`258574a6` inherited Card 104's unrecognized Status token `review`; Status
is `complete` and the index line moved with PR 251 at `944a4285`.
`## Stopped` is a recognised batch-card index section; `stopped` Status maps
only to it. Card 087 stays `Status: stopped` and the index line moved out of
Planned. Hermetic status fixtures inject via `--root`; the checker ignores
`SWALLOWTAIL_STATUS_CHECK_ROOT`, and pre-push unsets it before `qa:docs`.
`scripts/README.md` is owned by card 110; the new scripts are not
listed there.
