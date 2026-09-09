# g05.034 Release Lane Simplification

Status: ready; simplification scope compiled and delivered (cards 109-113 complete through PRs 247, 248, 250, 252, 253); acceptance awaits the v0.4.4 or v0.5.0 lane wall-clock record.
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Depends on: Contract 036; the v0.4.1, v0.4.2, and v0.4.3 lane records (g05.030, g05.032, g05.033); completed cards 093, 094, 103, 104; the operator direction of 2026-09-06
Vision tags: release, CI, dependability

## Purpose

Two patch releases cost most of two days. The operator's direction: the
lane must be short, simple, and dependable, and CI must be much faster.
This milestone removes the causes measured in those lanes rather than
adding process.

## Measured Causes

| Cost | Cause | Fixed by (folded card) |
| --- | --- | --- |
| ~30 min per local prepare attempt; 4 attempts on `v0.4.2`, 4-5 on `v0.4.1` | gate order runs two full clippy passes and the whole test suite before the cheap checks; every late failure discards them | 109 |
| the same clippy and tests run again in the exact-SHA hosted run | local prepare and hosted CI duplicate each other | 109 |
| 3 of 4 `v0.4.2` attempts failed on docs index drift | closeouts pushed to `main` without the docs check | 111 |
| review rounds on gate scripts pinned to the old version and the baseline directory roles | release mechanics need manual repointing and are undocumented | 110 |
| 370 integration test files, each a linked binary; nextest shards ~3.5 min, local suite far longer | link time dominates | 112 |
| MSRV floor job 5.4 min on macOS is the PR critical path; 3 macOS shards | job placement and shape | 113 |
| flaky fixtures forcing reruns | timing bounds, temp paths, teardown | 104 (in flight), 093, 094 |

## Target

The next release prepares in under 30 minutes wall clock from "gates green
on `main`" to "tag request presented", with one prepare attempt and one
hosted run. PR gate under 3 minutes. No rerun-to-green anywhere.

## Runway

1. Gate order and hosted-gate delegation (Contract 036 amendment) (card 109 complete).
2. Version-derived gate scripts and documented baseline roles (card 110 complete).
3. Docs check on push; status grammar that closeouts cannot break (card 111 complete).
4. Integration test binary consolidation, largest crates first (card 112 complete).
5. CI critical path: MSRV to Linux clippy-only on PRs, shard rebalancing, cache audit (card 113 complete).

Cards 112 and 113 touched no release surface and ran under the freeze in
parallel with the `v0.4.3` candidate. Cards 109-111 ran after the tag.

## Folded card evidence (g05.038)

- 109 Release Gate Order And Hosted Delegation — complete; ran after the `v0.4.3` tag
- 110 Version-Derived Release Scripts And Baseline Roles — complete; PR 252; exact-head re-review
- 111 Docs Check On Push And Closeout Grammar — complete; ran after the `v0.4.3` tag
- 112 Integration Test Binary Consolidation — complete; ran under the freeze
- 113 CI Critical Path — complete; ran under the freeze; operator workflow authority from card 095 carried

## Retired card dispatch (g05.038)

Card-level dispatch is retired: cards 109-113 are all complete, their serial
edges and concurrency notes are now owned by this task, and acceptance is
unchanged — the `v0.4.4` or `v0.5.0` lane wall clock still records against the
target.

## Acceptance

The `v0.4.4` or `v0.5.0` lane, whichever comes first, runs on the new shape
and records its wall clock against the target.
