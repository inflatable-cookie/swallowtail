# 037 ACP Observable Agent Activity

Status: planned
Owner: Tom
Created: 2026-07-29
Depends on: g02.036
Vision tags: ACP, protocol reuse, installed harnesses
Contract refs: 009, 011-012, 015, 017, 029, 035, 037, 044
Planning state: cards 125-127 planned

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

- [ ] Revalidate the maintained ACP schema and each selected harness range.
- [ ] Freeze shared message, thought, plan, tool, usage, mode, and unknown
      update corpora.
- [ ] Add bounded protocol classification without runtime or provider policy.
- [ ] Map exact activity profiles in Claude Agent, Gemini CLI, and Kimi Code.
- [ ] Preserve stdio and explicit remote ACP transport identity.

## Non-Goals

- one generic ACP provider adapter
- provider or model inference through transport identity
- widening write, approval, session, or authentication authority
- Grok Build activation
- consumer UI, persistence, or live harness testing

## Execution Plan

### Batch 37.1 — ACP Currentness And Corpus

- [ ] Execute card 125.
- [ ] Pin current schema authority and exact harness behavior milestones.
- [ ] Freeze deterministic shared and provider-specific deltas.

### Batch 37.2 — Shared Projection Records

- [ ] Execute card 126.
- [ ] Decode bounded ACP activity updates in the protocol boundary.
- [ ] Keep provider access, activity profile, and runtime emission in adapters.

### Batch 37.3 — Adapter Conformance

- [ ] Execute card 127.
- [ ] Prove Claude Agent, Gemini CLI, and Kimi Code mappings across their exact
      ranges and both supported host topologies.

## Acceptance Criteria

- [ ] tool calls and plans no longer become empty progress
- [ ] provider-visible thought chunks become reasoning summaries only
- [ ] shared protocol parsing creates no provider identity
- [ ] provider-specific capability and mode differences remain visible
- [ ] remote ACP adds no implicit authentication, fallback, or recovery
- [ ] exact schema and harness version milestones are enforced
- [ ] full ACP lifecycle, callback, continuity, and cleanup regressions pass

## Decision Gates

- Stop if current ACP authority changes the meaning of thought or tool
  updates.
- Keep per-adapter mapping when shared decoding would import provider policy.
- Grok remains held until its separate authentication gate is satisfied.

## Next Planning Checkpoint

After card 127, inventory the non-ACP harness routes against the realized
Codex and ACP profiles before starting g02.038.

