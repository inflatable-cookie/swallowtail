# 016 Attached Harness Probe Compatibility Truth

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.015
Vision tags: maintained compatibility, attached harnesses, deterministic evidence
Contract refs: 020, 029, 037
Planning state: cards 042-043 completed

## Problem

OpenCode's optional attached-server live probe still demands exact `1.14.48`,
while the production claim guarantees published points through `1.18.10` and
permits later stable versions as visibly unverified. The selector contradicts
the route it is meant to check.

## Goal

Make attached-harness live evidence follow the same compatibility
classification as production without weakening health or protocol checks.

## Goals

- [x] audit attached-runtime probes for obsolete exact-version assertions
- [x] classify OpenCode health evidence through its public server claim
- [x] prove qualified, unverified-newer, incompatible, and malformed cases
- [x] retain the frozen OpenAPI and selected-path checks
- [x] compile the gated live target without requiring a running server
- [x] return to the g03 maintenance checkpoint

## Execution Plan

### Batch 16.1 — Probe Invariant And Deterministic Classification

- [x] Execute card 042.
- [x] reconcile OpenCode and Ollama attached-probe posture
- [x] replace the obsolete OpenCode exact pin with claim-based classification
- [x] freeze permitted and rejected health observations

### Batch 16.2 — Gated Selector And Maintenance Acceptance

- [x] Execute card 043.
- [x] compile the feature-gated live target without provider effects
- [x] run focused OpenCode and docs validation once
- [x] record the live endpoint as optional operator evidence

## Boundaries

- no production compatibility-range change
- no provider prompt, authentication, catalogue, session, or workspace effect
- no automatic server startup or endpoint discovery
- no hard denial solely because a stable server is newer than the guarantee
- no Gemini implementation or backlog promotion
- no consumer edit, installation, publication, or broad workspace suite

## Acceptance Criteria

- [x] exact `1.18.10` passes the deterministic probe classifier
- [x] later stable versions remain permitted and visibly unverified
- [x] below-baseline, gap, prerelease, malformed, and unhealthy evidence fails
- [x] the live selector retains health, OpenAPI, and selected-path checks
- [x] focused and gated-target validation pass
- [x] one sole Next Task pointer remains

## Next Planning Checkpoint

Return to g03 maintenance after card 043. Select later work only from new
consumer evidence, material non-deferred drift, or the planned prepared-facade
usability reassessment.
