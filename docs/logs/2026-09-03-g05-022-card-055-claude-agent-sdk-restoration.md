# 2026-09-03 g05.022 Card 055 Claude Agent SDK Restoration

Status: complete; pending independent exact-head review
Owner: Tom

## Result

The provider-free `claude-agent.sdk` route is restored from canonical main at
`53153af1`, after g05.025 card 061 merged. It is not the reverted PR 188 head:
the three findings that rejected that head are repaired at their causes.

## Cleanup Authority Before Effects

`open_session` reserves three operation-scoped reap lanes from the exact
selected `ScopedTaskService` — open guardian, pump, and close guardian — and
starts both guardian tasks, before it acquires a credential, resolves a working
resource, starts the sidecar, or contacts the provider. A host that cannot
commit those lanes, or cannot create those workers, refuses the whole operation
there, with no effect taken. The grant is opaque owned authority, not a boolean
support probe.

Reservation and worker creation fail differently, and both are pre-effect for
that reason. A reservation guarantees the later handoff of a worker that exists;
it does not make creating one infallible, and an operating system can refuse a
thread. So the close guardian's worker is created while the operation still owns
nothing to lose. After open returns, activating that guardian is a slot write
and a signal: it cannot fail while a live process, pump, and two leases are
already held.

Because the reservation is held for the life of the operation, the later
transfer cannot be refused while the work is unfinished. That removes the
refusal path PR 188 answered by dropping a live handle, which on the real local
host synchronously joins the worker and blocks the caller past its deadline.

## One Enclosing Guardian

Close hands the connection, sidecar process, pump, any remaining turn-deadline
task, and both leases to a single reservation-backed guardian task. That
guardian runs the whole ordered continuation: interrupt a live turn, the sidecar
close command and its own bounded native join, the declared host termination
request, root/process observation, the pump join, working-resource release, then
credential release. Every release therefore happens after the scoped work that
used it stopped.

Ownership moves before the public cleanup future exists. `close` activates the
guardian synchronously and only then builds the future the runtime bounds, so a
runtime that refuses an already-elapsed deadline, a missing time service, or the
wrong host without ever polling that future cannot strand live state.

The caller waits for that continuation inside its own cleanup deadline. Expiry,
caller cancellation, and a dropped or rejected cleanup future all transfer the
guardian — never the pump alone — under the held exact-host, exact-scope
reservation and report `close_cleanup_unconfirmed` without waiting. That
transfer is what `Drop` does for both the open guard and the session guardian:
each holds a real `LocalJoinedTask` whose ordinary drop synchronously joins, so
handing it to the owning host is the only non-blocking move. The transferred
guardian keeps the process and both leases until its ordered cleanup finishes.
`AcceptedForReap` remains ownership transfer only and never becomes join or
cleanup evidence.

Open failure and open cancellation keep the same shape through `OpenGuard`: the
guard is armed before the first acquisition, is itself reservation-backed, its
cleanup is claimed or run under one atomic ledger transition, and dropping it
transfers rather than joins.

A session dropped without close hands its whole state — connection, process,
pump, and both leases — to the same guardian, which skips the cooperative stages
because no caller deadline exists there.

## Cleanup Truth

Unchanged from the accepted posture. `OwnedTreeEmpty` is the only basis for
`Clean`. On ordinary macOS a confirmed root exit after the declared descendant
termination attempt is `Degraded`; an observed survivor or an unconfirmed root
exit is `Failed`. A survivor outranks even an emptiness claim. Windows stays
unsupported because no tree owner survives the root there.

## Proof

The provider-free fixture now models the local host rather than a friendlier
fake: handles own their worker threads, `join` blocks, drop joins, reservations
are exact-host and exact-scope, an unreserved task cannot be upgraded through a
late transfer, and transferred work moves to a retained reaper that only an
outer owner joins. No fixture discards a worker handle.

The three original blockers are falsified directly:

- `an_unreserved_stalled_task_keeps_ordinary_join_and_drop_ownership` and
  `a_host_that_cannot_reserve_reap_is_refused_before_any_effect` prove the
  pre-admission is load-bearing and that refusal happens before any credential,
  process, or sidecar contact.
