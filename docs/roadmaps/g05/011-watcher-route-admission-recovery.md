# g05.011 Watcher Route Admission Recovery

Status: ready; card 029 ready
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Depends on: Contracts 059-060; g05.006; g05.007 evidence stop
Vision tags: watchers, in-turn control, provider bridge

## Purpose

Resolve the credential-free route blockers before spending another live
provider turn. Card 026 proved that `--tools Read,Glob,Grep` filters built-ins,
not MCP tools. The current watcher command's `--bare` flag instead excludes the
credential paths available to the consumed live envelope.

## Runway

1. Card 026 froze exact `2.1.251` tool-admission semantics and rejected MCP
   suppression without changing production.
2. Card 029 compares credential-preserving watcher-only isolation shapes and
   repairs `--bare` only if private authority remains exact.
3. Reassess live acceptance after focused validation. A new provider turn
   still needs explicit operator authorization.

## Boundary

The provider-neutral watcher registry, activity projector, HTTP bridge, and
Contracts 059-060 are not redesigned. No capability claim, provider contact,
login, paid work, credential read, or new live attempt belongs to either
recovery card.

## Batch Cards

- [026 Claude Watcher Tool Admission Evidence And Repair](batch-cards/026-claude-watcher-tool-admission-evidence-and-repair.md) — complete; hypothesis rejected; `--bare` blocker named
- [029 Claude Watcher Credential-Preserving Isolation](batch-cards/029-claude-watcher-credential-preserving-isolation.md) — ready

## Acceptance

- [x] exact watcher MCP tool admission is proved provider-free
- [ ] one credential-preserving, ambient-isolating watcher command is proved or
      the lane stops honestly
- [ ] another live attempt cannot repeat the named authentication blocker
