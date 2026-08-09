# 190 Idiom Engine Trait And Conformance

Status: completed
Owner: Tom
Updated: 2026-08-09

## Goal

Realize the engine trait under Contract 055: bounded selection and a
fail-soft signal sink.

## Scope

- `IdiomSource::select(ctx) -> bounded IdiomSet` with scope-then-confidence
  ordering
- `IdiomSink::record(signal)` fail-soft on the `DiagnosticObserver` model:
  missing sink is a no-op, failing sink never fails the operation
- testkit conformance fixtures over the trait surface

## Out Of Scope

- backend implementations beyond deterministic test doubles
- injection into session preparation
- registry client

## Acceptance Criteria

- [x] selection ordering and bounded-output fixtures pass
- [x] missing-sink no-op and failing-sink non-interference fixtures pass
- [x] testkit cross-check covers the trait without provider or consumer work

## Validation

- [x] `effigy validate:focused swallowtail-idioms swallowtail-testkit` —
      108 tests pass
- [x] `effigy package:verify-affected swallowtail-idioms swallowtail-testkit`
      — extracted package proof passes (idioms added to the internal patch
      set)
- [x] `cargo fmt --check` and warnings-denied clippy pass
- [x] repaired pre-existing stale activity-inventory truth: the committed
      provider-wide harness activity fixture already carried
      `command-code.headless`; the test expected counts and route set were
      updated to 22 routes and 29 prepared profiles
