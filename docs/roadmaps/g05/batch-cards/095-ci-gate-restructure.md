# 095 CI Gate Restructure

Status: complete; PR 230 merged as `ba8275eb`; PR gate 9m14s to 4m48s
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
the stable job took 4m35s and the pinned floor took 9m06s.

After timing evidence: representative green PR 230 run [33971014129](https://github.com/inflatable-cookie/swallowtail/actions/runs/33971014129)
on `af86aa9ee576b555d8ef64a0183374f17c5d1fdd` took 4m48s wall-clock. The
stable format/lint job took 1m57s, nextest shards took 3m28s, 3m23s, and
2m23s, the process-spawning shard took 2m58s, and the pinned floor took 1m53s
with its PR `Test` step skipped. The longest required job was Documentation
and semantic API at 4m45s. The representative wall-clock round trip fell by
4m26s (48%).

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

Post-merge verification: main-push run [33971762581](https://github.com/inflatable-cookie/swallowtail/actions/runs/33971762581)
on `ba8275ebf7623724ec61398acd1117d9c5c08c9a` was cancelled. Its `Pinned
MSRV floor` job entered the `Test` step at 14:26:48Z and the step ended
`cancelled` at 14:29:49Z, so the pinned MSRV `Test` step did execute but did
not complete successfully. The follow-up keeps PR cancellation enabled and
makes only a push to `main` non-canceling.
