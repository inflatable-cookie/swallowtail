# Provider Session Management Runtime Role

Date: 2026-07-26

## Change

Card 047 realizes the low-level runtime boundary from Contract 038.

The new management plan binds:

- the exact preflight plan and opaque session management binding
- archive, restore, or deletion with its promised strength
- required initial provider state and affected scope
- caller-asserted inactivity
- before-dispatch-only or provider-native cancellation posture
- an optional deadline and its required host services

Archive, restore, and delete use separate request types and separate methods on
`ProviderSessionManagementDriver`. A request must match its immutable plan,
execution host, and available services before adapter effects.

The common outcome keeps the target binding, exact effect truth, safe provider
request reference, rate evidence, and safe diagnostic. An unconfirmed
after-dispatch deletion exposes no confirmed deletion strength.

Adapter-local facades can wrap the plan with
`PreparedProviderSessionManagementEvidence`. The unchanged low-level role
remains directly usable.

## Boundary

- no consumer thread, persistence, UI, or confirmation policy
- no active-session registry or handle discovery
- no generic state setter
- no retry, provider fallback, or provider search
- no provider-native close method
- no driver-owned cleanup widening
- no production provider adapter

## Validation

- `cargo test -p swallowtail-core -p swallowtail-runtime`: 115 passed
- focused core/runtime Clippy with warnings denied: passed
- `effigy check:rust`: all workspace crates and targets pass

## Next

Card 048 adds deterministic local and remote-authoritative conformance before
any production adapter uses the role.
