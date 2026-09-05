# 095 CI Gate Restructure

Status: ready; `v0.4.1` is tagged; operator workflow authority granted 2026-09-05
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../031-ci-latency.md`
Depends on: the `v0.4.1` tag; operator workflow authority of 2026-09-05

## Goal

Bring the pull-request gate to about five minutes without weakening the
release floor.

## Scope

1. Split "Stable format, lint, test, and guides" into parallel jobs: format
   plus both clippy passes; nextest; examples plus metadata plus route and
   guide contracts. Share `Swatinem/rust-cache` keys so each restores the
   same target.
2. Shard the nextest job with `--partition count:N/M` across three runners,
   and give the process-spawning suites (Claude Agent sidecar asset and SDK
   driver, Pi sidecar driver, and any other test that spawns Node or a fake
   native child) their own shard via a nextest filter.
3. Make "Pinned MSRV floor" clippy-only on `pull_request`; run its full
   `cargo test` on `push` to `main` and on `workflow_dispatch`. The release
   lane's exact-SHA workflow-dispatch run therefore keeps the complete
   floor.
4. Move format, clippy, examples, metadata, route guides, and the semantic
   API job to `ubuntu-latest`; keep the nextest shards and the MSRV floor on
   `macos-latest` as verified-target evidence.
5. Keep every existing step somewhere on every PR; remove nothing. Document
   the resulting required-check names in the PR and get the coordinator's
   agreement before changing branch protection.
6. Measure before and after on one representative PR and record both in
   the card result.

## Out Of Scope

Test content; `config/release.toml` gates; branch-protection changes
without coordinator agreement; any production source.

## Acceptance Criteria

- [ ] representative PR all-green in about five minutes
- [ ] full macOS pinned floor still runs on `main` pushes and dispatches
- [ ] every prior step still runs on every PR
- [ ] required-check set documented; strength unchanged

## Validation

- the workflow runs green on the card's own PR
- `effigy qa:docs`, `effigy qa:northstar`, `git diff --check`

## Auto-Continuation

No. Stop for exact-head review.
