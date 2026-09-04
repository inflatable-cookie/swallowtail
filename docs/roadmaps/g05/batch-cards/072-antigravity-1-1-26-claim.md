# 072 Antigravity 1.1.26 Claim

Status: planned; gated behind card 071 admitting a Contract 029 segment
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../027-antigravity-1-1-26-useful-newer.md`
Depends on: completed card 071 with an admitted Contract 029 segment

## Goal

Extend only the Antigravity catalogue and headless claims proved by card
071, validate, and stop for exact-head review.

## Scope

1. Apply exactly what card 071 proved: extend Maintained through the
   admitted ceiling on both claims, qualify every published intermediate
   hop it corroborated, and keep `1.1.8` incompatible.
2. Keep baseline `1.1.9`, both claim ids, both behavior revisions, and
   `AllowUnverified`. Raise `ANTIGRAVITY_LATEST_QUALIFIED_VERSION` only to
   the admitted ceiling.
3. Use the first unpublished later stable as the synthetic
   `UnverifiedNewer` point and say so where the repository records that
   convention.
4. Update selection tests, the route and feature matrix cells, the
   Antigravity prepared integration guide, architecture ceilings that name
   this bound, `CHANGELOG.md` `[Unreleased]`, the standing-lane claim
   paragraph, and one claim log.
5. Add mutation-sensitive tests that fail if the boundary moves, a gap
   closes, or an unqualified hop becomes admissible.
6. Keep public API changes at zero. Do not touch the Contract 061
   projection code. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, ACP-registry `antigravity-acp`, Gemini, provider contact,
host update, feature-specific widening, or release work.

## Acceptance Criteria

- only the admitted segment changes
- every negative point and gap survives
- guide, matrix, changelog, standing lane, and log agree with `selection.rs`
- focused, package, API, route, docs, and Northstar gates pass

## Validation

- `effigy validate:focused swallowtail-adapter-antigravity`
- `effigy package:verify-affected swallowtail-adapter-antigravity`
- `effigy qa:routes`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Stop for exact-head review.
