# 082 Claude Pi And OpenCode Usage-Evidence Implementation

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../025-provider-feature-matrix-no-closure-programme.md`
Depends on: card 081

## Objective

Project terminal cumulative token usage through the Claude Agent ACP, Pi RPC,
and OpenCode prepared operations.

## Scope

1. Parse Claude prompt-response usage without treating `usage_update.used` as
   input tokens.
2. Sum disjoint Pi assistant-message usage inside one operation and emit once
   at settlement.
3. Sum disjoint OpenCode step-finish usage inside one operation and emit once
   at idle.
4. Add one optional provider-neutral `reasoning_tokens` dimension to
   `TokenUsage`; preserve input, output, reasoning, cache-read, and cache-write
   fields without inferring cross-field totals.
5. Reject malformed, missing, negative, fractional, overflowing, duplicated
   provider identifiers where available, or post-terminal usage emission.
6. Advertise `UsageReporting` through each prepared run and session capability
   profile actually covered.
7. Change exactly three matrix cells from `No` to `Yes`.
8. Keep cost, context occupancy, Kimi, and unrelated feature cells unchanged.

## Acceptance Criteria

- [x] Claude, Pi, and OpenCode emit exact typed cumulative usage
- [x] multi-step Pi and OpenCode operations do not double-count
- [x] every changed cell has a realized public prepared path
- [x] no route or credential fallback appears
- [x] supported and rejected inputs have deterministic coverage
- [x] local and remote-authoritative topology pass where applicable
- [x] package examples compile without live access

## Stop Conditions

- card 081 has not promoted the aggregation rule
- a provider record cannot be classified as disjoint or cumulative
- live authentication becomes necessary before deterministic fixtures

## Auto-Continuation

Completed. Card 083 owns package-snapshot proof and continuation.
