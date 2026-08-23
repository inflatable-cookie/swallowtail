# 193 Gemini Live Thinking-Level Evidence

Status: promoted
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.046 / 127

## Question

Can exact production route `gemini.live`, fixed model
`gemini-3.1-flash-live-preview`, and the current `v1beta` raw-WebSocket facade
bind caller-selected `minimal|low|medium|high` through
`generationConfig.thinkingConfig.thinkingLevel` while satisfying Contracts
027, 037, and 040?

Yes, for dispatch. All four values are admitted deliver-now at a new exact
opaque facade point with a new adapter-private behavior revision, while the
superseded point and its proof are retained verbatim. Provider acceptance,
effective reasoning depth, and thought disclosure remain unclaimed.

## Method And Evidence Boundary

Official Google Gemini API documentation was fetched on 2026-08-23. The review
used only public, secret-free documentation plus exact repository source and
fixtures. It did not authenticate, inspect an account, key, or project, open a
socket, send a setup frame, invoke the model, or mutate provider state. No
live Gemini call was made.

The fetched HTML specimens and SHA-256 digests are:

| Surface | URL | Page date | Specimen SHA-256 |
| --- | --- | --- | --- |
| Exact model page | <https://ai.google.dev/gemini-api/docs/models/gemini-3.1-flash-live-preview> | 2026-08-18 | `681dedb9c99523122132bcf30c5c9f7c990a35b16387d67e50d02e0afab426d8` |
| Live API capabilities | <https://ai.google.dev/gemini-api/docs/live-api/capabilities> | 2026-08-05 | `0525a0985cf7330e8f0aa5c1130252c3a431a5c0a3bb1321aaaaed1e63edd19e` |
| Live WebSocket reference | <https://ai.google.dev/api/live> | 2026-06-01 | `46583da172de9fae08c16024b61e6819a006a2e75353e53561aec0fe7bd9abf2` |
| Generate-content reference | <https://ai.google.dev/api/generate-content> | 2026-08-17 | `002a3e78ac14a1ea9cbcc16f03caa8ff9dcb242aae11b8deb6037ef72c2dc53e` |
| Thinking guide | <https://ai.google.dev/gemini-api/docs/thinking> | 2026-08-17 | `4f477711a584d0e3d5479503134dcb969b20d4d207bb734ef589b3249869998a` |

Page dates are the pages' own `Last updated ... UTC` stamps. The digests
identify the complete fetched HTML response. They are provenance for the dated
review; they do not turn a mutable provider page into a permanent
compatibility guarantee.

## Frozen Official Findings

### The field exists on the exact surface

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

`thinkingConfig` is not in that exclusion list. The generate-content reference
defines it as a `GenerationConfig` member:

> **ThinkingConfig** Config for thinking features. Fields: `includeThoughts`
> boolean … `thinkingBudget` integer … `thinkingLevel` enum (`ThinkingLevel`)
> Optional. Controls the maximum depth of the model's internal reasoning
> process before it produces a response. The default value is model-dependent.
> … Recommended for Gemini 3 or later models. Use with earlier models results
> in an error.

The Live capabilities guide shows the field set on this exact model in the
initial session config, in both first-party SDKs:

> ```
> model = "gemini-3.1-flash-live-preview"
> config = types.LiveConnectConfig(
>     response_modalities=["AUDIO"]
>     thinking_config=types.ThinkingConfig(thinking_level="low",)
> )
> ```
> ```
> const model = 'gemini-3.1-flash-live-preview';
> const config = { responseModalities: [Modality.AUDIO],
>                  thinkingConfig: { thinkingLevel: 'low' }, };
> ```

The illustrative `generationConfig` sketch on the Live reference page lists
only nine members and omits `thinkingConfig`. It is an example, not the field
list; the typed `BidiGenerateContentSetup` field table and its explicit
unsupported set are the authority, and the capabilities guide sets the field
on this exact model. The repository already dispatches
`thinkingConfig.thinkingLevel` on this surface in production.

### The value set is closed and exact

The generate-content reference gives the complete enum:

