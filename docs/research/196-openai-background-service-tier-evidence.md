# 196 OpenAI Background Service-Tier Evidence

Status: promoted; dispatch-only deliver-now for explicit `default`
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.049 / 136-138

## Question

Can exact production route `openai.background`, model `gpt-5.6`, and the
current Responses background facade bind an adapter-local service-tier
selection while satisfying Contracts 021, 029, 037, 040, 048, and 049?

## Method And Boundary

Current official OpenAI documentation was retrieved on 2026-08-23. Retrievals
were read-only and used no API key, account, project, catalogue, prompt,
provider request, or paid operation. Digests below identify the fetched
markdown documentation bodies on that date; they are not compatibility
guarantees. Markdown bodies are the documented page form obtained by appending
`.md` to the official URL.

The route is `openai.background`, driver `swallowtail.openai.background`,
exact model route `openai.public.gpt-5.6.background`, model `gpt-5.6`, axis
`openai.responses-background-facade`, current facade point
`openai-responses-background-2026-08-23-service-tier`, private behavior
`openai.responses-background-v3`, and claim
`openai.responses-background-window-1`. The superseded point
`openai-responses-background-2026-08-23` / `openai.responses-background-v2`
is historical proof, not executable. The only candidate is adapter-local
typed selection of Responses `service_tier`.

