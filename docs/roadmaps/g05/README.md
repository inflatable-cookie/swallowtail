# g05 Harness Skill Visibility And Process Observability

Status: active
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-31

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
| Prove one skill-discovery route and one watcher-enforcement route without flattening provider behavior. | Qoder and Claude live proofs stopped; card 020 Linux turn consumed; live claim withheld | Contracts 058-060; Research 256-261; card 011 live stop; g05.006 card 019; g05.007 card 020 | `g05.002`, `g05.003`, `g05.006`, and `g05.007` |
| Publish bounded consumer-facing observation without raw process or ambient-host leakage. | planned | selected activity and privacy contracts | after host registry and route proof |

## Planned Next Roadmaps

- [g05.007 Claude Watcher Live Acceptance](007-claude-watcher-live-acceptance.md) — stopped after live evidence; card 020 Linux Haiku turn consumed; ordered recorder kept only JoinedZero; claims withheld
- [g05.006 Watcher Proof Repair](006-watcher-proof-repair.md) — merged through PR 126 at `c8691e84`; card 019 credential-free lifecycle feed and Stop-reentry oracle; no live claim
- [g05.005 Claude Code 2.1.251 Useful Newer](005-claude-code-2-1-251-useful-newer.md) — completed standing currentness; Research 261; cards 017-018; prerequisite to watcher card 010 reassessment
- [g05.004 Qwen Headless 0.22.3 Useful Newer](004-qwen-headless-0-22-3-useful-newer.md) — completed standing currentness
- [g05.003 Operation-Scoped Watcher Proof](003-operation-scoped-watcher-proof.md) — stopped after live evidence; exact Haiku never created a host watcher; first route claim withheld
- [g05.002 Effective Harness Skill Visibility Proof](002-effective-harness-skill-visibility-proof.md) — stopped after Research 256; card 004 complete; cards 005-006 remain planned
- [g05.001 Harness Skill And Watcher Surface Inventory](001-harness-skill-and-watcher-surface-inventory.md) — completed; Research 255, operator decisions, Contracts 058-059, and proof dispositions closed

g05 has seven numbered roadmaps: four completed milestones including standing
currentness at 004 and 005, honest evidence stops at 002, 003, and 007.

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
closed Research 261 and raised both Claude Code axes through official
`2.1.251`. Card 010 bound exact `2.1.251` with operation-private MCP, settings,
skill, Stop continuation, and deterministic provider-free fixtures. It does
not admit a watcher range or support claim. Card 011 consumed its one
authorized exact `claude-haiku-4-5` turn on 2026-08-30. Exact identity and the
frozen digest matched, but the host registry never observed a watcher. The
same-turn oracle remains unproved and the first route claim stays withheld.
Prototype head `49f2692f` remains unmerged: its live selector cannot observe
Stop re-entry, its adapter-local activity feed emits terminal-only activity
despite the existing runtime projector, and its failed assertion leaves an
empty temporary workspace. The operator selected repair before any fresh live
authorization. Contract 059 now requires bounded lossless lifecycle delivery
independent of provider stdout cadence. g05.006 card 019 completed that
credential-free repair: a host-owned lifecycle feed, `project_watcher_activity`
as the only projector, a bounded Stop-reentry recorder, panic-safe workspace
cleanup, and deterministic counterexamples. PR 126 landed by fast-forward at
`c8691e84` after exact-head review and five green CI jobs. The live selector was
not run and no watcher support claim is published. Card 011 and g05.003 remain
evidence stops. The operator separately authorized g05.007 card 020 for exactly
one fresh `2.1.251` turn using exact `claude-haiku-4-5`, with no fallback or
rerun. The first card 020 worker stopped before contact because its Linux host
could not satisfy the Darwin-only digest envelope; PR 127 remains unmerged.
The operator selected Linux on 2026-08-31 and authorized a bounded
per-platform digest repair before the same one-shot turn. Card 020 repaired
selection at `adb04f17`, then consumed the turn. The ordered recorder kept
only `JoinedZero`. Watcher claims stay unpublished. No second provider turn,
Darwin dispatch, or merge is authorized.

g05.004 standing currentness closed Research 258 and cards 012-013 for Qwen
`0.22.3` without moving the generation pointer.

Contract 029 currentness remains standing and does not move this pointer.
Bedrock items 79-80 and the compact deferred-route note remain parked.

## Milestones

- [007 Claude Watcher Live Acceptance](./007-claude-watcher-live-acceptance.md) — stopped after live evidence; card 020 Linux Haiku turn consumed; claims withheld
- [006 Watcher Proof Repair](./006-watcher-proof-repair.md) — completed and merged through PR 126 at `c8691e84`; card 019; no provider authorization; claims unpublished
- [005 Claude Code 2.1.251 Useful Newer](./005-claude-code-2-1-251-useful-newer.md) — completed (standing currentness), cards 017-018
- [004 Qwen Headless 0.22.3 Useful Newer](./004-qwen-headless-0-22-3-useful-newer.md) — completed (standing currentness), cards 012-013
- [003 Operation-Scoped Watcher Proof](./003-operation-scoped-watcher-proof.md) — stopped after live evidence; exact Haiku never created a host watcher; prototype unmerged
- [002 Effective Harness Skill Visibility Proof](./002-effective-harness-skill-visibility-proof.md) — stopped after evidence; card 004 complete; cards 005-006 planned
- [001 Harness Skill And Watcher Surface Inventory](./001-harness-skill-and-watcher-surface-inventory.md) — completed; cards 001-003 closed
