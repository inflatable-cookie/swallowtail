# 2026-08-29 g05.003 Process Containment Decision

Status: complete
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

## Evidence

PR 117's review fixture demonstrated a closed-pipe descendant leaving the
owned process group through `setsid`. Apple XNU no longer supports recursive
`kqueue` process tracking, and `launchd` cleanup is process-group based.
Research 259 freezes those sources and the contrasting Windows/Linux
containment shapes.

## Planning Result

Card 009 is revised and ready to repair/restack PR 117. Its registry, wait,
retirement, rollback, and wake-up work may remain, but default process-backed
watcher registration and process-table observation as containment evidence
must not. Card 010 stays gated until an exact containment-capable host
composition is proved.

## Next Move

Run the replacement card 009 worker handoff, update PR 117 from current pushed
`main`, and return the exact head for review. Do not merge or begin card 010.
