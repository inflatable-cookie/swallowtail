# 2026-09-03 g05.025 Reap Reservation Prerequisite Compilation

Canonical main was refreshed to containment merge `3da1b323`. PR 188 review
comments 5524712917 and 5524729651 remain the binding negative evidence: a late
relinquishment refusal can drop and synchronously join the production local task
handle; transferred pump work can outlive resource and credential leases; and
the rejected integrated fixture discarded its worker instead of proving
host-owned reap. PR 193 withdrew `claude-agent.sdk` and preserved card 060.

Contracts 009, 010, 017, 019, and 047 now require a real operation-scoped reap
reservation, or equivalent atomic guarantee, before provider work and before
credential, resource, process, or task acquisition. A boolean capability probe
is insufficient because shutdown can race it. While the grant is live, one
valid exact-host/exact-scope handoff cannot fail for capacity or lifecycle
reasons. Selected-host shutdown closes reservation admission, settles issued
reservations and accepted tasks, then joins reapers outside the task tree.
Ordinary spawn, explicit join, and join-on-drop remain unchanged.

g05.025 card 061 is the sole ready task. It implements only the shared runtime,
local host lifecycle, and real-host proof. The later g05.022 card 055 re-entry
must put pump, process, resource, and credential ownership under one enclosing
guardian. Cleanup order is interrupt, native close, force-stop, root/process
observation, pump completion/join, resource release, then credential release.
Caller deadline transfers the guardian, not the pump. `AcceptedForReap` remains
ownership transfer, never join or cleanup success.

`claude-agent.sdk` stays withdrawn. The `v0.4.0` release lane stays frozen.
This compilation authorizes no runtime or adapter implementation, provider
contact, merge, tag, publish, or release-readiness work.
