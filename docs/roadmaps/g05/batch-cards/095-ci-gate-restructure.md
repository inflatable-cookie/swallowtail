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

## Result

Implemented the bounded CI restructure in `.github/workflows/ci.yml` and
`.config/nextest.toml`:

- split stable format/lint, nextest, process-spawning nextest, and contracts
  into parallel jobs;
- put non-target checks on `ubuntu-latest`, retaining nextest and the pinned
  floor on `macos-latest`;
- count-partitioned ordinary nextest across three macOS runners and isolated
  the six process-spawning binaries in a dedicated profile/job;
- kept the pinned MSRV clippy check on pull requests and its full test on
  `main` pushes, tag pushes, and workflow dispatch;
- added `main` pushes to the existing CI triggers without changing branch
  protection.

Before timing evidence: representative green PR 226 run [33958420553](https://github.com/inflatable-cookie/swallowtail/actions/runs/33958420553)
on `11be445f7ccb4bcaa1a358da1ff1522a3b9a3c7d` took 9m14s wall-clock;
the stable job took 4m35s and the pinned floor took 9m06s. The after timing
will be recorded from this card's green PR run before exact-head review.

Required-check names for the new layout:

- Stable format and lint
- Stable nextest (shard 1/3), Stable nextest (shard 2/3), Stable nextest (shard 3/3)
- Stable process-spawning nextest
- Documentation and semantic API
- Roadmap number uniqueness
- Pinned MSRV floor
- Dependency security, licenses, and sources
- External Git-source consumer

Branch protection was not changed; any required-check policy change remains a
coordinator decision.
