# 2026-08-30 g05.006 Card 019 Watcher Proof Repair

Status: complete
Owner: Tom
Card: 019
Milestone: g05.006
Contract: 044, 059, 060

## Result

Credential-free repair of the three reviewed defects in prototype head
`49f2692f`. Work ran on worktree
`/Users/tom/Dev/worktrees/swallowtail-g05-006-watcher-proof-repair` and branch
`worker/g05-006-watcher-proof-repair` from pushed `main`. The prototype was
not merged or cherry-picked whole.

A host-owned `WatcherLifecycleFeed` retains accepted, running, and terminal
snapshots independently of provider stdout. The Claude pump polls that feed
concurrently with process output and projects only through
`project_watcher_activity`. Joined cleanup is not a second completed activity.
`HostWatcher` complete-lifecycle is advertised only on watcher opt-in.

The future-live oracle is an ordered conjunction of bounded facts: MCP
initialize, tools list, start, active completion-gate, Stop hook, same-session
continuation, wait or stop, joined zero, and provider success. Proactive wait,
direct gate, reorder, and terminal-only traces fail. The live probe records
those facts from the host-local reserved-operation log plus Hook activity; it
no longer treats `WATCHER_LIVE_OK` plus completed terminal as sufficient.
Temporary workspaces use a Drop owner established before assertions. The live
selector was not run.

Card 011 and g05.003 remain evidence stops. No watcher support, matrix, guide,
or version-range claim is published.

## Public surfaces

- `swallowtail-runtime`: `WatcherLifecycleFeed`, `WatcherLifecycleSubscription`,
  `WatcherHostService::open_lifecycle_feed`
- `swallowtail-host-local`: `WatcherBridgeProofKind` and
  `LocalHostServices::watcher_bridge_proof(turn)` (reserved operation names
  for one turn only)

Review of PR 126 required two revisions. The second scopes bridge proof to
the owning turn, drives fake Stop-reentry through one in-order recording
seam, preserves first-drain lifecycle errors, proves exact-once
started→updated→completed on cancel/deadline/provider-failure, proves
pre-pump failure cleanup and same-turn retry, and restores the 390
god-file baseline (341 warnings / 49 errors).

## Evidence

- Silent fast-watcher and interleaved host-local feed fixtures
- Silent-provider Claude HostWatcher started/updated/completed
- Fake-provider Stop-reentry conjunction plus oracle negatives
- Feed overflow, closed, and revision-regression fail closed
- Panic Drop workspace cleanup
- Live probe compiled `--no-run` only

## Next

Orchestrator review of this PR. After merge, reassess whether the repaired
oracle is strong enough to request fresh live authorization. Do not run the
live selector from this closeout.
