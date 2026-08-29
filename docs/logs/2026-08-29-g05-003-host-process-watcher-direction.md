# 2026-08-29 g05.003 Host-Process Watcher Direction

Status: complete
Owner: Tom
Milestone: g05.003
Card: 014 compilation
Research: 259
Contract: 059

## Decision

The watcher feature reports and controls processes the agent starts through a
host-approved operation. The process runs in the ordinary host environment.
The consuming application sees bounded running, progress, terminal, and
failure state; model and operator controls address the same turn-owned watcher.

“Background” describes concurrent app-visible work inside the turn. It does
not mean a detached daemon that survives the turn. The turn waits for the
managed process and its supervision to finish or stops and joins them during
cleanup.

## Boundary

The existing local process handle, process group, output readers, stop
escalation, and supervisor join are the correct execution mechanics. They are
not a sandbox. A child that deliberately calls `setsid` or daemonizes can leave
the process group; Swallowtail makes no claim to discover or kill it, and hosts
must not approve watcher operations intended to detach.

Containers, VMs, cgroups, Job Objects, privileged helpers, and platform hard
containment are not prerequisites for this feature.

## Planning Result

Contract 059, Contract 010, the product guardrails, and Research 259 now carry
the narrower lifecycle boundary. The Docker research card, triage note, and
worker handoff were withdrawn before launch. Replacement card 014 is ready to
repair the pre-1.0 host-local API and default composition. Card 010 follows it,
then card 011 closes route acceptance and consumer projection.

## Next Move

Execute card 014 in one worker PR. Do not start Claude route wiring in the same
PR.
