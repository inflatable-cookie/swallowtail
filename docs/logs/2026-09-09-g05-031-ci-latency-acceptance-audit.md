# 2026-09-09 g05.031 CI Latency Acceptance Audit

Date: 2026-09-09
Task: `../roadmaps/g05/031-ci-latency.md`
Sources frozen: GitHub Actions/PR reads on 2026-09-09 against
`inflatable-cookie/swallowtail`; workflow and nextest at planning HEAD
`3f26d01b`; pre-Card-095 inventory at `ba8275eb^1` (`1b1b023c`).

## Result

Honest evidence stop. Card 095 made the PR gate typically faster than five
minutes. The original four-clause target is not fully proven.

| Clause | Verdict |
| --- | --- |
| Typical PR all-green in about five minutes | **pass**. Median 2m 43s (163s) over 20 accepted-head PR `CI` runs through PR 305. p75 2m 46s. Range 2m 30s–4m 49s. Both outliers still under five minutes |
| Full macOS pinned floor on `main` pushes and release-candidate dispatches | **fail**. Current `Pinned MSRV floor` and `Pinned MSRV floor tests` both `runs-on: ubuntu-latest`. Reused g05.034 runs execute clippy+test on Linux, not macOS. Only `Stable process-spawning nextest` remains on `macos-latest` |
| Every test that ran before still runs somewhere on every PR | **pass**. Pre-Card-095 nextest default suite maps to current `ci` shards ∪ `ci-process`. The skipped PR `cargo +MSRV test` is a pinned duplicate of that population, not a removed suite |
| Required-check set documented and unchanged in strength | **unproven**. `GET /branches/main/protection` is 404 *Branch not protected*; rulesets are `[]`. Card 095 documented names; GitHub does not enforce them. Job names have drifted since |

This audit does not repair the workflow or reinterpret the macOS floor as
platform-neutral. Coordinator closeout sets `Status: stopped` and reconciles
reserved indexes.

## Clock Semantics

- **Population.** Latest 20 merged PRs through PR 305 with a completed
  pull-request `CI` run at the accepted head (`headRefOid` at merge). No
  merged-PR exclusions. Closed-not-merged #299, #295, #290 are outside the
  merged population. Did not extend backward.
- **User-visible all-green clock.** Workflow run `created_at` → `updated_at`
  at completion. Typical scored from the **median** against about five
  minutes (300s). p75 and range stay visible.
- **Critical-path job span.** Min executed-job `started_at` → max executed-job
  `completed_at`. Skipped jobs (`Pinned MSRV floor tests` on every PR) are
  out of the span.
- **Strata.** `docs` = every changed path is docs/markdown/AGENTS. `mixed` =
  docs plus any of crates, scripts, Effigy, baselines. No pure-code PR in
  this window.
- **No new CI.** Existing runs only.

## Sample

All 20 accepted-head runs `conclusion=success`. Every run skips
`Pinned MSRV floor tests` and keeps `Pinned MSRV floor` clippy green on
`ubuntu-latest`; process-spawning stays on `macos-latest`.

| PR | Stratum | Run | Created UTC | Completed UTC | Wall | Crit | Longest executed job |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 305 | docs | `34386540550` | 18:02:03 | 18:04:42 | 2m 39s | 2m 36s | process-spawning 2m 30s |
| 304 | mixed | `34382391834` | 17:21:20 | 17:24:06 | 2m 46s | 2m 44s | docs/API 2m 43s |
| 303 | docs | `34373452391` | 15:55:40 | 16:00:29 | 4m 49s | 4m 46s | process-spawning 4m 40s |
| 302 | mixed | `34363974685` | 14:29:47 | 14:34:12 | 4m 25s | 3m 22s | docs/API 2m 55s |
| 301 | mixed | `34360730337` | 14:00:13 | 14:02:57 | 2m 44s | 2m 40s | docs/API 2m 35s |
| 300 | docs | `34348370780` | 11:58:24 | 12:00:56 | 2m 32s | 2m 28s | nextest 1/2 2m 28s |
| 298 | mixed | `34340173476` | 10:25:57 | 10:28:38 | 2m 41s | 2m 38s | docs/API 2m 37s |
| 297 | mixed | `34325286198` | 07:43:10 | 07:45:54 | 2m 44s | 2m 40s | docs/API 2m 40s |
| 296 | mixed | `34270852994` | 19:45:52 | 19:48:26 | 2m 34s | 2m 31s | docs/API 2m 30s |
| 294 | mixed | `34250945965` | 16:25:21 | 16:28:05 | 2m 44s | 2m 40s | docs/API 2m 40s |
| 293 | mixed | `34244310105` | 15:22:17 | 15:24:48 | 2m 31s | 2m 27s | nextest 1/2 2m 26s |
| 292 | mixed | `34227013114` | 12:36:14 | 12:38:56 | 2m 42s | 2m 38s | nextest 2/2 2m 38s |
| 291 | mixed | `34224774498` | 12:12:19 | 12:15:08 | 2m 49s | 2m 45s | docs/API 2m 45s |
| 289 | mixed | `34205076716` | 08:32:46 | 08:35:41 | 2m 55s | 2m 43s | docs/API 2m 42s |
| 288 | mixed | `34211370441` | 09:41:16 | 09:43:58 | 2m 42s | 2m 38s | docs/API 2m 38s |
| 287 | mixed | `34210258402` | 09:29:17 | 09:31:47 | 2m 30s | 2m 26s | nextest 2/2 2m 25s |
| 286 | docs | `34202257540` | 08:01:09 | 08:03:56 | 2m 47s | 2m 43s | docs/API 2m 43s |
| 285 | mixed | `34171318676` | 23:49:37 | 23:52:22 | 2m 45s | 2m 41s | docs/API 2m 40s |
| 284 | docs | `34170652454` | 23:37:24 | 23:40:03 | 2m 39s | 2m 36s | docs/API 2m 35s |
| 283 | docs | `34169566459` | 23:17:58 | 23:20:40 | 2m 42s | 2m 39s | docs/API 2m 38s |

