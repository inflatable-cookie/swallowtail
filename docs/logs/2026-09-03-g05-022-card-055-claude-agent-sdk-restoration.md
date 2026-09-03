# 2026-09-03 g05.022 Card 055 Claude Agent SDK Restoration

Status: complete; pending independent exact-head review
Owner: Tom

## Result

The provider-free `claude-agent.sdk` route is restored from canonical main at
`53153af1`, after g05.025 card 061 merged. It is not the reverted PR 188 head:
the three findings that rejected that head are repaired at their causes.

## Reap Authority Before Effects

`open_session` reserves three operation-scoped reap lanes from the exact
selected `ScopedTaskService` — open guardian, pump, and close guardian — before
it acquires a credential, resolves a working resource, starts the sidecar,
spawns any task, or contacts the provider. An unsupported, closing, or
capacity-exhausted host refuses the whole operation there, with no effect taken.
The grant is opaque owned authority, not a boolean support probe.

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

The caller waits for that continuation inside its own cleanup deadline. On
expiry it transfers the guardian — never the pump alone — under the held
exact-host, exact-scope reservation and reports
`close_cleanup_unconfirmed` without waiting. The transferred guardian keeps the
process and both leases until its ordered cleanup finishes. `AcceptedForReap`
remains ownership transfer only and never becomes join or cleanup evidence.

Open failure keeps the same shape through `OpenGuard`: the guard is armed before
the first acquisition, is itself reservation-backed, and its cleanup is claimed
or run under one atomic ledger transition.

A session dropped without close hands its pump back to the owning host through
the same reservation rather than joining it on the dropping thread.

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
