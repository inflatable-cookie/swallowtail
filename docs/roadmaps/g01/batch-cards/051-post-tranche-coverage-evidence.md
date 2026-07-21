# 051 Post-Tranche Coverage Evidence

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../016-post-tranche-coverage-checkpoint.md`

## Objective

Select the next highest-information provider/transport proof after the initial
diversity tranche.

## Scope

- current official xAI WebSocket/direct API, a second ACP agent, SDK-native
  routes, and owned self-hosted authority
- compare transport novelty, auth authority, lifecycle, deterministic fixtures,
  contract pressure, and consumer value
- one sequenced recommendation and its missing shared contracts

## Out Of Scope

- provider implementation
- model download or owned serving
- several similar HTTP/SSE or JSONL adapters

## Acceptance Criteria

- [x] every candidate uses current official or maintained-project evidence
- [x] the next proof is selected or the exact operator ambiguity is named
- [x] missing shared contracts are promoted before implementation cards
- [x] the remaining provider sequence is concrete, not a generic list

## Evidence

- Research 005 revalidates xAI WebSocket, Kimi Code ACP `0.28.0`, current
  Qwen/Kimi SDK routes, owned llama.cpp, and the adjacent Monkey boundary.
- xAI WebSocket is selected without an operator policy ambiguity: it is an
  explicitly scoped provider-supported public API route and live auth remains
  separately gated.
- Contract 016 promotes resource-free direct sessions, session-bound endpoint
  and credential leases, continuation and reconnect limits, cancellation by
  close, and exact provider-billed-cost evidence.
- Roadmap 017 and cards 052-054 compile the fixture, driver, and conformance
  runway. Card 052 is the sole ready next task.
- No credential store or provider account was inspected; no authenticated
  request ran.

## Validation

- source-link and authority review
- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- the first choice would establish unsettled provider or access policy
- evidence does not support a stable fixture boundary

## Auto-Continuation

No. Compile the selected proof only after its contracts and operator authority
are clear.
