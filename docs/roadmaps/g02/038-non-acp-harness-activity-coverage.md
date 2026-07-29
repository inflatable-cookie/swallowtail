# 038 Non-ACP Harness Activity Coverage

Status: active
Owner: Tom
Created: 2026-07-29
Depends on: g02.037
Vision tags: harness breadth, HTTP, RPC, headless streams
Contract refs: 009, 011-012, 022-023, 028-029, 037, 039, 042-044
Planning state: card 128 complete; card 129 ready; cards 130-131 planned

## Problem

OpenCode HTTP/SSE, Pi RPC, Kimi local server, Anthropic Managed Agents, and
the headless JSON or JSONL harness routes expose materially different activity
surfaces. Current projections preserve output and selected callbacks but do
not provide provider-wide activity truth.

## Generation Runway Goal

Cover every remaining production harness route through exact transport-native
profiles rather than manufacturing a common agent loop.

## Goals

- [ ] Classify every non-ACP harness activity surface.
- [ ] Freeze exact version and protocol corpora before mapping.
- [ ] Map HTTP, SSE, WebSocket, RPC, JSON, and JSONL activity honestly.
- [ ] Preserve harness-owned versus consumer-owned tool execution.
- [ ] Publish a complete harness-route activity matrix.

## Non-Goals

- several nearly identical adapters without shared information
- inferred steps from terminal output
- changing harness retention, retry, isolation, or server ownership
- direct-model routes
- consumer UI, persistence, or provider effects

## Execution Plan

### Batch 38.1 — Exact Harness Inventory

- [x] Execute card 128.
- [x] Audit OpenCode, Pi, Kimi local server, Anthropic Managed Agents, Claude
      Code headless, Gemini headless, Kimi headless, and Qwen headless.
- [x] Freeze selected lifecycle, detail, unknown, and failure fixtures.

### Batch 38.2 — HTTP, Server, And RPC Projection

- [ ] Execute card 129.
- [ ] Map OpenCode, Pi, Kimi local server, and Managed Agents.

### Batch 38.3 — Headless Stream Projection

- [ ] Execute card 130.
- [ ] Map Claude Code, Gemini, Kimi, and Qwen completion and tool evidence.

### Batch 38.4 — Harness Closeout

- [ ] Execute card 131.
- [ ] Machine-check every production harness route and package-facing facade.

## Acceptance Criteria

- [ ] every production harness route has an exact activity profile
- [ ] no absent lifecycle phase is invented
- [ ] headless completion-only routes remain completion-only
- [ ] callbacks and harness-owned tools remain distinct
- [ ] recovery, reattachment, and activity updates remain distinct
- [ ] unknown semantic records are visible or fail safely
- [ ] all guaranteed route versions have deterministic evidence

## Decision Gates

- Stop one route when current authoritative evidence cannot distinguish a
  semantic activity from transport noise.
- Ask the operator only if two routes require a new product priority, not for
  ordinary evidence-ranked mapping.
- Keep exact absence rather than parsing human terminal prose.

## Next Planning Checkpoint

After card 131, freeze the complete harness result and audit direct inference
for applicable activity without borrowing harness semantics.
