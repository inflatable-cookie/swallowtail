# 2026-09-03 g05.025 Card 061 Reserved Reapable Task Runtime

Status: complete; merged at `53153af1`
Owner: Tom

## Result

Card 061 adds a provider-neutral, operation-scoped reap reservation. The exact
selected `ScopedTaskService` grants owned authority before credential,
working-resource, process, task, or provider effects. `spawn_reapable` consumes
that authority and binds one exact-host/exact-scope task before its future is
polled. Unsupported, closing, capacity-exhausted, forged, and mismatched cases
fail closed before work starts. The surface has no boolean support probe.

## Local Host

Each admitted reservation creates one bounded host-local reaper lane before
effects. A synchronized reservation state makes ordinary task completion and
later handoff mutually exclusive: an unfinished exact-authority transfer cannot
race a reaper exit, while a finished task keeps ordinary join ownership.
Dropping an unused grant releases the lane without starting work. Explicit
outer-host shutdown closes admission, waits issued grants and bound tasks to
settle, then joins retained reapers outside the ordinary task tree. Capacity is
per host; there is no global registry or parking.

Ordinary `spawn`, explicit `join`, and synchronous join-on-drop remain on their
existing path. An unreserved task cannot be upgraded through a late support
check. `AcceptedForReap` remains ownership transfer only, not join or cleanup
completion.

## Exact-Head Repair

Independent review rejected PR 195 at `dc8b0a25`: reserved `join` moved both the
worker handle and reap permit into a lazy future. Dropping that future unpolled
detached live work and released the reservation, allowing outer shutdown to
return early.

The repaired reserved path hands the worker to its already-admitted reaper
before returning the join observation future. The reaper records and wakes the
join result, but future cancellation discards observation only. Host ownership
and the shutdown barrier remain until the worker is joined and the reservation
settles. Ordinary unreserved join remains lazy and otherwise unchanged.

## Proof

Deterministic real-`LocalHostServices` cases cover unsupported, closing, and
capacity refusal before simulated credential, resource, process, task, and
provider effects; binding before poll; exact host, local lifecycle, and scope;
forged authority; unused release; the issued-reservation/shutdown race; caller
return followed by eventual reap; outer-owner shutdown; a captured service
clone; finished-task ordinary join; and the blocking join-on-drop mutation that
a boolean probe would restore. Added exact-head regression tests drop a reserved
join future on another thread both before polling and after one pending poll,
hold shutdown blocked while the task remains stalled, then release the task and
observe task completion, host join/reap, and shutdown. A separate case observes
successful active reserved join through the new cancellation-safe future.

## Public Surface

`swallowtail-runtime` adds opaque `TaskReapReservation` ownership plus
`ScopedTaskService::{reserve_reap, spawn_reapable}` with unsupported defaults.
`swallowtail-host-local` implements both methods and adds a builder-only reap
capacity bound. No consumer-facing task handle, provider type, adapter, route,
or process behavior changes. `swallowtail-testkit` required no source change.

## Validation

- focused runtime, host-local, and testkit validation: 467 tests passed; clippy passed
- affected-package source proof: runtime, host-local, and testkit passed
- semantic API: 40-package v0.3.3 gate passed; only the intended additive surfaces
- docs and Northstar gates: passed
- god-file scan: inherited 386 findings; 336 warning, 43 high, 7 critical; no new finding
- `git diff --check`: passed

## Boundary

At card 061 closeout, `claude-agent.sdk` stayed withdrawn, card 055 remained
frozen until independent exact-head review and merge, and the `v0.4.0` release
lane remained frozen without tag, publish, provider session, or release
readiness. Card 061 merged at `53153af1`. Card 055 then restored
`claude-agent.sdk` and merged through PR 196 at `493f8194`, completing g05.022
and unpausing g05.021 with card 050 ready.

## Authority

- [g05.025](../roadmaps/g05/025-reserved-reapable-task-lifecycle.md)
- [card 061](../roadmaps/g05/batch-cards/061-reserved-reapable-task-runtime.md)
- [Contract 009](../contracts/009-async-operation-lifecycle.md)
- [Contract 010](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
- [Contract 019](../contracts/019-embedded-sdk-and-cloud-client-boundary.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
