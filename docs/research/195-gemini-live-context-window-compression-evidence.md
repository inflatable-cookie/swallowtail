# 195 Gemini Live Context-Window-Compression Evidence

Status: promoted
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.048 / 133-135

## Question

Can exact production route `gemini.live`, fixed model
`gemini-3.1-flash-live-preview`, and the current `v1beta` raw-WebSocket facade
bind an adapter-local sliding-window context-compression selection while
satisfying Contracts 027, 037, and 040?

Yes, for one default-only dispatch shape. The deliver-now set is omission or
the exact provider-default sliding-window object
`{ "slidingWindow": {} }`. Explicit trigger and target token forms remain
withheld because their model-specific accepted numeric domain and rejection
behavior are not closed. Provider acceptance, effective compression, retained
history, duration, token savings, and semantic continuity remain unclaimed.

## Method And Evidence Boundary

Official Google Gemini documentation was fetched on 2026-08-23. The review used
only public, secret-free documentation plus exact repository source and
deterministic fixtures. It did not authenticate, inspect an account, key, or
project, open a socket, send a setup frame, invoke the model, or mutate provider
state. No live Gemini call was made.

The fetched HTML specimens and SHA-256 digests are:

| Surface | URL | Page date | Specimen SHA-256 |
| --- | --- | --- | --- |
| Exact model page | <https://ai.google.dev/gemini-api/docs/models/gemini-3.1-flash-live-preview> | 2026-08-18 | `cb32e1f6959347d24ab5953bbfe53961ec8b5435f296190da8f560aaa9bcff23` |
| Live WebSocket reference | <https://ai.google.dev/api/live> | 2026-06-01 | `d85e15157f4c2573f36947281518bb1b31ce92cc709282e5cbc5f51cedfb7fe3` |
| Session management | <https://ai.google.dev/gemini-api/docs/live-api/session-management> | 2026-06-01 | `39200c637912531a78edf60f230d10ad7937384418b1389b81ff6e6870307315` |
| Live best practices | <https://ai.google.dev/gemini-api/docs/live-api/best-practices> | 2026-06-01 | `c94f0a208567f4be9b69112372a4b57af8a114fd4d6a94674bf2644f288efa08` |
| Generic API reference | <https://ai.google.dev/api/generate-content> | 2026-08-17 | `8f525c8a895009af0718f50ba74ee8a89cab901b030c042240c2a35212add0f5` |

Page dates are the pages' own `Last updated ... UTC` stamps. Digests identify
the complete fetched HTML responses for this dated review; they do not make a
mutable provider page a permanent compatibility guarantee.

## Frozen Official Findings

### Exact Live applicability

The Live reference binds the route already used by the adapter:

`wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent`

The same reference defines optional
`BidiGenerateContentSetup.contextWindowCompression` with type
`ContextWindowCompressionConfig`. It documents the configuration as enabling
automatic context reduction when the context grows. The config is a union of
`slidingWindow` and `triggerTokens`.

The session-management page shows the default sliding-window selection in the
first-party SDKs, including the exact JavaScript object shape
`{ slidingWindow: {} }`. It also uses the exact model
`gemini-3.1-flash-live-preview` in its Live session material. This closes the
field, shape, model, and route applicability for the default-only object.

The best-practices page describes context compression and gives illustrative
trigger/sliding examples. Those examples explain provider behavior but do not
close a portable numeric domain or prove that the example values are accepted
for this exact model and facade.

### Explicit numeric forms

The Live reference documents `triggerTokens` as `int64` and
`SlidingWindow.targetTokens` as `int64`. The generic API reference closes the
protobuf JSON representation: both are JSON strings using the `int64` format.
That closes representation, not acceptance.

The official material does not close all of the route-local facts required for
an explicit numeric API: positive lower bound, zero semantics, negative
semantics, maximum or model-specific context domain, trigger/target ordering,
overflow handling, or rejection behavior. A best-practices example such as
`25,000` trigger and `8,000` sliding is illustrative, not a domain proof. The
official Google Go corpus also demonstrates string-encoded explicit values, but
it does not close this exact model's accepted domain or provider rejection
semantics.

Therefore explicit `triggerTokens`, explicit `targetTokens`, and an explicit
pair are withheld. The adapter has no numeric constructor, parser, default,
clamp, alias, or inferred fallback for them. JSON numbers, fractional values,
zero, negative values, overflowing values, unknown members, and partial or
aliased forms have no deliver-now representation and must not be added by
inference.

