# 015 Claude Code Watcher Bridge Transport Evidence

Status: ready
Owner: Tom
Created: 2026-08-29
Updated: 2026-08-29
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: completed cards 009 and 014; Contracts 010, 041, and 059; Research 257
Research: 260

## Goal

Determine the exact operation-private transport that can connect Claude Code's
watcher MCP tools and Stop hook to the in-process `WatcherHostService` without
ambient configuration, generic tool execution, or unowned background work.
Revalidate the seam on the current Claude Code line before card 010 wiring.

## Scope

1. Freeze the current installed Claude Code identity and official MCP, settings,
   hooks, skill-loading, and headless behavior that changed after Research 257's
   `2.1.220..=2.1.241` window.
2. Compare the exact admitted private MCP transports. For each viable HTTP or
   stdio shape, trace who binds or launches it, how Claude reaches it, how the
   active runtime turn and watcher host port are correlated, and how spoofing
   or cross-turn calls fail closed.
3. Identify the smallest host-owned endpoint, IPC, or helper lease needed for
   the bridge. Do not treat the sign-in loopback callback port or
   `ServingEndpointService` as generic listener authority.
4. Prove ownership and cleanup for the MCP server or helper, Stop hook,
   operation-private skill/settings material, Claude process, watcher work, and
   every cancellation, deadline, hook failure, channel failure, and provider
   terminal path.
5. Separate current prompt-free/package evidence from the still-unrun live
   same-turn re-entry proof. Run an authenticated or paid provider turn only
   after explicit operator authorization.
6. Return Research 260 with one exact viable bridge, the smallest required
   contract/architecture delta, or an honest stop. Rewrite card 010's gate but
   do not implement the route.

## Output

Research 260, frozen Claude-local evidence where useful, one lane log, and an
exact card 010 readiness disposition. No production MCP server, listener,
helper, skill, hook, or route code.

## Evidence Stop

Research 260 freezes the current native Claude Code `2.1.251` point and the
current official MCP, hooks, headless, skills, settings, and CLI surfaces. The
provider-side composition remains a candidate, but no qualified current
version segment or live same-turn proof is admitted.

The evidence run is complete. The card remains in the orchestrator-owned ready
bucket until the post-review closeout reconciles shared roadmap indexes; this
does not authorize a second dispatch or card 010 implementation.

The current host registers `WatcherHostService` and owns joined ordinary
process supervision. It has no MCP listener, provider-to-existing-process
handoff, or operation-private IPC service. `ServingEndpointService` publishes
an endpoint observed from an owned child and does not bind; the loopback
callback service is sign-in-only.

HTTP is the smallest future carrier but requires a new host-owned listener and
bridge contract. Stdio requires a provider-launched helper and a host IPC
handoff, which the current contracts do not provide. Card 010 remains planned.

## Acceptance Criteria

- [x] the current provider-side MCP and Stop-hook surface is frozen, with no
      qualified current version segment claimed
- [x] HTTP, stdio, SSE, WebSocket, and reverse-direction `claude mcp serve`
      ownership are compared against the host boundary
- [x] the missing host binding, private correlation requirements, and
      joined-cleanup delta are explicit
- [x] omission remains the exact empty strict MCP command with no watcher
      service requirement or materialization
- [x] live same-turn re-entry remains an explicit, unauthorized blocker
- [x] card 010 remains planned because transport, current-version, and live
      acceptance decisions are unresolved

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- a viable bridge requires an ambient listener, persistent user/project
  configuration, provider-launched unowned helper, arbitrary command authority,
  raw PID, or generic consumer-tool executor
- endpoint authentication, turn correlation, or joined cleanup cannot be
  represented without a new contract decision
- current-version evidence requires login, credentials, paid work, or a model
  prompt without explicit operator authorization
- Research 257's same-turn behavior no longer applies to the current version
  segment

## Auto-Continuation

No. Return Research 260 and one reviewable PR. Do not start card 010.
