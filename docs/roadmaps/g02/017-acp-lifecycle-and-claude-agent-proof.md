# 017 ACP Lifecycle And Claude Agent Proof

Status: completed
Owner: Tom
Created: 2026-07-26
Depends on: g02.015
Vision tags: ACP v1, Claude Agent, capability negotiation
Contract refs: 011, 015, 017, 029, 035, 037-038
Planning state: cards 052-054 completed

## Problem

Stable ACP v1 now defines capability-gated session close and delete.
Swallowtail's frozen ACP subset predates them. Claude Agent already advertises
both across the qualified range, but the adapter does not map either method.

## Goals

- [x] Refresh the bounded ACP v1 codec and corpus without widening unrelated
      session behavior.
- [x] Use native ACP close during Claude Agent handle cleanup where qualified.
- [x] Add bound Claude Agent session deletion with honest ACP and
      provider-specific semantics.
- [x] Prove stdio and explicit remote-ACP composition without transport
      fallback.

## Execution Plan

### Batch 17.1 — Protocol Currentness

- [x] Execute card 052 after g02.015 closes.

### Batch 17.2 — Claude Agent Mapping

- [x] Execute card 053 after the codec and tagged behavior corpus pass.

### Batch 17.3 — Portability Conformance

- [x] Execute card 054 after production mapping is complete.

## Acceptance Criteria

- [x] close and delete require their independent negotiated capabilities
- [x] ACP history removal is not labeled hard deletion by default
- [x] Claude tagged fixtures prove exact stronger or weaker deletion semantics
- [x] missing capability stops before dispatch
- [x] close preserves provider history and delete remains user-directed
- [x] all process, connection, timer, resource, and credential work joins

## Planning Gap

Card 052 resolves the planning gap. Claude close preserves history and rejects
missing or repeated targets. Claude delete removes the primary local
transcript and sibling session directory, rejects missing or repeated targets,
and qualifies `ProviderDataDeleted` with `ProviderDefinedDescendants`.
Card 053 maps that exact behavior without widening portable ACP truth.
Prepared sessions return a delete binding without load or resume. Qualified
native close remains distinct from connection cleanup, and unverified-newer
deletion requires explicit prepared-facade acceptance. Card 054 owns
cross-transport and shared-conformance closeout.

Card 054 completes that closeout. The production stdio path passes the shared
effect-boundary and cleanup matrix. The real remote ACP WebSocket transport
carries the same qualified lifecycle records under both host topologies and
cannot recover through stdio. Remote ACP remains a provider-neutral
unauthenticated transport proof, not a claimed remote Claude route.
