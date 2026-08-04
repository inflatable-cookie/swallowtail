# 048 Cross-Process Active Operation Reconciliation

Status: active
Owner: Tom
Updated: 2026-08-04

## Purpose

Define how a consumer reconciles a locally active turn or structured run after
its runtime handle was lost while provider-owned work or retained history may
remain.

## Separate Operations

Reconciliation is a read-only operation. It is not:

- provider-session catalogue or import
- session load, resume, or stream reattachment
- prompt retry or replay
- provider cancellation
- callback recovery or answer submission
- provider-session management

Session reconciliation uses one already persisted `SessionResumeBinding`. Run
reconciliation uses one persisted `ProviderRunCheckpoint`. A raw provider
session id, provider run id, catalogue candidate, diagnostic, path, or provider
payload is not admission authority.

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

Some routes also require an exact durable event position. A
`ProviderOperationCheckpoint` binds the provider session, consumer runtime
turn, exact provider turn, and adapter-owned opaque cursor. Qualified runtime
events may carry the newest checkpoint. Its persisted form is versioned,
bounded, integrity-checked, and restorable only under the exact current
session binding and attachment fingerprint.

Consumers persist the complete opaque record. They do not parse, edit, merge,
compare, or manufacture cursor bytes. A checkpoint from another runtime turn,
provider turn, session, route, instance, host, model, resource, access posture,
or provider-state policy fails closed.

## Structured-Run Agreement

`ProviderRunReconciliationAgreement` binds:

- the consumer-unique interrupted `RuntimeRunId`
- the exact provider `RunRef`
- one restored `ProviderRunCheckpoint`
- a positive maximum recovered-output byte bound
- an optional deadline

`ProviderRunCheckpoint` binds the same runtime and provider run ids plus
adapter-owned opaque cursor bytes. Its persisted form is versioned, bounded,
integrity-checked, and restorable only under the exact driver, configured
instance, target, host, access profile, model route, protocol facade, and
interface evidence.

Run reconciliation is a distinct capability, operation shape, and driver
role. It cannot manufacture a provider session around a sessionless run.

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

`InterruptedRunState` is `Active`, `Completed`, `Failed`, `Cancelled`,
`InactiveUnresolved`, or `Unknown`. Run observations always carry the exact
provider run reference. Recovered output and usage are optional and bounded;
non-terminal observations carry neither. A terminal provider payload from a
different run fails closed.

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

`kimi-code.local-server` implements exact-turn reconciliation for qualified
externally attached `0.28.1..=0.31.1` servers. The restored checkpoint supplies
the exact `{seq, epoch}` position and turn. One read-only session lookup and
one WebSocket subscription must match the bound session and cwd. Subscription
acknowledgement fixes a finite current sequence; accepted durable events are
validated strictly through that sequence and the observer then closes.

An exact retained `turn.ended` event maps completed, failed, blocked, or
cancelled truth. Exact waiting events map provider input wait. If no new event
exists, matching busy session state may report `Active`; an idle session
without terminal evidence is `InactiveUnresolved`. Gaps, resync, epoch drift,
foreign turns, stale cursors, and attachment drift fail closed. No prompt,
abort, callback response, session resume, import, or management action is
dispatched.

`openai.background` implements exact-run reconciliation for the exact prepared
public Responses route. One `GET /v1/responses/{response_id}` request maps
queued or in-progress to active, completed to completed with bounded output and
usage, incomplete or failed to failed, and cancelled to cancelled. It sends no
create, prompt, retry, stream attachment, cancel, delete, callback, or session
operation. The restored response/cursor checkpoint must match the current
route binding exactly.

ACP `session/load` is not a reconciliation operation. Stable ACP defines load
as restoring resumable session context, connecting requested MCP servers, and
returning a ready session after replay. A route cannot make that operation
read-only by closing the resulting handle immediately. Claude Agent ACP and
Kimi ACP therefore retain ordinary load/replay support but no reconciliation
mapping.

Other routes remain unclaimed until the route-specific gates in Research 099
are satisfied. Capability does not inherit across another transport in the
same provider family.

## Conformance

Portable and route tests must cover:

- exact binding and runtime-turn correlation
- checkpoint persistence, corruption, version, bound, and attachment checks
- active, inactive-unresolved, unknown, and exact terminal rules as applicable
- strict provider-turn attribution for terminal states
- bounded complete and incomplete replay snapshots
- cancellation and deadline before and during observation
- wrong session, route, host, resource, model, and stale references
- no prompt, abort, callback answer, provider request, import, or management
  side effect
- joined cleanup and provider-session preservation
- exact run binding, bounded recovered output, and no state-changing request
