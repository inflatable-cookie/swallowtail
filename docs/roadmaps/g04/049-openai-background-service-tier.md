# g04.049 OpenAI Background Service Tier

Status: stopped after evidence
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Depends on: per-route feature completion programme; g04.044
Vision tags: explicit selection, provider truth, operational controls
Contract refs: 009, 011, 014, 021, 029, 037, 040, 048, 049, 052
Research: 102, 191, 196

## Problem

Production route `openai.background` fixes exact model `gpt-5.6`, the public
Responses facade, reasoning, output bounds, structured output, temporary
retention, one in-process stream reattachment, cancellation, deletion,
controlled detachment, and exact-run reconciliation. It does not expose the
Responses `service_tier` request or returned effective tier.

Current official OpenAI Responses create and retrieve references name
`service_tier` on the same request and response object. Omission behaves as
`auto`; explicit values may select standard, Flex, Fast/Priority, or
access-controlled Ultrafast processing, and the returned tier may differ from
the requested value. Exact value, model, account, lifecycle, and observation
boundaries therefore need qualification before delivery.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind the smallest route-local
OpenAI Background service-tier subset without weakening model, reasoning,
structured-output, retention, reattachment, detachment, reconciliation,
access, billing, or compatibility truth.

## Goals

- [x] freeze current official Responses create, retrieve, streaming,
      background, exact-model, and service-tier evidence
- [x] classify omission and every current request/response enum value,
      including aliases, defaults, access gates, and resolved-value drift
- [x] classify ordinary, detachable, and reconciled run profiles separately
- [x] promote Research 196 with an exact deliver-now table or honest stop
- [ ] preserve prior request bytes and claims when service tier is omitted
- [ ] bind only admitted values through typed adapter-local prepared state
- [ ] keep input, plan/evidence, driver, request body, and deterministic
      response parsing in exact agreement
- [ ] compose with every admitted reasoning value and structured-output mode
- [ ] publish requested/dispatched/resolved truth only where the exact route
      exposes it; never infer price, latency, capacity, or entitlement

## Non-Goals

- a portable Fast, speed, priority, service-tier, or quality capability
- Codex CLI Fast mode, Chat Completions, Batch, Realtime, or another model
- project-setting mutation, tier enrollment, account inspection, quota lookup,
  reservation, capacity purchase, billing calculation, or price guarantee
- aliases accepted merely because the provider accepts them
- provider fallback, overflow, substitution, retry, or automatic tier choice
- hosted search, tools, prompt caching, verbosity, Pro mode, or multi-agent
- live credentials, provider requests, paid work, release, or currentness work

## Named Scope

The lane is restricted to route `openai.background`, driver
`swallowtail.openai.background`, exact model route
`openai.public.gpt-5.6.background`, model `gpt-5.6` (the documented alias for
`gpt-5.6-sol`), axis `openai.responses-background-facade`, and current exact
facade point `openai-responses-background-2026-08-23`.

Card 136 must enumerate the exact current create-request and returned-response
service-tier domains rather than copying a partial prose list. It must
distinguish omission, `auto`, `default`, `flex`, request aliases, canonical
returned values, access-controlled values, unknown future values, and any
schema-only value. A request spelling that resolves to another returned value
is not automatically an admissible public alias.

The control remains adapter-local. No `Capability`, shared generation-control
field, generic provider-settings map, or sibling-route behavior is planned.
Cards 137-138 may execute only for Research 196 deliver-now rows that can fail
closed before endpoint or credential work.

Ordinary attached runs, controlled detachment, and later exact-run
reconciliation are separate dispositions. A selected value must not disappear
from durable truth, be reconstructed from project defaults, or be presented as
the resolved tier. If current checkpoint/reconciliation surfaces cannot retain
the necessary truth without a shared contract change, withhold that profile or
stop; do not widen the shared checkpoint ad hoc.

The official response field may prove the processing tier actually reported by
the provider. Card 136 must decide whether the existing route can expose that
observation without a breaking or shared portable API. Dispatch-only delivery
is permissible only when documentation makes the unobserved resolved tier and
its consequences explicit.

An empty Research 196 deliver-now set is an honest stop.

## Execution Plan

### Batch 49.1 — Exact Service-Tier Evidence

- [x] Execute card 136.
- [x] freeze official and repository request, response, lifecycle, access, and
      facade evidence
- [x] promote Research 196 with value/profile/observation dispositions

### Batch 49.2 — Conditional Adapter-Local Binding

