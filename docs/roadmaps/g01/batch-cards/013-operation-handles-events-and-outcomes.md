# Operation Handles, Events, And Outcomes

Status: completed
Owner: Tom
Roadmap: 005 Async Runtime and Conformance
Updated: 2026-07-19

## Goal

Implement provider-neutral run, session, turn, and serving-handle contracts
with ordered bounded events and exactly one terminal outcome.

## Scope

- request, run, session, turn, callback, and serving-instance identities
- run, session, turn, and serving handle roles
- bounded ordered event stream and coalescible-event marker
- terminal and cleanup outcomes
- cancellation scopes, acknowledgement, deadline, and close semantics
- late-event and overflow failure behavior
- deterministic lifecycle unit tests

## Out Of Scope

- concrete host task/process/network implementations
- provider event translation
- event persistence or replay
- consumer validation or acceptance

## Acceptance Criteria

- start precedes progress/output
- semantic events are never silently dropped
- exactly one terminal outcome wins under completion/cancel races
- cancellation, timeout, provider, host, runtime, and cleanup failure remain
  distinguishable
- attached service handles expose no generic stop authority

## Validation

- focused lifecycle tests
- `effigy qa`
- `git diff --check`

## Closeout

- Request, run, session, turn, callback, serving-instance, and scope identities
  are distinct validated types; provider run, session, and turn references
  remain separate opaque core records.
- Run, turn, session, attached-serving, and owned-serving handles expose
  lifecycle-specific authority. Attached handles have no stop method.
- The bounded event channel enforces one start boundary, monotonic sequence,
  deliberate coalescing, semantic-overflow failure, terminal close, and late
  event quarantine.
- Cancellation is scoped and idempotent. Terminal completion is first-wins and
  keeps cancellation, timeout, provider, host, runtime, and cleanup outcomes
  distinct.
