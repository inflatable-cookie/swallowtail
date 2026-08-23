# 196 OpenAI Background Service-Tier Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.049 / 136

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
`openai-responses-background-2026-08-23`, private behavior
`openai.responses-background-v2`, and claim
`openai.responses-background-window-1`. The only candidate is adapter-local
typed selection of Responses `service_tier`.

The adapter implementation and fixtures were inspected but not changed. No
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

- `default` is described as standard processing, but this caller’s project
  default is unobserved
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
| requested | explicit create `service_tier` | documented |
| planned / dispatched | adapter prepared/driver/request agreement | not present today |
| provider-accepted | create accepted the requested value | not proven without a live or fixture response |
| returned | Response `service_tier` | documented; may differ |
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
delivery is allowed only for values whose safe use does not need resolved-tier
evidence.

Every explicit value is in the official “parameter set, returned value may
differ” regime. Fast mode documents silent downgrade to `default` with
standard billing. Flex and Ultrafast change cost, capacity, or access. Safe
use of those values requires resolved-tier evidence. Dispatch-only is
withheld.

### Lifecycle profiles

Ordinary attached runs keep selected input in process memory. One in-process
reattachment uses the same memory. Current create fixtures omit `service_tier`;
adding a value would change those bytes.

`ProviderRunCheckpoint` stores plan fingerprint, runtime run id, provider
response id, and an opaque sequence cursor. It does not store a selected or
returned service tier. Controlled detachment persists only that checkpoint.
Restart reconciliation sends one `GET /v1/responses/{id}`. Retrieve can carry
a returned tier, but selected/requested truth is gone unless reconstructed
from project defaults or invented into the shared checkpoint.

Detachment and reconciliation therefore cannot retain selected versus returned
tier without a shared checkpoint or contract change. Those profiles are
withheld. A narrower ordinary-run subset may proceed only when the other
gates also pass. They do not.

Background, stream, store, cancel, and delete docs do not mention
`service_tier`. The field sits on the same create/retrieve object this route
already uses. Independent field presence is not admission.

### Composition and omission

Current omitted create bytes are the frozen fixture:

`background=true`, `stream=true`, `store=false`, model `gpt-5.6`, positive
`max_output_tokens`, no `service_tier`.

Reasoning `none|low|medium|high|xhigh|max` and absent/selected provider-native
structured output already compose on that omitted path. Official create
documents `service_tier` independently of `reasoning` and `text.format`. There
is no composed official specimen for every admitted reasoning value plus
structured output plus each enum member plus this route’s lifecycle. Because
no value is deliver-now, that composition is not a binding gap; it remains a
reopen requirement.

### Facade

Any admitted value would change exact opaque facade behavior and would need a
new facade point, private behavior revision, and model-route revision while
retaining `openai-responses-background-2026-08-23` /
`openai.responses-background-v2` as superseded proof. No value is admitted, so
the current point, private behavior, claim id, and model-route identity stay
unchanged. Contract 029 currentness is not widened.

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
It is the current production create shape.

### S2 — illustrative explicit `default`

```json
{"model":"gpt-5.6","input":[{"role":"user","content":[{"type":"input_text","text":"<user-input>"}]}],"background":true,"stream":true,"store":false,"max_output_tokens":256,"service_tier":"default"}
```

Not admitted. Shows the extra create field without claiming project access or
returned-tier equality.

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
| Explicit `default` | withheld | Standard processing is documented, but returned mode may differ, this caller’s project default is unobserved, and the current route cannot expose resolved-tier evidence. |
| `flex` | withheld | Beta/limited availability, possible `429`, official fallback is `auto`/omission, no access proof. |
| `fast` | withheld | Request alias of Fast mode; GPT-5.6 returns `priority`. Not an admissible public alias. |
| `priority` | withheld | Canonical Fast request/returned spelling for GPT-5.6; silent downgrade to `default` is documented; access and observation are missing. |
| `ultrafast` | withheld | Explicitly access-controlled. Public payg profile does not prove entitlement. |
| `scale` | withheld | Schema-only; no field-docstring request/response meaning. |
| Unknown future strings | withheld | Fail closed; not aliases. |
| Ordinary attached run with any explicit value | withheld | Observation and access gates fail. |
| One in-process reattachment with any explicit value | withheld | Same as ordinary; no admitted selection to retain in memory. |
| Controlled detachment | withheld | Checkpoint cannot retain selected/returned tier without a shared change. |
| Restart reconciliation | withheld | Retrieve can carry returned tier; selected truth is not in the checkpoint. |
| Reasoning `none\|low\|medium\|high\|xhigh\|max` with omitted tier | unchanged | Existing omitted-path composition remains the claim. |
| Provider-native structured output with omitted tier | unchanged | Existing omitted-path composition remains the claim. |
| Account/project access, enrollment, quota, capacity | evidence-gated | No live inspection is authorized or available. |

## Candidate Disposition

Deliver-now rows: none.

The candidate is an evidence stop, not deliver-now. Official docs freeze the
complete current enum, omission-as-`auto`, Fast aliasing, Ultrafast access
control, and requested-versus-returned drift. They do not establish this
route’s access, a current-API observation path, or durable selected/returned
truth across detachment and reconciliation.

No new portable contract is required to explain the stop. Adding a portable
Fast/speed/priority capability, a shared generation-control field, or a
checkpoint mutation would exceed this lane. The missing proof is access,
observation, and lifecycle retention, not a reason to widen Contracts 021,
029, 037, 040, 048, or 049.

The current opaque facade point
`openai-responses-background-2026-08-23` remains unchanged. No private
behavior revision is assigned because no additive behavior has been admitted.
The adapter runtime, omitted create bytes, reasoning, structured output,
retention, reattachment, cancellation, deletion, detachment, and
reconciliation behavior stay as they are.

## Decision

Card 136 is complete as an evidence stop. Cards 137 and 138 are blocked and
were not executed. No service-tier selection, request field, or returned-tier
parser is admitted.
