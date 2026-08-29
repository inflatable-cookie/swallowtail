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

## Acceptance Criteria

- [ ] the provider-side MCP and Stop-hook transport is exact for a named
      qualified version segment
- [ ] one host-side binding reaches the same turn-owned
      `WatcherHostService` without ambient discovery or generic tool authority
- [ ] endpoint, token, path, helper, and provider ids stay private and cannot
      authorize foreign or stale calls
- [ ] server/helper tasks and operation-private material have explicit
      cancellation, failure, release, and joined-cleanup order
- [ ] omission remains the exact empty strict MCP command with no watcher
      service requirement or materialization
- [ ] live same-turn re-entry is either proved under explicit authorization or
      remains a named blocker
- [ ] card 010 is marked ready only if no contract, transport, version, or live
      acceptance decision remains unresolved

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
