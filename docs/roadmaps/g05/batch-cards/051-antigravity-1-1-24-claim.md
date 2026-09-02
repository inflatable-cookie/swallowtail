# 051 Antigravity 1.1.24 Claim

Status: completed
Owner: Tom
Created: 2026-09-02
Updated: 2026-09-02
Milestone: `../021-antigravity-1-1-24-useful-newer.md`
Depends on: completed card 050 with an admitted Contract 029 segment

## Goal

Extend only the Antigravity catalogue and headless claims proved by
card 050, validate, and stop for exact-head review.

## Scope

1. Raise `ANTIGRAVITY_LATEST_QUALIFIED_VERSION` from `1.1.17` to exact
   `1.1.24` for a compatible extension on both claims.
2. Keep baseline `1.1.9`, claim ids `release-window-1`, both behavior
   revisions, and `AllowUnverified`.
3. Qualify published intermediates `1.1.18` through `1.1.23`. Keep `1.1.8`
   incompatible. Use unpublished `1.1.25` as synthetic `UnverifiedNewer`.
4. Update adapter tests, Antigravity guide, route and feature matrices,
   changelog Unreleased, and one claim log.
5. Keep identity and claim as two commits in one PR.
6. Do not edit Next Task, generation runway state, standing-lanes Next
   Task, or architecture.

## Out Of Scope

Another family, provider contact, install, live probe, Gemini deferral,
Claude Code, Next Task, generation pointer, execution of downloaded
binaries, or unrelated cleanup.

## Acceptance Criteria

- the range admits only the card-050 segment
- gaps and unmapped extras remain truthful
- production docs and matrices match code
- focused adapter validation passes
- Next Task / northstar-loop state stay unchanged

## Validation

```sh
cargo fmt -p swallowtail-adapter-antigravity -- --check
effigy validate:focused swallowtail-adapter-antigravity
effigy package:verify-affected swallowtail-adapter-antigravity
```

If `qa:docs:index:roadmaps`, `qa:docs:index:roadmaps:g05`, or
`qa:docs:next-action:roadmaps` fail because Next Task still names the
post-Codex checkpoint or because a worker is in flight, leave the
generation docs alone.

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or
consumer checks.

## Auto-Continuation

No. Review and merge. Do not change Next Task. Do not start a second
family from this PR.

## Result

The Antigravity catalogue and headless axes qualify official `1.1.24`.
Published intermediates `1.1.18` through `1.1.23` are qualified. `1.1.8`
stays incompatible. Unpublished `1.1.25` stays `UnverifiedNewer`.
`AllowUnverified` remains. Identity evidence remains the identity-only
commit. Next Task and northstar-loop state were not changed.
