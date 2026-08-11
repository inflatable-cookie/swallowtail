# 213 v0.3.2 Annotated Source Tag

Status: completed
Owner: Tom
Created: 2026-08-11
Milestone: `../067-v0-3-2-source-patch-release.md`
Depends on: card 212; separate exact operator authorization

## Goal

Create and push one annotated immutable `v0.3.2` source tag at the exact
CI-green candidate commit.

## Acceptance

- [x] local and remote peeled tag resolve to the exact green commit
- [x] earlier tags remain unchanged
- [x] no crates.io, GitHub Release, binary, consumer, or provider mutation
      runs

## Completion Evidence

- annotated tag object: `702f355631bb6fe8fe6cb098f48887df8ef8ca43`
- peeled release commit: `a859d56b47b1bc2975df7d0516ca96fd8e485b35`
- local and remote tag objects and peeled commits match
- tag CI run `31536392314` passed all five jobs at immutable ref `v0.3.2`
- earlier tag objects `v0.1.0` through `v0.3.1` remain unchanged
- Effigy execution stayed fail-closed against committed prepared-state drift;
  the accepted explicit annotated-tag fallback created only the tag

## Auto-Continuation

No. Stop after exact tag and closeout evidence.
