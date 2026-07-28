# 022 Structured-Run Projection And Direct Coverage

Status: completed
Owner: Tom
Created: 2026-07-27
Depends on: g02.007-g02.011
Vision tags: operation diversity, prepared facade, direct inference
Contract refs: 004-014, 016, 023-025, 029-030, 037, 039
Planning state: cards 071-073 completed

## Problem

The solution matrix reports twelve structured-run gaps. Several selected
routes already expose a naturally bounded provider request or first response,
but Swallowtail registers only their interactive role.

The first tranche must prove the shared single-turn boundary across stateless
HTTP/SSE and connection-scoped WebSocket execution before repeated harness
projections.

## Goals

- [x] Promote the provider-neutral bounded single-turn projection contract.
- [x] Add resource-free Alibaba Responses structured execution.
- [x] Add no-tool DeepSeek structured execution beside direct continuation.
- [x] Add one-response xAI WebSocket structured execution.
- [x] Keep direct provider semantics, storage, cancellation, usage, and cost
      exact.

## Non-Goals

- generic interactive-session-to-run conversion
- provider, route, model, access, or retry fallback
- Gemini Live or OpenAI Realtime structured roles
- consumer-owned DeepSeek tool continuation inside one run
- live credentials or paid inference

## Execution Plan

### Batch 22.1 — Evidence And Contract

- [x] Execute card 071.

### Batch 22.2 — Resource-Free HTTP/SSE

- [x] Execute card 072.

### Batch 22.3 — One-Response WebSocket

- [x] Execute card 073 after card 072 closes.

## Acceptance Criteria

- [x] each direct route independently registers and prepares `StructuredRun`
- [x] one operation sends one inference response and exposes no continuation
      handle
- [x] unsupported tools, attachments, schema, retention, or policy fail before
      provider effects unless exact fixtures qualify them
- [x] xAI sends no previous-response id and closes after one response
- [x] local and remote-authoritative fixtures join network and credential work
- [x] existing interactive and continuation roles remain unchanged

## Decision Gates

- Stop if a route needs hidden state or fallback absent from Contract 039.
- Keep provider-specific event codecs and failure mapping in the owning
  adapter.
- Do not infer structured capability from catalogue membership.

## Next Planning Checkpoint

After card 073, continue to g02.023. Reassess only if the direct proofs expose
a missing common lifecycle record.
