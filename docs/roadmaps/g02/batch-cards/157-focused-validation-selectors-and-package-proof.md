# 157 Focused Validation Selectors And Package Proof

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../046-validation-latency-and-proof-routing.md`

## Goal

Implement the selected focused validation and affected-package proof paths
without weakening existing milestone or release gates.

## Scope

1. Add one focused selector accepting one to four explicit Cargo package
   names. Run their nextest suites and warnings-denied all-target clippy in
   one Cargo invocation per proof kind.
2. Add one affected-package selector accepting the same explicit scope.
   Assemble and inspect each archive independently, then compile extracted
   packages against local unpublished dependencies through one shared target.
3. Reject empty, unknown, duplicate, non-workspace, and unsafe inferred scope
   before package work.
4. Preserve exact package contents, path-leak checks, local dependency patches,
   and non-zero failure propagation.
5. Document evidence tiers and these normal-path budgets:
   - static docs, route, format, API, and metadata bundle: five seconds warm
   - focused one-to-four-package proof: two minutes warm
   - affected one-to-four-package archive proof: three minutes warm
6. Add deterministic argument, archive-content, success, and representative
   failure evidence without running full workspace tests.

## Acceptance Criteria

- [x] selectors accept explicit package scope without unsafe inference
- [x] affected archives remain independently assembled and inspected
- [x] extracted packages compile with shared work where safe
- [x] typical one-to-four-package warm paths meet the selected budgets
- [x] existing workspace, candidate, and release selectors remain unchanged
- [x] success and representative failure paths are covered
- [x] docs identify the new normal development path

## Validation

- focused selector and script tests
- representative multi-package extracted compile
- existing package metadata and public-API gates
- Effigy task inventory
- `git diff --check`

## Stop Conditions

- Stop if shared compilation hides missing packaged files or path leakage.
- Stop if package scope requires changed-file inference.
- Do not make live provider access part of validation.
- Do not replace or publish a release candidate.

## Auto-Continuation

Yes. Continue to card 158 after focused proof passes.

## Evidence

- `validate:focused` accepts one to four explicit workspace package names,
  then runs one nextest and one warnings-denied all-target clippy invocation.
- `package:verify-affected` independently assembles and audits each archive,
  generates an offline temporary subset lock, and compiles all selected
  extracted packages through one shared target.
- Both selectors reject empty, oversized, duplicate, unknown, and option-like
  scope before package work. No changed-file inference exists.
- Deterministic plan, argument-failure, and unsafe archive-member tests pass
  through `validate:selectors:test`.
- Real Pi plus xAI focused proof passed 64 tests and clippy in four seconds.
- The same two archives assembled, passed content audits, and compiled from
  extracted sources in five seconds. The first attempt exposed the expected
  full-workspace-lock mismatch; temporary subset lock generation now occurs
  offline before locked compilation.
- Package metadata, the 24-crate public-API declaration baseline, shell syntax,
  and bounded shellcheck pass.
- Existing workspace, package, candidate, consumer, MSRV, and live selectors
  are unchanged.
