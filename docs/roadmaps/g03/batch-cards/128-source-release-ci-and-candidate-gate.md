# 128 Source Release CI And Candidate Gate

Status: complete
Owner: Tom
Created: 2026-08-05
Milestone: `../043-v0-1-0-source-release-readiness.md`
Depends on: card 127

## Goal

Replace inactive registry candidate machinery with one repeatable source-tag
gate and matching GitHub CI.

## Scope

1. Mark all packages non-publishable for the source release.
2. Reconcile the 27-package metadata and dependency topology.
3. Add stable, MSRV, docs, security, and external-source jobs.
4. Retire stale crates.io candidate selectors from the active release path.
5. Keep historical candidate evidence clearly historical.

## Validation

- release selector plan and deterministic execution
- workflow syntax and task parity
- `effigy package:metadata`
- `effigy qa`

## Evidence

- all 27 packages inherit `publish = false`
- the active Effigy release configuration carries 11 source-only gates
- stable, Rust 1.90, Rust 1.94.1 Bedrock, docs/API, security, metadata, QA,
  external-source, and full-test gates pass
- GitHub CI matches those lanes and passes `actionlint`
- the deliberate dependency refresh advances the Bedrock SDK graph and records
  every direct retain decision
- old `.crate` selectors and evidence are no longer presented as active
- Effigy first-tag/current-version mode selects `0.1.0` and `v0.1.0`
- read-only release simulation passes all 11 configured gates
- the plan contains only changelog promotion; it does not rewrite the already
  correct workspace version

## Release Simulation

Effigy `v0.8.17+local.f51274c.dirty` adds the explicit bounded mode. The
simulation reports current, planned, and suggested version `0.1.0`, tag
`v0.1.0`, 11 of 11 passing gates, no blockers, and no release-state write.
The stable suite passes 1,463 tests with 11 skipped. No prepare, tag, commit,
push, registry, GitHub Release, authenticated provider, or consumer mutation
ran.

## Auto-Continuation

No. Card 129 freezes evidence only from an exact clean candidate commit.
