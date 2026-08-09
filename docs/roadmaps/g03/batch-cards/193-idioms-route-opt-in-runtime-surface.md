# 193 Idioms Route Opt-In Runtime Surface

Status: completed
Owner: Tom
Updated: 2026-08-09

## Goal

Realize the runtime surface for Contract 056: host ports, session option,
and the fixed fold rule.

## Scope

- optional `IdiomSource` and `IdiomSink` ports on the execution-host
  service set on the `DiagnosticObserver` model
- `IdiomSessionOption` field on `SessionOptions` with source reference and
  maximum
- the fixed fold rule: one line per constraint with scope and provenance
  labels, bounded bytes with truncation marker, explicit order against
  consumer-supplied developer instructions
- runtime and testkit conformance fixtures

## Out Of Scope

- prepared-plan binding and capability gate (card 194)
- route proofs and Nucleus adoption
- learned backends and the correction-loop proxy

## Acceptance Criteria

- [x] fold determinism, bounds, and truncation fixtures pass
- [x] consumer instructions preserved with explicit order
- [x] recorder no-op and failing-sink non-interference fixtures pass
- [x] default behavior unchanged: no option, no idioms work

## Validation

- [x] `effigy validate:focused swallowtail-idioms swallowtail-runtime
      swallowtail-testkit` — 297 tests pass
- [x] `effigy package:verify-affected swallowtail-idioms swallowtail-runtime
      swallowtail-testkit` — extracted package proof passes
- [x] `cargo fmt --check` and warnings-denied clippy pass
- [x] runtime dependency floor extended with `swallowtail-idioms` per
      Contract 056; architecture note updated
