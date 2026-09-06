# g05.034 Release Lane Simplification

Status: ready; cards 109-113 compiled; 112 and 113 may start under the freeze because they touch no release surface; 109-111 after the `v0.4.3` tag
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Depends on: Contract 036; the v0.4.1, v0.4.2, and v0.4.3 lane records (g05.030, g05.032, g05.033); cards 093, 094, 103, 104; the operator direction of 2026-09-06
Vision tags: release, CI, dependability

## Purpose

Two patch releases cost most of two days. The operator's direction: the
lane must be short, simple, and dependable, and CI must be much faster.
This milestone removes the causes measured in those lanes rather than
adding process.

## Measured Causes

| Cost | Cause | Card |
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

1. Card 109: gate order and hosted-gate delegation (Contract 036 amendment).
2. Card 110: version-derived gate scripts and documented baseline roles.
3. Card 111: docs check on push; status grammar that closeouts cannot break.
4. Card 112: integration test binary consolidation, largest crates first.
5. Card 113: CI critical path: MSRV to Linux clippy-only on PRs, shard
   rebalancing, cache audit.

Cards 112 and 113 touch no release surface and may run under the freeze in
parallel with the `v0.4.3` candidate. Cards 109-111 run after the tag.

## Batch Cards

- [109 Release Gate Order And Hosted Delegation](batch-cards/109-release-gate-order-and-hosted-delegation.md) — planned; after the `v0.4.3` tag
- [110 Version-Derived Release Scripts And Baseline Roles](batch-cards/110-version-derived-release-scripts-and-baseline-roles.md) — planned; after the `v0.4.3` tag
- [111 Docs Check On Push And Closeout Grammar](batch-cards/111-docs-check-on-push-and-closeout-grammar.md) — planned; after the `v0.4.3` tag
- [112 Integration Test Binary Consolidation](batch-cards/112-integration-test-binary-consolidation.md) — planned; may start under the freeze
- [113 CI Critical Path](batch-cards/113-ci-critical-path.md) — planned; may start under the freeze; operator workflow authority from card 095 carries

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.
Shared surfaces for every card: the usual roadmap, index, generation, and
log surfaces; `PAPERCUTS.md` append only; each card's `## Result`.

| Field | 109 | 110 | 111 | 112 | 113 |
| --- | --- | --- | --- | --- | --- |
| Readiness | after the `v0.4.3` tag | after the `v0.4.3` tag | after the `v0.4.3` tag | ready now | ready now |
| Owned mutable paths | `config/release.toml`; `docs/contracts/036-*.md` (delegation clause only, Chatterbox co-signs); `docs/guides/release-playbook.md` (new); `scripts/README.md` | `scripts/check-public-api.sh`, `check-package-metadata.sh`, `check-provider-route-matrix.sh`, `check-consumer-front-door.py`, `scripts/README.md`; `release-baselines/README.md` (new); removal of `release-baselines/public-api-unreleased`; root `README.md` posture lines | `scripts/check-roadmap-status-drift.py`; `scripts/install-git-hooks.sh` (new) and the hook; `effigy` task wiring in `effigy.toml` or equivalent; `docs/contracts/001-working-rules.md` closeout line | `crates/*/tests/**` for the crates named in the card; no `src` | `.github/workflows/ci.yml`; `.config/nextest.toml` |
| Forbidden paths | crates; baselines; CI workflow | crates; contracts; CI workflow | crates; baselines | every `src`; CI workflow; baselines | crates; contracts; scripts |
| Serial edges | none | none | none | none | none |
| Concurrency | all five approved concurrent; 112 and 113 also concurrent with g05.033 | | | | |
| Worker class | release-mechanics worker | scripts worker | scripts worker | Rust test-refactor worker | CI worker with the card 095 workflow authority |
| Validation | one dry prepare on a throwaway branch proving cheap gates first and heavy gates skipped under `--hosted` evidence | the four scripts pass on `main` without an explicit version; `effigy package:api` green | a deliberately broken index is rejected at push; the checker's grammar documented in the script header | `effigy validate:focused` per touched crate; test count unchanged; nextest binary count before and after recorded | PR gate wall time before and after recorded on two runs each; all jobs green |
| Review oracle | one prepare attempt never re-runs a heavy gate the hosted run already proved at the same SHA | no version literal remains in the five scripts | a closeout cannot land drift on `main` | no test lost, no test renamed | no check lost from the PR gate |
| Stop conditions | Effigy cannot express gate ordering or skipping (route to the Effigy Chatterbox) | a script genuinely needs a version the tree cannot supply | a hook cannot be installed reliably | a crate's tests share global state across files | a check cannot move off macOS without losing coverage |
| Escalation owner | operator via Chatterbox; coordinator for mechanical blockers | | | | |

## Acceptance

The `v0.4.4` or `v0.5.0` lane, whichever comes first, runs on the new shape
and records its wall clock against the target.
