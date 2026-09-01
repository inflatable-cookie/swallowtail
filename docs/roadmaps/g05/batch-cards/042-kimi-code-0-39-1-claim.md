# 042 Kimi Code 0.39.1 Claim

Status: ready
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../016-kimi-code-0-39-1-useful-newer.md`
Depends on: completed card 041 with at least one admitted Contract 029 segment

## Goal

Extend only the `kimi-code.executable` claims proved by card 041, validate,
and stop for exact-head review.

## Scope

1. Apply only what card 041 proved, per axis. Encode the honest split if one
   axis qualifies and another does not. Do not force a family-wide ceiling.
   A correction that lowers a wrong bound is in scope; so is a stop that
   excludes exact newer points.
2. Preserve every existing negative point and exact gap on both claims.
3. Preserve the `kimi-code.local-server` claim, fixtures, route, guide, and
   matrix cell exactly.
4. Use the first unpublished later stable as the synthetic `UnverifiedNewer`
   point, and say so where the repository records that convention. Where an
   axis stops, encode the exact newer points as exclusions so
   `AllowUnverified` cannot admit them, and prove the classification.
5. Update selection tests, route and feature matrices, the Kimi prepared
   integration guide, architecture ceilings that name this bound, changelog,
   standing lane, and one claim log.
6. Add cross-corpus proof modules rather than growing an existing file. Hold
   the current god-file baseline.
6a. Add mutation-sensitive routing and authority tests that fail if the
   boundary point moves, the legacy-flag logic flips, `system.version` is
   treated as v1, an ambient legacy-flag assumption is invented, a historical
   fixture is left contradictory, or the excluded ACP points become
   admissible.
7. Keep public API changes at zero. Stop rather than invent shared types.
8. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, local-server claim change, Kimi Platform Chat, provider
contact, authentication, install, live probe, Gemini, skill, projection,
papercut, g05.009 card 034, release, or execution of downloaded official
binaries.

## Acceptance Criteria

- each range admits only the card-041 segment for its own axis
- exact gaps, negative points, and the unpublished-stable convention stay truthful
- production docs and matrices match code
- public API surface is unchanged
- god-file count does not rise
- current-main validation passes and the PR is mergeable

## Validation

```sh
cargo fmt -p swallowtail-adapter-kimi
effigy validate:focused swallowtail-adapter-kimi
effigy package:verify-affected swallowtail-adapter-kimi
effigy package:api
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g05
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy --json scan god-files
git diff --check
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or consumer
checks.

## Stop Conditions

- card 041 records stop or new-driver-or-facade
- applying the identity disposition needs a new contract or public operation
- qualification would widen `kimi-code.local-server` or flatten onto it
- qualification would answer the g05.009 provider-operation observation gate
- the official point moves before the claim is complete

## Auto-Continuation

No. Review and merge. Do not start a second family from this PR.

## Result

Pending.
