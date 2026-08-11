# 213 v0.3.2 Annotated Source Tag

Status: planned
Owner: Tom
Created: 2026-08-11
Milestone: `../067-v0-3-2-source-patch-release.md`
Depends on: card 212; separate exact operator authorization

## Goal

Create and push one annotated immutable `v0.3.2` source tag at the exact
CI-green candidate commit.

## Acceptance

- [ ] local and remote peeled tag resolve to the exact green commit
- [ ] earlier tags remain unchanged
- [ ] no crates.io, GitHub Release, binary, consumer, or provider mutation
      runs

## Auto-Continuation

No. Stop after exact tag and closeout evidence.