- `an_unfinished_guardian_is_transferred_without_releasing_either_lease` proves
  the close-guard scope is transferred, the session scope is not, and neither
  lease is released while the continuation is live; releasing the pump alone
  then completes the ordered cleanup with no further route call.
- `the_real_local_host_retains_transfers_and_reaps_the_close_guardian` runs the
  integrated deadline proof on real `LocalHostServices`, returns inside the
  caller bound, and has the outer owner join what it accepted.

`a_shutdown_that_starts_after_the_grant_cannot_refuse_the_handoff` covers the
shutdown race, and `a_stalled_open_returns_on_the_deadline_against_the_real_local_task_host`
keeps the public open path on the real task seam.

## Exact-Head Repair

Independent review rejected `046ab101` on ownership boundaries rather than on
the ordinary polled deadline path, which it re-derived as correct. Three
repairs, each with a mutation-sensitive proof on real `LocalHostServices`:

- **Cleanup could be dropped before the guardian existed.** `close` moved the
  handle into a future the runtime can refuse before polling; the handle's drop
  then transferred only the pump. `close` now activates the guardian before that
  future exists, and the handle's drop hands over everything.
  `a_runtime_rejected_close_still_completes_the_ordered_cleanup` and
  `dropping_close_before_any_poll_still_completes_the_ordered_cleanup` assert the
  sidecar received the cooperative close command and that the ordered cleanup
  finished, which lazy activation cannot produce.
- **Cancellation had no drop handoff.** `OpenGuard` and `SessionGuardian` each
  held a real `LocalJoinedTask` and joined it on drop. Both now transfer
  through the held reservation.
  `cancelling_a_pending_open_starts_its_ordered_cleanup_without_blocking` and
  `dropping_close_after_one_pending_poll_hands_the_guardian_to_the_host` drop
  the public future on another thread and fail if that drop does not return.
- **Cancellation transferred ownership without starting cleanup.** A second
  exact-head review found that `OpenGuard::drop` relinquished the task without
  releasing its cleanup signal, so a cancelled pending open could hold its
  credential and working resource until the abandoned open deadline arrived.
  The drop now triggers that signal before the handoff. The cancellation proof
  no longer fires the host clock at all: it asserts the ordered
  resource-then-credential release happened while `deadlines_fired()` is still
  false. Removing the trigger fails it with
  `host cleanup never recorded CredentialRelease`.
- **Guardian creation was fallible after effects.** It is now started before
  them, so a failed worker creation refuses while nothing is owned.
  `a_close_guardian_that_cannot_start_refuses_before_any_effect` injects the
  post-reservation spawn failure and asserts zero credential acquisitions and no
  sidecar contact.

Each repair was mutated back and the named proofs failed: removing the drop
handoff produced three `did not return inside 5s` failures, restoring lazy
activation failed the two unpolled/rejected cases, moving the guardian's
creation after the credential acquisition failed the spawn-failure case with
`left: 1, right: 0`, and removing the cancellation signal failed the open
cancellation case with `host cleanup never recorded CredentialRelease`.

## Identity And Policy

Rechecked immediately before implementation, without executing anything
downloaded. The subscription article still leads with the paused-changes notice
at its stated `June 16, 2026` update, so the preserved statement — Agent SDK,
`claude -p`, and third-party app usage draw from the user's own subscription
limits — is unchanged. Official npm `dist-tags` are still exactly `latest` and
`next` at `0.3.259`, carrying native `2.1.259`, matching the frozen Research 280
ledger. Nothing stale is carried forward.

## Boundary

No tag, publish, release preparation, live provider turn, login, or merge. The
`v0.4.0` release lane and g05.021/cards 050-052 remain frozen until this
restoration passes independent exact-head review and the operator separately
authorizes a fresh audit.

## Authority

- [g05.022](../roadmaps/g05/022-claude-agent-dual-route-parity.md)
- [card 055](../roadmaps/g05/batch-cards/055-claude-agent-sdk-provider-free-foundation.md)
- [Contract 009](../contracts/009-async-operation-lifecycle.md)
- [Contract 010](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
- [Contract 019](../contracts/019-embedded-sdk-and-cloud-client-boundary.md)
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
- [Research 278](../research/278-claude-agent-sdk-route-evidence.md)
- [Research 280](../research/280-claude-agent-sdk-0-3-259-identity.md)
