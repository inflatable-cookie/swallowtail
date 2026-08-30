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

Review of PR 126 required three revisions. The latest bounds retired bridge
proof, snapshots only after in-flight connections join, drives the
direct-gate counterexample through the production recording seam, and
formats the CI-red fake-process constructor.

PR 126 merged by fast-forward at `c8691e84` after exact-head review. Stable,
documentation/API, pinned Rust `1.95.0`, dependency policy, and external
source-consumer CI were green. No provider turn ran during review or merge.

## Evidence

- Silent fast-watcher and interleaved host-local feed fixtures
- Silent-provider Claude HostWatcher started/updated/completed
- Fake-provider Stop-reentry conjunction plus oracle negatives
- Feed overflow, closed, and revision-regression fail closed
- Panic Drop workspace cleanup
- Live probe compiled `--no-run` only

## Next

The repaired oracle is ready for a separately authorized live acceptance
attempt. The operator decides whether to authorize one fresh exact Claude Code
`2.1.251` turn. Card 011 and g05.003 remain stopped until then. Do not run the
live selector from this closeout.
