# 039 Direct Inference Activity Truth

Status: completed
Owner: Tom
Created: 2026-07-29
Depends on: g02.038
Vision tags: direct inference, attached runtimes, operation truth
Contract refs: 006, 009, 014, 016, 019, 021-022, 024-027, 030-031, 037,
  039-041, 044
Planning state: cards 132-134 completed

## Problem

Direct inference can expose assistant output, reasoning summaries,
provider-owned tools, consumer tool continuation, and provider observations.
It does not expose a harness work log. Realtime media and serving-only routes
have still different operation shapes.

## Generation Runway Goal

Give consumers exact observable truth across all entry points without making
direct APIs look like agent harnesses.

## Goals

- [x] Classify activity applicability for every direct, attached, realtime,
      catalogue, and serving route.
- [x] Map provider-supplied assistant, reasoning-summary, and tool activity.
- [x] Correlate direct tool continuation without exposing private continuation.
- [x] Keep realtime-media lifecycle and transcripts separate.
- [x] Mark catalogue and serving-only operations not applicable.

## Non-Goals

- inventing plans, commands, file changes, or subagents
- consumer orchestration or tool execution
- flattening realtime media into ordinary text turns
- model catalogue or serving lifecycle changes
- live credentials, paid inference, or runtime ownership changes

## Execution Plan

### Batch 39.1 — Applicability And Corpora

- [x] Execute card 132.
- [x] Revalidate exact hosted, direct, attached, and realtime surfaces.
- [x] Classify every activity kind as supported, unavailable, or not
      applicable.

### Batch 39.2 — Text Direct Projection

- [x] Execute card 133.
- [x] Add missing assistant, reasoning-summary, provider-tool, and
      consumer-tool mappings only where documented.

### Batch 39.3 — Realtime And Negative Closeout

- [x] Execute card 134.
- [x] Prove the realtime boundary and catalogue or serving
      non-applicability.
- [x] Run full direct-route regression and package-facing checks.

## Acceptance Criteria

- [x] every production direct route has an exact activity profile
- [x] direct inference never claims harness work
- [x] provider and consumer tool ownership remain distinct
- [x] private reasoning continuation never becomes display content
- [x] realtime transcripts retain their dedicated lifecycle
- [x] catalogue and serving-only routes expose no fake activity
- [x] attached runtime ownership and residency remain unchanged

## Decision Gates

- Stop when a provider supplies raw reasoning without an intended display
  contract.
- Keep identity-only or unavailable disclosure when content cannot be safely
  normalized.
- A separate realtime tool lifecycle requires exact protocol evidence before
  inclusion.

## Next Planning Checkpoint

Roadmap g02.040 owns the provider-wide matrix, extracted-package acceptance,
and consumer handoff. Card 135 is ready.
