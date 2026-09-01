# 038 Claude Code 2.1.252 Claim

Status: ready
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../014-claude-code-2-1-252-useful-newer.md`
Depends on: completed card 037 with an admitted Contract 029 segment

## Goal

Extend only the Claude Code headless and response-only claims proved by
card 037, validate, and stop for exact-head review.

## Scope

1. Raise the latest qualified point only to exact `2.1.252` for a compatible
   extension on both axes.
2. Preserve unpublished `2.1.244` and `2.1.249`.
3. Preserve maximum-turn and every other feature-specific exact set at
   independently proved points.
4. Keep watcher exact `2.1.251`. Do not widen watcher help, digest, or live
   authorization.
5. Use unpublished `2.1.253` as the synthetic later `UnverifiedNewer` point.
6. Update selection tests, route and feature matrices, Claude guide,
   changelog, standing lane, and one claim log.
7. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, provider contact, install, live probe, watcher
authorization, skill, projection, papercut, Research 213, release, or
unrelated cleanup.

## Acceptance Criteria

- the range admits only the card-037 segment
- unpublished gaps, watcher exact pin, and feature-specific sets remain
  truthful
- production docs and matrices match code
- current-main validation passes and the PR is mergeable

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g05
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or consumer
checks.

## Stop Conditions

- card 037 records stop or new-driver-or-facade
- applying the identity disposition needs a new contract or public operation
- qualification would silently widen a feature-specific exact version set
  or watcher authorization
- the official point moves before the claim is complete

## Auto-Continuation

No. Review and merge. Same-repo merge order is serial: PR 157 lands before
this implementation PR. Do not start a second family from this PR.
