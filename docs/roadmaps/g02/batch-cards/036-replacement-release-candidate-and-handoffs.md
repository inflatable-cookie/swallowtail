# 036 Replacement Release Candidate And Handoffs

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../012-provider-wide-acceptance-and-candidate-return.md`
Supersedes: `016-replacement-release-candidate-and-handoffs.md`

## Objective

Replace the unpublished `0.1.0` candidate with one backed by provider-wide
prepared-facade and packaged consumer runtime evidence.

## Governing Refs

- Contracts 029 and 036-037
- completed card 035
- release and package topology architecture
- retained superseded candidate evidence

## Scope

1. Classify the final public API delta against the unreleased baseline.
2. Re-run package metadata, dependencies, API, docs, MSRV, content, route
   matrix, and checksum gates from one clean commit.
3. Replace candidate archives and evidence atomically.
4. Update release notes and consumer handoffs.
5. Retain prior candidates as superseded historical evidence.
6. Stop before every external release mutation.

## Acceptance Criteria

- [x] one exact active unpublished `0.1.0` candidate remains
- [x] archives and evidence match one clean source commit
- [x] packaged provider-wide and consumer runtime proof passes
- [x] handoffs describe the adapter-local prepared normal path
- [x] exact provider guarantees and unverified-newer posture are recorded
- [x] one explicit publication decision remains

## Validation

- `effigy package:check`
- isolated candidate staging through the package prepare implementation
- `effigy package:candidate:verify`
- provider-wide and consumer package selectors
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- any candidate artifact differs from its recorded source
- a production route or consumer proof fails
- release credentials or external mutation become necessary
- two active candidates would remain ambiguous

## Auto-Continuation

No. Return the exact candidate to the sole publication decision.

## Execution Evidence

The active unpublished candidate is built from clean synthetic source commit
`73c7f5b5b5611ef20bdcc1572deeb39ca50630e1`, based on repository commit
`91a0774010ee83594a4565e1b4e2b0daa998db28`. Its package checksum-manifest
digest is
`1442fdea7f8426fd3dcd74ef8513a0945761798877208e5f9a1454720591eac5`.

Reproducibility regenerates the 23-package set from the bundled source and
matches archive plus audited file-list checksums. Packaged facade evidence
passes 20 suites and 65 tests across all 22 production routes; its evidence
digest is
`e73a67fd06617675c9a84f4fb171409d4fdd973feaef439d91c35260bde38818`.
Nucleus passes 14 tests with two live probes ignored, Soundcheck passes four
with one live probe ignored, and packaged Codex passes 89 tests. Consumer
evidence is
`fe760fb4a91a273ebaff16ae7a7d3618356a6761689f4aac6a0002a56922ab07`.

The superseded candidate remains immutable at
`.effigy/release-candidates/superseded/0.1.0-6c0e8d9b5b05/`. The earlier
provisional candidate also remains retained. Only
`.effigy/release-candidates/0.1.0/` is active.

Public-declaration comparison against the superseded source changes 20
packages: core, runtime, local host, testkit, and all 16 adapters. The two
protocol crates and remote ACP transport are unchanged. Because no Swallowtail
release exists, this is a deliberate candidate-breaking replacement of the
initial `0.1.0` baseline, not a released compatibility event.

No registry, owner, credential, upload, tag, push, workflow, release, or
consumer mutation occurred.
