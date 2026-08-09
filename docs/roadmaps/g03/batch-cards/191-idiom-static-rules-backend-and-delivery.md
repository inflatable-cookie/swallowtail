# 191 Idiom Static-Rules Backend And Delivery

Status: completed
Owner: Tom
Updated: 2026-08-09

## Goal

Realize the static-rules backend and session-preparation `IdiomSet` delivery
under Contract 055.

## Scope

- static-rules backend implementing `IdiomSource` from portable rule records
- host-facing delivery seam that hands a bounded `IdiomSet` to the consumer
  at session preparation without composing or mutating prompt text
- headless static-only posture documented on the seam
- a consumer-style fixture proving the delivery path

## Out Of Scope

- learned and registry backends
- prompt composition and injection
- adapter or harness route changes

## Acceptance Criteria

- [x] static selection proof passes deterministic fixtures
- [x] delivery seam is documented and compile-checked; no prompt mutation
- [x] headless posture holds: no signals, no learned layer

## Validation

- [x] `effigy validate:focused swallowtail-idioms swallowtail-testkit` —
      112 tests pass
- [x] `effigy package:verify-affected swallowtail-idioms swallowtail-testkit`
      — extracted package proof passes
- [x] `cargo fmt --check` and warnings-denied clippy pass
- [x] `prepare_session_idioms` delivery fixture proves user-plus-scope
      selection, session bounding, and record immutability
