# g04.039 xAI Responses Reasoning And Output Bounds

Status: ready
Owner: Tom
Created: 2026-08-22
Depends on: per-route feature completion programme
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 011, 020, 024, 029, 037, 040, 041, 052
Research: 004, 067, 169; 187 to be produced by card 107

## Problem

`xai.responses-websocket` currently prepares exact model routes for one-response
runs and connection-local continuation, but its driver rejects reasoning and
maximum-output-token policy. The official WebSocket body follows Responses
create semantics and current Grok reasoning models expose route-specific effort
sets and optional output bounds.

The official model surface has moved since the initial feature inventory. Grok
4.5 and 4.6 have different effort sets, Grok 4.6 documents no intrinsic text
output limit, and multi-agent effort controls agent count rather than reasoning
depth. The route needs fresh exact evidence before any model or value is bound.

## Generation Runway Goal

Deliver the fifth route-local control family from the per-route feature
programme: exact xAI Responses WebSocket reasoning and caller output bounds on
only the model/value/profile combinations admitted by Research 187.

## Goals

- [ ] freeze current official WebSocket, Responses-body, model, reasoning, and
      output-bound evidence
- [ ] classify structured and connection-local interactive profiles separately
- [ ] distinguish Grok 4.5, Grok 4.6, model aliases, and multi-agent semantics
- [ ] map only exact reasoning-depth values to portable `ReasoningSelection`
- [ ] add an exact positive maximum-output-token input only where the selected
      model and WebSocket body support it
- [ ] retain both controls through immutable plan/evidence and every admitted
      continuation request
- [ ] reject unqualified models, values, aliases, profile combinations, and
      mismatches before network work
- [ ] publish dispatch truth without claiming provider acceptance, effective
      reasoning depth, or an exact generated length
- [ ] leave shared architecture, matrices, changelog, programme, indexes, and
      roadmap-front-door deltas for orchestrator closeout after merge

## Non-Goals

- xAI web search, X search, code execution, files, tools, or citations
- `grok-4.20-multi-agent`, agent-count controls, Grok Bot, or Grok Build
- prompt caching, encrypted reasoning export, durable provider storage, warmup,
  background mode, or connection reattachment
- a provider-neutral raw options map, reasoning string, or model-family default
- output-length effectiveness claims or synthetic client-side truncation
- live provider requests, account inspection, billing, release, or publication

## Named Scope

The milestone is restricted to the existing `xai.responses-websocket` route,
`xai-responses-websocket-2026-04-23` facade, public API-key access profile,
one-response structured run, and serial connection-local interactive session.

Card 107 must recheck official current WebSocket mode, Responses request shape,
model pages, reasoning guide, and output-bound behavior. It must freeze exact
model identifiers rather than infer compatibility from the separate language-
model catalogue. A required facade revision, Contract 040 change, or empty
useful subset stops implementation for orchestrator review.

Research 187 must classify reasoning and output bounds independently. A model
may admit one control without the other. Multi-agent effort is never portable
reasoning depth. For sessions, one preparation-time selection must remain fixed
across the first response, every later turn, and fresh replacement after
connection loss.

## Execution Plan

### Batch 39.1 — Exact Current Evidence

- [ ] Execute card 107.
- [ ] freeze official sources, stable specimens, digests, and exact dispositions
- [ ] promote Research 187 with model/value/profile/control rows

### Batch 39.2 — Prepared Binding

- [ ] Execute card 108 after card 107.
- [ ] bind only deliver-now reasoning and output controls through prepared input,
      capability constraints, immutable evidence, driver policy, and protocol
- [ ] preserve current absent-control request bytes and behavior

### Batch 39.3 — Dispatch And Acceptance

- [ ] Execute card 109 after card 108.
- [ ] prove exact first-turn, later-turn, restoration, and failure request bodies
- [ ] update the xAI guide and report the deferred shared-surface delta

## Acceptance Criteria

- [ ] only Research 187 model/value/profile/control combinations prepare
- [ ] request, plan constraints, prepared evidence, configured driver, and wire
      values agree exactly
- [ ] reasoning depth stays distinct from multi-agent count and model defaults
- [ ] output bounds remain requested dispatch controls, not effective-length
      observations
- [ ] absent controls preserve current request JSON and public behavior
- [ ] unsupported combinations fail before endpoint or credential use
- [ ] deterministic QA makes no provider call or account inspection
- [ ] route-local closeout records the exact deferred shared-surface delta

## Lane Runway

- predecessors: g04.035 Cursor, g04.036 Ollama, g04.037 Anthropic, g04.038
  DeepSeek
- this milestone: xAI Responses WebSocket reasoning and output bounds
- execution topology: one serial worker lane, cards 107-109
- next route family: selected by orchestrator after merge from the remaining
  per-route feature inventory

## Decision Gates

- Stop if current official evidence requires a new facade revision or contract.
- Stop if the existing route cannot name an exact qualified model subset.
- Stop if WebSocket and Responses HTTP semantics diverge for either control.
- Stop if session requests cannot keep one fixed selection across continuation.
- Stop if output bounds require client-side truncation or an effectiveness claim.
- Stop before treating multi-agent effort as portable reasoning depth.

## Batch Cards

- [107-xai-responses-control-evidence.md](batch-cards/107-xai-responses-control-evidence.md) — ready
- [108-xai-responses-control-binding.md](batch-cards/108-xai-responses-control-binding.md) — ready after 107
- [109-xai-responses-control-acceptance.md](batch-cards/109-xai-responses-control-acceptance.md) — ready after 108

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application And Enforcement](../../contracts/040-generation-control-application-and-enforcement.md)
- [xAI Prepared Integration](../../guides/xai-prepared-integration.md)
- [xAI WebSocket Mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)
- [xAI Reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning)