The adapter implementation and fixtures were inspected for card 136, then
updated for cards 137-138 to bind only the admitted ordinary-run subset. No
live provider operation, credential work, account inspection, project-setting
read, enrollment check, quota lookup, or paid inference was used. The
specimens below are secret-free documentation-shape specimens, not captured
provider responses.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Responses create](https://developers.openai.com/api/reference/cli/resources/responses/methods/create) | create-request `service_tier` field, optional/nullable posture, complete enum listing on the Response object, omission default | 2026-08-23 | `11f6ac52ac177e7b9173a74e151c1504e1635c9664861ba1dceb3292961a5b47` |
| [Responses retrieve](https://developers.openai.com/api/reference/cli/resources/responses/methods/retrieve) | returned/retrieved Response `service_tier` field and the same enum | 2026-08-23 | `7d3504ef4d133845ba5b5172b8b90fe8b995f2888f5c9d825d826d65a1b06f19` |
| [Responses streaming events](https://developers.openai.com/api/reference/resources/responses/streaming-events) | authoritative `ServiceTierResponses` schema, seven-member enum, field docstring, OpenAPI ref `#/components/schemas/ServiceTierResponses` | 2026-08-23 | `24a3657b7df1c8f19dcebb8ebd53a8022f6b65d07f98b6027f2008e6133a5c92` |
| [Background mode](https://developers.openai.com/api/docs/guides/background) | asynchronous create, temporary retention, polling, stream reattachment, cancellation, deletion; no service-tier text | 2026-08-23 | `8e92a07ac95cbd20c8306bbe762fa314c202bf18ef59a51f43a029bb4529a9dd` |
| [GPT-5.6 Sol](https://developers.openai.com/api/docs/models/gpt-5.6-sol) | exact `gpt-5.6` alias, reasoning vocabulary, Responses support, feature list | 2026-08-23 | `40a25ce3dc6924d0f0618d23814044e667cc5f71ed2c702d8512d31060871db4` |
| [Fast mode](https://developers.openai.com/api/docs/guides/fast-mode) | `fast`/`priority` request spellings, GPT-5.6 returned `priority`, project default, ramp-rate downgrade to `default`, billing | 2026-08-23 | `de79774b0a9806df2754ace2da0e2aeabfc9254a8dab2b37d270c18ca1125e66` |
| [Flex processing](https://developers.openai.com/api/docs/guides/flex-processing) | `flex` request value, `gpt-5.6` example, beta/limited availability, `429 Resource Unavailable`, retry-to-omission/`auto` | 2026-08-23 | `b5a4ac92cdd1cfcd31d923c2516a876f4bdf5ef02517932c7fac6608b4ca9cc2` |

Create, retrieve, streaming-events, background, and exact-model markdown
bodies match the 2026-08-22/23 Research 191 corpus. Fast mode and Flex were
not in that corpus. `/api/docs/guides/priority-processing` redirects to Fast
mode. `/api/docs/guides/ultrafast` and `/api/docs/guides/scale-tier` return
404.

## Frozen Official Semantics

### Request and response enum

Create, retrieve, and streaming-events share one Responses enum,
`ServiceTierResponses`. The complete current members are:

`auto`, `default`, `flex`, `scale`, `priority`, `fast`, `ultrafast`

The create CLI summary hides the tail as `"auto" or "default" or "flex" or 4
more`. That prose list is not the schema. The Response object on create and
retrieve enumerates all seven literals. The field is optional and nullable.
Schema default is `auto`. Official text: when not set, behavior is `auto`.

The same field docstring applies to request and returned Response:

- `auto` uses the Project service-tier setting; unless configured otherwise
  the project uses `default`
- `default` is standard pricing and performance for the selected model
- `flex` selects Flex Processing
- Fast mode is requested as `fast` or `priority`; the response shows
  `priority` whether the request used `fast` or `priority`
- `ultrafast` is access-controlled Ultrafast Processing, currently available
  for `gpt-5.6-sol`; a response served through it shows `ultrafast`
- `scale` is a schema member with no field-docstring meaning

When the parameter is set, the response includes `service_tier` for the
processing mode actually used. That returned value may differ from the
request.

### Aliases, defaults, and unknown values

- Omission is the current route bytes. It is not an explicit `auto`. Do not
  serialize `auto` to mimic omission.
- Explicit `auto` is a distinct request that defers to an unobserved project
  setting.
- `fast` is a provider-accepted request spelling. On GPT-5.6 and earlier, the
  canonical returned value is `priority`. A request spelling that resolves to
  another returned value is not an admissible public alias.
- `priority` is the canonical Fast-mode request and GPT-5.6 returned value.
- `default` is both the standard-processing selection and the documented
  Fast-mode ramp-rate downgrade returned value.
- `scale` remains schema-only until a Responses field description defines
  request versus returned meaning.
- Unknown future strings are outside the frozen enum. They are not aliases
  and are not admitted.

### Model applicability

The exact model page aliases `gpt-5.6` to GPT-5.6 Sol and lists Responses,
streaming, structured outputs, and the reasoning set
`none|low|medium|high|xhigh|max`. It does not list service tier, Fast, Flex,
Scale, or Ultrafast as model features.

Independent pages still name this model:

- Fast mode: speed increase for `gpt-5.6-sol`; examples use `gpt-5.6-sol`;
  GPT-5.6 responses return `priority` for `fast` or `priority` requests
- Flex: example uses `gpt-5.6`; Flex is beta with limited model availability
  pointed at the pricing page
- Ultrafast: field docstring says currently available for `gpt-5.6-sol`

Public model-page silence is not a model incompatibility by itself. It is
also not route-level access or composition proof.

### Access

The route's public API-key payg profile proves API-key billing and provider
support. It proves no project setting, Flex enrollment, Fast/Priority
enrollment, Ultrafast entitlement, Scale capacity purchase, quota, or
region/account gate.

- `default` is standard processing. Official docs distinguish it from `auto`.
  Explicit `default` selects standard pricing and performance. Project-default
  uncertainty is not an access blocker for this value. There is no documented
  enrollment gate.
- `auto` requires that unobserved project setting
- Flex may return `429 Resource Unavailable` without charge; official retry
  advice is exponential backoff or dropping back to `auto`/omission
- Fast mode may silently downgrade to `default` and standard rates under a
  ramp-rate limit; availability can be regional; Fast FAQ and account-director
  paths are outside this access profile
- Ultrafast is explicitly access-controlled
- `scale` has no field-level access rule; Fast-mode FAQ treats Scale Tier as
  purchased TPM, separate from Fast

Public field/model documentation is not this caller’s access qualification.

### Requested versus returned truth

These states stay distinct:

| State | Meaning | Proven here? |
| --- | --- | --- |
| omitted | field absent from create bytes | yes; current fixture |
| requested | explicit create `service_tier` | documented; explicit `default` is bound |
| planned / dispatched | adapter prepared/driver/request agreement | yes for explicit `default` on ordinary attached + one in-process reattachment |
| provider-accepted | create accepted the requested value | not proven without a live or fixture response |
| returned | Response `service_tier` | documented; may differ; parser ignores the field |
| effective / billed | processing mode actually used and charged | returned field is the documented actual-mode signal; no billed-cost mapping exists on this route |
| observed | consumer-visible returned tier | current route does not expose it |

Do not treat a request value as effective, billed, or observed. Do not
calculate cost or latency from it. Fast-mode docs give GPT-5.6 Fast pricing
separately from Standard; that is not a Swallowtail billing claim.

### Observation

Current public prepared API exposes portable `ProviderObservation::Usage` and
related portable observations. `ResponseSnapshot` parses `id`, `status`,
`output`, and `usage`. It does not read `service_tier`. Extra JSON fields are
ignored, so a returned tier is neither validated nor failed closed.

Exposing exact returned-tier observation through the current route-local API
would require a new adapter-local public surface or a shared
`ProviderObservation` variant. Both are outside this lane. Dispatch-only
delivery is admitted for explicit `default` on ordinary attached runs and one
in-process reattachment: requested, planned, and dispatched states only. The
parser continues to ignore returned `service_tier` and does not fail closed on
absence or mismatch.

`auto`, `flex`, `priority`, `fast`, `ultrafast`, and `scale` remain withheld.
Safe use of those values requires resolved-tier evidence, access proof, or a
canonical request spelling this route will not alias.

### Lifecycle profiles

Ordinary attached runs keep selected input in process memory. One in-process
reattachment uses the same memory. Explicit `default` is therefore retainable
for those profiles without a shared checkpoint change.

`ProviderRunCheckpoint` stores plan fingerprint, runtime run id, provider
response id, and an opaque sequence cursor. Shared checkpoint fields do not
carry selected or returned service tier. The adapter-owned cursor marks a
selected-tier run as non-reconcilable. Controlled detachment persists only
the shared checkpoint, so selected truth would disappear. Restart
reconciliation would send one `GET /v1/responses/{id}` and could carry a
returned tier, but selected/requested truth is gone unless reconstructed from
project defaults or invented into the shared checkpoint.

Detachment and restart reconciliation therefore cannot retain selected versus
returned tier without a shared checkpoint or contract change. Those profiles
are withheld. Explicit `default` plus `with_active_run_detachment` rejects
before effects. A checkpoint exported from a selected-tier run is rejected by
`prepare_run_reconciliation` before network work. Reconciliation does not
restore a create `service_tier` field.

Background, stream, store, cancel, and delete docs do not mention
`service_tier`. The field sits on the same create/retrieve object this route
already uses. Independent field presence is not admission of withheld values.

### Composition and omission

Current omitted create bytes are the frozen fixture:

`background=true`, `stream=true`, `store=false`, model `gpt-5.6`, positive
`max_output_tokens`, no `service_tier`.

Reasoning `none|low|medium|high|xhigh|max` and absent/selected provider-native
structured output already compose on that omitted path. Official create
documents `service_tier` independently of `reasoning` and `text.format`.
Explicit `default` composes with those same reasoning values and with absent
or selected structured output on ordinary attached runs. Omitted-path bytes
stay unchanged.

### Facade

Admitted explicit `default` changes exact opaque facade behavior. The current
executable point is `openai-responses-background-2026-08-23-service-tier`
with private behavior `openai.responses-background-v3`. The claim id stays
`openai.responses-background-window-1` as one Maintained exact segment. The
prior point `openai-responses-background-2026-08-23` /
`openai.responses-background-v2` is retained as superseded proof and is not
executable. Direct `OpenAiBackgroundDriver::new()` still omits `service_tier`.
Contract 029 currentness is not widened.

## Secret-Free Specimens

These snippets are non-exhaustive and non-live. S2–S6 are composition sketches
and illustrative Response excerpts, not admitted requests or captured provider
bodies. Digests cover the compact JSON shown, with no trailing newline.

### S1 — current omitted create bytes

```json
{"model":"gpt-5.6","input":[{"role":"user","content":[{"type":"input_text","text":"Say hello"}]}],"background":true,"stream":true,"store":false,"max_output_tokens":64}
```

S1 is the compact form of
`crates/swallowtail-adapter-openai/tests/fixtures/openai-responses-2026-07-21/create-request.json`.
It remains the omitted production create shape.

### S2 — explicit `default`

```json
{"model":"gpt-5.6","input":[{"role":"user","content":[{"type":"input_text","text":"<user-input>"}]}],"background":true,"stream":true,"store":false,"max_output_tokens":256,"service_tier":"default"}
```

Admitted as dispatch-only on ordinary attached runs and one in-process
reattachment. Does not claim project access, returned-tier equality, cost, or
latency. The production fixture uses `max_output_tokens: 64` and the same
`service_tier` field.

### S3 — illustrative Fast request alias `fast`

```json
{"model":"gpt-5.6","input":[{"role":"user","content":[{"type":"input_text","text":"<user-input>"}]}],"background":true,"stream":true,"store":false,"max_output_tokens":256,"service_tier":"fast"}
```

Not admitted. `fast` is a request spelling, not a GPT-5.6 returned canonical
value.

### S4 — illustrative retrieved Fast actual-mode excerpt

```json
{"id":"resp_example","status":"completed","service_tier":"priority","output":[{"type":"message","role":"assistant","content":[{"type":"output_text","text":"<assistant-output>"}]}],"usage":{"input_tokens":0,"output_tokens":0,"total_tokens":0}}
```

Non-exhaustive. For GPT-5.6, a `fast` or `priority` request may return
`priority`.

### S5 — illustrative Fast ramp-rate downgrade excerpt

```json
{"id":"resp_example","status":"completed","service_tier":"default","output":[{"type":"message","role":"assistant","content":[{"type":"output_text","text":"<assistant-output>"}]}],"usage":{"input_tokens":0,"output_tokens":0,"total_tokens":0}}
```

Non-exhaustive. Documents returned/requested mismatch. It is not billed-cost
evidence.

### S6 — illustrative Ultrafast returned-value excerpt

```json
{"id":"resp_example","status":"completed","service_tier":"ultrafast","output":[{"type":"message","role":"assistant","content":[{"type":"output_text","text":"<assistant-output>"}]}],"usage":{"input_tokens":0,"output_tokens":0,"total_tokens":0}}
```

Non-exhaustive. Shows the documented returned spelling. It does not prove this
caller can request or receive it.

| Specimen | SHA-256 |
| --- | --- |
| S1 | `1367547907c99320517fe1d6d4de4ca342d4788f9cbf1b095d3e6b3f77d2fc97` |
| S2 | `dba68e237bb3754203028c2f7653c53164e92e58c21e22b3fe786877e708a335` |
| S3 | `1fb7ca358e1dd4f250fd8ae11d11bacf32492c83e207960f2f3481e28c21d969` |
| S4 | `47f87443c19748d72170f0ed92399eb8a74e55bb63e7eb3f974e204102eab01e` |
| S5 | `d7613a781496b49a6bd8553785b9ca50ac60e5032a35c743dc53618e6b8f9cb3` |
| S6 | `e48e044ecc388739f1c7e6399e018475daccbcad1fad27aeebc6d6194dc1c859` |

## Compatibility Classification

| Combination or truth | Disposition | Reason |
| --- | --- | --- |
| Omission / absent `service_tier` | unchanged | Current create fixture and encoder omit the field. Preserve those bytes. |
| Explicit `auto` | withheld | Distinct from omission; depends on unobserved project settings. |
| Explicit `default` on ordinary attached + one in-process reattachment | deliver-now, dispatch-only | Official docs distinguish `default` from `auto`; no enrollment gate; selected input stays in process memory. Claims requested/planned/dispatched only. |
| Explicit `default` with `with_active_run_detachment` | withheld; reject before effects | Shared checkpoint cannot retain selected/returned tier. |
| Restart reconciliation of a selected-tier run | withheld; reject before effects | Adapter-owned cursor marks the checkpoint non-reconcilable; selected truth is not in shared checkpoint fields. |
| `flex` | withheld | Beta/limited availability, possible `429`, official fallback is `auto`/omission, no access proof. |
| `fast` | withheld | Request alias of Fast mode; GPT-5.6 returns `priority`. Not an admissible public alias. |
| `priority` | withheld | Canonical Fast request/returned spelling for GPT-5.6; silent downgrade to `default` is documented; access and observation are missing. |
| `ultrafast` | withheld | Explicitly access-controlled. Public payg profile does not prove entitlement. |
| `scale` | withheld | Schema-only; no field-docstring request/response meaning. |
| Unknown future strings | withheld | Fail closed; not aliases. |
| Reasoning `none\|low\|medium\|high\|xhigh\|max` with omitted or explicit `default` | compose | Existing omitted-path composition remains; explicit `default` composes on ordinary runs. |
| Provider-native structured output with omitted or explicit `default` | compose | Same as reasoning. |
| Account/project access, enrollment, quota, capacity | evidence-gated | No live inspection is authorized or available. |

## Candidate Disposition

Deliver-now rows:

| Value | Profile | Observation | Disposition |
| --- | --- | --- | --- |
| omission | all existing profiles | none | unchanged; no `service_tier` field |
| explicit `default` | ordinary attached + one in-process reattachment | dispatch-only | deliver-now |
| explicit `default` + `with_active_run_detachment` | — | — | reject before effects |
| reconciliation of a selected-tier checkpoint | — | — | reject before network; do not restore selected tier |
| `auto`, `flex`, `priority`, `fast`, `ultrafast`, `scale`, unknown | all | — | withheld / reject at the type boundary |

Contract 040 allows qualified dispatch without claiming acceptance or effective
value. g04.049 allows dispatch-only when those limitations are documented.
Gemini Live recent controls use that posture.

Do not claim accepted, effective, billed, observed returned tier, cost, or
latency. Do not fail closed on missing or mismatched returned `service_tier`.

No new portable contract is required. Adding a portable Fast/speed/priority
capability, a shared generation-control field, or a checkpoint mutation would
exceed this lane.

## Decision

Card 136 admits explicit `default` as dispatch-only deliver-now for ordinary
attached runs and one in-process reattachment. Cards 137 and 138 bind that
subset to facade `openai-responses-background-2026-08-23-service-tier` and
behavior `openai.responses-background-v3`. Detachment, reconciliation, `auto`,
`flex`, `priority`, `fast`, `ultrafast`, and `scale` stay withheld.
