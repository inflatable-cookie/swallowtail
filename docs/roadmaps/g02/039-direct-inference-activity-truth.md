# 039 Direct Inference Activity Truth

Status: planned
Owner: Tom
Created: 2026-07-29
Depends on: g02.038
Vision tags: direct inference, attached runtimes, operation truth
Contract refs: 006, 009, 014, 016, 019, 021-022, 024-027, 030-031, 037,
  039-041, 044
Planning state: cards 132-134 planned

## Problem

Direct inference can expose assistant output, reasoning summaries,
provider-owned tools, consumer tool continuation, and provider observations.
It does not expose a harness work log. Realtime media and serving-only routes
have still different operation shapes.

## Generation Runway Goal

Give consumers exact observable truth across all entry points without making
direct APIs look like agent harnesses.

## Goals

- [ ] Classify activity applicability for every direct, attached, realtime,
      catalogue, and serving route.
- [ ] Map provider-supplied assistant, reasoning-summary, and tool activity.
- [ ] Correlate direct tool continuation without exposing private continuation.
- [ ] Keep realtime-media lifecycle and transcripts separate.
- [ ] Mark catalogue and serving-only operations not applicable.

## Non-Goals

- inventing plans, commands, file changes, or subagents
- consumer orchestration or tool execution
- flattening realtime media into ordinary text turns
- model catalogue or serving lifecycle changes
- live credentials, paid inference, or runtime ownership changes

## Execution Plan

### Batch 39.1 — Applicability And Corpora

- [ ] Execute card 132.
- [ ] Revalidate exact hosted, direct, attached, and realtime surfaces.
- [ ] Classify every activity kind as supported, unavailable, or not
      applicable.

### Batch 39.2 — Text Direct Projection

- [ ] Execute card 133.
- [ ] Add missing assistant, reasoning-summary, provider-tool, and
      consumer-tool mappings only where documented.

### Batch 39.3 — Realtime And Negative Closeout

- [ ] Execute card 134.
- [ ] Prove the realtime boundary and catalogue or serving
      non-applicability.
- [ ] Run full direct-route regression and package-facing checks.

## Acceptance Criteria

- [ ] every production direct route has an exact activity profile
- [ ] direct inference never claims harness work
- [ ] provider and consumer tool ownership remain distinct
- [ ] private reasoning continuation never becomes display content
- [ ] realtime transcripts retain their dedicated lifecycle
- [ ] catalogue and serving-only routes expose no fake activity
- [ ] attached runtime ownership and residency remain unchanged

## Decision Gates

- Stop when a provider supplies raw reasoning without an intended display
  contract.
- Keep identity-only or unavailable disclosure when content cannot be safely
  normalized.
- A separate realtime tool lifecycle requires exact protocol evidence before
  inclusion.

## Next Planning Checkpoint

After card 134, review the provider-wide matrix for unexplained gaps before
starting package acceptance and the consumer handoff.

