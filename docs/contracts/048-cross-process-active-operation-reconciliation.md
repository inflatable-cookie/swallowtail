# 048 Cross-Process Active Operation Reconciliation

Status: active
Owner: Tom
Updated: 2026-08-04

## Purpose

Define how a consumer reconciles a locally active turn after its runtime handle
was lost while provider-owned work or retained history may remain.

## Separate Operations

Reconciliation is a read-only operation. It is not:

- provider-session catalogue or import
- session load, resume, or stream reattachment
- prompt retry or replay
- provider cancellation
- callback recovery or answer submission
- provider-session management

The operation uses one already persisted `SessionResumeBinding`. A raw provider
session id, catalogue candidate, diagnostic, path, or provider payload is not
admission authority.

## Immutable Agreement

`ProviderSessionReconciliationAgreement` binds:

- the exact durable session binding and prepared route
- the consumer-unique interrupted `RuntimeTurnId`
- an optional exact provider `TurnRef`
- maximum replay items and bytes
- an optional deadline

The observation operation always acquires read-only working-resource authority.
It does not inherit wider write or callback authority from the original
session binding.

The consumer persists the local turn as active before dispatch. It persists an
exact provider turn reference as soon as the qualified adapter exposes one.
Reusing that runtime turn id for later work is invalid.

## State And Attribution

`InterruptedTurnState` is one of:

- `Active`
- `WaitingForProviderInput`
- `Completed`
- `Failed`
- `Cancelled`
- `InactiveUnresolved`
- `Unknown`

`InterruptedTurnAttribution` is either `ExactProviderTurn` or
`ProviderSession`.

`ProviderSessionReconciliationObservation` constructs either form explicitly
before it can enter the validated outcome.

Terminal states require exact provider-turn attribution. Session-scoped status
can report active, waiting, inactive-unresolved, or unknown. It cannot infer
completion from an idle session or from new output appearing in retained
history.

Unknown, missing, stale, cross-route, cross-instance, cross-host,
cross-resource, or mismatched model bindings fail closed. A missing exact turn
does not fall back to a similarly timed or similarly worded turn.

## Replay Snapshot

Reconciliation may return ordered `SessionReplayItem` records for the exact
bound session. The records form a bounded replacement snapshot, not an
append-only delta. `replay_complete` states whether the adapter returned the
whole qualified snapshot within the agreed item and byte bounds.

Consumers own durable merge and presentation. An incomplete snapshot cannot
delete unmatched consumer records or prove provider history absence.

## Authority Boundary

The role grants no control handle. It cannot:

- cancel or abort work
- answer, reopen, or manufacture callbacks
- admit provider requests
- send a prompt, steering message, or follow-up
- create, import, load, or resume a session
- control subagents
- complete or fail another runtime operation

Observation of `Active` blocks a new turn on the same consumer thread unless a
separately qualified control or reattachment path resolves it. Observation of
`InactiveUnresolved` allows the consumer to stop showing live work, but its
durable terminal label remains consumer policy.

## Cleanup

All reconciliation transport, process, task, access, and resource work joins
before outcome. Cleanup preserves the attached provider service and bound
session. Cleanup failure fails the observation and does not change provider
state.

The contract does not redefine `TurnHandle::close`. Preserving provider work
during controlled application shutdown needs an explicit detach disposition
and route proof; it cannot be inferred from reconciliation support.

## Current Production Mappings

`codex.app-server` implements exact-turn and session-scoped reconciliation for
the qualified thread-catalogue range `0.105.0..=0.146.0`. One read-only
`thread/read(includeTurns: true)` response must match the exact thread, cwd,
source, and optional requested turn id. Exact `inProgress`, `completed`,
`failed`, `interrupted`, and `cancelled` statuses map directly. Absent status
is `Unknown`; an absent requested turn fails closed. No `turn/start`,
`turn/interrupt`, `thread/resume`, or lifecycle method is dispatched.

`opencode.http` implements session-scoped reconciliation for qualified
`1.14.48..=1.18.10` server segments. It revalidates health, exact session,
directory, version, status, and bounded retained history. It issues no prompt,
abort, delete, callback, import, load, or resume request.

OpenCode `prompt_async` supplies no exact prompt/turn reference. `Active` and
`InactiveUnresolved` are therefore honest; terminal states are unavailable.

Other routes remain unclaimed until the route-specific gates in Research 099
are satisfied. Capability does not inherit across another transport in the
same provider family.

## Conformance

Portable and route tests must cover:

- exact binding and runtime-turn correlation
- active, inactive-unresolved, unknown, and exact terminal rules as applicable
- strict provider-turn attribution for terminal states
- bounded complete and incomplete replay snapshots
- cancellation and deadline before and during observation
- wrong session, route, host, resource, model, and stale references
- no prompt, abort, callback answer, provider request, import, or management
  side effect
- joined cleanup and provider-session preservation
