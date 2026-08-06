# 142 v0.2.0 Annotated Source Tag

Status: planned
Owner: Tom
Created: 2026-08-06
Milestone: `../046-v0-2-0-muse-and-rust-floor-source-release.md`
Depends on: card 141

## Goal

After separate exact authorization, create and push one annotated `v0.2.0`
source tag at the exact CI-green release commit.

## Scope

1. Reconfirm clean local and remote candidate identity and tag absence.
2. Preview the exact Effigy execution mutation.
3. Create and push annotated `v0.2.0` without a GitHub Release or registry
   publication.
4. Verify local and remote annotated-tag identity and close release evidence.

## Acceptance

- [ ] local and remote peeled tag resolve to the exact green release commit
- [ ] the tag annotation is `v0.2.0`
- [ ] `v0.1.0` and `v0.1.1` remain unchanged
- [ ] no crates.io, GitHub Release, binary, consumer, or provider mutation runs

## Stop Conditions

- stop without a separate exact operator authorization
- stop if the tag exists, candidate identity drifts, or execution plans any
  excluded side effect
- never move or recreate a failed or partially published tag

## Auto-Continuation

No. Stop after exact tag and closeout evidence.