### Provider behavior and observable boundary

The Live reference describes a sliding window that discards older content at
the beginning while preserving system instructions/prefix material and starting
at a user role. It describes provider defaults for trigger and target values.
Those are provider behavior statements, not Swallowtail observations.

`BidiGenerateContentSetupComplete` has no fields. It confirms neither the
selected compression object nor an effective context window. Swallowtail can
claim requested, planned, and dispatched state from its typed selection, plan,
evidence, and deterministic setup bytes only. It cannot claim acceptance,
effective trigger, retained history, compression duration, token savings, or
semantic continuity from setup completion, usage, transcripts, or successful
turns.

The provider reference permits configuration changes on session resumption.
This route deliberately keeps one selected value immutable across its initial
setup, one planned rollover/resume setup, and fresh realtime restoration. That
is adapter behavior, not a provider guarantee.

## Frozen Route Evidence

Exact route source and fixture truth after the binding batch:

| Item | Exact current value | Source |
| --- | --- | --- |
| Route | `gemini.live` | `crates/swallowtail-adapter-gemini/src/live.rs` |
| Driver | `swallowtail.gemini.live` | `src/live.rs` |
| Model | `gemini-3.1-flash-live-preview` | `src/live_selection.rs` |
| Model resource | `models/gemini-3.1-flash-live-preview` | `src/live.rs` |
| Facade axis | `gemini.live-facade`, `Opaque`, `QualifiedOnly` | `src/live_selection.rs` |
| Current facade | `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-context-compression-2026-08-23` | `src/live_selection.rs` |
| Private behavior | `gemini.live-preview-manual-pcm-rollover-thinking-output-max-context-compression-v4` | `src/live_selection.rs` |
| Claim | `gemini.live-preview-window-4` | `src/live_selection.rs` |
| Model-route revision | `prepared-4` | `src/prepared_live_profile/plan.rs` |
| Access | `gemini.authorization-api-key.project`, `ApiKey`, provider-supported | `src/live_selection.rs` |
| Base capabilities | `StreamingEvents`, `UsageReporting`, `Interruption(ActiveResponse)`, `RealtimeMedia`, `PlannedConnectionRollover(1)` | `src/live_selection.rs` |
| Prepared selection | optional `GeminiLiveContextWindowCompression` | `src/prepared_live_profile/input.rs` |
| Admitted constructor | `GeminiLiveContextWindowCompression::sliding_window()` | `src/live_compression.rs` |
| Shared request | unchanged; no portable compression field | `swallowtail-runtime` |
| Setup encoding | optional top-level `contextWindowCompression.slidingWindow` | `src/live_protocol/client.rs` |
| Initial setup | selected object with no resumption handle | `src/live/session.rs` |
| Rollover setup | same selected object with latest private handle | `src/live/session/rollover.rs` |
| Restoration | same plan, request, and bound driver in a fresh realtime replacement | `src/prepared_live_profile/session.rs` |

The existing omission fixtures remain byte-identical:
`tests/fixtures/gemini-live-2026-07-22/client-setup-initial.json` and
`client-setup-resume.json`. New deterministic fixtures record the selected
default-only object for initial and resume setup. Setup completion remains a
dispatch boundary with no compression-specific response fields.

## Exact Disposition

### Model, facade, and revisions

| Item | Disposition |
| --- | --- |
| `gemini-3.1-flash-live-preview` | Deliver now; the only qualified model |
| `...BidiGenerateContent.thinking-output-max-context-compression-2026-08-23` | Current exact facade point |
| `gemini.live-preview-manual-pcm-rollover-thinking-output-max-context-compression-v4` | Current private behavior revision |
| `gemini.live-preview-window-4` | Current claim revision |
| `prepared-4` | Current adapter-owned model-route revision |
| `...BidiGenerateContent.thinking-output-max-2026-08-23` | Retain as `GEMINI_LIVE_OUTPUT_MAXIMUM_SUPERSEDED_FACADE_REVISION`; non-executable |
| `...BidiGenerateContent.thinking-2026-08-23` | Retain as `GEMINI_LIVE_THINKING_SUPERSEDED_FACADE_REVISION`; non-executable |
| `...BidiGenerateContent` | Retain as `GEMINI_LIVE_SUPERSEDED_FACADE_REVISION`; non-executable |
| another Gemini model, API version, or route | Reject; no inference |

