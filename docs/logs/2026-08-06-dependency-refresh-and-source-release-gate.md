# 2026-08-06 Dependency Refresh And Source Release Gate

## Result

The pre-tag dependency sweep ran with Cargo resolver 3 and the Rust 1.90
workspace floor. Bedrock moved to the newest generated SDK releases that keep
its accepted Rust 1.94.1 exception:

- `aws-sdk-bedrock` 1.148.0 → 1.150.0
- `aws-sdk-bedrockruntime` 1.136.0 → 1.139.0
- their selected AWS runtime support graph moved with them

The workspace-scoped refresh selected no further changes, then the full
`cargo update --verbose` advanced 20 floor-compatible transitive packages.
The active lock is therefore the deliberately refreshed release graph, not a
lock file regenerated after release gates.

## Retain Decisions

- `async-tungstenite` 0.34.1: retain. Version 0.35 is a transport API migration
  across Claude Agent and remote ACP, not a lock refresh. Schedule it with
  route corpus review after 0.1.0.
- `base64` 0.22.1: retain. Version 0.23 is a breaking direct-dependency upgrade
  across six adapters. Defer the coordinated migration.
- `sha2` 0.10.9: retain. Version 0.11 is a breaking direct-dependency upgrade
  in runtime and host-local code. Defer the coordinated migration.
- `agent-client-protocol-schema` 1.5.0: retain transitively under the exact ACP
  2.0 transport SDK. It cannot be upgraded independently.
- the remaining Cargo-reported newer packages are transitive and outside
  currently admitted upstream ranges or resolver-selected floor compatibility.
  No manifest pin was added to hold them back.

## Release Gate

All 27 packages now inherit `publish = false`. The active Effigy release
configuration runs format, both Clippy feature postures, the full suite,
repository QA, denied-missing-doc Rustdoc, semantic API, metadata, dependency
policy, compiler floors, and an isolated external Git-source consumer.

The older `.crate` candidate selectors are removed from the active Effigy
surface. Their retained files are labelled historical.

## Validation

- focused Bedrock validation passed 28 tests and warnings-denied Clippy
- extracted 58-file Bedrock package compiled at Rust 1.94.1
- Rust 1.90 non-Bedrock and Rust 1.94.1 Bedrock Clippy and full tests passed
- 27-package metadata and dependency topology passed
- dependency advisory, license, and source policy passed
- external synthetic exact-revision snapshot consumer passed while the
  pre-commit release worktree was dirty; card 129 owns canonical-HEAD proof
- GitHub Actions workflow passed `actionlint`

No authenticated provider, consumer repository, tag, push, GitHub Release, or
registry mutation ran.

## Release Simulation

Effigy now supports an explicit `initial-tag-current-version` release setting.
It is bounded to a changelog with no released versions and a matching local tag
that does not exist. Swallowtail enables that mode rather than inventing a
prior version or bypassing release orchestration.

Read-only `effigy release simulate` selected current, planned, and suggested
version `0.1.0`, tag `v0.1.0`, and one changelog mutation. It ran all 11 gates
in 52,143 ms with no blockers. The stable suite passed 1,463 tests with 11
skipped; the Rust floor, API, docs, metadata, source-consumer, QA, security,
format, and both lint postures also passed. The simulation found no prepared
state and wrote none.

No release prepare, tag, commit, push, GitHub Release, registry mutation,
authenticated provider work, or consumer edit ran.
