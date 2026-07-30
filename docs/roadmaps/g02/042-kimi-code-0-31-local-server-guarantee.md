# 042 Kimi Code 0.31 Local-Server Guarantee

Status: completed
Owner: Tom
Created: 2026-07-30
Depends on: g02.041
Vision tags: harness compatibility, version ranges, local server
Contract refs: 029, 032, 037-038, 042, 044
Planning state: cards 140-141 completed

## Problem

Roadmap 041 deliberately left Kimi Code local-server `0.31.0` unverified
because its subagent status broadcaster changed. The delta is now classified
and the installed foreground server is available for bounded live proof.

## Generation Runway Goal

Move the separate Kimi local-server guarantee to the installed maintained
release without flattening its new status behavior into the earlier revision.

## Goals

- [x] Freeze the exact `0.31.0` status-projection delta.
- [x] Prove installed startup, bearer enforcement, metadata, and catalogue.
- [x] Add an explicit `0.31.0` local-server behavior milestone.
- [x] Extend the maintained guarantee through `0.31.0`.
- [x] Keep later releases visible as unverified newer.

## Non-Goals

- provider inference or callback testing
- portable exposure of Kimi status model, context, or usage fields
- session deletion
- Python `kimi-cli`
- consumer repository edits
- registry publication

## Execution Plan

### Batch 42.1 — Evidence And Corpus

- [x] Execute card 140.
- [x] Promote Research 069.
- [x] Freeze the richer subagent status payload and unchanged projection.

### Batch 42.2 — Range Extension And Closeout

- [x] Execute card 141.
- [x] Advance the claim and behavior segments.
- [x] Refresh current route and version documentation.
- [x] Run focused Kimi local-server and docs validation.

## Acceptance Criteria

- [x] `0.30.0` and `0.31.0` classify as qualified local-server releases
- [x] `0.31.0` binds a distinct behavior revision
- [x] `0.32.0` remains visible unverified newer
- [x] richer subagent status remains non-rendered progress
- [x] installed bearer-protected metadata and catalogue smoke passes
- [x] no secret or raw live payload enters repository state

## Decision Gates

- Do not widen portable activity content from provider UI status fields.
- Do not infer inference, callback, archive, or restore behavior from the live
  read-only smoke.
- Do not run broad workspace or package suites for this bounded range change.

## Next Planning Checkpoint

The milestone is complete. No ready card remains. Grok may resume after the
operator supplies authorized account state; another stabilization target may
start independently.
