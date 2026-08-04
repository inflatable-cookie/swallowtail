# 2026-08-04 Controlled Shutdown Active Operation Detachment

## Outcome

Swallowtail now distinguishes controlled local detachment from completion,
cancellation, timeout, failure, stream reattachment, and crash reconciliation.
The portable boundary adds exact run/turn scope, an optional object-safe handle
control, idempotent local acknowledgement, and `TerminalStatus::Detached`.
Unsupported handles retain the default absence of a control.

Detachment does not synthesize provider-terminal activity. Activity projections
leave unresolved activities open when local observation ends as detached.
Ordinary close without an admitted request retains its previous cancellation
behavior, and cancellation wins disposition races.

## OpenCode Acceptance

`OpenCodeSessionProfileInput::with_active_turn_detachment()` selects the first
production mapping for qualified read-only interactive sessions. The immutable
plan binds active-turn scope, durable provider-session preservation, exact
route/resource authority, a persisted resume binding, and the existing
session-scoped reconciliation route.

The turn control stops and joins the local SSE attachment. It issues no
`/abort`, retry, second prompt, callback answer, load, resume, import, status,
or delete request during detach and close. Callback-bearing sessions and
structured runs remain excluded.

The deterministic acceptance exports the exact binding before dispatch can be
lost, detaches and closes the local handles, reconstructs the binding across a
simulated process boundary, and reconciles the same OpenCode session as active.
Sequential cancellation and terminal rejection cases preserve the fail-closed
boundary. Ordinary profiles expose no control.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-opencode`
  — 284 tests passed across five binaries; checks and clippy passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-opencode`
  — all three extracted packages compiled
- `cargo check --workspace --all-targets`
- `effigy qa:docs`
- `cargo fmt --all -- --check`
- `git diff --check`
- no authenticated provider work or live provider operation

## Current State

Cards 071-072 and roadmap g03.028 are complete. The sole Next Task has returned
to the g03 evidence gate. Kimi local-server detachment still depends on exact
cursor-checkpoint persistence and reconciliation; other routes retain the
promotion gates in Research 100.
