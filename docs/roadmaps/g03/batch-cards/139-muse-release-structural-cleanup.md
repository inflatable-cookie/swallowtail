# 139 Muse Release Structural Cleanup

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../046-v0-2-0-muse-and-rust-floor-source-release.md`
Depends on: card 138

## Goal

Remove both new error-severity Muse structural-size findings before release
preparation without changing public API or runtime behavior.

## Scope

1. Split event parsing, validation, and tests into focused private modules.
2. Split the corpus integration tests into focused test modules while keeping
   the existing two package test binaries.
3. Preserve exact fixtures, diagnostics, lifecycle order, bounds, and prepared
   facade behavior.
4. Prove the package API remains identical to its unreleased baseline.

## Acceptance

- [x] no Muse source or test file has an error-severity god-file finding
- [x] default package test-binary count remains two
- [x] all Muse tests and the package-independent corpus validator pass
- [x] extracted-package and semantic API proof pass
- [x] no provider, release, consumer, or external mutation runs

## Evidence

- Event terminal projection, validation/diagnostics, and unit tests now live in
  focused private submodules. Public API and diagnostic codes are unchanged.
- Corpus tests now use artifact, prepared-facade, rejection, and common helper
  modules under the existing `corpus` integration-test binary.
- Effigy doctor drops from 24 to the 22 inherited error findings. Muse has no
  error-severity finding; `events.rs` remains only in warning range.
- The five-test corpus validator, 20 tests across two binaries, warnings-denied
  check, extracted-package proof, and semantic API comparison pass.
- No live provider or external release mutation ran.

## Validation

- `python3 scripts/check-muse-code-corpus.py`
- `effigy validate:focused swallowtail-adapter-muse`
- `effigy package:verify-affected swallowtail-adapter-muse`
- `effigy package:api`
- `effigy doctor`

## Stop Conditions

- stop if the split needs public compatibility changes
- stop if test consolidation weakens distinct malformed, bounds, activity, or
  live-order evidence

## Auto-Continuation

Yes. Continue to card 140 only after Muse contributes no error-severity
structural finding and all focused proof passes.
