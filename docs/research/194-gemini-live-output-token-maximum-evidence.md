# 194 Gemini Live Output-Token-Maximum Evidence

Status: promoted
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.047 / 130

## Question

Can exact production route `gemini.live`, fixed model
`gemini-3.1-flash-live-preview`, and the current `v1beta` raw-WebSocket facade
bind one positive caller-selected maximum through
`generationConfig.maxOutputTokens` while satisfying Contracts 027, 037, and
040?

Yes, for dispatch. Every positive integer through the exact model's output
token limit of 65,536 is admitted deliver-now at a new exact opaque facade
point with a new adapter-private behavior revision, while the current
thinking-capable point and its proof are retained verbatim. Provider
acceptance and effective generated length remain unclaimed.

## Method And Evidence Boundary

Official Google Gemini API documentation was fetched on 2026-08-23. The review
used only public, secret-free documentation plus exact repository source and
fixtures. It did not authenticate, inspect an account, key, or project, open a
socket, send a setup frame, invoke the model, or mutate provider state. No
live Gemini call was made.

The fetched HTML specimens and SHA-256 digests are:

| Surface | URL | Page date | Specimen SHA-256 |
| --- | --- | --- | --- |
| Exact model page | <https://ai.google.dev/gemini-api/docs/models/gemini-3.1-flash-live-preview> | 2026-08-18 | `efb7fd6c9815c297c7f83aa57f933e8ece30332f0ee93082085cd044bea9534a` |
| Live WebSocket reference | <https://ai.google.dev/api/live> | 2026-06-01 | `7bcf916d51bdaaeff73161fc67ffb1ab2a32298c83a932505cbda7d98b30b414` |
| Generate-content reference | <https://ai.google.dev/api/generate-content> | 2026-08-17 | `1e8768a582275a37ad11c382cb268aa89c2925a8505f240a0b71d366f3280290` |
| Live API capabilities | <https://ai.google.dev/gemini-api/docs/live-api/capabilities> | 2026-08-05 | `87ec3368db132bb7c50a7a391d8c87f0ec45f6be511bb0369528daf0f4087d6a` |

Page dates are the pages' own `Last updated ... UTC` stamps. The digests
identify the complete fetched HTML response. They are provenance for the dated
review; they do not turn a mutable provider page into a permanent
compatibility guarantee.

## Frozen Official Findings

### The field exists on the exact Live surface

The Live WebSocket reference binds the route this adapter already uses:

> `wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent`
> Note: The URL is for version `v1beta`.

`BidiGenerateContentSetup` carries `model` (required), `generationConfig`
(optional, type `GenerationConfig`), `systemInstruction`, `tools[]`,
`realtimeInputConfig`, `sessionResumption`, `outputAudioTranscription`, and
others. The reference names the exact `GenerationConfig` fields the Live setup
does not support:

> The following fields are not supported: `responseLogprobs`,
> `responseMimeType`, `logprobs`, `responseSchema`, `stopSequence`,
> `routingConfig`, `audioTimestamp`

`maxOutputTokens` is not in that exclusion list. The same Live reference's
example session configuration includes it under `generationConfig`:

> ```
> "generationConfig": {
>   "candidateCount": integer,
>   "maxOutputTokens": integer,
>   "temperature": number,
>   "topP": number,
>   "topK": integer,
>   "presencePenalty": number,
>   "frequencyPenalty": number,
>   "responseModalities": [string],
>   "speechConfig": object,
>   "mediaResolution": object
> }
> ```

The generate-content reference defines the typed field:

> **maxOutputTokens** integer Optional. The maximum number of tokens to include
> in a response candidate. Note: The default value varies by model, see the
> `Model.output_token_limit` attribute of the Model returned from the getModel
> function.

and opens `GenerationConfig` with the model-specificity warning:

> Configuration options for model generation and outputs. Not all parameters
> are configurable for every model.

That warning is why catalogue presence alone is not enough. For this composed
route the Live reference both accepts `GenerationConfig` and names
`maxOutputTokens` in its Live example while leaving it out of the unsupported
set. The exact model page supplies the matching `output_token_limit` the field
note itself points at. The Live capabilities guide does not add a separate
`maxOutputTokens` sample; it is silent rather than excluding. Silence there
does not override the Live reference's typed setup table, unsupported set, and
example.

