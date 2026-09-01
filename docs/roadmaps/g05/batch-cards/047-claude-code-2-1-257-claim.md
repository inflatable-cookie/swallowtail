# 047 Claude Code 2.1.257 Claim

Status: completed
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../019-claude-code-2-1-257-useful-newer.md`
Depends on: completed card 046 with an admitted Contract 029 segment

## Goal

Extend only the Claude Code headless and response-only claims proved by
card 046, validate, and stop for exact-head review.

## Scope

1. Raise the latest qualified point only to exact `2.1.257` for a compatible
   extension on both axes.
2. Preserve unpublished `2.1.244` and `2.1.249`. Add hop-skipped unpublished
   `2.1.253`, `2.1.254`, `2.1.255`, and `2.1.256` to the deny lists.
3. Preserve maximum-turn and every other feature-specific exact set at
   independently proved points.
4. Keep watcher exact `2.1.251`. Do not widen watcher help, digest, or live
   authorization.
5. Use unpublished `2.1.258` as the synthetic later `UnverifiedNewer` point.
6. Update selection tests, route and feature matrices, Claude guide,
   changelog, standing lane, and one claim log.
7. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, provider contact, install, live probe, watcher
authorization, skill, projection, papercut, Research 213, g05.009 card 034,
release, or unrelated cleanup.

## Acceptance Criteria

- the range admits only the card-046 segment
- unpublished gaps, watcher exact pin, and feature-specific sets remain
  truthful
- production docs and matrices match code
- current-main validation passes and the PR is mergeable

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent -- --check
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
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

- card 046 records stop or new-driver-or-facade
- applying the identity disposition needs a new contract or public operation
- qualification would silently widen a feature-specific exact version set
  or watcher authorization
- the official point moves before the claim is complete

## Auto-Continuation

No. Review and merge. Do not start a second family from this PR.

## Result

The Claude Code headless and response-only axes qualify official `2.1.257`.
Unpublished `2.1.244` and `2.1.249` remain incompatible. Hop-skipped
unpublished `2.1.253` through `2.1.256` are gaps. Watcher stays exact
`2.1.251`. Maximum-turn and other feature-specific exact sets remain bounded
through `2.1.241`. Identity evidence remains commit `de3b94a9`.
