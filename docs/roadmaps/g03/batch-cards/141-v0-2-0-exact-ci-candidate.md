# 141 v0.2.0 Exact CI Candidate

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../046-v0-2-0-muse-and-rust-floor-source-release.md`
Depends on: card 140

## Goal

Prove the exact clean `v0.2.0` release commit through canonical GitHub CI.

## Scope

1. Commit and push the operator-accepted release tree to canonical `main`.
2. Require every stable, unified Rust `1.95.0` floor, docs/API, supply-chain,
   and external-consumer job to pass against the exact commit.
3. Record the commit and workflow evidence without creating a tag.

## Acceptance

- [x] remote `main` resolves to the accepted release commit
- [x] all five workflow jobs pass against that exact SHA
- [x] no tag, GitHub Release, registry, consumer, or provider mutation runs

## Stop Conditions

- the operator authorized the Rust-floor workflow replacement on 2026-08-06
- stop on any CI failure
- never tag a commit other than the exact green candidate

## Auto-Continuation

No. Await separate exact tag authorization after CI passes.

## Completion Evidence

- release commit: `0104b8948ad141f5c42ad752127203b9b1d72db5`
- GitHub CI run `31128930466` completed successfully with all five jobs
- local `HEAD`, remote `main`, and the workflow head SHA matched exactly
- no tag or other excluded release mutation ran during this card
