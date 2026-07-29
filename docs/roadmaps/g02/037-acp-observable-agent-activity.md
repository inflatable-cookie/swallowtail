# 037 ACP Observable Agent Activity

Status: completed
Owner: Tom
Created: 2026-07-29
Depends on: g02.036
Vision tags: ACP, protocol reuse, installed harnesses
Contract refs: 009, 011-012, 015, 017, 029, 035, 037, 044
Planning state: cards 125-127 completed

## Problem

Claude Agent, Gemini CLI, and Kimi Code share ACP update shapes, but current
adapter projections flatten plans, tool calls, and tool-call updates into
generic progress. Message and thought chunks survive without stable activity
identity or exact route fidelity.

## Generation Runway Goal

Prove one shared protocol activity boundary across several provider-specific
harnesses without flattening their access, version, mode, callbacks, or
session behavior.

## Goals

- [x] Revalidate the maintained ACP schema and each selected harness range.
- [x] Freeze shared message, thought, plan, tool, usage, mode, and unknown
      update corpora.
- [x] Add bounded protocol classification without runtime or provider policy.
- [x] Map exact activity profiles in Claude Agent, Gemini CLI, and Kimi Code.
- [x] Preserve stdio and explicit remote ACP transport identity.

## Non-Goals

- one generic ACP provider adapter
- provider or model inference through transport identity
- widening write, approval, session, or authentication authority
- Grok Build activation
- consumer UI, persistence, or live harness testing

## Execution Plan

### Batch 37.1 — ACP Currentness And Corpus

- [x] Execute card 125.
- [x] Pin current schema authority and exact harness behavior milestones.
- [x] Freeze deterministic shared and provider-specific deltas.

### Batch 37.2 — Shared Projection Records

- [x] Execute card 126.
- [x] Decode bounded ACP activity updates in the protocol boundary.
- [x] Keep provider access, activity profile, and runtime emission in adapters.

### Batch 37.3 — Adapter Conformance

- [x] Execute card 127.
- [x] Prove Claude Agent, Gemini CLI, and Kimi Code mappings across their exact
      ranges and both supported host topologies.

## Acceptance Criteria

- [x] tool calls and plans no longer become empty progress
- [x] provider-visible thought chunks are classified by exact adapters as
      reasoning summaries, warnings, other display activity, or exclusions
- [x] shared protocol parsing creates no provider identity
- [x] provider-specific capability and mode differences remain visible
- [x] remote ACP adds no implicit authentication, fallback, or recovery
- [x] exact schema and harness version milestones are enforced
- [x] full ACP lifecycle, callback, continuity, and cleanup regressions pass

## Decision Gates

- Stop if current ACP authority changes the meaning of thought or tool
  updates.
- Keep per-adapter mapping when shared decoding would import provider policy.
- Grok remains held until its separate authentication gate is satisfied.

## Next Planning Checkpoint

Roadmap g02.037 is closed. Card 128 starts the non-ACP harness inventory
against the realized Codex and ACP profiles.