- [ ] Execute card 137 only when card 136 admits a non-empty deliver-now set.
- [ ] bind one exact typed selection through preparation, evidence, driver,
      request encoding, and qualified response parsing
- [ ] preserve exact omission and every existing lifecycle path

Card 137 is blocked. Research 196 admits no deliver-now row.

### Batch 49.3 — Route-Local Acceptance

- [ ] Execute card 138 only after card 137.
- [ ] prove admitted values, rejected boundaries, composition, reattachment,
      cancellation, deletion, and every admitted detachment disposition
- [ ] update route-local guidance and reserve the shared closeout delta

Card 138 is blocked. There is no admitted dispatch to prove.

## Acceptance Criteria

- [ ] only Research 196 deliver-now values and profiles prepare
- [ ] omission preserves the prior create request exactly
- [ ] input, plan/evidence, driver, request, and any reported response tier
      agree without default substitution or aliasing
- [ ] unsupported values and every knowable drift reject before effects
- [ ] reasoning and structured output compose without semantic drift
- [ ] reattachment, cancellation, deletion, detachment, and reconciliation
      retain their existing truth
- [ ] no project/account setting, tier entitlement, cost, latency, capacity,
      acceptance, or fallback claim is inferred
- [ ] default QA performs no credential, account, provider, or paid work

## Lane Runway

- predecessor: g04.048 Gemini Live context-window compression
- this milestone: OpenAI Background service-tier evidence and conditional
  route-local binding
- execution topology: one serial worker lane, cards 136-138
- next route family: selected by the orchestrator after evidence, review, and
  merge closeout; no later family is precompiled here

## Decision Gates

- Stop if the exact current request and returned-response enum domains cannot
  be closed without inference.
- Stop if the exact `gpt-5.6` alias, background/stream/store composition, or
  selected profile applicability is not source-backed.
- Stop if an admitted value depends on unobserved account/project authority
  that the current access boundary cannot express.
- Stop if requested and returned tier truth cannot remain distinct.
- Stop if detachment or reconciliation loses selected/resolved tier truth; a
  narrower ordinary-run subset may proceed only when Research 196 says so.
- Stop if delivery needs a portable capability, shared checkpoint mutation,
  live proof, unplanned contract change, or breaking API.

## Batch Cards

- [136-openai-background-service-tier-evidence.md](batch-cards/136-openai-background-service-tier-evidence.md) — complete; evidence stop
- [137-openai-background-service-tier-binding.md](batch-cards/137-openai-background-service-tier-binding.md) — blocked
- [138-openai-background-service-tier-acceptance.md](batch-cards/138-openai-background-service-tier-acceptance.md) — blocked

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 102 OpenAI Background Run Checkpoint And Reconciliation](../../research/102-openai-background-run-checkpoint-and-reconciliation.md)
- [Research 191 OpenAI Background Web Search Evidence](../../research/191-openai-background-web-search-evidence.md)
- [Contract 021 Provider-Owned Background Run](../../contracts/021-provider-owned-background-run-and-temporary-retention-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 048 Cross-Process Active Operation Reconciliation](../../contracts/048-cross-process-active-operation-reconciliation.md)
- [Contract 049 Controlled Shutdown Detachment](../../contracts/049-controlled-shutdown-active-operation-detachment.md)
- [OpenAI Background Prepared Integration](../../guides/openai-background-prepared-integration.md)
- [Responses Create](https://developers.openai.com/api/reference/cli/resources/responses/methods/create)
- [Responses Retrieve](https://developers.openai.com/api/reference/cli/resources/responses/methods/retrieve)
- [GPT-5.6 Sol](https://developers.openai.com/api/docs/models/gpt-5.6-sol)
- [Fast mode](https://developers.openai.com/api/docs/guides/fast-mode)
- [Flex processing](https://developers.openai.com/api/docs/guides/flex-processing)

## Evidence Stop

Research 196 is promoted, but its deliver-now set is empty. Official OpenAI
documentation freezes the complete current `service_tier` enum, omission as
project-configured `auto`, Fast request aliasing to returned `priority` on
GPT-5.6, access-controlled Ultrafast, schema-only `scale`, and
requested-versus-returned drift. It does not prove this payg caller's
enrollment, a current-API observation path, or durable selected/returned
truth across detachment and reconciliation. Cards 137 and 138 are therefore
blocked. The existing `openai-responses-background-2026-08-23` facade point
and omitted create bytes remain unchanged.
