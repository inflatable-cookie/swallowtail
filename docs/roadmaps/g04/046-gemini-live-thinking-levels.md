# g04.046 Gemini Live Thinking Levels

Status: active
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Depends on: per-route feature completion programme; g01.033; g04.045
Vision tags: explicit selection, provider truth, realtime continuity
Contract refs: 011, 027, 029, 037, 040, 050, 052
Research: 021, 193

## Problem

Production route `gemini.live` fixes exact model
`gemini-3.1-flash-live-preview`, the hosted `v1beta` raw-WebSocket facade,
manual asymmetric PCM, output transcription, and one provider-planned
rollover. Its setup frame currently sends
`thinkingConfig.thinkingLevel=MINIMAL` on both initial and resumed
connections, but the prepared facade exposes no caller reasoning selection
and the route does not claim `ReasoningSelection`.

Current official Gemini model and Live API documentation names Thinking on
this exact model and lists `minimal`, `low`, `medium`, and `high` thinking
levels. That is a promising exact-transport mapping to the existing portable
`ReasoningMode`, not yet a production claim. The evidence gate must prove the
current model/facade composition, omission behavior, rollover stability, and
Contract 029 revision before binding.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind caller-selected Gemini
Live thinking levels without changing the fixed route, model, media profile,
access boundary, rollover semantics, or claims beyond dispatch.

## Goals

- [x] freeze current official exact-model and Live-facade evidence for
      `thinkingLevel`
- [x] classify exact portable mappings for
      `minimal|low|medium|high` and reject every other value without aliasing
- [x] prove that omitted selection retains the current exact `MINIMAL` setup
      frame while remaining distinct from an explicit caller selection
- [x] decide the exact opaque facade point and private behavior revision
- [ ] bind only Research 193 deliver-now rows through typed prepared input,
      immutable plan/evidence, the realtime request, driver validation, and
      setup serialization
- [ ] retain the same selected level across initial setup, one planned
      rollover/resume setup, and fresh realtime working-state restoration
- [ ] preserve media, transcription, activity, usage, cancellation, deadline,
      rollover, credential, socket, and joined-cleanup truth
- [ ] publish only exact route-local dispatch truth

## Non-Goals

- `thinkingBudget`, `includeThoughts`, thought summaries, context compression,
  output-token controls, tools, or automatic activity changes
- Gemini CLI ACP/headless, another Gemini API or model, Vertex AI, consumer
  login, OAuth, browser transport, WebRTC, SIP, or ephemeral client tokens
- generic provider settings, numeric-budget translation, aliases, clamping,
  defaults chosen by the adapter, retry, or route/model fallback
- live credentials, account inspection, provider requests, paid work,
  release, or Contract 029 currentness widening

## Named Scope

The lane is restricted to production route `gemini.live`, driver
`swallowtail.gemini.live`, exact model
`gemini-3.1-flash-live-preview`, axis `gemini.live-facade`, the current exact
`v1beta` `BidiGenerateContent` raw-WebSocket surface, and provider-supported
project authorization API-key access.

Candidate portable-to-wire mappings are exact:

| Portable value | Candidate setup value |
| --- | --- |
| `minimal` | `MINIMAL` |
| `low` | `LOW` |
| `medium` | `MEDIUM` |
| `high` | `HIGH` |

Card 127 must confirm every row. `off`, `default`, `xhigh`, `max`, numeric
budgets, and arbitrary aliases are not candidates. No mapping is inferred for
another Gemini route or model.

The current absence path is not provider-field absence: existing fixtures
send `MINIMAL`. Delivery must preserve those exact setup bytes when no caller
selection is present. Explicit `minimal` is still a planned
`ReasoningSelection`; omission remains fixed route behavior and must not gain
that capability by inference.

Adding a selected value requires the generic realtime open-session request to
carry an optional portable `ReasoningMode`. Sibling realtime drivers must
continue to reject an unsupported low-level reasoning request before endpoint,
credential, or socket work. This is a narrow shared runtime carrier, not a
generic realtime generation-settings map.

An empty Research 193 deliver-now set is an honest stop. Contracts must not be
weakened to make the field fit.

## Execution Plan

### Batch 46.1 — Exact Model And Live-Facade Evidence

- [x] Execute card 127.
- [x] freeze current official model/Live documentation and exact current
      route, setup, resume, and prepared-facade evidence
- [x] promote Research 193 with value, omission, continuity, version, and
      compatibility dispositions

### Batch 46.2 — Conditional Prepared Binding

- [ ] Execute card 128 only if card 127 admits a non-empty deliver-now set.
- [ ] carry exact reasoning through shared request state and the owning Gemini
      prepared route
- [ ] reject unsupported values and request/plan/driver drift before effects

### Batch 46.3 — Route-Local Acceptance

- [ ] Execute card 129 only after card 128.
- [ ] prove exact initial, rollover, restoration, omission, rejection, and
      unchanged lifecycle behavior
- [ ] update route-local guidance and record the deferred shared closeout delta

## Acceptance Criteria

- [ ] only Research 193 deliver-now values prepare
- [ ] input, capability constraint, plan, evidence, request, driver, and every
      setup frame agree exactly
- [ ] omission preserves the current `MINIMAL` initial and resume setup bytes
- [ ] unsupported and drifted selections fail before endpoint, credential, or
      socket work
- [ ] no value is clamped, aliased, substituted, inferred, retried, or routed
- [ ] one planned rollover and fresh restoration retain the selected level
      without turning either path into retry or consumer resume
- [ ] existing media, transcription, activity, usage, cancellation, deadline,
      provider failure, and cleanup behavior remain intact
- [ ] docs claim dispatch only; provider acceptance, effectiveness, and thought
      disclosure remain unclaimed
- [ ] default QA uses no credential, account, provider request, or paid work

## Lane Runway

- predecessor: g04.045 Claude Code headless structured-output evidence stop
- this milestone: Gemini Live thinking-level evidence and conditional binding
- execution topology: one serial worker lane, cards 127-129
- next route family: selected by the orchestrator after evidence, review, and
  merge closeout; no later family is precompiled here

## Decision Gates

- Stop if current official evidence no longer names this exact model, Live
  surface, field, or all four candidate levels.
- Stop if omission cannot preserve the current exact `MINIMAL` setup.
- Stop if a selected value cannot remain immutable across initial setup,
  planned rollover, and fresh restoration.
- Stop if the request carrier would create a generic settings map or let a
  sibling realtime driver ignore an unsupported selection.
- Stop if delivery requires `thinkingBudget`, context compression, live proof,
  a contract amendment, compatibility inference, or breaking public API.

## Batch Cards

- [127-gemini-live-thinking-level-evidence.md](batch-cards/127-gemini-live-thinking-level-evidence.md) — complete
- [128-gemini-live-thinking-level-binding.md](batch-cards/128-gemini-live-thinking-level-binding.md) — conditional
- [129-gemini-live-thinking-level-acceptance.md](batch-cards/129-gemini-live-thinking-level-acceptance.md) — conditional

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 021 Gemini Live Portability Selection](../../research/021-gemini-live-portability-selection.md)
- [Contract 027 Planned Connection Rollover](../../contracts/027-planned-connection-rollover-and-realtime-continuity.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Realtime Prepared Integration](../../guides/realtime-prepared-integration.md)
- [Gemini 3.1 Flash Live Preview](https://ai.google.dev/gemini-api/docs/models/gemini-3.1-flash-live-preview)
- [Gemini Live API Capabilities](https://ai.google.dev/gemini-api/docs/live-api/capabilities)