### The numeric domain is closed by the exact model limit

The exact model page for `gemini-3.1-flash-live-preview` lists:

> **Token limits** Input token limit 131,072 · Output token limit 65,536

`GenerationConfig.maxOutputTokens` is an optional integer whose own note ties
its default to `Model.output_token_limit`. The exact composed upper bound for
caller selection on this route is therefore 65,536. Values above that limit,
zero, negatives, fractions, overflowing forms, aliases, and clamped
substitutions are not candidates. Contract 040 already requires a positive
caller maximum; the existing `NonZeroU64` carrier excludes zero structurally.

Official Go samples type the field as `int32`. 65,536 fits signed 32-bit
storage, so the model limit is the binding upper bound rather than the int32
ceiling. This record does not widen the domain to `2_147_483_647`.

### The surface returns no configuration confirmation

`BidiGenerateContentSetupComplete` in the Live reference is exact:

> **BidiGenerateContentSetupComplete** This type has no fields. Sent in
> response to a `BidiGenerateContentSetup` message from the client.

No server message echoes the applied generation config or output-token
maximum. Deterministic setup bytes can prove dispatch only.

### Configuration mutability across resumption

The Live reference states:

> You cannot update the configuration while the connection is open. However,
> you can change the configuration parameters, except the model, when pausing
> and resuming via the session resumption mechanism.

The provider therefore permits a different maximum on a resumed connection. It
does not require one. Immutability across rollover is a Swallowtail-side
guarantee this route must enforce itself; it is not provider-enforced and must
not be presented as such.

## Frozen Route Evidence

Exact current repository truth at worker base
`277c982d216001839a7a3ef8be1988a3e9de0bf5`:

| Item | Exact current value | Source |
| --- | --- | --- |
| Route | `gemini.live` | `src/live.rs` |
| Driver | `swallowtail.gemini.live` | `src/live.rs` |
| Model | `gemini-3.1-flash-live-preview` | `src/live_selection.rs` |
| Model resource | `models/gemini-3.1-flash-live-preview` | `src/live.rs` |
| Facade axis | `gemini.live-facade`, `Opaque`, `QualifiedOnly` | `src/live_selection.rs` |
| Facade point | `...BidiGenerateContent.thinking-2026-08-23` | `src/live_selection.rs` |
| Behavior revision | `gemini.live-preview-manual-pcm-rollover-thinking-v2` | `src/live_selection.rs` |
| Claim id | `gemini.live-preview-window-2` | `src/live_selection.rs` |
| Model-route revision | `prepared-2` | `src/prepared_live_profile/plan.rs` |
| Access | `gemini.authorization-api-key.project`, `ApiKey`, provider-supported | `src/live_selection.rs` |
| Base capabilities | `StreamingEvents`, `UsageReporting`, `Interruption(ActiveResponse)`, `RealtimeMedia`, `PlannedConnectionRollover(1)` | `src/live_selection.rs` |
| Optional reasoning | `ReasoningSelection` + exact `ReasoningMode` when selected | `src/prepared_live_profile/session.rs` |
| Prepared input | request id, media, deadline, rollover, optional reasoning; no maximum | `src/prepared_live_profile/input.rs` |
| Shared request | `OpenRealtimeMediaSessionRequest` already has `with_maximum_output_tokens` | `swallowtail-runtime` |
| Setup encoder | `generationConfig` with modalities, speech, and `thinkingConfig`; no `maxOutputTokens` | `src/live_protocol/client.rs` |
| Initial frame | one `ClientFrame::Setup { handle: None }` | `src/live/session.rs` |
| Rollover frame | one `ClientFrame::Setup { handle: Some(_) }` | `src/live/session/rollover.rs` |
| Restoration | `PreparedWorkingStateRestoration::fresh_realtime_session_replacement` reusing the same plan and request | `src/prepared_live_profile/session.rs` |

