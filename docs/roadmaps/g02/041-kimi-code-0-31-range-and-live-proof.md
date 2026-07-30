# 041 Kimi Code 0.31 Range And Live Proof

Status: completed
Owner: Tom
Created: 2026-07-30
Depends on: g02.040
Vision tags: harness compatibility, version ranges, live evidence
Contract refs: 011, 023, 033, 036-037, 042, 044
Planning state: cards 138-139 completed

## Problem

Swallowtail guarantees Kimi Code only through `0.29.2`. The maintained native
distribution and an authorized local account are now at `0.31.0`. The
guarantee should move only where exact source and live evidence support it.

## Generation Runway Goal

Qualify current Kimi ACP and headless behavior without widening the changed
local-server route or adding a redundant Python adapter.

## Goals

- [x] Compare exact `0.30.0` and `0.31.0` source against `0.29.2`.
- [x] Run bounded authenticated headless and ACP smoke probes.
- [x] Extend only the ACP and headless maintained windows.
- [x] Preserve local-server `0.31.0` as unverified newer.
- [x] Close the separate Python Kimi proposal.
- [x] Leave Grok and provider-session binding persistence under their existing
      promotion gates.

## Non-Goals

- Python `kimi-cli` implementation
- Kimi local-server `0.31.0` qualification
- callback, tool, workspace-write, deletion, quota, or billing proof
- consumer repository edits
- registry publication

## Execution Plan

### Batch 41.1 — Currentness And Live Evidence

- [x] Execute card 138.
- [x] Promote Research 068.
- [x] Select route-specific range changes.

### Batch 41.2 — Range Extension And Closeout

- [x] Execute card 139.
- [x] Advance claim identities and deterministic range coverage.
- [x] Refresh route, matrix, backlog, and currentness documentation.
- [x] Run focused Kimi and docs validation.

## Acceptance Criteria

- [x] ACP and headless guarantee exact `0.31.0`
- [x] `0.30.0` remains an explicit intermediate evidence point
- [x] local-server guarantee still ends at `0.29.2`
- [x] stable versions above `0.31.0` remain visible, not denied
- [x] no live secret, provider payload, or session id enters repository state
- [x] Python Kimi is closed without conflating distributions

## Decision Gates

- Do not qualify local-server `0.31.0` without a broadcaster delta corpus.
- Do not infer tool, callback, quota, or catalogue behavior from two smoke
  prompts.
- Ask the operator before adding another Kimi distribution.

## Next Planning Checkpoint

The milestone is complete. No ready card remains. Grok may resume after the
operator supplies authorized account state; another stabilization target may
start independently.
