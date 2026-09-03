# g05.024 Scoped Task Relinquishment

Status: completed; provider-neutral ownership transfer and host-owned supervised reap
Owner: Tom
Created: 2026-09-03
Depends on: g05.023; Contracts 010 and 019; PR 188 exact-head review
Vision tags: caller deadlines, scoped tasks, selected hosts, cleanup truth

## Purpose

Close the shared ownership gap found while reviewing PR 188: a caller-bound
operation can reach its deadline while its host-scoped `JoinedTask` is still
running, but drop and join remain blocking ownership paths.

## Direction

Add one provider-neutral operation to `ScopedTaskService`. The selected host
takes back an unfinished task only under the exact execution-host and
`ScopeId` binding and accepts responsibility for eventual reap. The result says
only `AcceptedForReap`; it never says joined or cleanup complete.

The local host uses a per-transfer reaper whose handle is retained by an outer
selected-host lifecycle owner. Task-service clones keep only weak handoff
authority. Explicit host shutdown outside the task tree joins all retained
reapers. This adds no adapter-global registry, parking lot, follow-up call, or
weakened `LocalJoinedTask` drop/join rule. Rejection leaves the caller's handle
intact.

## Batch Card

- [060 Scoped Task Relinquishment And Host Reap](batch-cards/060-scoped-task-relinquishment-and-host-reap.md) — complete; shared runtime seam, local reaper, deterministic authority/liveness proof

## Acceptance

- relinquishment returns before an unfinished task completes
- the selected host reaps the task after later completion without another call
- explicit selected-host shutdown outside the task tree joins its reapers and
  accepted work, even when a worker captures a task-service clone
- wrong-host, wrong-scope, unsupported, finished, and repeated transfer fail closed
- ordinary joined tasks still join explicitly or on drop
- accepted-for-reap cannot be used as joined or cleanup-completion evidence
- no provider-specific type or task handle reaches consumer API outside the
  existing provider-neutral runtime seam

## Outcome

Card 060 adds the exact ownership-transfer seam and local host implementation.
Contracts 009, 010, and 019 now distinguish joined completion from host-accepted
reap ownership. This remains valid after PR 188 containment. It does not reserve
a non-fallible transfer before provider work or own an enclosing cleanup
continuation and its leases; later rejected review proved that separate shared
runtime prerequisite is required.

## Boundaries

No Claude adapter implementation, provider contact, release operation,
version-currentness work, global executor, adapter-global state, or change to
ordinary joined-task ownership belongs here.
