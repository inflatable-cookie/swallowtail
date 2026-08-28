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
| Inventory exact prompt-free skill-discovery and background-process surfaces across production harness routes. | evidence complete | Research 255; Contracts 013, 017, 023, 029, 034, 041, 044, 047, 052 | `g05.001` card 001 |
| Settle vocabulary, provenance, privacy, process ownership, watcher lifecycle, and turn-completion policy. | operator decisions recorded | Research 255; current architecture; Contracts 013, 017, 041, 044 | `g05.001` card 002 |
| Promote architecture and contracts for any selected portable observation or control seam. | Contracts 058-059 promoted | evidence and operator decisions | `g05.001` card 003 |
| Prove one skill-discovery route and one watcher-enforcement route without flattening provider behavior. | ready in parallel | Contracts 058-059 | `g05.002` and `g05.003` |
| Publish bounded consumer-facing observation without raw process or ambient-host leakage. | planned | selected activity and privacy contracts | after proof routes |

## Planned Next Roadmaps

- [g05.004 Qwen Headless 0.22.3 Useful Newer](004-qwen-headless-0-22-3-useful-newer.md) — completed standing currentness
- [g05.003 Operation-Scoped Watcher Proof](003-operation-scoped-watcher-proof.md) — ready; Claude evidence card 007 and portable core card 008 ready in parallel
- [g05.002 Effective Harness Skill Visibility Proof](002-effective-harness-skill-visibility-proof.md) — ready; Qoder evidence card 004 ready
- [g05.001 Harness Skill And Watcher Surface Inventory](001-harness-skill-and-watcher-surface-inventory.md) — completed; Research 255, operator decisions, Contracts 058-059, and proof dispositions closed

g05 has four numbered roadmaps: two completed milestones, no evidence stop, and
two ready proof milestones.

## Current Boundary

g05.001 and cards 001-003 are closed. Contracts 058 and 059 now govern the two
independent surfaces. Qoder is evidence only for effective skill visibility;
Claude Code headless is evidence only for an operation-private watcher MCP,
skill, and pre-terminal hook seam. Neither route advertises the capability.

Cards 004, 007, and 008 can run in parallel. The two evidence lanes own unique
research and route-local fixtures. The core lane owns provider-neutral records,
runtime roles, state transitions, activity projection, and testkit assertions
without host process or adapter work. Cards 005-006 and 009-011 remain planned
behind explicit positive evidence and implementation dependencies.

Contract 029 currentness remains standing and does not move this pointer.
Bedrock items 79-80 and the compact deferred-route note remain parked.

## Milestones

- [004 Qwen Headless 0.22.3 Useful Newer](./004-qwen-headless-0-22-3-useful-newer.md) — completed (standing currentness), cards 012-013
- [003 Operation-Scoped Watcher Proof](./003-operation-scoped-watcher-proof.md) — ready; cards 007-008 ready
- [002 Effective Harness Skill Visibility Proof](./002-effective-harness-skill-visibility-proof.md) — ready; card 004 ready
- [001 Harness Skill And Watcher Surface Inventory](./001-harness-skill-and-watcher-surface-inventory.md) — completed; cards 001-003 closed
