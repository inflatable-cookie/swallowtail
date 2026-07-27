# Provider Session Management Conformance

Date: 2026-07-26

## Change

Card 048 completes the provider-neutral foundation from Contract 038.

`swallowtail-testkit` now exposes a public persistent-session management
fixture and assertion pack. It composes only public core, runtime, and testkit
surfaces.

The pack proves:

- archive, restore, and all three deletion strengths
- qualified and visible unverified-newer execution
- incompatible and unsupported failure before dispatch
- local and remote-authoritative host identity
- target-only and provider-defined descendant scope
- already-absent and unconfirmed-after-effect truth
- every constructible binding field and immutable plan field
- cancellation and deadlines before and after dispatch
- joined task, resource, and credential release order

Typed requests retain a management-scoped cancellation token. Binding
validation now includes integration and transport family identity. A copied
provider session reference cannot substitute for the binding fixed by the
immutable plan.

The existing 13 synthetic operation profiles are unchanged.

## Boundary

- no provider method, payload, endpoint, or executable behavior
- no consumer thread persistence or deletion policy
- no active-session lookup
- no provider history browser
- no implicit retry or fallback
- no stronger deletion truth inferred from acknowledgement or absence

## Validation

- core, runtime, and testkit: 180 tests pass
- `effigy check:rust`: passed
- `effigy format:check`: passed
- `effigy qa:docs`: passed
- `effigy qa:northstar`: passed
- final doctor scan: no new structural findings
- `git diff --check`: passed

## Next

Roadmap 015 is complete. Card 049 is ready to freeze Codex lifecycle methods
and semantics across the maintained executable range before any production
mapping.
