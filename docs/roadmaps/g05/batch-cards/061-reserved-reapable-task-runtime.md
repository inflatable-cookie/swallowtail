# 061 Reserved Reapable Task Runtime

Status: complete; implemented on an unmerged branch pending independent exact-head review
Owner: Tom
Created: 2026-09-03
Milestone: `../025-reserved-reapable-task-lifecycle.md`
Depends on: completed card 060; Contracts 009, 010, 017, 019, and 047;
  PR 188 review comments 5524712917 and 5524729651; PR 193 containment

## Goal

Give a caller-bound operation a provider-neutral, operation-scoped guarantee—
obtained before effects—that its enclosing task can later transfer to the exact
selected host for eventual reap without a shutdown, capacity, or lifecycle
refusal.

## Scope

1. Add the smallest provider-neutral reservation or equivalent atomic
   guaranteed-reapable shape to the scoped-task service. Bind it to one exact
   execution host, `ScopeId`, and later task handoff. Bind or consume the grant
   into one new task before its work is polled; bound reserved capacity cannot
   be released independently from the task.
2. Require reservation admission before provider work or credential, working-
   resource, process, or task acquisition. Reject unsupported, closing, and
   capacity-exhausted hosts at that boundary.
3. Make the held grant reserve all capacity and lifecycle authority needed for
   one later valid exact-host/exact-scope relinquishment. Keep forged, wrong-
   host, wrong-scope, released, repeated, and finished-task cases fail-closed.
4. Extend the selected local-host lifecycle so shutdown stops new reservations,
   waits issued reservations to release or settle and accepted tasks to finish,
   then joins retained reapers outside the ordinary task tree.
5. Keep ordinary `spawn`, explicit `join`, and `LocalJoinedTask` join-on-drop
   behavior unchanged. Release of an unused reservation is non-blocking and
   does not create a detached task class.
6. Add deterministic real-`LocalHostServices` proofs for reservation refusal
   before effects, shutdown racing an issued reservation, later accepted reap,
   outer reaper join, clone ownership, and the production blocking-drop
   counterexample. A fixture that discards its worker `JoinHandle` is invalid.
7. Reconcile Contracts 009, 010, 017, 019, and 047, semantic API evidence,
   runtime/host lifecycle guidance, changelog, and the god-file baseline for the
   behavior actually delivered.

## Out Of Scope

Claude adapter or sidecar implementation; restoration of `claude-agent.sdk`;
provider contact; credential acquisition; working-resource acquisition;
process launch; a global executor or task registry; detached work; default
timeouts; release preparation; merge; tag; publish; version currentness.

## Acceptance Criteria

- [x] one exact-host/exact-scope reservation is granted before any operation
      effect and is visibly owned until unused release or task transfer
- [x] unsupported, closing, and capacity-exhausted hosts reject before
      credentials, resources, processes, tasks, or provider work
- [x] after grant, valid exact-host/exact-scope relinquishment cannot fail for
      shutdown, capacity, or host-lifecycle reasons
- [x] the grant binds to one task before work is polled and releases only when
      unused, ordinarily completed/joined, or transferred with that task
- [x] a boolean capability probe plus later unreserved transfer is absent and a
      mutation to that shape fails the shutdown-race oracle
- [x] caller expiry can transfer a real stalled local task without blocking on
      its synchronous join-on-drop behavior
- [x] eventual task completion is reaped with no second adapter call, discarded
      worker handle, or adapter-global parking
- [x] shutdown stops admission, settles all live reservations and accepted
      tasks, then joins retained reapers outside the task tree
- [x] wrong host, wrong scope, released or forged reservation, repeated
      transfer, and finished task retain honest ordinary ownership/failure
- [x] `AcceptedForReap` remains distinct from joined task and cleanup success
- [x] existing ordinary spawn, explicit join, and drop-join proofs remain green
- [x] no provider-specific type or consumer-facing task handle is added

## Outcome

`ScopedTaskService` now issues one opaque `TaskReapReservation` and starts its
task through `spawn_reapable`. Host-local reserves a bounded reaper lane before
task work, binds the grant to the exact local lifecycle and scope, and
synchronizes ordinary completion with later transfer so acceptance cannot race
reaper exit. Explicit shutdown closes admission, waits issued or bound work to
settle, then joins the lanes outside the task tree. Unreserved tasks retain the
existing spawn, join, and join-on-drop path. No provider or adapter changed.

## Validation

```sh
cargo fmt -p swallowtail-runtime -p swallowtail-host-local
effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-testkit
effigy package:verify-affected swallowtail-runtime swallowtail-host-local swallowtail-testkit
effigy package:api
effigy qa:docs
effigy qa:northstar
effigy --json scan god-files
git diff --check
```

Do not run provider/live probes, release commands, broad workspace tests, or
adapter validation. Add another shared package only if the runtime boundary
necessarily changes it and record why.

Result: focused validation passed 464 tests plus clippy; affected-package proof
passed for all three named packages; semantic API passed all 40 v0.3.3 packages;
docs, Northstar, and diff checks passed; the inherited god-file census remained
386 findings with no new entry.

## Review Oracle

Invariant: a reservation granted before effects makes one valid later handoff
to the exact selected host and scope non-fallible for capacity and lifecycle,
while preserving host-owned eventual reap and ordinary task semantics.

Smallest counterexample: real `LocalHostServices` reports support, shutdown
closes handoff admission, a stalled local task reaches its caller deadline, and
relinquishment returns the handle; the error arm drops it and synchronously
joins forever.

Required proof: the real blocking local handle, an issued-reservation/shutdown
barrier, an accepted task released after caller return, explicit outer-owner
reaper join, pre-effect rejection counters, and mutation that removes the held
reservation or permits late refusal.

## Auto-Continuation

No. Stop for independent exact-head review. Card 055 remains blocked until this
card merges and is then reassessed from canonical main.

## Stop Conditions

Stop on a design that checks only a boolean, reserves after effects, can refuse
a valid reserved transfer after shutdown starts, releases capacity before
handoff settles, detaches or forgets a worker, joins reapers inside the task
tree, changes ordinary spawn/join/drop, requires adapter code, or treats
`AcceptedForReap` as terminal cleanup evidence.
