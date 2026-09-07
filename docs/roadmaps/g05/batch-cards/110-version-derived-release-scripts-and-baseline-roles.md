# 110 Version-Derived Release Scripts And Baseline Roles

Status: complete; PR 252 merged at `258574a6`
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../034-release-lane-simplification.md`
Depends on: the g05.034 measured causes table

## Goal

Remove the manual version repointing and the baseline-directory confusion that cost review rounds on both patch lanes.

## Scope

1. The four gate scripts and the consumer front-door script derive the current version from `Cargo.toml` (`workspace.package.version`) and the immutable previous version from the newest tagged baseline directory; no version literal remains.
2. `release-baselines/README.md` documents the two roles: `public-api-<current>` is the working baseline that absorbs additive API between tags; `public-api-<previous>` is immutable and guards removals. Delete `public-api-unreleased`.
3. Root `README.md` release-posture lines derive or are removed from the gate set.
4. Card 101/106's "scripts repointed" step disappears from the release manifest template.

## Acceptance Criteria

- [x] no version literal in the five scripts; they pass on `main` unchanged
- [x] baseline roles documented; stale directory removed
- [x] `effigy package:api` and the other gates green

## Validation

- `effigy package:api`; `effigy package:metadata`; `effigy qa:routes`; the front-door script
- `git diff --check`

## Review Oracle

See the g05.034 manifest row.

## Auto-Continuation

No. Stop for exact-head review.

## Result

PR 252. First head `10e5859c` accepted direction with two merge blockers, now
fixed: previous is the greatest tagged `public-api-*` directory strictly older
than current (fixture: `public-api-0.5.0` cannot become previous when current
is `0.4.3`); front-door unchanged-set fallbacks again require both package
phrases and all three route phrases, substituting only `previous_tag`.

Named gates and the identity fixture are green. `PAPERCUTS.md` left open:
manifest is append-only, and Contract 036 remains Card 109. The scripts-repointed
playbook line is Card 109. No merge, tag, or live action.