> **ThinkingLevel** Allow user to specify how much to think using enum instead
> of integer budget. Enums: `THINKING_LEVEL_UNSPECIFIED` Default value.
> `MINIMAL` Little to no thinking. `LOW` Low thinking level. `MEDIUM` Medium
> thinking level. `HIGH` High thinking level.

Two current model-specific pages name all four selectable levels for this
exact model. The model page (2026-08-18) says:

> **Thinking configuration**: Gemini 3.1 uses `thinkingLevel` (with settings
> like `minimal`, `low`, `medium`, and `high`) instead of `thinkingBudget`.
> The default is `minimal` to optimize for lowest latency.

and lists **Thinking: Supported** in its capability table for model code
`gemini-3.1-flash-live-preview`. The capabilities guide (2026-08-05) repeats
it for the same model:

> Gemini 3.1 models use `thinkingLevel` to control thinking depth, with
> settings like `minimal`, `low`, `medium`, and `high`. The default is
> `minimal` to optimize for lowest latency.

The softening word "like" cannot widen the set: the reference enum is closed,
so the candidate space is exactly `MINIMAL|LOW|MEDIUM|HIGH` plus the
unspecified default.

Level support is per-model and is not transferable. The Thinking guide's
current control table shows `gemini-3.1-pro-preview` supporting `low, medium,
high` and `gemini-3-pro-preview` supporting `low, high`. That table covers
Interactions-API text models and does not list
`gemini-3.1-flash-live-preview`, so the two model-specific pages above are the
governing evidence for this route and no neighbouring model's set is inherited.

`thinkingBudget` remains a Gemini 2.5 control. The capabilities comparison
table assigns it to Gemini 2.5 Flash Live and assigns `thinkingLevel` to
Gemini 3.1 Flash Live. No numeric budget is translated here.

### The wire casing is the enum name

Proto JSON writes an enum as its declared name. The reference declares
`MINIMAL`, `LOW`, `MEDIUM`, and `HIGH`; the SDK samples show the lowercase
spellings the SDKs normalize. The raw-WebSocket route sends the declared name,
which is what the current production setup frames already contain
(`"thinkingLevel":"MINIMAL"`). No casing alias is introduced.

### The surface returns no configuration confirmation

`BidiGenerateContentSetupComplete` in the Live reference is exact:

> **BidiGenerateContentSetupComplete** This type has no fields. Sent in
> response to a `BidiGenerateContentSetup` message from the client.

No server message echoes the applied generation config, thinking level, or
thinking state. `outputAudioTranscription` is configured and not thought
material, and `includeThoughts` is neither sent nor in scope.

### Configuration mutability across resumption

The Live reference states:

> You cannot update the configuration while the connection is open. However,
> you can change the configuration parameters, except the model, when pausing
> and resuming via the session resumption mechanism.

The provider therefore permits a different level on a resumed connection. It
does not require one. Immutability across rollover is a Swallowtail-side
guarantee this route must enforce itself; it is not provider-enforced and must
not be presented as such.

## Frozen Route Evidence

Exact current repository truth at planning base
`c8335e5e44c94b3e43d57b4bbfd45dce74aad477`:

| Item | Exact current value | Source |
| --- | --- | --- |
| Route | `gemini.live` | `src/live.rs` |
| Driver | `swallowtail.gemini.live` | `src/live.rs` |
| Model | `gemini-3.1-flash-live-preview` | `src/live_selection.rs` |
| Model resource | `models/gemini-3.1-flash-live-preview` | `src/live.rs` |
| Facade axis | `gemini.live-facade`, `Opaque`, `QualifiedOnly` | `src/live_selection.rs` |
| Facade point | `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent` | `src/live_selection.rs` |
| Behavior revision | `gemini.live-preview-manual-pcm-rollover-v1` | `src/live_selection.rs` |
| Claim id | `gemini.live-preview-window-1` | `src/live_selection.rs` |
| Access | `gemini.authorization-api-key.project`, `ApiKey`, provider-supported | `src/live_selection.rs` |
| Capabilities | `StreamingEvents`, `UsageReporting`, `Interruption(ActiveResponse)`, `RealtimeMedia`, `PlannedConnectionRollover(1)` | `src/live_selection.rs` |
| Prepared input | `RequestId`, `RealtimeMediaConfig`, `Option<Deadline>`, `PlannedConnectionRolloverPolicy` | `src/prepared_live_profile/input.rs` |
| Request | `OpenRealtimeMediaSessionRequest` with media, deadline, output maximum, provider-state policy, rollover | `swallowtail-runtime/src/realtime_media/request.rs` |
| Setup encoder | `ClientFrame::Setup { handle }`, `thinkingConfig.thinkingLevel` hard-coded `MINIMAL` | `src/live_protocol/client.rs` |
| Initial frame | one `ClientFrame::Setup { handle: None }` | `src/live/session.rs` |
| Rollover frame | one `ClientFrame::Setup { handle: Some(_) }` | `src/live/session/rollover.rs` |
| Restoration | `PreparedWorkingStateRestoration::fresh_realtime_session_replacement` reusing the same plan and request | `src/prepared_live_profile/session.rs` |

