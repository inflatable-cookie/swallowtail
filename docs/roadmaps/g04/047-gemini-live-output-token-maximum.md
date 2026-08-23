# g04.047 Gemini Live Output-Token Maximum

Status: complete pending merge
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Depends on: per-route feature completion programme; g04.046
Vision tags: explicit selection, provider truth, realtime continuity
Contract refs: 011, 027, 029, 037, 040, 050, 052
Research: 021, 193, 194

## Problem

Production route `gemini.live` fixes exact model
`gemini-3.1-flash-live-preview`, the hosted `v1beta` raw-WebSocket facade,
manual asymmetric PCM, output transcription, caller-selectable thinking, and
one provider-planned rollover. Its prepared facade does not expose an output-
token maximum even though the shared realtime request already carries an
optional positive maximum and the route inventory classifies this control as
ready under Contract 040.

Current official Gemini references say `BidiGenerateContentSetup` accepts a
`GenerationConfig`, define `GenerationConfig.maxOutputTokens`, and list an
exact-model output-token limit of 65,536. They also warn that not every
generation parameter is configurable for every model. This is a promising
exact route-local mapping, not yet a production claim. The evidence gate must
settle exact Live applicability, numeric domain, omission, continuity, and
Contract 029 revision before binding.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind one positive Gemini Live
output-token maximum without changing the route, model, media profile,
reasoning semantics, access boundary, rollover semantics, or claims beyond
dispatch.

## Goals

- [x] freeze current official exact-model, Live-facade, `GenerationConfig`,
      and `maxOutputTokens` evidence
- [x] classify the exact positive numeric domain without deriving setter
      support from the model catalogue alone
- [x] prove that omitted maximum retains the current exact initial and resumed
      setup bytes
- [x] decide the exact opaque facade point and private behavior revision
- [x] bind only a Research 194 deliver-now maximum through typed prepared
      input, immutable plan/evidence, the existing realtime request, driver
      validation, and setup serialization
- [x] retain the same selected maximum across initial setup, one planned
      rollover/resume setup, and fresh realtime working-state restoration
- [x] compose with every admitted thinking level without changing thinking
      vocabulary, defaults, or claims
- [x] preserve media, transcription, activity, usage, cancellation, deadline,
      rollover, credential, socket, and joined-cleanup truth
- [x] publish only exact route-local dispatch truth

## Non-Goals

- client-side truncation, generated-length guarantees, token counting, stop
  sequences, context-window controls, or automatic clamping
- new portable runtime fields, generic provider settings, aliases, defaults
  chosen by the adapter, retry, or route/model fallback
- thinking-level changes, `thinkingBudget`, context compression, tools,
  automatic activity, or another Gemini feature family
- Gemini CLI ACP/headless, another Gemini API or model, Vertex AI, consumer
  login, OAuth, WebRTC, SIP, or ephemeral client tokens
- live credentials, account inspection, provider requests, paid work,
  release, or Contract 029 currentness widening

## Named Scope

The lane is restricted to production route `gemini.live`, driver
`swallowtail.gemini.live`, exact model
`gemini-3.1-flash-live-preview`, axis `gemini.live-facade`, and the current
exact `v1beta` `BidiGenerateContent` raw-WebSocket surface.

The candidate control is
`generationConfig.maxOutputTokens`. Card 130 must prove that the exact Live
setup and model admit it, then close the value domain. The model page's 65,536
output-token limit is an upper-bound candidate, not sufficient setter evidence
by itself. Zero is structurally excluded by the existing `NonZeroU64` carrier;
negative, fractional, overflowing, aliased, clamped, and catalogue-only values
are not candidates.

The current absence path sends no `maxOutputTokens` member. Delivery must
preserve the exact current initial and resume setup bytes when no maximum is
selected. A selected value must be represented by
`Capability::OutputTokenLimit` plus exact
`CapabilityConstraint::OutputTokenMaximum`, remain immutable across setup,
rollover, and restoration, and never become an effective-length observation.

The shared `OpenRealtimeMediaSessionRequest` already carries
`Option<NonZeroU64>`. No shared runtime change is planned. Existing sibling
realtime routes retain their own bounds and behavior.

An empty Research 194 deliver-now set is an honest stop. Contracts must not be
weakened and a live provider call must not be introduced to make the field fit.

## Execution Plan

### Batch 47.1 — Exact Model And Live-Facade Evidence

- [x] Execute card 130.
- [x] freeze current official model, Live, and generation-config evidence plus
      exact current route and fixture truth
- [x] promote Research 194 with applicability, domain, omission, continuity,
      version, and compatibility dispositions

### Batch 47.2 — Conditional Prepared Binding

- [x] Execute card 131 only if card 130 admits a non-empty deliver-now set.
- [x] carry the exact maximum through Gemini prepared state and the existing
      shared request
- [x] reject unsupported values and request/plan/driver drift before effects

### Batch 47.3 — Route-Local Acceptance

- [x] Execute card 132 only after card 131.
- [x] prove exact initial, rollover, restoration, omission, composition,
      rejection, and unchanged lifecycle behavior
- [x] update route-local guidance and record the deferred shared closeout delta

## Acceptance Criteria

- [x] only Research 194 deliver-now values prepare
- [x] input, capability constraint, plan, evidence, request, driver, and every
      setup frame agree exactly
- [x] omission preserves current initial and resume setup bytes
- [x] unsupported and drifted maxima fail before endpoint, credential, or
      socket work
- [x] no value is clamped, aliased, substituted, inferred, retried, or routed
- [x] one planned rollover and fresh restoration retain the selected maximum
- [x] selected reasoning and selected maximum compose without semantic drift
- [x] existing media, transcription, activity, usage, cancellation, deadline,
      provider failure, and cleanup behavior remain intact
- [x] docs claim dispatch only; provider acceptance and effective generated
      length remain unclaimed
- [x] default QA uses no credential, account, provider request, or paid work

## Lane Runway

- predecessor: g04.046 Gemini Live thinking levels
- this milestone: Gemini Live output-token-maximum evidence and conditional
  binding
- execution topology: one serial worker lane, cards 130-132
- next route family: selected by the orchestrator after evidence, review, and
  merge closeout; no later family is precompiled here

## Decision Gates

- Stop if current official evidence cannot prove `maxOutputTokens` on this
  exact Live model and facade without inference.
- Stop if the positive numeric domain or omission bytes cannot be closed.
- Stop if one selected maximum cannot remain immutable across initial setup,
  planned rollover, and fresh restoration.
- Stop if delivery needs a new shared carrier, client-side truncation, live
  proof, contract amendment, compatibility inference, or breaking API.
- Stop if output maximum changes thinking selection or another realtime route.

## Batch Cards

- [130-gemini-live-output-token-maximum-evidence.md](batch-cards/130-gemini-live-output-token-maximum-evidence.md) — done
- [131-gemini-live-output-token-maximum-binding.md](batch-cards/131-gemini-live-output-token-maximum-binding.md) — done
- [132-gemini-live-output-token-maximum-acceptance.md](batch-cards/132-gemini-live-output-token-maximum-acceptance.md) — done

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 027 Planned Connection Rollover](../../contracts/027-planned-connection-rollover-and-realtime-continuity.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Realtime Prepared Integration](../../guides/realtime-prepared-integration.md)
- [Gemini 3.1 Flash Live Preview](https://ai.google.dev/gemini-api/docs/models/gemini-3.1-flash-live-preview)
- [Gemini Live WebSocket Reference](https://ai.google.dev/api/live)
- [Gemini GenerationConfig Reference](https://ai.google.dev/api/generate-content)