Current deterministic fixtures
`tests/fixtures/gemini-live-2026-07-22/client-setup-initial.json` and
`client-setup-resume.json` both omit `maxOutputTokens`. Digests:

| Fixture | SHA-256 |
| --- | --- |
| `client-setup-initial.json` | `3c2f423f0981a50b1a04210e14744da3f660ee2acc0f3c37096a9ef21ed1be43` |
| `client-setup-resume.json` | `641e9628592a923031da8374f7270ae8ccc891f1e1c4a999cbd237fe33f4edf1` |

The thinking-level fixtures differ only in `thinkingLevel` and, for resume,
`sessionResumption.handle`. None claim an output-token maximum.

## Exact Disposition

### Model and facade

| Item | Disposition |
| --- | --- |
| `gemini-3.1-flash-live-preview` | Deliver now; the only qualified model |
| `...BidiGenerateContent.thinking-output-max-2026-08-23` | Mint as the exact new facade point for thinking-capable plus output-maximum behavior |
| `gemini.live-preview-manual-pcm-rollover-thinking-output-max-v3` | Mint as the private behavior revision for the new point |
| `gemini.live-preview-window-3` | Mint as the claim revision |
| `...BidiGenerateContent.thinking-2026-08-23` | Retain verbatim as the superseded thinking-capable proof; not a supported claim |
| `gemini.live-preview-manual-pcm-rollover-thinking-v2` | Retain verbatim as the frozen proof of the superseded thinking point |
| `...BidiGenerateContent` | Retain as deeper historical proof named by Research 193; still not a supported claim |
| another Gemini model, API version, or route | Reject; no inference |

Adding optional `OutputTokenLimit` changes capability behavior on the exact
opaque axis. Contract 029's upgrade workflow therefore requires a new exact
point rather than rewriting the meaning of
`...thinking-2026-08-23`. The point keeps the provider RPC name and adds a
dated Swallowtail qualification suffix. It is an opaque Swallowtail label, not
a claim that the provider renamed its RPC. The provider surface itself is
unchanged: `maxOutputTokens` is an existing `GenerationConfig` member of the
same `v1beta` `BidiGenerateContent` method.

The thinking-capable proof is kept, not erased.
`...thinking-2026-08-23` and
`gemini.live-preview-manual-pcm-rollover-thinking-v2` remain named verbatim
here and become `GEMINI_LIVE_SUPERSEDED_FACADE_REVISION`. The frozen thinking
corpus frames remain byte-unchanged, and a deterministic test must prove a plan
bound to the superseded thinking point is rejected before endpoint, credential,
or socket work. The driver continues to require the exact current facade, so
the deeper historical `...BidiGenerateContent` point stays non-executable.

`swallowtail-core` still permits only one exact opaque segment. The thinking
point cannot remain a concurrent live claim. Consumers must publish a new
configured-instance revision when they move to the new point. The adapter-owned
model-route revision advances to `prepared-3`.

### Portable value mapping

| Portable value | Wire `maxOutputTokens` | Evidence | Disposition |
| --- | --- | --- | --- |
| `NonZeroU64` in `1..=65_536` | exact integer | Live example field, GenerationConfig definition, exact-model output limit 65,536 | Deliver now |
| omitted | member absent | current initial and resume fixtures | Deliver now as omission |
| `0` | — | excluded by `NonZeroU64` / Contract 040 | Reject before effects |
| `> 65_536` | — | above exact-model output limit | Reject before effects |
| negative, fractional, non-integer | — | not a positive integer field | Reject before effects |
| overflowing `u64` forms beyond the admitted domain | — | not exact | Reject before effects |
| aliases, defaults chosen by the adapter, clamped neighbours | — | not exact | Reject before effects |
| catalogue-only inference without a caller value | — | omission is not a selection | Reject as a claim; omission path adds no capability |

The adapter sends the exact selected positive integer. It never clamps to
65,536, substitutes a default, aliases a foreign bound, truncates client-side,
counts tokens, or reports an effective generated length from output,
transcript, or usage.

Representative deliver-now bounds for acceptance coverage are the domain
minimum `1`, a mid-range value such as `1_024`, and the domain maximum
`65_536`. Reject coverage includes `65_537`.

