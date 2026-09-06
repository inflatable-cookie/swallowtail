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

- [x] PR gate under 3 min on two consecutive runs; dispatch run under 5
- [x] every current check still present
- [x] timings recorded in the card Result

## Validation

- two green PR runs and one green workflow-dispatch run on the branch
- `git diff --check`

## Review Oracle

See the g05.034 manifest row.

## Auto-Continuation

No. Stop for exact-head review.

## Result

### What changed

`.github/workflows/ci.yml` and `.config/nextest.toml` only.

- Pinned MSRV floor moved to `ubuntu-latest` and split into two jobs.
  `Pinned MSRV floor` runs `cargo +1.95.0 clippy` on every event.
  `Pinned MSRV floor tests` runs `cargo +1.95.0 test --workspace
  --all-features` and is gated to `main`, workflow dispatch, and tags, as
  before. They now run beside each other instead of in series.
- The non-process nextest shards moved to `ubuntu-latest` and dropped from
  three count partitions to two. `ci-process` stays on `macos-latest`.
- The macOS process job selects the three packages that own the six
  `ci-process` binaries instead of building the whole workspace. The Linux
  shards still run `--workspace` under the `ci` profile, which excludes
  exactly those six binaries, so no test moved and none is covered twice.
- Cache audit: the nextest jobs and the doc/public-API job each got their
  own `shared-key`; `cache-on-failure: true` on every `rust-cache` use.

Job names changed: `Stable nextest (shard N/3)` became `(shard N/2)`, and
`Pinned MSRV floor tests` is new. `main` has no branch protection, so no
required-check list needed updating.

### The measurement that drove the shape

macOS shard 3/3 on the baseline run 34052744897: `Finished test profile
in 2m 44s`, then `Summary [17.133s] 893 tests run`. Build and link is 91%
of the job. Test duration is not the cost, so rebalancing partitions by
binary duration cannot pay: each partition builds and links the whole
workspace to run a third of the tests. The third shard bought about 10
seconds of test wall clock for a full duplicate build, so it went.

For the same reason a shared nextest archive was not adopted. It would
serialise one build, an upload, and a download ahead of every shard that
today builds in parallel from a warm cache, against a test phase measured
in seconds. No evidence supports it here; revisit if card 112 changes the
build/run ratio.

Cache finding: the shards previously shared `shared-key: stable` with the
lint and contract jobs. That was harmless while they ran on macOS, where
no other job used the key. On Linux the same key holds check-only
artifacts, which are not reusable for a codegen build, so the test jobs
needed their own key.

### Timings

Before (3 macOS shards, MSRV floor on macOS):

| Run | Event | Wall clock | Critical path |
| --- | --- | --- | --- |
| 34051758371 | pull request | 3m27s | nextest shards |
| 34052333044 | pull request | 4m17s | shard 2/3 4m13s |
| 34052744897 | pull request | 4m07s | shard 3/3 3m56s |
| 34053787506 | workflow dispatch | 8m24s | MSRV floor macOS 5m23s, plus ~3m macOS runner queue |

After:

| Run | Event | Wall clock | Notes |
| --- | --- | --- | --- |
| 34057083777 | pull request | 5m24s | cold: every new cache key missed |
| 34057405212 | pull request | 2m40s | warm; docs/API 2m36s is now the longest job |
| 34057905627 | pull request | failed | timing flake, below |
| 34057557650 | workflow dispatch | 5m53s | MSRV clippy 1m38s then test 3m51s in series |
| 34057905516 | workflow dispatch | 4m25s | after the split; MSRV tests 4m20s on a cold key |
| 34058205228 | pull request | 2m40s | green; shards 2m11s and 2m05s, macOS process job 2m15s |
| 34058373009 | pull request | 2m23s | green; shards 2m18s each, macOS process job 1m43s |

Target met: two consecutive green PR runs under 3 minutes (34058205228
at 2m40s and 34058373009 at 2m23s)
and a green workflow-dispatch run at 4m25s. The macOS runner queue that
cost the baseline dispatch about 3 minutes largely went away with the
macOS job count down from four to one.

The commit that records these numbers gets a run of its own; it is the
exact-head run for review and is expected to sit in the same band. Every
timing above is the run's own `createdAt` to `updatedAt`, so queue time is
counted.

### Flake surfaced, not owned here

Run 34057905627 failed on
`swallowtail-adapter-anthropic::managed_prepared_facade
prepared_interrupt_deletes_owned_resources_before_credential_release`:
`remote_state_unconfirmed` instead of `Cancelled` after 4.73s of bounded
recovery. The same SHA passed the same shard in dispatch run 34057905516,
and the test passed on Linux in both earlier runs. This is the bounded
wall-clock class cards 093, 094, and 104 own; `crates/` is a forbidden
path for this card, so it is recorded here and in `PAPERCUTS.md` rather
than fixed. No retry policy was added: the milestone forbids
rerun-to-green.
