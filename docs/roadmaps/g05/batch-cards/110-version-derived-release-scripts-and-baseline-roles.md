# 110 Version-Derived Release Scripts And Baseline Roles

Status: planned; after the `v0.4.3` tag
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

- [ ] no version literal in the five scripts; they pass on `main` unchanged
- [ ] baseline roles documented; stale directory removed
- [ ] `effigy package:api` and the other gates green

## Validation

- `effigy package:api`; `effigy package:metadata`; `effigy qa:routes`; the front-door script
- `git diff --check`

## Review Oracle

See the g05.034 manifest row.

## Auto-Continuation

No. Stop for exact-head review.
