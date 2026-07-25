# 009 Remaining Harness Facades

Status: complete
Owner: Tom
Created: 2026-07-25
Depends on: g02.008
Vision tags: harness diversity, ACP, RPC, structured CLI
Contract refs: 015, 017, 023, 028-029, 032-035, 037
Planning state: cards 024-026 complete

## Problem

The remaining harnesses share some host mechanics but differ across ACP
negotiation, persistent sessions, RPC scheduling, structured CLI execution,
and attached HTTP service lifecycle.

## Goals

- [x] Add prepared Claude Agent and Gemini ACP routes.
- [x] Add prepared Pi RPC and Qwen headless routes.
- [x] Add the prepared OpenCode HTTP/SSE route.
- [x] Keep remote ACP transport explicit and composable.

## Execution Plan

- [x] Card 024: Claude Agent and Gemini ACP.
- [x] Card 025: Pi RPC and Qwen headless.
- [x] Card 026: OpenCode attached HTTP/SSE and remote-ACP composition review.

## Acceptance Criteria

- [x] exact executable observation and configuration posture remain visible
- [x] ACP, RPC, structured CLI, and HTTP lifecycle stay distinct
- [x] provider-native permissions and optional sandbox claims are not widened
- [x] attached service ownership and affinity remain explicit
- [x] every harness route passes its existing low-level conformance

## Decision Gate

All eight harness routes have an adapter-local prepared normal path. Then
g02.010 covers hosted direct and provider-owned state.

Decision gate passed. Card 027 starts g02.010 with separate Kimi Platform and
DeepSeek compatible-chat facades.
