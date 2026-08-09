# 191 Idiom Static-Rules Backend And Delivery

Status: ready
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

- static selection proof passes deterministic fixtures
- delivery seam is documented and compile-checked; no prompt mutation
- headless posture holds: no signals, no learned layer

## Validation

- `effigy validate:focused swallowtail-idioms`
- `effigy package:verify-affected swallowtail-idioms`
