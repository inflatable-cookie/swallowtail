# 034 Working-State Restoration Facade

Status: completed
Owner: Tom
Created: 2026-08-05
Depends on: g03.033
Vision tags: restart continuity, prepared integration, portable recovery
Contract refs: 017, 037, 046, 048, 050
Planning state: cards 087-089 completed

## Problem

Consumers must currently branch between session reconciliation, run
reconciliation, and ACP load/replay. Duplicating that qualification downstream
encourages stateful continuation to be reported as read-only terminal truth.

## Generation Runway Goal

Expose one exact-once prepared restoration operation while preserving route,
authority, and evidence strength.

## Goals

- [x] add the provider-neutral prepared facade and outcomes
- [x] add honest Claude Agent ACP and Kimi ACP continuation recovery
- [x] wrap all five qualified reconciliation routes
- [x] publish route guidance and package evidence

## Execution Plan

- [x] card 087: realize the runtime facade kernel and conformance
- [x] card 088: map Claude Agent ACP and Kimi ACP continuation recovery
- [x] card 089: map qualified reconciliation routes and close public acceptance

## Boundaries

- no generic provider router, default provider, credential fallback, or model inference
- no dynamic fallback after reconciliation dispatch
- no terminal inference from replay or provider prose
- no new load, reconciliation, callback, interruption, management, or cleanup authority
- no authenticated provider work

## Acceptance Criteria

- [x] one consuming prepared facade executes every supported method
- [x] outcomes retain session, run, and recovery strength explicitly
- [x] ACP recovery returns replay plus a live session and no lost-turn state claim
- [x] reconciliation remains read-only and failure cannot invoke load
- [x] all seven production mappings pass deterministic focused validation

## Lane Runway

Complete. The seven mappings share one consuming facade and preserve exact
route-specific preparation. g03 returns to its evidence gate.
