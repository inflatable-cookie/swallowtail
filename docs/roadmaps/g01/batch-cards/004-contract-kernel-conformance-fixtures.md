# Contract Kernel Conformance Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Prove the Contract 003 edge cases with deterministic testkit fixtures.

## Scope

- unsupported capability rejection
- provider reference opacity
- safe diagnostic redaction
- provider-extension isolation
- unknown addition handling

Build reusable fixture cases in `swallowtail-testkit`. Keep core tests focused
on individual record behavior; use the testkit to exercise the public API as a
consumer would.

## Out Of Scope

- live provider accounts
- process or network integration
- runtime traits or async selection
- provider-specific wire types
- consumer dependencies

## Acceptance Criteria

- fixture cases cover capability rejection, reference opacity, diagnostic
  redaction, extension isolation, and both unknown-extension policies
- fixtures consume only `swallowtail-core` public API
- fixtures expose assertions or case records reusable by later adapter crates
- test failures identify the violated contract rule without provider access
- no execution or transport abstraction is introduced

## Validation

- `effigy test --plan`
- `effigy test:rust` — ten tests pass across core and testkit
- `effigy lint:rust`
- `effigy check:rust`
- dependency and public-API audit
- `git diff --check`

## Stop Condition

Stop if a fixture requires async behavior, provider wire knowledge, or a new
runtime contract. Record that need for the validation card instead.

## Outcome

`swallowtail-testkit` now exposes a canonical Contract 003 fixture and five
independent assertion helpers. External integration tests prove the helpers use
only exported core and testkit APIs. A deliberate negative case confirms
failures name the violated contract rule.
