# 049 Codex 0.152.1 Claim

Status: ready
Owner: Tom
Created: 2026-09-02
Updated: 2026-09-02
Milestone: `../020-codex-0-152-1-useful-newer.md`
Depends on: completed card 048 with an admitted Contract 029 segment

## Goal

Extend only the Codex exec and app-server claim proved by card 048, validate,
and stop for exact-head review.

## Scope

1. Raise the latest qualified point only to exact `0.152.1` for a compatible
   extension.
2. Preserve model-verbosity, fast-mode, personality, plan-mode-effort, and
   all feature-specific sets at their independently proved points.
3. Preserve unpublished and incompatible gaps.
4. Update selection/compatibility fixtures, route and feature matrices, Codex
   guide, changelog, standing lane, and one claim log.
5. Keep identity and claim as two commits in one PR.

## Out Of Scope

Another family, provider contact, install, live probe, watcher, skill,
projection, papercut, release, execution of downloaded binaries, or
unrelated cleanup.

## Acceptance Criteria

- the range admits only the card-048 segment
- exact feature pins and gaps remain truthful
- production docs and matrices match code
- current-main validation passes and the PR is mergeable

## Validation

- `cargo fmt -p swallowtail-adapter-codex -- --check`
- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Review and merge, then queue the next all-route checkpoint.

## Result

Pending card 048's admitted segment.
