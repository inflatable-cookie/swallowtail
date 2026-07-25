# 007 Provider-Wide Facade Contract And Foundation

Status: completed
Owner: Tom
Created: 2026-07-25
Depends on: g02.003
Vision tags: portable integration, explicit authority, consumer simplicity
Contract refs: 005-011, 029, 032-037
Planning state: cards 017-019 complete

## Problem

Codex proves preparation but still leaves some operation wiring to consumers,
and the other production routes expose only low-level construction.

## Goals

- [x] Promote one provider-wide prepared and bound-operation contract.
- [x] Add shared evidence and conformance without provider-specific types.
- [x] Complete Codex typed bound execution as the reference.
- [x] Preserve every low-level role and all explicit authority.

## Execution Plan

### Batch 7.1 — Contract Promotion

- [x] Execute card 017.
- [x] Inventory all 22 production routes and six implementation families.
- [x] Promote Spec 006 into Contract 037 and architecture.

### Batch 7.2 — Shared Foundation

- [x] Execute card 018.
- [x] Add provider-neutral facade evidence and assertion helpers.
- [x] Prove no provider selection, hidden authority, or lifecycle flattening.

### Batch 7.3 — Codex Reference

- [x] Execute card 019.
- [x] Bind separate exec, catalogue, and session operations.
- [x] Retain the current prepared values and low-level escape hatch.

## Acceptance Criteria

- [x] every facade exposes inspectable preparation evidence before effects
- [x] bound execution delegates to an existing low-level role
- [x] operation-specific content and authority remain explicit
- [x] exact and unverified-newer compatibility remain visible
- [x] Codex consumers need no manual driver/request seam

## Evidence Requirements

- shared deterministic facade assertion pack
- Codex exec, catalogue, read-only, and bounded-workspace bound-operation tests
- low-level regression suite
- public API and documentation checks

## Decision Gate

Card 019 passes without changing operation semantics. Roadmap g02.008 is ready;
card 020 starts the representative Kimi Code ACP facade.
