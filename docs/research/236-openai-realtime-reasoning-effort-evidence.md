# 236 OpenAI Realtime Reasoning-Effort Evidence

Status: promoted
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.084 / 236-237
Binding: delivered on facade `openai-realtime-reasoning-2026-08-27`

## Question

Which exact `openai.realtime` model, reasoning-effort value, session/response
operation, and lifecycle rows can bind caller selection on the existing dated
Realtime facade without borrowing Responses semantics or requiring live media?

Yes, for session-scoped dispatch on the fixed model. All five Realtime effort
values are admitted and now bound at exact opaque facade
`openai-realtime-reasoning-2026-08-27` with private behavior
`openai.realtime-manual-pcm-reasoning-v2`. Historical
`openai-realtime-2026-07-22` / `openai.realtime-manual-pcm-v1` remains
superseded proof. Provider acceptance beyond matching `session.updated`
acknowledgement, effective reasoning depth, and observed reasoning remain
unclaimed.

## Method And Evidence Boundary

Official OpenAI Realtime documentation and API reference were fetched on
2026-08-27. The review used only public, secret-free documentation plus exact
repository source and fixtures. It did not authenticate, inspect an account, key,
endpoint, WebSocket, device, paid request, or mutate provider state. No live
Realtime call was made.

The fetched markdown specimens and SHA-256 digests are:

| Surface | URL | Review date | Specimen SHA-256 |
| --- | --- | --- | --- |
| Exact model page | <https://developers.openai.com/api/docs/models/gpt-realtime-2.1> | 2026-08-27 | `d4334816945e326392cbea854ecd481102166e5a5686c201ddc6a7a7dbf826da` |
| Realtime guide | <https://developers.openai.com/api/docs/guides/realtime> | 2026-08-27 | `b8d3363a7f351f52883b23a855d85c0d1e5cc52d547b5db4b644a979eae98109` |
| Realtime prompting guide | <https://developers.openai.com/api/docs/guides/realtime-models-prompting> | 2026-08-27 | `d4f890ba51c595530adab90a835ea7ef453124b3bbe6ea9a9ea798ce5b573921` |
| Realtime client events reference | <https://developers.openai.com/api/reference/resources/realtime/client-events> | 2026-08-27 | `605aa7022bb4bd0fe6680de790c99e7988b4b513b604776eb4e47bb3aba0645e` |
| Realtime server events reference | <https://developers.openai.com/api/reference/resources/realtime/server-events> | 2026-08-27 | `0258c7367e033d1a8a9a6e224c7ac70047a93f9201cde84a0e0baaccb7fca323` |

Review dates are the worker retrieval dates. The digests identify the complete
fetched markdown response. They are provenance for the dated review; they do
not turn a mutable provider page into a permanent compatibility guarantee.

## Frozen Official Findings

### The fixed model is reasoning-capable on Realtime

The exact model page for `gpt-realtime-2.1` states:

> Reasoning model with tool use

> GPT-Realtime-2.1 supports speech-to-speech interactions with configurable
> reasoning effort, instruction following, and tool use for complex voice-agent
> workflows.

The same page lists **Reasoning token support** and says:

> GPT-Realtime-2.1 supports configurable reasoning effort. Higher reasoning
> effort can increase latency and output token usage.

Its endpoint table admits only Realtime (`v1/realtime`) for this model.
Responses and Chat Completions are not supported on this model page. Shared
OpenAI catalogue or Background Responses evidence therefore cannot substitute
for Realtime transport proof.

The Realtime guide names the same model for low-latency voice agents and says:

> Realtime 2 adds reasoning to speech-to-speech workflows. Start with
> `reasoning.effort` set to `low` for most production voice agents, then adjust
> based on latency tolerance and task complexity.

The prompting guide closes the candidate effort set for Realtime 2 models:

| Effort | Prompting-guide role |
| --- | --- |
| `minimal` | Lowest latency, simple tasks |
| `low` | Recommended production default |
| `medium` | Multi-step tasks |
| `high` | High-precision workflows |
| `xhigh` | Critical planning or triage |

The guide text names `gpt-realtime-2`; the model page and Realtime schema
model enum both include exact `gpt-realtime-2.1`. Model-specific membership is
therefore closed on `gpt-realtime-2.1`, not inferred from Background or
Responses models.

### The wire field exists on the exact Realtime surface

The GA Realtime client reference binds `session.update` to
`RealtimeSessionCreateRequest`, whose members include optional `reasoning` with
type `RealtimeReasoning`. That object carries optional `effort` with closed enum
`RealtimeReasoningEffort`:

> `minimal`, `low`, `medium`, `high`, `xhigh`

The reference docstring names reasoning-capable Realtime models such as
`gpt-realtime-2`. The schema model enum on the same surface includes exact
`gpt-realtime-2.1`.

The same `RealtimeReasoning` object appears on `response.create` through
`RealtimeResponseCreateParams.reasoning`. The client reference is exact:

