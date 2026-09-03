# 060 Scoped Task Relinquishment And Host Reap

Status: complete; exact-host/scope transfer and host-owned supervised reap
Owner: Tom
Created: 2026-09-03
Milestone: `../024-scoped-task-relinquishment.md`
Depends on: Contracts 010 and 019; PR 188 exact-head ownership finding

## Goal

Let a caller-bound operation return at its deadline without dropping or joining
an unfinished host-scoped task, while keeping that task owned and eventually
reaped by the selected execution host.

## Direction

Extend the existing provider-neutral task service instead of creating a
consumer handle or adapter registry. `relinquish` receives the exact scope and
the caller's optional joined-task slot. Success occurs only after the host
accepts ownership, clears the slot, and returns `AcceptedForReap`. Failure
retains the handle for ordinary join/drop ownership.

## Scope

- add the provider-neutral relinquishment outcome and service operation
- keep the handle-side transfer hook hidden within the runtime seam
- bind local task handles to their exact execution host and scope
- retain each per-transfer reaper under an outer selected-host lifecycle owner
- give task-service clones only weak reaper-handoff authority
- explicitly join all retained reapers outside the task tree during host
  shutdown
- preserve explicit join and join-on-drop for every ordinary path
- reconcile Contracts 009, 010, and 019, runtime architecture, lifecycle
  guidance, semantic API evidence, and changelog
- leave card 055 and PR 188 unchanged until this prerequisite lands

## Out Of Scope

Claude SDK implementation; adapter migration; provider contact; a global task
registry or executor; release preparation; tags; version-currentness work;
changing cleanup truth after acceptance for reap.

## Acceptance Criteria

- [x] a stalled local task can be relinquished without blocking its caller
- [x] later completion is reaped autonomously with no second failure or call
- [x] explicit selected-host shutdown outside the task tree accounts for and
      joins the reaper; discarding the reaper handle would fail the
      deterministic lifecycle proof
- [x] a worker may capture and drop a task-service clone without becoming the
      reaper join owner or deadlocking shutdown
- [x] wrong execution host, wrong scope, and repeated ownership fail closed
      while retaining caller ownership on rejection
- [x] an already-finished task rejects relinquishment and joins normally
- [x] ordinary `LocalJoinedTask` explicit join and drop-join behavior is unchanged
- [x] the public outcome says only `AcceptedForReap` and docs forbid using it
      as joined or cleanup-completion evidence
- [x] no provider-specific type or new task handle enters consumer-facing API

## Validation

```sh
effigy validate:focused swallowtail-runtime swallowtail-host-local
effigy package:verify-affected swallowtail-runtime swallowtail-host-local
effigy package:api
effigy qa:docs
effigy qa:northstar
effigy --json scan god-files
git diff --check
```

No broad workspace suite, live probe, provider session, release command, or
version-currentness checkpoint is authorized.

## Review Oracle

Invariant: after acceptance, the caller owns no task handle, the exact selected
host owns eventual reap, and no code can interpret that transfer as a join.

Smallest counterexample: a stalled task reaches the caller deadline and either
blocks on handle drop, moves into adapter-global storage with no reaper, crosses
host/scope authority, or yields `Clean` from accepted-for-reap.

Required proof: a controlled stalled task, later release and observed reap,
outer host shutdown held until the supervised reaper completes, a worker-held
service clone dropped before completion, wrong-host/scope and repeat failures,
a finished-task ordinary join, semantic API evidence, and unchanged local
drop-join coverage.

## Outcome

`ScopedTaskService::relinquish` now takes a mutable optional `JoinedTask` slot.
The local task validates its exact host and scope, rejects a finished or already
transferred worker, registers its reaper with the outer selected-host lifecycle,
and only then hands over the worker and releases caller ownership. Task-service
clones keep weak handoff authority only. Explicit selected-host shutdown outside
the task tree joins every retained reaper. The one public outcome is
`AcceptedForReap`. Existing task handles still join explicitly or on drop.

## Auto-Continuation

No. This independent prerequisite remains complete after PR 188 containment.
The SDK route stayed withdrawn until g05.025/card 061's separate
shared-runtime reservation/reapable-task prerequisite was implemented, merged
at `53153af1`, and accepted at its exact head; card 055 then restored it.