### Omission boundary

| State | Capability claimed | Initial setup bytes | Resume setup bytes |
| --- | --- | --- | --- |
| no caller maximum | none for `OutputTokenLimit` | current fixtures without `maxOutputTokens` | current fixtures without `maxOutputTokens` |
| explicit `1..=65_536` | `OutputTokenLimit` + exact `OutputTokenMaximum` | `maxOutputTokens` set to that integer | same integer with resumption handle |

Omission preserves both current fixture digests byte-identical and adds no
`OutputTokenLimit` capability, constraint, or evidence row. Nothing infers the
capability from provider defaults or from the model catalogue limit.

### Reasoning composition

| Maximum | Reasoning | Result |
| --- | --- | --- |
| omitted | omitted | current default `MINIMAL` bytes; no reasoning or output-limit capability |
| omitted | `minimal\|low\|medium\|high` | current thinking fixtures; no output-limit capability |
| `1..=65_536` | omitted | `maxOutputTokens` plus default `MINIMAL`; output-limit capability only |
| `1..=65_536` | `minimal\|low\|medium\|high` | both fields exact; both capabilities claimed |

Neither control aliases, defaults, or rewrites the other. Thinking casing,
default `MINIMAL` omission, admitted value set, and Research 193 claim truth
remain unchanged.

### Continuity

| Path | Selection source | Result |
| --- | --- | --- |
| initial setup | prepared request | selected maximum encoded once, or omitted |
| planned rollover setup (one, private handle) | same prepared request held by the open session | same maximum re-encoded with the resumption handle |
| fresh working-state restoration | same plan and same request cloned into the replacement | same maximum on the new session's initial setup |

The selected maximum is fixed at preparation and is immutable for the life of
the prepared session. The provider permits a changed configuration on a
resumed connection; this route does not use that latitude. Rollover remains an
in-session connection replacement with a private replaceable handle, not
retry, not consumer resume, and not durable provider state. Restoration
remains a fresh session that loses prior connection state.

### Contract 040 evidence truth

| State | Claimable here | Reason |
| --- | --- | --- |
| requested | yes | typed optional prepared input on the existing shared carrier |
| planned | yes | exact capability, constraint, and immutable plan |
| dispatched | yes | deterministic setup-frame bytes on both connections |
| accepted | no | `BidiGenerateContentSetupComplete` has no fields; nothing confirms the field |
| effective | no | no surface returns an applied or enforced generated length |
| observed | no | usage and transcripts are not reinterpreted as a bound |

Docs and diagnostics may claim qualified dispatch only. Setup completion
proves the setup message as a whole was not rejected; it is not per-field
acceptance and must not be published as such.

### Compatibility

No Contract 029 currentness widening, no version-range change, no newer-version
posture change, no support-authority change, no shared runtime carrier change,
and no contract amendment are required. The route remains a provider-supported
preview on an opaque qualified-only single-point claim. Sibling realtime
routes retain their own independent maximum domains.

## Promotion

Research 194 promotes the following deliver-now subset for card 131:

- exact model `gemini-3.1-flash-live-preview`
- exact new facade point
  `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-2026-08-23`,
  with the superseded thinking point and its `-thinking-v2` behavior retained as
  frozen evidence
- new adapter-private behavior revision
  `gemini.live-preview-manual-pcm-rollover-thinking-output-max-v3` and claim
  revision `gemini.live-preview-window-3`
- model-route revision `prepared-3`
- positive portable domain `1..=65_536` mapped to exact setup
  `generationConfig.maxOutputTokens`
- existing shared carrier
  `OpenRealtimeMediaSessionRequest::with_maximum_output_tokens` and exact
  `CapabilityConstraint::OutputTokenMaximum`
- omission preserving the current exact initial and resume setup bytes with no
  `OutputTokenLimit` capability claimed
- composition with omitted reasoning and every admitted Research 193 thinking
  level without changing thinking semantics

Card 131 may bind this subset. Card 132 must prove exact preparation, initial
and resumed setup bytes, one planned rollover, fresh restoration, omission,
reasoning composition, and rejection without claiming provider acceptance or
effective generated length.
