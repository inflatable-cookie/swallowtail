# 038 Non-ACP Harness Activity Coverage

Status: completed
Owner: Tom
Created: 2026-07-29
Depends on: g02.037
Vision tags: harness breadth, HTTP, RPC, headless streams
Contract refs: 009, 011-012, 022-023, 028-029, 037, 039, 042-044
Planning state: cards 128-131 complete

## Problem

OpenCode HTTP/SSE, Pi RPC, Kimi local server, Anthropic Managed Agents, and
the headless JSON or JSONL harness routes expose materially different activity
surfaces. Current projections preserve output and selected callbacks but do
not provide provider-wide activity truth.

## Generation Runway Goal

Cover every remaining production harness route through exact transport-native
profiles rather than manufacturing a common agent loop.

## Goals

- [x] Classify every non-ACP harness activity surface.
- [x] Freeze exact version and protocol corpora before mapping.
- [x] Map HTTP, SSE, WebSocket, RPC, JSON, and JSONL activity honestly.
- [x] Preserve harness-owned versus consumer-owned tool execution.
- [x] Publish a complete harness-route activity matrix.

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

- [x] Execute card 129.
- [x] Map OpenCode, Pi, Kimi local server, and Managed Agents.

### Batch 38.3 — Headless Stream Projection

- [x] Execute card 130.
- [x] Map Claude Code, Gemini, Kimi, and Qwen completion and tool evidence.

### Batch 38.4 — Harness Closeout

- [x] Execute card 131.
- [x] Machine-check every production harness route and package-facing facade.

## Acceptance Criteria

- [x] every production harness route has an exact activity profile
- [x] no absent lifecycle phase is invented
- [x] headless completion-only routes remain completion-only
- [x] callbacks and harness-owned tools remain distinct
- [x] recovery, reattachment, and activity updates remain distinct
- [x] unknown semantic records are visible or fail safely
- [x] all guaranteed route versions have deterministic evidence

## Decision Gates

- Stop one route when current authoritative evidence cannot distinguish a
  semantic activity from transport noise.
- Ask the operator only if two routes require a new product priority, not for
  ordinary evidence-ranked mapping.
- Keep exact absence rather than parsing human terminal prose.

## Next Planning Checkpoint

After card 131, freeze the complete harness result and audit direct inference
for applicable activity without borrowing harness semantics.
