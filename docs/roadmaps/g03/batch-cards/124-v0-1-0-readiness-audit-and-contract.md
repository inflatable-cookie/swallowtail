# 124 v0.1.0 Readiness Audit And Contract

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../043-v0-1-0-source-release-readiness.md`
Depends on: card 123

## Goal

Audit the current codebase and replace the stale crates.io candidate assumption
with an exact GitHub source-tag boundary before release work starts.

## Scope

1. Inventory package, route, source, test, toolchain, and guide state.
2. Run deterministic QA, Rust floors, docs lint, dependency advisories, and
   structural scans.
3. Classify release blockers separately from internal maintenance debt.
4. Promote the selected source-tag contract and bounded implementation runway.

## Validation

- `effigy qa`
- Rust `1.90.0` non-Bedrock workspace check
- Rust `1.94.1` Bedrock check
- workspace Rustdoc with missing documentation warned
- package metadata, API, and release-gate probes
- dependency advisory, currentness, size, duplicate, and suppression scans

## Completion

- Research 111 records the passing functional baseline and four blocking
  release areas.
- Contract 036 now selects one 27-package `v0.1.0` source tag with no registry
  publication.
- release architecture records the exact current package topology and Git
  consumption shape.
- g03.043 sequences security, public API, consumer docs, CI, candidate proof,
  and the separately authorized tag handoff.
- no external release mutation or authenticated work ran.

## Auto-Continuation

No. Card 125 is ready for dependency security and policy.
