# g04.050 DeepSeek Structured-Run Thinking Mode

Status: complete
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Depends on: per-route feature completion programme; g04.038
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 009, 011, 014, 024, 029, 030, 037, 040, 041, 052
Research: 023, 186, 197 promoted by card 139

## Problem

Production route `deepseek.continuation` fixes exact model
`deepseek-v4-pro`, the OpenAI Chat Completions facade, one positive output
maximum, and exact `low|high|max` reasoning effort. Every structured run and
direct-continuation request also fixes `thinking.type=enabled`.

Current official DeepSeek V4 documentation exposes independent
`thinking.type=enabled|disabled` selection and says V4 Pro supports both
thinking and non-thinking modes. The existing route cannot request the
non-thinking form. It also cannot treat that switch as another reasoning
effort: disabled Chat Completions requests must not invent a portable effort,
and the tool-bearing continuation proof depends on private
`reasoning_content` replay.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind explicit non-thinking mode
for one-request DeepSeek V4 Pro structured runs while preserving the enabled
effort ladder and the enabled-only direct-continuation boundary.

## Goals

- [x] freeze current official Chat Completions, Thinking Mode, model, tool,
      cache, and response evidence
- [x] classify explicit enabled, explicit disabled, omission, effort-field
      presence, response `reasoning_content`, and unknown future values
- [x] classify one-request structured runs and direct continuation separately
- [x] promote Research 197 with an exact deliver-now table or honest stop
- [x] preserve every existing enabled request and public constructor
- [x] bind only admitted structured-run state through typed adapter-local
      input, immutable plan/evidence, driver, and exact request encoding
- [x] omit `reasoning_effort` and portable `ReasoningSelection` when disabled
      unless exact evidence proves another representation
- [x] reject disabled continuation and every field/profile mismatch before
      endpoint, credential, or provider work
- [x] publish dispatch truth without exposing private reasoning or claiming
      provider acceptance, effective mode, quality, latency, or price

## Non-Goals

- disabling thinking on direct tool continuation or weakening Contract 030
- mapping disabled mode to portable `ReasoningMode("none")`
- changing `low|high|max`, accepting `medium|xhigh`, or aliasing effort values
- V4 Flash, vision, retired aliases, Responses API, Anthropic facade, or `/v1`
- consumer-visible `reasoning_content`, durable private continuation, or
  automatic tool execution
- a provider-neutral thinking boolean or generic provider-settings map
- live credentials, provider requests, paid work, release, or currentness work

## Named Scope

The lane is restricted to route `deepseek.continuation`, driver
`swallowtail.deepseek.direct`, exact model route and model
`deepseek-v4-pro`, axis `deepseek.openai-chat-facade`, and current exact facade
point `deepseek-openai-chat-2026-07-22`.

Card 139 must re-fetch current official evidence and freeze deterministic
repository truth. It must settle whether an explicit disabled request is
exactly `thinking:{"type":"disabled"}` with `reasoning_effort` absent, what
response fields are legal, whether the current facade and private behavior
revision remain sufficient, and whether the unmanaged-cache acceptance
boundary changes.

Thinking mode remains adapter-local. No shared `Capability`, portable
`ReasoningMode` value, or generic options field is planned. A disabled run
must not claim `ReasoningSelection`; the immutable plan and prepared evidence
must instead retain exact route-local mode truth strongly enough for driver and
request agreement.

Direct continuation stays enabled-only. Its initial request, tool-result
continuation, later user turn, and fresh restoration require one fixed
`low|high|max` effort and bounded private `reasoning_content` replay. Cards
140-141 must not add a disabled session path, silently enable thinking, or
reinterpret an absent private trace.

Existing enabled calls and exact request bytes must remain unchanged. An empty
Research 197 deliver-now set is an honest stop.

## Execution Plan

### Batch 50.1 — Exact Thinking-Mode Evidence

- [x] Execute card 139.
- [x] freeze official and repository request, response, model, cache, facade,
      and lifecycle evidence
- [x] promote Research 197 with value/profile/field dispositions

### Batch 50.2 — Conditional Structured-Run Binding

- [x] Execute card 140 only when card 139 admits a non-empty deliver-now set.
- [x] add one typed adapter-local structured-run selection
- [x] preserve enabled input, plan, evidence, driver, and request behavior
- [x] keep direct continuation enabled-only

