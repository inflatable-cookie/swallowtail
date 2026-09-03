# g05.025 Reserved Reapable Task Lifecycle

Status: ready; card 061 first
Owner: Tom
Created: 2026-09-03
Depends on: g05.024 card 060; Contracts 009, 010, 017, 019, and 047;
  rejected PR 188 exact-head review; PR 193 containment
Vision tags: caller deadlines, scoped tasks, host shutdown, ordered cleanup

## Purpose

Close the provider-neutral gap left after card 060. A selected host must commit
before operation effects that one enclosing cleanup task can later transfer for
autonomous reap. The commitment must survive shutdown and capacity races so a
caller deadline can never fall back to blocking task drop.

## Goals

- [ ] establish an operation-scoped reap reservation, or equivalent atomic
      guaranteed-reapable capability, before provider work or acquisition
- [ ] make valid exact-host/exact-scope relinquishment non-fallible for capacity
      and lifecycle reasons while the reservation is live
- [ ] bind the reservation to one enclosing task before its work begins so
      reserved capacity cannot disappear independently from that task
- [ ] make shutdown close reservation admission, settle existing reservations
      and accepted tasks, then join reapers outside the task tree
- [ ] keep ordinary spawn, explicit join, and join-on-drop ownership unchanged
- [ ] leave the withdrawn `claude-agent.sdk` route and frozen `v0.4.0` release
      lane untouched during the shared-runtime batch

## Execution Plan

- [ ] **Batch 1 — shared-runtime reservation and host lifecycle.** Execute card
      061 across `swallowtail-runtime`, `swallowtail-host-local`, and only the
      shared fixtures needed for real-host proof. Bind the grant to one exact
      host and scope, prove shutdown races, and preserve ordinary tasks.
- [ ] **Batch 2 — adapter re-entry.** After card 061 merges and passes
      independent exact-head review, reassess and execute g05.022 card 055 on a
      fresh branch from canonical main. The adapter guardian owns the pump,
      process, resource, and credential through ordered cleanup. This planning
      change does not execute that batch.
- [ ] **Planning checkpoint.** After the adapter re-entry passes exact-head
      review, reconcile route truth and decide whether the frozen g05.021
      release-readiness audit may restart. No release action is implied.

## Boundaries

- The reservation is an admission grant, not a capability boolean. A probe can
  become stale before handoff when shutdown or capacity changes.
- Unsupported or closing hosts reject before credentials, resources, processes,
  tasks, or provider work.
- The shared-runtime card does not add a provider, adapter guardian, global task
  registry, detached task, default timeout, or cleanup-success inference.
- `AcceptedForReap` remains ownership transfer only. It is never join evidence,
  cleanup completion, `Clean`, or `Degraded` by itself.
- Do not restore `claude-agent.sdk`, merge, tag, publish, run a provider session,
  or resume release readiness in this milestone's first batch.

## Batch Card

- [061 Reserved Reapable Task Runtime](batch-cards/061-reserved-reapable-task-runtime.md) — ready; provider-neutral reservation, real local-host lifecycle, and shutdown-race proof

## Adapter Re-entry Dependency

g05.022 card 055 remains the sole adapter implementation card. It is blocked on
accepted card 061. Its later guardian must own pump, process, resource, and
credential state; preserve interrupt → native close → force-stop → root/process
observation → pump completion/join → resource release → credential release;
and transfer the enclosing guardian rather than the pump at the caller deadline.

## Acceptance Criteria

- [ ] reservation refusal on an unsupported, closing, or capacity-exhausted host
      occurs before every operation effect
- [ ] a live reservation cannot lose a race with selected-host shutdown or later
      capacity pressure
- [ ] the reservation binds to the enclosing task before work is polled and
      releases only through unused release, ordinary completion/join, or
      accepted transfer
- [ ] accepted work remains host-owned until completion and reap, including when
      the caller has returned
- [ ] shutdown accounts for unused live reservations, transferred tasks, and
      retained reapers without running shutdown inside the task tree
- [ ] mutation from reserved handoff to boolean-probe-plus-late-relinquishment
      reproduces the blocking-drop counterexample
- [ ] ordinary task spawn/join/drop tests remain unchanged and green
- [ ] card 055 names the enclosing-guardian ownership and ordered cleanup
      dependency without restoring adapter code
- [ ] the roadmap front door names only card 061 and g05.021 remains frozen

## Review Oracle

Invariant: once the exact host grants a reservation before effects, one later
unfinished task in that host and scope can always transfer without blocking the
caller, including after shutdown begins.

Smallest counterexample: a boolean probe returns true, shutdown closes the
reaper lifecycle, the operation starts a stalled task, and deadline
relinquishment is refused; dropping the returned real local task handle then
synchronously joins past the caller bound.

Required proof: real `LocalHostServices`, controlled reservation and shutdown
races, unavailable-host pre-effect ordering, eventual accepted-task reap,
outer-owner reaper join, a blocking-drop mutation, and unchanged ordinary task
ownership tests.

## Stop Conditions

Stop if the design relies on a boolean probe, permits reserved exact-scope
handoff refusal after effects, detaches or forgets a task, moves ownership into
adapter-global state, changes ordinary spawn/join/drop, runs shutdown inside the
task tree, or needs provider or adapter implementation to prove the shared
boundary.
