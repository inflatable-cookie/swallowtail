# 113 CI Critical Path

Status: planned; ready now; operator workflow authority from card 095 carries
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../034-release-lane-simplification.md`
Depends on: the g05.034 measured causes table

## Goal

PR gate under three minutes; hosted run under five, without losing a check.

## Scope

1. Pinned MSRV floor (5.4 min, macOS, the critical path): move to `ubuntu-latest`; on pull requests run `cargo +msrv clippy` only, `cargo test` at MSRV only on `main` and workflow dispatch. MSRV is a compile floor, not a platform check.
2. nextest shards: rebalance by binary duration after card 112; move shards that spawn no processes to Linux; keep `ci-process` on macOS.
3. Cache audit: verify `Swatinem/rust-cache` hit rates per job; add `cache-on-failure`; share the nextest archive between shards if it pays.
4. Record PR gate and dispatch wall clock on two runs each, before and after.

## Acceptance Criteria

- [ ] PR gate under 3 min on two consecutive runs; dispatch run under 5
- [ ] every current check still present
- [ ] timings recorded in the card Result

## Validation

- two green PR runs and one green workflow-dispatch run on the branch
- `git diff --check`

## Review Oracle

See the g05.034 manifest row.

## Auto-Continuation

No. Stop for exact-head review.