### Batch 50.3 — Route-Local Acceptance

- [x] Execute card 141 only after card 140.
- [x] prove admitted and rejected request, response, composition, and lifecycle
      boundaries
- [x] update route-local guidance and reserve the shared closeout delta

## Acceptance Criteria

- [x] only Research 197 deliver-now model/value/profile combinations prepare
- [x] enabled `low|high|max` request bytes and continuation behavior remain
      unchanged
- [x] disabled structured runs carry exact mode truth without a false portable
      reasoning selection
- [x] input, plan/evidence, driver, request, and qualified response parsing
      agree without defaults, aliases, or fallback
- [x] disabled continuation and every knowable mismatch reject before effects
- [x] private reasoning remains undisclosed and unmanaged-cache acceptance
      remains exact
- [x] default QA performs no credential, account, provider, or paid work
- [x] g04.050 closes one route-local family and leaves generation-boundary
      reassessment to the orchestrator after merge

## Lane Runway

- predecessor: g04.049 OpenAI Background service tier
- this milestone: DeepSeek V4 Pro structured-run thinking-mode evidence and
  conditional adapter-local binding
- execution topology: one serial worker lane, cards 139-141
- generation boundary: final numbered roadmap in g04; reassess only after its
  evidence, review, merge, and closeout

## Decision Gates

- Stop if current official model, field, effort-omission, response, cache, or
  facade truth cannot be closed without inference.
- Stop if disabled mode requires a false portable `ReasoningSelection` or
  shared generic thinking capability.
- Stop if the existing plan/evidence boundary cannot retain exact adapter-local
  mode truth without an unplanned contract change.
- Stop if disabled response handling would expose or infer hidden reasoning.
- Stop if delivery changes direct continuation, tool-loop bounds, private
  replay, model, endpoint, credentials, or currentness.
- Stop before provider acceptance, effective mode, quality, latency, price, or
  cache-effect claims.

## Batch Cards

- [139-deepseek-structured-run-thinking-mode-evidence.md](batch-cards/139-deepseek-structured-run-thinking-mode-evidence.md) — complete
- [140-deepseek-structured-run-thinking-mode-binding.md](batch-cards/140-deepseek-structured-run-thinking-mode-binding.md) — complete
- [141-deepseek-structured-run-thinking-mode-acceptance.md](batch-cards/141-deepseek-structured-run-thinking-mode-acceptance.md) — complete

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 023 DeepSeek V4 Direct Continuation Boundary](../../research/023-deepseek-v4-direct-continuation-boundary.md)
- [Research 186 DeepSeek Reasoning Control Evidence](../../research/186-deepseek-reasoning-control-evidence.md)
- [Research 197 DeepSeek Structured-Run Thinking-Mode Evidence](../../research/197-deepseek-structured-run-thinking-mode-evidence.md)
- [Contract 030 Consumer-Owned Direct Tool Continuation](../../contracts/030-consumer-owned-direct-tool-continuation.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [DeepSeek Prepared Integration](../../guides/deepseek-prepared-integration.md)
- [DeepSeek Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/)
- [DeepSeek Thinking Mode](https://api-docs.deepseek.com/guides/thinking_mode/)
- [DeepSeek Models And Pricing](https://api-docs.deepseek.com/quick_start/pricing/)

## Closeout

g04.050 closes the DeepSeek V4 Pro structured-run thinking-mode family at the
route-local dispatch boundary. The only new selection is
`DeepSeekThinkingMode::disabled()` for one-request structured runs. It has
typed input/evidence/driver binding, no portable `ReasoningSelection`, no
`reasoning_effort`, and explicit `thinking.type=disabled`. Existing enabled
`low|high|max` structured runs and every direct-continuation path remain
unchanged and enabled-only.

The worker validated deterministic protocol and prepared-facade behavior,
including ordinary disabled responses and fail-closed private-reasoning drift.
The current facade, private behavior revision, model route, cache acceptance,
and Contract 029 currentness claim remain unchanged. No provider acceptance,
effective mode, quality, latency, price, cache-effect, or private-reasoning
claim was added.

The reserved closeout names the shared delta for the orchestrator: architecture,
Contract 029, route/feature matrices, programme/front-door indexes, changelog,
the g04 generation boundary, and the sole Next Task pointer remain untouched
by this worker.
