# g05 Harness Skill Visibility And Process Observability

Status: active
Owner: Tom
Created: 2026-08-28

## Purpose

Give consuming applications truthful visibility into skills available to an
exact harness session and dependable operation-scoped process activity. Keep
distribution membership, host configuration, model visibility, process
authority, activity projection, and turn completion distinct.

g05 does not make Swallowtail a shell, daemon manager, arbitrary process
controller, skill registry, or consumer UI. Prompt text is not enforcement.

## Generation Runway

| Goal | State | Governing refs | First milestone |
| --- | --- | --- | --- |
| Inventory exact prompt-free skill-discovery and background-process surfaces across production harness routes. | active | Contracts 013, 017, 023, 029, 034, 041, 044, 047, 052; promoted triage | `g05.001` |
| Settle vocabulary, provenance, privacy, process ownership, watcher lifecycle, and turn-completion policy. | planned | current architecture; Contracts 013, 017, 041, 044 | after `g05.001` evidence |
| Promote architecture and contracts for any selected portable observation or control seam. | planned | evidence and operator decisions | after boundary selection |
| Prove one skill-discovery route and one watcher-enforcement route without flattening provider behavior. | planned | promoted contracts | after contract promotion |
| Publish bounded consumer-facing observation without raw process or ambient-host leakage. | planned | selected activity and privacy contracts | after proof routes |

## Planned Next Roadmaps

- [g05.001 Harness Skill And Watcher Surface Inventory](001-harness-skill-and-watcher-surface-inventory.md) — ready; cards 001-003; Research 255 reserved

g05 has one numbered roadmap: no completed milestone, no evidence stop, and one
ready milestone at 001.

## Current Boundary

Only card 001 is ready. It writes evidence. It does not inject a skill, start or
stop a process, run a provider prompt, inspect user homes, mutate harness
configuration, add a route, or define public API. Cards 002-003 remain planned
behind evidence and operator decisions.

Contract 029 currentness remains standing and does not move this pointer.
Bedrock items 79-80 and the compact deferred-route note remain parked.

## Milestones

- [001 Harness Skill And Watcher Surface Inventory](./001-harness-skill-and-watcher-surface-inventory.md) — ready, cards 001-003; Research 255 reserved