> The `response.create` event includes inference configuration like
> `instructions` and `tools`. If these are set, they will override the
> Session's configuration for this Response only.

So Realtime exposes two distinct dispatch timings:

1. session-scoped via `session.update.session.reasoning.effort`
2. per-response override via `response.create.response.reasoning.effort`

These are Realtime transport fields. They are not Responses request-body
semantics even though both use `reasoning.effort` spelling.

### Session acknowledgement is bounded and distinct from usage

The client reference states that after `session.update` the server responds
with `session.updated` showing the full, effective configuration. The server
events reference types that event's `session` object as
`RealtimeSessionCreateResponse`, which includes optional `reasoning` with the
same `RealtimeReasoning` shape.

That gives a bounded static confirmation story for session-scoped dispatch:
the provider contract says the effective session object, including
`reasoning.effort` when present, is returned on `session.updated`. This lane
does not claim live provider behavior from that contract alone.

`response.done.usage` on Realtime exposes `input_token_details` and
`output_token_details` with audio and text subdivisions only. The reference
does not expose a Realtime `reasoning_tokens` field or a selected-effort echo on
`response.done`. Reasoning-token presence therefore cannot confirm the caller's
selected effort on this route.

Observed reasoning in audio or transcript output is likewise out of scope for
dispatch proof and remains unclaimed.

### Mid-session mutation is provider-permitted but route-withheld

The client reference allows `session.update` at any time for any field except
`voice` and `model`. Reasoning is not excluded. A future implementation could
therefore change effort mid-session at the provider layer.

The current Swallowtail dated route fixes generation controls at preparation and
encodes them once during post-connect session configuration. Immutability across
the open session is a route-side guarantee this evidence lane recommends for
implementation; it is not provider-enforced.

## Frozen Route Evidence

Exact current repository truth at worker/dispatch base
`0808a6cff4f48895e700f82b89f0d267d3f47c25`. Handoff planning base:
`59c8238623dfdda61a87c7147b5240d87d611ebb`.

| Item | Exact current value | Source |
| --- | --- | --- |
| Route | `openai.realtime` | `src/realtime.rs` |
| Driver | `swallowtail.openai.realtime` | `src/realtime.rs` |
| Model | `gpt-realtime-2.1` | `src/realtime_selection.rs` |
| Facade point | `openai-realtime-reasoning-2026-08-27` (current); `openai-realtime-2026-07-22` superseded | `src/realtime_selection.rs` |
| Access | public API-key, `api.openai.com` | `src/realtime_selection.rs` |
| Prepared profile | manual mono PCM16 24 kHz, two turns, no rollover | `docs/guides/realtime-prepared-integration.md` |
| Session encoder | `ClientEvent::SessionUpdate` with optional `reasoning.effort` and optional `max_output_tokens` | `src/realtime_protocol/client.rs` |
| Response encoder | bare `response.create` with no override params | `src/realtime_protocol/client.rs` |
| Preflight | admits Research 236's five exact values; rejects every other | `src/prepared_realtime_profile/session.rs`, `src/realtime.rs` |
| Deterministic acknowledgement | matching `session.updated.session.reasoning.effort` required for explicit selection | `src/realtime/session.rs` |
| Dated fixture README | historical no-reasoning corpus retained; reasoning fixtures under `openai-realtime-reasoning-effort-2026-08-27` | fixture READMEs |

Research 049 classified `openai.realtime` reasoning as `U` against the realized
adapter. That disposition remains true for current production. This research
promotes a future deliver-now subset without changing production code.

## Exact Disposition

### Current production

| Item | Disposition |
| --- | --- |
| caller reasoning selection on `openai-realtime-reasoning-2026-08-27` for the five Research 236 values | Deliver now; matching `session.updated` acknowledgement required |
| any other portable reasoning value on the current facade | Reject before access, credential, connection, or media work |
| any caller reasoning selection on superseded `openai-realtime-2026-07-22` | Reject; historical point is no longer executable |
| omission | Deliver now; current session bytes and lifecycle unchanged |
| Responses reasoning vocabulary imported as Realtime proof | Reject; wrong transport |

### Future session-scoped binding on the fixed model

Each row was classified independently against the Realtime reference enum, the
exact model page, and the prompting guide. Portable values use Swallowtail
`ReasoningMode` spelling; wire values are exact Realtime strings.

| Portable `ReasoningMode` | Wire `reasoning.effort` | Operation | Evidence | Disposition |
| --- | --- | --- | --- | --- |
| `minimal` | `minimal` | `session.update` after connect configure | Realtime enum member 0; prompting guide; not in Background vocabulary | Deliver now |
| `low` | `low` | `session.update` after connect configure | enum; Realtime guide default recommendation | Deliver now |
| `medium` | `medium` | `session.update` after connect configure | enum; prompting guide | Deliver now |
| `high` | `high` | `session.update` after connect configure | enum; prompting guide | Deliver now |
| `xhigh` | `xhigh` | `session.update` after connect configure | enum; prompting guide | Deliver now |
| `none` | — | — | Background-only value; absent from Realtime enum | Reject before effects |
| `max` | — | — | Background-only value; absent from Realtime enum | Reject before effects |
| `off`, `default`, `on`, `auto`, numeric budget, alias, or casing variant | — | — | not exact Realtime vocabulary | Reject before effects |
| any value on a model other than `gpt-realtime-2.1` | — | — | route fixes one exact model | Reject before effects |