Dates: PRs 305–297 and 294–287 on 2026-09-09 except 296–283 on 2026-09-08
and 285–283 on 2026-09-07. PR 288 completed 09:43:58 after PR 289's
08:35:41; both are in mergedAt order, not run-completion order.

### Timing statistics (wall clock)

| Stat | All (n=20) | docs (n=6) | mixed (n=14) |
| --- | --- | --- | --- |
| min | 2m 30s (150s) | 2m 32s | 2m 30s |
| median | **2m 43s (163s)** | 2m 40s | 2m 44s |
| p75 | 2m 46s (166s) | 2m 46s | 2m 46s |
| max | 4m 49s (289s) | 4m 49s | 4m 25s |

Critical-path median 2m 40s; range 2m 26s–4m 46s. Typical (median) is about
two and a half minutes, not five. The tail is two runs: PR 303
process-spawning 4m 40s (docs tag record); PR 302 wall 4m 25s vs crit
3m 22s (queue after a cancelled earlier SHA). Neither is hidden, and
neither reaches five minutes.

### Cancellations and reruns

Accepted-head PR `CI` runs used for latency: 20/20 success. No accepted-head
cancellation. No accepted-head failure.

Two accepted heads had a second successful PR run at the same SHA (latency
uses the later completion):

- PR 300: `34345052216` (11:20:45Z–11:23:23Z) then `34348370780`
  (11:58:24Z–12:00:56Z). Same SHA as closed PR 299, then PR 300.
- PR 291: `34224112702` then `34224774498`, seven minutes apart.

Branch-level noise on earlier SHAs, not scored as accepted-head latency:

- cancelled (concurrency `cancel-in-progress`): PR 302 `34363897505`, PR 289
  `34204860429`, PR 288 `34203609920`
- failed earlier SHA: PRs 298, 297, 296 (two), 292, 285 (two)
- extra earlier-SHA successes: PRs 294, 293, 292, 289, 288, 285, 284, 283

None of those are rerun-to-green of the accepted head.

PR 301 is docs-heavy mixed (1471 files; 11 non-doc: `effigy.toml` plus
roadmap-status/number scripts). PR 302 is mixed by `effigy.toml` only.

## Pre-Card-095 command map

Inventory from `ba8275eb^1` `.github/workflows/ci.yml`. Current paths from
HEAD `.github/workflows/ci.yml` and `.config/nextest.toml`.

| Pre-Card-095 PR command | Then | Current PR path |
| --- | --- | --- |
| `cargo fmt --all -- --check` | `stable` on `macos-latest` | `Stable format and lint` on `ubuntu-latest` |
| `cargo clippy … --all-features` | same | same job |
| `cargo clippy … --no-default-features` | same | same job |
| `cargo nextest run --workspace --all-features --locked` (default profile, whole suite) | `stable` on `macos-latest` | `Stable nextest` `count:1/2` and `2/2` with profile `ci` on `ubuntu-latest`, plus `Stable process-spawning nextest` with profile `ci-process` on `macos-latest` for the six filtered binaries in `swallowtail-adapter-claude-agent`, `swallowtail-adapter-pi`, `swallowtail-host-local` |
| `cargo check --workspace --examples --locked` | `stable` | `Stable examples, metadata, and route contracts` |
| `scripts/check-package-metadata.sh` | `stable` | same job |
| route/guide/front-door/literal-version scripts | `stable` | same job |
| `RUSTDOCFLAGS='-D missing_docs' cargo doc …` | `docs-and-api` | unchanged job |
| `scripts/check-public-api.sh` | `docs-and-api` | unchanged job |
| roadmap number collision scripts | `roadmap-numbers` | unchanged job |
| MSRV `cargo clippy --workspace --all-targets --all-features` | `rust-floor` on `macos-latest` | `Pinned MSRV floor` on `ubuntu-latest` (clippy still on every PR) |
| MSRV `cargo test --workspace --all-features --locked` | `rust-floor` on `macos-latest`, every PR | **skipped on `pull_request`**. Job `Pinned MSRV floor tests` runs on `workflow_dispatch`, `main`, and `v*` tags, on `ubuntu-latest`. Same test population still runs on every PR via stable nextest |
| `cargo deny check advisories licenses sources` | `supply-chain` | unchanged job |
| `scripts/verify-source-consumer.sh` | `external-source` | unchanged job |

