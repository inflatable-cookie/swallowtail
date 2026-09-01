# 040 Pi RPC 0.84.4 Claim

Status: ready
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../015-pi-rpc-0-84-4-useful-newer.md`
Depends on: completed card 039 with an admitted Contract 029 segment

## Goal

Extend only the Pi RPC `pi.package` claim proved by card 039, validate,
and stop for exact-head review.

## Scope

1. Raise the latest qualified point only to exact `0.84.4` for a compatible
   extension of `pi.rpc.strict-lf-v0.84.0-message-update-delta`.
2. Preserve unpublished `0.83.1`.
3. Preserve `pi.sdk-sidecar` exact `0.84.2` package, Node, wire, and
   source-tag axes.
4. Use unpublished `0.84.5` as the synthetic later `UnverifiedNewer` point.
5. Update selection tests, route and feature matrices, Pi RPC guide,
   architecture ceilings that name this bound, changelog, standing lane,
   and one claim log.
6. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, provider contact, install, live probe, sidecar pin change,
Oh My Pi, Gemini, skill, projection, papercut, g05.009 card 034, release,
or execution of downloaded official binaries.

## Acceptance Criteria

- the range admits only the card-039 segment
- unpublished gaps and sidecar exact pin remain truthful
- production docs and matrices match code
- current-main validation passes and the PR is mergeable

## Validation

```sh
cargo fmt -p swallowtail-adapter-pi
effigy validate:focused swallowtail-adapter-pi
effigy package:verify-affected swallowtail-adapter-pi
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

- card 039 records stop or new-driver-or-facade
- applying the identity disposition needs a new contract or public operation
- qualification would silently raise the sidecar pin or flatten onto Oh My Pi
- the official point moves before the claim is complete

## Auto-Continuation

No. Review and merge. Do not start a second family from this PR.
