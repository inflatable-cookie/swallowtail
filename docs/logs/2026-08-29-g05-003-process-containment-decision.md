# 2026-08-29 g05.003 Process Containment Decision

Status: superseded by host-process watcher direction
Owner: Tom
Milestone: g05.003
Card: 009 revision
Research: 259
Contract: 059

## Decision

Keep Contract 059's hard no-outliving invariant. Process-backed watcher
support now requires an exact host containment backend. The watcher registry,
ordinary process service, root handle, process group, output pipes, process
table, and operating-system name do not imply that capability.

The default macOS local process boundary does not qualify. A host without a
contained execution lease omits process-backed watcher support or rejects its
start before work. Windows Job Objects, Linux cgroup v2, consumer supervisors,
containers, and VMs remain candidates only; each needs its own authority and
conformance proof.

This was the decision used to repair PR 117. It was superseded later on
2026-08-29 when the operator clarified that the watcher feature is lifecycle
feedback and control for ordinary host-managed processes, not hard security
containment for deliberately escaped descendants.

## Evidence

PR 117's review fixture demonstrated a closed-pipe descendant leaving the
owned process group through `setsid`. Apple XNU no longer supports recursive
`kqueue` process tracking, and `launchd` cleanup is process-group based.
Research 259 freezes those sources and the contrasting Windows/Linux
containment shapes.

## Historical Planning Result

Card 009 completed on fast-forwarded PR 117 at `ad51b0e7`. Its registry is
available without claiming a default process backend; process-backed starts
reject before work until an exact containment backend is injected. Card 010
stays gated until an exact containment-capable host composition is proved and
implemented.

The later OCI/Docker research detour was withdrawn before launch. No container
probe or runtime implementation ran.

## Current Direction

Use Research 259 only for the explicit non-claim: ordinary process groups do
not contain a deliberately detached child. Revised Contract 059 permits honest
host-process watcher supervision without containers. Replacement card 014 owns
the implementation repair before card 010.
