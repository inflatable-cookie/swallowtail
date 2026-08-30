# g05 Harness Skill Visibility And Process Observability

Status: active
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-30

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
| Promote architecture and contracts for any selected portable observation or control seam. | Contracts 058-060 promoted | evidence and operator decisions | `g05.001` card 003; `g05.003` card 016 |
| Prove one skill-discovery route and one watcher-enforcement route without flattening provider behavior. | Qoder proof stopped; watcher lifecycle, registry, supervision, and HTTP bridge core complete | Contracts 058-060; Research 256-260 | `g05.002` and `g05.003` |
| Publish bounded consumer-facing observation without raw process or ambient-host leakage. | planned | selected activity and privacy contracts | after host registry and route proof |

## Planned Next Roadmaps

- [g05.005 Claude Code 2.1.251 Useful Newer](005-claude-code-2-1-251-useful-newer.md) — ready standing currentness; prerequisite to watcher card 010
- [g05.004 Qwen Headless 0.22.3 Useful Newer](004-qwen-headless-0-22-3-useful-newer.md) — completed standing currentness
- [g05.003 Operation-Scoped Watcher Proof](003-operation-scoped-watcher-proof.md) — ready; card 016 landed; card 010 waits on g05.005; card 011 keeps the separate live gate
- [g05.002 Effective Harness Skill Visibility Proof](002-effective-harness-skill-visibility-proof.md) — stopped after Research 256; card 004 complete; cards 005-006 remain planned
- [g05.001 Harness Skill And Watcher Surface Inventory](001-harness-skill-and-watcher-surface-inventory.md) — completed; Research 255, operator decisions, Contracts 058-059, and proof dispositions closed

g05 has five numbered roadmaps: two completed milestones, honest evidence stops
at 002, the ready watcher milestone at 003, and ready standing currentness at
005.

## Current Boundary

g05.001 and cards 001-003 are closed. Contracts 058 and 059 govern the two
independent surfaces. Research 256 returns an honest empty Qoder roster set, so
cards 005-006 stay planned. Research 257 admits the Claude Code watcher seam.
Card 008 closed the provider-neutral watcher core. Research 259 proves the
default macOS process boundary cannot supply hard descendant containment, but
the operator clarified that the watcher feature does not require that security
boundary. Card 009 completed the registry on restacked PR 117. Card 014 landed
through PR 118, restoring ordinary host-approved process supervision with an
explicit detached-process non-claim. Card 015 and Research 260 landed through
PR 119 at `c36e11ad`: current Claude `2.1.251` exposes the provider-side seam,
but Swallowtail has no provider-to-host MCP bridge contract or current live
same-turn proof. On 2026-08-30 the operator selected the minimal HTTP boundary.
Contract 060 owns the closed provider-neutral operation bridge. Card 016 landed
the host service, private authority, closed HTTP/MCP surface, terminal barrier,
and joined cleanup. The post-bridge orchestrator checkpoint found that card 010
could not depend on a live proof owned by later card 011. g05.005 cards 017-018
now own the base Claude Code `2.1.251` currentness prerequisite. After that
lands, card 010 may become a credential-free binding card; card 011 alone keeps
the separately authorized live same-turn gate and first route claim.
g05.004 standing currentness closed Research 258 and cards 012-013 for Qwen
`0.22.3` without moving the generation pointer.

Contract 029 currentness remains standing and does not move this pointer.
Bedrock items 79-80 and the compact deferred-route note remain parked.

## Milestones

- [005 Claude Code 2.1.251 Useful Newer](./005-claude-code-2-1-251-useful-newer.md) — ready (standing currentness), cards 017-018
- [004 Qwen Headless 0.22.3 Useful Newer](./004-qwen-headless-0-22-3-useful-newer.md) — completed (standing currentness), cards 012-013
- [003 Operation-Scoped Watcher Proof](./003-operation-scoped-watcher-proof.md) — ready; card 016 landed; card 010 waits on currentness; card 011 waits on binding and live authorization
- [002 Effective Harness Skill Visibility Proof](./002-effective-harness-skill-visibility-proof.md) — stopped after evidence; card 004 complete; cards 005-006 planned
- [001 Harness Skill And Watcher Surface Inventory](./001-harness-skill-and-watcher-surface-inventory.md) — completed; cards 001-003 closed
