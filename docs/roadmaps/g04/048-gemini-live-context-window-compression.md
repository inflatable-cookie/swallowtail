# g04.048 Gemini Live Context-Window Compression

Status: planned
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Depends on: per-route feature completion programme; g04.047
Vision tags: explicit selection, provider truth, realtime continuity
Contract refs: 011, 027, 029, 037, 040, 050, 052
Research: 021, 193, 194, 195

## Problem

Production route `gemini.live` fixes exact model
`gemini-3.1-flash-live-preview`, hosted `v1beta` raw WebSocket, manual
asymmetric PCM, output transcription, caller-selectable thinking and output
maximum, and one provider-planned rollover. It does not expose
`BidiGenerateContentSetup.contextWindowCompression`.

Current official Google material names context-window compression on Live and
shows sliding-window setup on this exact model. It describes an omitted
`triggerTokens` provider default and a nested `targetTokens` default, but the
exact accepted explicit-value domain and JSON integer representation still
need qualification. Setup completion has no fields, so deterministic evidence
can prove dispatch only.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind the smallest route-local
Gemini Live sliding-window compression subset without changing route, model,
media, reasoning, output maximum, access, rollover, or lifecycle truth.

## Goals

- [ ] freeze current official exact-model, Live-reference, session-management,
      and best-practices evidence
- [ ] classify omitted, provider-default sliding window, and explicit
      trigger/target shapes, including integer encoding and rejection domain
- [ ] promote Research 195 with a precise deliver-now table or an honest stop
- [ ] preserve prior setup bytes when compression is omitted
- [ ] bind only admitted shapes through adapter-local typed prepared state
- [ ] keep one selected configuration immutable across initial setup, planned
      rollover/resume setup, and fresh working-state restoration
- [ ] compose with every admitted thinking level and both omitted/selected
      output maximum
- [ ] publish setup-dispatch truth without acceptance, effectiveness,
      retention, duration, or token-saving claims

## Non-Goals

- a portable context-window capability, shared realtime carrier, generic
  provider settings map, client-side truncation, token counting, or summaries
- automatic defaults chosen by Swallowtail, aliases, clamps, retries, route or
  model fallback, or configuration mutation during rollover
- long-session guarantees, compression-effect observation, retained-history
  claims, provider acceptance, or semantic continuity strengthening
- tools, automatic activity, text/image/video input, another Gemini route or
  model, Vertex AI, browser access, or consumer login
- credentials, account inspection, live provider requests, paid work, release,
  or Contract 029 currentness widening

## Named Scope

The lane is restricted to route `gemini.live`, driver
`swallowtail.gemini.live`, model `gemini-3.1-flash-live-preview`, axis
`gemini.live-facade`, and the current exact `v1beta`
`BidiGenerateContent` raw-WebSocket surface.

Candidate wire field:
`setup.contextWindowCompression.slidingWindow`. Candidate shapes are exact
omission, provider-default `{ "slidingWindow": {} }`, and evidence-qualified
explicit `triggerTokens` plus nested `targetTokens`. Default-only is a
candidate, not preapproved. Partial explicit shapes, zero, negative,
fractional, unordered, overflowing, aliased, or clamped values prepare only if
Research 195 explicitly admits them; otherwise they reject before effects.

The control is adapter-local, following the route-local prepared-selection
shape used for Ollama `num_ctx`. No `Capability` variant and no shared
`OpenRealtimeMediaSessionRequest` field are planned. Delivery that needs either
is a stop.

The current absence path sends no `contextWindowCompression` member. That path
must remain byte-exact. A selected configuration must remain identical across
initial setup, one provider-planned rollover/resume setup, and fresh realtime
working-state restoration. Existing latest-resumable-handle and terminal
failure rules remain unchanged.

An empty Research 195 deliver-now set is an honest stop. Contract 027 now
permits only the evidence-qualified extension described above; it does not
preapprove a numeric value or claim effective compression.

## Execution Plan

### Batch 48.1 — Exact Compression Evidence

- [ ] Execute card 133.
- [ ] freeze official and repository evidence, exact shapes, domain, omission,
      continuity, composition, and version dispositions
- [ ] promote Research 195 with a deliver-now table or stop

### Batch 48.2 — Conditional Adapter-Local Binding

- [ ] Execute card 134 only if card 133 admits a non-empty deliver-now set.
- [ ] bind one exact typed Gemini selection through prepared state, driver
      validation, and initial/resume setup encoding
- [ ] mint the Research 195 Contract 029 facade/private behavior revisions

### Batch 48.3 — Route-Local Acceptance

- [ ] Execute card 135 only after card 134.
- [ ] prove exact omission, admitted forms, rollover, restoration, composition,
      rejection, and unchanged lifecycle behavior
- [ ] update route-local guidance and reserve the shared closeout delta

## Acceptance Criteria

- [ ] only Research 195 deliver-now shapes prepare
- [ ] input, plan, evidence, driver, and every setup frame agree exactly
- [ ] omission preserves prior initial and resume setup bytes
- [ ] unsupported and drifted values fail before endpoint, credential, or
      socket work
- [ ] no value is defaulted by the adapter, clamped, aliased, substituted,
      inferred, retried, or routed elsewhere
- [ ] one planned rollover and fresh restoration retain the selected config
- [ ] every thinking/output-maximum combination composes without semantic drift
- [ ] resumable-handle, media, transcription, activity, usage, cancellation,
      deadline, provider failure, and joined cleanup truth remain intact
- [ ] docs claim dispatch only; provider acceptance and effective compression
      remain unclaimed
- [ ] default QA performs no credential, account, provider request, or paid work

## Lane Runway

- predecessor: g04.047 Gemini Live output-token maximum
- this milestone: Gemini Live context-window-compression evidence and
  conditional route-local binding
- execution topology: one serial worker lane, cards 133-135
- next route family: selected by the orchestrator after evidence, review, and
  merge closeout; no later family is precompiled here

## Decision Gates

- Stop if current official evidence cannot close an exact setup shape on this
  model and facade without inference.
- Stop if integer wire form, explicit numeric domain, omission, or rejection
  behavior cannot be closed.
- Stop if one selected config cannot remain immutable across initial setup,
  planned rollover, and fresh restoration.
- Stop if delivery needs a portable capability, shared carrier, live proof,
  compatibility inference, or breaking API.
- Stop if compression changes thinking, output maximum, handle truth, or
  another realtime route.

## Batch Cards

- [133-gemini-live-context-window-compression-evidence.md](batch-cards/133-gemini-live-context-window-compression-evidence.md) — ready
- [134-gemini-live-context-window-compression-binding.md](batch-cards/134-gemini-live-context-window-compression-binding.md) — conditional
- [135-gemini-live-context-window-compression-acceptance.md](batch-cards/135-gemini-live-context-window-compression-acceptance.md) — conditional

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 027 Planned Connection Rollover](../../contracts/027-planned-connection-rollover-and-realtime-continuity.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Realtime Prepared Integration](../../guides/realtime-prepared-integration.md)
- [Gemini Live WebSocket Reference](https://ai.google.dev/api/live)
- [Gemini Live Session Management](https://ai.google.dev/gemini-api/docs/live-api/session-management)
- [Gemini Live Best Practices](https://ai.google.dev/gemini-api/docs/live-api/best-practices)
