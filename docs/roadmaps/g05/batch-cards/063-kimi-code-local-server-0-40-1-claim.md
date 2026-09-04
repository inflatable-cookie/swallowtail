# 063 Kimi Code Local Server 0.40.1 Claim

Status: planned; gated behind card 062 admitting a Contract 029 segment
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../026-kimi-code-local-server-0-40-1-useful-newer.md`
Depends on: completed card 062 with an admitted Contract 029 segment

## Goal

Extend only the `kimi-code.local-server` claim proved by card 062, validate,
and stop for exact-head review.

## Scope

1. Apply exactly what card 062 proved: extend the latest maintained segment,
   add a private milestone, or add a new revision. Do not force the
   installed-harness ceiling onto this family.
2. Preserve every existing exact point, gap, and deprecated segment on the
   claim.
3. Preserve `kimi-code.acp` and `kimi-code.headless` claims, fixtures,
   guides, and matrix cells exactly.
4. Use the first unpublished later stable as the synthetic `UnverifiedNewer`
   point and say so where the repository records that convention.
5. Update selection tests, the route and feature matrix cells, the Kimi local
   server prepared integration guide, architecture ceilings that name this
   bound, `CHANGELOG.md` `[Unreleased]`, the standing-lane claim paragraph,
   and one claim log.
6. Add mutation-sensitive tests that fail if the boundary point moves, the
   authority conclusion from card 062 is contradicted, or the excluded points
   become admissible.
7. Keep public API changes at zero. Stop rather than invent shared types.
8. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, ACP or headless changes, provider contact, live server, host
update, feature-specific widening, g05.009 card 034, or release work.

## Acceptance Criteria

- only the admitted segment changes
- every negative point and gap survives
- guide, matrix, changelog, standing lane, and log agree with `selection.rs`
- focused, package, API, route, docs, and Northstar gates pass

## Validation

- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-kimi`
- `effigy qa:routes`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Stop for exact-head review.
