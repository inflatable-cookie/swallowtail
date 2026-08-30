# 2026-08-30 g05.003 Card 011 Live Stop Review

Status: complete; evidence stop
Owner: Tom
Card: 011
Contracts: 044, 059, 060
Worker head: `49f2692f`

## Sanitized Result

The one authorized provider session ran through exact installed Claude Code
`2.1.251` with its frozen native digest and exact `claude-haiku-4-5`. The host
registry never observed a turn-owned watcher. The required sequence—watcher
start, Stop block, same-conversation re-entry, explicit wait or stop, zero
active or unjoined work, and joined success—was not exercised.

No raw provider payload, command, endpoint, credential, process output, or
private path is retained in this log. The attempt is consumed. No watcher
capability, matrix, guide, or version-range claim follows.

## Branch Review

The worker returned pushed head `49f2692f` without a PR. Independent review
keeps that prototype unmerged:

- **oracle gap:** the live selector observes registry presence and final output
  but not a Stop-hook event or same-conversation re-entry. It could pass after
  proactive model wait without proving the smallest counterexample.
- **execution miss:** the adapter adds a second watcher activity projector
  instead of using the existing runtime projector. It emits terminal-only
  HostWatcher activity, so an application cannot display a watcher while it is
  running.
- **validation gap:** the live test removes its temporary workspace only after
  success assertions. The failed run left one empty workspace behind.

Credential-free focused validation passed 327 tests. Affected-package proof
passed. Those checks prove the prototype compiles and its fixtures are
internally green; they do not close the three review findings or the live
oracle.

## Current State

Card 011 is complete as an evidence attempt and stopped as a capability proof.
g05.003 is stopped after live evidence. The first route claim remains
unpublished. No second provider session or merge is authorized.

## Next

Choose the next g05 direction. Any retry needs a proof design that establishes
exact MCP tool discovery and invocation, directly observes Stop re-entry, and
publishes complete in-progress watcher lifecycle through the provider-neutral
activity seam before fresh live authorization.
