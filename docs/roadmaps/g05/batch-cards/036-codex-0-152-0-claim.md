# 036 Codex 0.152.0 Claim

Status: completed
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../013-codex-0-152-0-useful-newer.md`
Depends on: completed card 035 with an admitted Contract 029 segment

## Goal

Extend only the Codex exec and app-server claim proved by card 035, validate,
and stop for exact-head review.

## Scope

1. Raise the latest qualified point only to exact `0.152.0` for a compatible
   extension.
2. Preserve model-verbosity, fast-mode, personality, plan-mode-effort, and all
   feature-specific sets at their independently proved points.
3. Preserve unpublished and incompatible gaps.
4. Update selection/compatibility fixtures, route and feature matrices, Codex
   guide, changelog, standing lane, and one claim log.
5. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, provider contact, install, live probe, watcher, skill,
projection, papercut, release, or unrelated cleanup.

## Acceptance Criteria

- the range admits only the card-035 segment
- exact feature pins and gaps remain truthful
- production docs and matrices match code
- current-main validation passes and the PR is mergeable

## Validation

- `cargo fmt -p swallowtail-adapter-codex -- --check`
- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Review and merge, then queue the next all-route checkpoint.

## Result

The Codex exec and app-server axes qualify official `0.152.0`. Unpublished
gaps including `0.151.1` remain incompatible. Model verbosity and every other
feature-specific exact set remain bounded through `0.149.1`. Focused,
affected-package, semantic API, route, docs, Northstar, format, and diff
gates pass without provider work. Identity evidence remains commit
`68a1099f`.