Per-response override rows are documented but withheld from deliver-now for
the current manual PCM route shape because the encoder emits bare
`response.create` and the prepared profile exposes no per-turn reasoning control.
They remain implementation leads, not current claims:

| Operation | Meaning | Disposition |
| --- | --- | --- |
| `response.create.response.reasoning.effort` | override session effort for one response | Withheld; alternate timing only |

### Lifecycle table for the fixed route shape

| Lifecycle phase | Session-scoped selection | Per-response override | Current production | Confirmation available statically |
| --- | --- | --- | --- | --- |
| open / post-connect configure | encode on first `session.update` | not used | no reasoning sent | `session.updated.session.reasoning.effort` contract |
| each manual response | inherited from session | possible at provider layer | bare `response.create` | none for selected effort |
| response cancellation | unchanged | unchanged | native cancel unchanged | none for effort |
| disconnect / invalidation | session lost | n/a | unchanged | n/a |
| fresh working-state restoration | re-encode from prepared request on new session | not used | no reasoning sent | same `session.updated` contract |

### Truth separation

| Kind | Meaning on this route | Claim status |
| --- | --- | --- |
| requested | caller-selected portable `ReasoningMode` | future implementation only |
| session-encoded | `session.update.session.reasoning.effort` | future implementation only |
| response-encoded | per-response override field | withheld operation timing |
| accepted | provider accepts the session update without error | bounded by `session.updated` contract; live acceptance unclaimed |
| effective | depth actually used by the model | unclaimed |
| returned | provider echoes effort on `session.updated` | bounded static contract only |
| token usage | `response.done.usage` totals and audio/text details | observable if present; not selected-effort proof |
| observed reasoning | audible or transcript reasoning | unclaimed |

### Omission boundary

| State | Capability claimed | Session update bytes |
| --- | --- | --- |
| no caller selection | none | current dated fixture bytes; no `reasoning` member |
| explicit `minimal`..=`xhigh` | future `ReasoningSelection` + exact `ReasoningMode` | adds `"reasoning":{"effort":"<value>"}` beside unchanged PCM profile |

Omission must keep the exact current session bytes, output-maximum behavior,
connection lifecycle, and fresh-restoration behavior. Nothing may infer
reasoning capability from omission.

### Facade recommendation for implementation

Implementation belongs in a later binding card, not this evidence lane. When
bound, the behavior should follow the Gemini Live pattern used in g04.046:

- retain `openai-realtime-2026-07-22` and its proof verbatim as the superseded
  historical point
- mint one new exact opaque facade point and adapter-private behavior revision
  for reasoning-capable session dispatch
- reject every unsupported portable value before endpoint, credential, or socket
  work

## Frozen Local Evidence

New OpenAI-local fixtures at
`crates/swallowtail-adapter-openai/tests/fixtures/openai-realtime-reasoning-effort-2026-08-27/`:

| File | Role |
| --- | --- |
| `README.md` | authority, boundary, non-claim |
| `reasoning-effort-session-update.json` | positive session-scoped dispatch specimen |
| `reasoning-effort-session-updated.json` | matching acknowledgement specimen |
| `reasoning-effort-response-create-override.json` | alternate per-response timing specimen |

## Contract Alignment

- Contract 011: provider-neutral reasoning vocabulary stays portable; Realtime
  wire values are route-local.
- Contract 020: catalogue presence is not Realtime reasoning proof.
- Contract 024: Realtime media direct-session boundary unchanged by this
  evidence-only lane.
- Contract 026: dated Realtime route shape unchanged in production.
- Contract 029: a future bind requires a new opaque facade point; no currentness
  move in this lane.
- Contract 037: unsupported values fail before effects.
- Contract 040: session-scoped dispatch only; no effective-depth claim.
- Contract 047: per-route feature evidence stays route-local.
- Contract 052: Responses reasoning evidence is not promoted to Realtime.

Research 049's `U` disposition for current production remains accurate. This
research closes the static evidence gap and admits five future session-scoped
rows on exact `gpt-realtime-2.1`.

## Unresolved Items

- Live provider acceptance and rejection behavior for each effort value on the
  exact manual PCM profile were not probed.
- Effective reasoning depth, preamble behavior, and tool-use interaction with
  effort selection remain unclaimed.
- Per-response override timing is documented but not qualified for the current
  prepared route shape.
- Default effort when omitted is described qualitatively in guides (`low`
  recommended) but not closed as an exact provider default on this route shape;
  omission therefore keeps current bytes and claims no reasoning capability.