The route claims no `ReasoningSelection` capability and exposes no caller
reasoning input. The deterministic fixtures
`tests/fixtures/gemini-live-2026-07-22/client-setup-initial.json` and
`client-setup-resume.json` both contain
`"thinkingConfig":{"thinkingLevel":"MINIMAL"}`; the two frames differ only in
`sessionResumption`.

## Exact Disposition

### Model and facade

| Item | Disposition |
| --- | --- |
| `gemini-3.1-flash-live-preview` | Deliver now; the only qualified model |
| `...BidiGenerateContent.thinking-2026-08-23` | Mint as the exact new facade point for the thinking-capable behavior |
| `gemini.live-preview-manual-pcm-rollover-thinking-v2` | Mint as the private behavior revision for the new point |
| `gemini.live-preview-window-2` | Mint as the claim revision |
| `...BidiGenerateContent` | Retain verbatim as the superseded historical point; not a supported claim |
| `gemini.live-preview-manual-pcm-rollover-v1` | Retain verbatim as the frozen proof of the superseded point |
| another Gemini model, API version, or route | Reject; no inference |

The adapter-private behavior changes: the driver stops encoding a fixed
constant and starts encoding a caller-selected level in both setup frames.
Contract 029's upgrade workflow requires a compatibility milestone when
capability behavior changes, so the thinking-capable behavior is qualified at
its own exact opaque point rather than replacing the meaning of the former one.

The point keeps the provider RPC name and adds a dated Swallowtail
qualification suffix, matching the repository's dated facade points. It is an
opaque Swallowtail qualification label, not a claim that the provider renamed
its RPC. The provider surface itself is unchanged: `thinkingConfig` is an
existing `GenerationConfig` member of the same `v1beta` `BidiGenerateContent`
method.

The former proof is kept, not erased. `...BidiGenerateContent` and
`gemini.live-preview-manual-pcm-rollover-v1` remain named verbatim here and in
`GEMINI_LIVE_SUPERSEDED_FACADE_REVISION`, the frozen corpus frames for that
behavior remain byte-unchanged, and a deterministic test proves a plan bound to
the superseded point is rejected before endpoint, credential, or socket work.

The former point cannot also remain a live claim segment. `swallowtail-core`
enforces "Opaque version windows permit one exact segment only", and a driver
descriptor holds one claim per axis. Keeping both as concurrent qualified
segments would require weakening Contract 029's opaque-axis rule, which this
lane's decision gates forbid. The retained-proof form above is the same shape
g04.044 used when the OpenAI background point advanced.

Consumers must publish a new configured-instance revision when they move to the
new point. The adapter-owned model-route revision advances to `prepared-2`.

### Portable value mapping

Each row was classified independently against the model-specific pages and the
closed reference enum.

| Portable `ReasoningMode` | Wire `thinkingLevel` | Evidence | Disposition |
| --- | --- | --- | --- |
| `minimal` | `MINIMAL` | model page, capabilities guide, reference enum, current production frames | Deliver now |
| `low` | `LOW` | model page, capabilities guide, reference enum, both SDK samples on this model | Deliver now |
| `medium` | `MEDIUM` | model page, capabilities guide, reference enum | Deliver now |
| `high` | `HIGH` | model page, capabilities guide, reference enum | Deliver now |
| `off`, `none`, `disabled` | — | no such enum member; `thinkingBudget=0` is a Gemini 2.5 control | Reject before effects |
| `default`, `on`, `auto`, `dynamic` | — | not a selection; `THINKING_LEVEL_UNSPECIFIED` is absence | Reject before effects |
| `xhigh`, `max` | — | not in the enum; other routes' vocabulary | Reject before effects |
| numeric budget of any form | — | `thinkingBudget` is a separate Gemini 2.5 field and is out of scope | Reject before effects |
| any other value, alias, or casing variant | — | not exact | Reject before effects |

