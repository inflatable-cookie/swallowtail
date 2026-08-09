# 190 Idiom Engine Trait And Conformance

Status: ready
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

- selection ordering and bounded-output fixtures pass
- missing-sink no-op and failing-sink non-interference fixtures pass
- testkit cross-check covers the trait without provider or consumer work

## Validation

- `effigy validate:focused swallowtail-idioms swallowtail-testkit`
- `effigy package:verify-affected swallowtail-idioms swallowtail-testkit`
