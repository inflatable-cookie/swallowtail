# 032 Interactive Access Policy Records

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../010-bounded-workspace-session-access.md`

## Objective

Realize the Contract 013 policy, capability requirements, and request binding
in core/runtime with deterministic preflight evidence.

## Acceptance

- [x] policy dimensions are typed and inspectable
- [x] read-only is the unchanged default
- [x] read-write requires exact capability and host-resource agreement
- [x] impossible network/search and callback combinations fail before effects
- [x] local and remote-authoritative fixtures retain host identity
- [x] no provider or consumer identity enters provider-neutral crates

## Evidence

- `swallowtail-core::session_access` owns the expanded provider-neutral record.
- Interactive `OperationRequirements` bind that record into immutable
  preflight plans.
- Runtime requests default to the same read-only record and reject plan or
  lease mismatch explicitly.
- `SessionAccessPreflightFixture` covers capability, service, extension,
  resource, and topology rejection before provider effects.

## Stop Condition

Stop if a convenience preset becomes the only observable policy shape.