`.config/nextest.toml` still partitions `ci` and `ci-process` as complementary
filters over the six named binaries. PR 230 exact-head review measured
`default` 3090 = `ci` 2905 ∪ `ci-process` 185, overlap 0, at `c80cd379`.
This audit does not re-list tests; it maps the committed filters and the
current three-package process job. Install-gated probes stay in the count
shards, as that review noted.

Card 095 kept nextest shards and the pinned floor on macOS. Card 113
(`a3031aab`, `ca12afa1`, g05.034) moved the floor and non-process shards to
Linux and split the floor test into its own job. Coverage of the PR suite
remains; the platform of the pinned floor does not.

## macOS pinned floor

Literal clause: *`main` pushes and release-candidate dispatches still run
the full macOS pinned floor.*

Current workflow: both `rust-floor` and `rust-floor-test` use
`ubuntu-latest`. `rust-floor-test` is gated to dispatch / `main` / tags.

Reused g05.034 source events (not rerun):

| Run | Event | SHA | Floor clippy | Floor tests |
| --- | --- | --- | --- | --- |
| `34345060452` | `workflow_dispatch` | `1fb5b16c` | ubuntu, success, Test step present | ubuntu, success (`11:20:54Z`–`11:24:21Z`) |
| `34348374964` | `workflow_dispatch` | `1fb5b16c` | ubuntu, success | ubuntu, **failure** on Test |
| `34350208617` | push `main` | `49c9e3b2` | ubuntu, success | ubuntu, success (`12:17:58Z`–`12:21:25Z`) |
| `34372646295` | push tag `v0.4.4` | `49c9e3b2` | ubuntu, success | ubuntu, success (`15:48:20Z`–`15:51:46Z`) |

The full pinned clippy-plus-test **command** still runs on dispatch and
`main`/tags. It does not run on macOS. Process-spawning nextest on
`macos-latest` is not the pinned MSRV floor.

## Required checks

Durable GitHub settings on 2026-09-09:

- `GET /repos/inflatable-cookie/swallowtail/branches/main/protection` → 404
  `Branch not protected`
- `GET /repos/inflatable-cookie/swallowtail/rulesets` → `[]`

PR 230 review comment `5552453295` (2026-09-05T14:24:40Z) recorded the same
404 at Card 095 merge and treated the Result's name list as coordinator
documentation, not a protection edit. Branch protection was not changed.

Card 095 documented required-check names (three nextest shards, no contracts
job, no floor-tests job). Current PR check rollup on PR 305:

- Stable format and lint
- Stable nextest (shard 1/2), Stable nextest (shard 2/2)
- Stable process-spawning nextest
- Stable examples, metadata, and route contracts
- Documentation and semantic API
- Roadmap number uniqueness
- Pinned MSRV floor
- Pinned MSRV floor tests (SKIPPED)
- Dependency security, licenses, and sources
- External Git-source consumer

Names drifted (3 shards → 2; floor tests split; contracts job was always
present and omitted from the Card 095 list). Strength cannot be proved from
repository settings: there is no GitHub-enforced required set. Merge remains
a coordinator procedure after green checks.

## Sources

- `.github/workflows/ci.yml` at `3f26d01b` and at `ba8275eb^1`
- `.config/nextest.toml` at HEAD
- PR #230 merge `ba8275eb` / head `c80cd379`; review `5552453295`
- Card 095 Result at merge; Card 113 commits `a3031aab`, `ca12afa1`
- g05.034 log and runs `34345060452`, `34348370780`, `34348374964`,
  `34350208617`, `34372646295`
- GitHub PR list merged through #305; Actions `CI` runs at each accepted
  head; job timestamps and labels

No identity or ordering conflict among those sources.

## Merge

PR [#306](https://github.com/inflatable-cookie/swallowtail/pull/306)
merged 2026-09-09 as `3b4f53788bb8f28f713cbeb016df5b1cdb7c2ef0` (head
`0523da4e`). Independent exact-head review `5606858468` recorded ready to
merge at that head with no follow-ups: three owned files only, forbidden
and reserved surfaces untouched, runner/trigger spot-checks corroborate the
load-bearing claims, `effigy qa:docs` and `effigy qa:northstar` passing
with a clean `git diff --check`.

## Next

Closed. g05.031 is `stopped`; the generation index entry moves with it and
Next Task advances to g05.029, the remaining ready canonical task. No
workflow, branch-protection, or target repair follows from this audit.