The adapter sends the exact selected level. It never clamps to a neighbouring
level, substitutes a default, aliases a foreign vocabulary, converts to a
budget, or reports an effective depth from output, transcript, or usage.

### Omission boundary

| State | Capability claimed | Initial setup bytes | Resume setup bytes |
| --- | --- | --- | --- |
| no caller selection | none | current `"thinkingLevel":"MINIMAL"` | current `"thinkingLevel":"MINIMAL"` |
| explicit `minimal` | `ReasoningSelection` + `ReasoningMode("minimal")` | identical bytes | identical bytes |
| explicit `low`/`medium`/`high` | `ReasoningSelection` + exact `ReasoningMode` | `LOW`/`MEDIUM`/`HIGH` | same value |

Omission is fixed route behavior, not provider-field absence, and not a
caller selection. It keeps both current fixtures byte-identical and adds no
capability, constraint, or evidence row. Explicit `minimal` is a planned
`ReasoningSelection` that happens to serialize to the same bytes; the two
states are distinguished in the capability profile, operation requirements,
plan, and prepared evidence, never in the frame. Nothing infers the capability
from the bytes.

### Continuity

| Path | Selection source | Result |
| --- | --- | --- |
| initial setup | prepared request | selected level encoded once |
| planned rollover setup (one, private handle) | same prepared request held by the open session | same level re-encoded with the resumption handle |
| fresh working-state restoration | same plan and same request cloned into the replacement | same level on the new session's initial setup |

The selected level is fixed at preparation and is immutable for the life of the
prepared session. The provider permits a changed configuration on a resumed
connection; this route does not use that latitude. Rollover remains an
in-session connection replacement with a private replaceable handle, not
retry, not consumer resume, and not durable provider state. Restoration
remains a fresh session that loses prior connection state.

### Contract 040 evidence truth

| State | Claimable here | Reason |
| --- | --- | --- |
| requested | yes | typed optional prepared input |
| planned | yes | exact capability, constraint, and immutable plan |
| dispatched | yes | deterministic setup-frame bytes on both connections |
| accepted | no | `BidiGenerateContentSetupComplete` has no fields; nothing confirms the field |
| effective | no | no surface returns an applied thinking level |
| observed | no | no thought summaries; `includeThoughts` is not sent and is out of scope |

Docs and diagnostics may claim qualified dispatch only. Setup completion
proves the setup message as a whole was not rejected; it is not per-field
acceptance and must not be published as such.

### Compatibility

No Contract 029 currentness widening, no version-range change, no newer-version
posture change, no support-authority change, and no contract amendment are
required. The route remains a provider-supported preview on an opaque
qualified-only single-point claim.

## Promotion

Research 193 promotes the following deliver-now subset for card 128:

- exact model `gemini-3.1-flash-live-preview`
- exact new facade point
  `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-2026-08-23`,
  with the superseded point and its `-v1` behavior retained as frozen evidence
- new adapter-private behavior revision
  `gemini.live-preview-manual-pcm-rollover-thinking-v2` and claim revision
  `gemini.live-preview-window-2`
- `ReasoningMode` values `minimal`, `low`, `medium`, and `high`, mapped to
  `MINIMAL`, `LOW`, `MEDIUM`, and `HIGH`
- one optional portable reasoning carrier on
  `OpenRealtimeMediaSessionRequest`, with sibling realtime drivers rejecting an
  unsupported value before endpoint, credential, or socket work
- omission preserving the current exact initial and resume setup bytes with no
  reasoning capability claimed

Card 128 may bind this subset. Card 129 must prove exact preparation, initial
and resumed setup bytes, one planned rollover, fresh restoration, omission,
and rejection without claiming provider acceptance, effective reasoning depth,
or thought disclosure.