The current point advances the exact opaque single-segment claim because the
adapter now dispatches one additional selected setup control. The provider RPC
name and API version are unchanged. Earlier points remain named frozen proof,
not concurrent supported segments. Consumers moving to the current point must
publish a new configured-instance revision.

### Configuration dispositions

| Candidate | Wire shape | Disposition |
| --- | --- | --- |
| Omission | no `contextWindowCompression` member | Deliver now; preserves prior setup bytes and claims no compression selection |
| Provider-default sliding window | `{"contextWindowCompression":{"slidingWindow":{}}}` | Deliver now; exact typed adapter-local selection |
| Empty/unknown compression object | no exact accepted shape | Withhold; no inference |
| `triggerTokens` only | `{"triggerTokens":"N"}` | Withhold; int64 string form is known, accepted domain is not |
| `slidingWindow.targetTokens` only | `{"slidingWindow":{"targetTokens":"N"}}` | Withhold; int64 string form is known, accepted domain is not |
| Explicit trigger plus target | both int64 strings | Withhold; domain, ordering, and rejection behavior are not closed |
| JSON number, fraction, zero, negative, overflow, alias, clamp, or fallback | not an admitted exact form | Withhold/reject before effects; no public representation |

### Deliver-now table

| Prepared input | Setup bytes | Contract 040 truth |
| --- | --- | --- |
| no compression selection | no `contextWindowCompression` member | omission only; no compression capability or effective claim |
| `Some(GeminiLiveContextWindowCompression::sliding_window())` | exact `contextWindowCompression.slidingWindow = {}` on initial and resume setup | selected, planned, and dispatched only |

The selected value is held in prepared evidence and the bound driver. It is
not added to a portable capability, shared realtime carrier, generic provider
settings map, or sibling route.

## Continuity And Composition Evidence

| Path | Selection source | Deterministic result |
| --- | --- | --- |
| Initial setup | prepared input → evidence → bound driver | exact selected object, or exact omission |
| One planned rollover | same session selection plus latest private handle | same object; handle changes only as existing rollover state requires |
| Fresh restoration | same plan/request and selected driver | same object on the fresh session's initial setup |
| Omitted compression | no selection | prior initial and resume setup bytes remain exact |
| Thinking `minimal|low|medium|high` | existing reasoning selection | compression object is unchanged |
| Output maximum omitted or `1..=65_536` | existing output selection | compression object is unchanged |

The deterministic acceptance suite proves selected setup on rollover,
selected setup on fresh restoration, omission byte equality, latest-handle
privacy, and all five reasoning states (omission plus four admitted levels)
crossed with omitted and representative output maxima. Existing lifecycle,
media, transcription, usage, interruption, deadline, provider-failure, and
cleanup coverage remains in the same package suite.

## Contract And Compatibility Boundary

| State | Claimable here | Reason |
| --- | --- | --- |
| Requested | yes | typed optional prepared selection |
| Planned | yes | immutable prepared evidence and exact route plan |
| Dispatched | yes | deterministic initial and resume setup bytes |
| Accepted | no | setup completion has no fields |
| Effective | no | no applied compression or retained-context observation exists |
| Duration/savings/continuity | no | provider descriptions are not adapter observations |

No Contract 029 currentness widening, ordered version range, newer-version
posture change, support-authority change, shared runtime carrier change, or
contract amendment is required. The route remains a provider-supported preview
with one exact opaque qualified point. Sibling realtime routes retain their
own controls and domains.

## Promotion

Research 195 promotes the following non-empty deliver-now subset for cards
134-135:

- exact model `gemini-3.1-flash-live-preview` on the current v1beta Live facade
- omission with no compression member
- default-only `GeminiLiveContextWindowCompression::sliding_window()` mapped to
  `contextWindowCompression.slidingWindow = {}`
- current facade/private behavior/claim/model-route revisions recorded above
- immutable initial, one planned rollover/resume, and fresh-restoration
  dispatch through adapter-local prepared state
- composition with existing thinking and output-maximum controls without a
  portable compression capability
- setup-dispatch claims only

Cards 134 and 135 bind and test this subset. Explicit numeric token controls
remain a future evidence gate; this promotion does not preapprove them.
