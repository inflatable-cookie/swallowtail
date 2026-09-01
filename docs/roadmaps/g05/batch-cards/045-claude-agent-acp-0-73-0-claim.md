# 045 Claude Agent ACP 0.73.0 Claim

Status: completed
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../018-claude-agent-acp-0-73-0-useful-newer.md`
Depends on: completed card 044 with an admitted Contract 029 segment

## Goal

Extend only the Claude Agent ACP claim proved by card 044, validate, and
stop for exact-head review.

## Scope

1. Raise the latest qualified point to exact `0.73.0` for a compatible
   extension of v7. Qualify published intermediates `0.71.0`, `0.72.0`,
   and `0.73.0`.
2. Preserve unpublished exclusion `0.58.0` and `AllowUnverified`.
3. Use unpublished `0.74.0` as the synthetic later `UnverifiedNewer` point.
4. Keep claim id `claude-agent.acp.window-2` and baseline `0.53.0`.
5. Update selection tests, route and feature matrices, Claude Agent guide,
   architecture, contracts that already state this window, changelog,
   standing lane, and one claim log.
6. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, provider contact, install, live probe, Claude Code,
watcher, skill, projection, papercut, g05.009 card 034, release, or
unrelated cleanup.

## Acceptance Criteria

- the range admits only the card-044 segment
- unpublished `0.58.0`, `AllowUnverified`, and other families remain truthful
- production docs and matrices match code
- current-main validation passes and the PR is mergeable

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
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

- card 044 records stop or new-driver-or-facade
- applying the identity disposition needs a new contract or public operation
- the official point moves before the claim is complete

## Auto-Continuation

No. Review and merge. Do not start a second family from this PR.

## Result

The Claude Agent ACP axis qualifies official `0.71.0`, `0.72.0`, and
`0.73.0` as a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`. Unpublished `0.58.0`
remains incompatible. `AllowUnverified` remains. Unpublished `0.74.0` is
the synthetic later `UnverifiedNewer` point. Host `0.63.0` stays
observation-only. Claude Code and the watcher stay untouched. Identity
evidence remains commit `af9ddfd4`.
