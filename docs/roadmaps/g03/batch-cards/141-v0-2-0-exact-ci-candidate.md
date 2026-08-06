# 141 v0.2.0 Exact CI Candidate

Status: ready
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

- [ ] remote `main` resolves to the accepted release commit
- [ ] all five workflow jobs pass against that exact SHA
- [ ] no tag, GitHub Release, registry, consumer, or provider mutation runs

## Stop Conditions

- the operator authorized the Rust-floor workflow replacement on 2026-08-06
- stop on any CI failure
- never tag a commit other than the exact green candidate

## Auto-Continuation

No. Await separate exact tag authorization after CI passes.
