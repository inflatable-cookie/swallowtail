# 133 v0.1.1 Exact CI Candidate

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../044-v0-1-1-source-patch-release.md`
Depends on: card 132

## Goal

Prove the exact clean `v0.1.1` release commit through canonical GitHub CI.

## Scope

1. Push the accepted release commit to canonical `main`.
2. Dispatch the existing source-release workflow for that exact commit.
3. Require every stable, floor, Bedrock, docs/API, supply-chain, and external
   consumer job to pass.
4. Record the exact commit and run URL without creating a tag.

## Acceptance

- [x] remote `main` resolves to the accepted release commit
- [x] all six workflow jobs pass against that exact SHA
- [x] no tag, GitHub Release, registry, consumer, or provider mutation runs

## Stop Conditions

- stop on any CI failure; the difference from local gates is the next finding
- never tag a commit other than the exact green candidate

## Auto-Continuation

Yes. The operator explicitly authorized the patch release. Continue to card
134 only after exact green CI evidence exists.

## Completion Evidence

- release commit `bd3f4bbdffc403897ece4499ee0904b1e8116639` was pushed to
  canonical `main` before tag creation
- GitHub Actions run `31107478654` passes all six jobs against that exact SHA
- stable, Rust 1.90, Bedrock Rust 1.94.1, docs/API, supply-chain, and external
  Git-source consumer jobs pass
- no tag existed during this card; no registry, GitHub Release, consumer, or
  provider mutation ran
