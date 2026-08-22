# 191 OpenAI Background Web Search Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-22
Updated: 2026-08-23
Card: g04.043 / 119

## Question

Can exact `gpt-5.6` on the qualified `openai.background` Responses route bind
one bounded provider-owned `web_search` tool through portable
`ExternalSearchPolicy::Enabled` while preserving background, streaming,
retention, reattachment, retrieval, generation-control, and activity truth?

## Method And Boundary

Current official OpenAI documentation was initially retrieved on 2026-08-22;
the standard streaming-events and exact model pages were rechecked on
2026-08-23. The retrievals were read-only and used no API key, account,
project, catalogue, prompt, provider request, or paid operation. The source
digests below identify the fetched documentation bodies on those dates; they
are not compatibility guarantees.

The route is `openai.background`, driver `swallowtail.openai.background`,
exact model route `openai.public.gpt-5.6.background`, model `gpt-5.6`, axis
`openai.responses-background-facade`, and current facade point
`openai-responses-background-2026-07-21`. The only candidate is the
provider-owned Responses `web_search` tool selected through portable
`ExternalSearchPolicy::Enabled`.

The adapter implementation and fixtures were inspected but not changed. No
live provider operation, credential work, account inspection, installation,
provider prompt, or external search was used. The specimens below are
secret-free documentation-shape specimens, not captured provider responses.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Web search guide](https://developers.openai.com/api/docs/guides/tools-web-search) | `web_search`, legacy `web_search_preview`, external access, tool choice, source inclusion, search event and output descriptions | 2026-08-22 | `e78e836f7c1aca4c70ec7fe85f45c5807f466c05c5d77d69c548a464b1d5447c` |
| [Responses create](https://developers.openai.com/api/reference/cli/resources/responses/methods/create) | `tools`, `tool_choice`, `max_tool_calls`, `include`, reasoning, structured output, streaming request fields | 2026-08-22 | `11f6ac52ac177e7b9173a74e151c1504e1635c9664861ba1dceb3292961a5b47` |
| [Responses retrieve](https://developers.openai.com/api/reference/cli/resources/responses/methods/retrieve) | retained response retrieval, `include`, and stream cursor fields | 2026-08-22 | `7d3504ef4d133845ba5b5172b8b90fe8b995f2888f5c9d825d826d65a1b06f19` |
| [Background mode](https://developers.openai.com/api/docs/guides/background) | asynchronous execution, temporary retention, polling, stream reattachment, cancellation, and deletion | 2026-08-22 | `8e92a07ac95cbd20c8306bbe762fa314c202bf18ef59a51f43a029bb4529a9dd` |
| [GPT-5.6 model page](https://developers.openai.com/api/docs/models/gpt-5.6-sol) | exact alias, reasoning, streaming, structured output, and web-search support | 2026-08-23 | `40a25ce3dc6924d0f0618d23814044e667cc5f71ed2c702d8512d31060871db4` |
| [Responses streaming events](https://developers.openai.com/api/reference/resources/responses/streaming-events) | standard Responses web-search lifecycle event names and shapes | 2026-08-23 | `24a3657b7df1c8f19dcebb8ebd53a8022f6b65d07f98b6027f2008e6133a5c92` |

## Frozen Official Semantics

### Tool, model, network, and bound

- New integrations use hosted `web_search`. `web_search_preview` remains a
  legacy Responses surface and is withheld from this lane.
- The exact model page aliases `gpt-5.6` to GPT-5.6 Sol and lists streaming,
  structured outputs, and web search as supported features. It lists reasoning
  efforts `none`, `low`, `medium`, `high`, `xhigh`, and `max`; it does not list
  `minimal`. This proves model feature support individually, not the full
  background composition.
- `external_web_access` is explicit provider search posture: `true` permits
  live web access; `false` restricts the search to cached or indexed content.
  Omitting it leaves a provider default, so a future binding would have to
  choose a value explicitly. No account or project policy was inspected.
- `max_tool_calls` is a maximum total count across built-in tools, not a
  per-tool limit. A fixed `1` is a positive bounded candidate. The docs do not
  prove that this candidate composes with this route's complete background
  request and lifecycle.
- `tool_choice: "auto"` leaves search optional. `required` or a specific
  built-in-tool choice can force a call, but neither is selected here because
  enablement must remain distinct from invocation.
- `include: ["web_search_call.action.sources"]` requests search-source
  evidence. It does not prove that the model searched, that a source became an
  assistant citation, or that a search was billed.

### Background and lifecycle

The background guide independently documents asynchronous creation, polling
through queued and in-progress states, temporary retention when `store=false`,
and cancellation/deletion operations. The retrieve surface documents
continuing a stream with `starting_after`, and background streaming requires
the original operation to have been created with `stream=true`.

Those facts preserve the existing search-free route claim. They do not prove
that a background stream carries web-search lifecycle items in the same
sequence as assistant output, that a reattached cursor resumes a search item
without loss, or that a retrieved terminal response preserves source and
usage truth. The exact combined cases therefore remain evidence-gated.

### Events, output, usage, billing, and failure

The standard Responses streaming-events reference names
`response.web_search_call.in_progress`, `response.web_search_call.searching`,
and `response.web_search_call.completed` event shapes. It also describes a
`web_search_call` output item distinct from an assistant message, with source
inclusion under the search action. Response usage is a separate structured
object.

This is enough to identify the provider concepts that must stay separate:
selection, invocation, searching, completed search, sources, assistant output,
usage, and terminal status. It is not enough to freeze the exact composed
event sequence, source-item payload, failure/rejection shape, or usage mapping
for this background route. The documentation can expose that search tools may
carry tool-call billing, but it provides no route-level billed-use evidence.
No billing claim follows.

The current adapter has no search item or search event parser, its request
builder has no provider-tool fields, its driver rejects consumer tools and
external network enablement, and its response snapshot has no search/source
representation. Unknown search events would fail the current strict SSE
parser. That gap is a compatibility finding, not permission to invent a
provider payload or portable activity event.

### Reasoning and structured output

The exact model page lists `none`, `low`, `medium`, `high`, `xhigh`, and `max`,
but not `minimal`. The current route guide
(`docs/guides/openai-background-prepared-integration.md`) and preparation
validator (`crates/swallowtail-adapter-openai/src/prepared_profile/background.rs`)
admit `minimal`. That is an inherited exact-model mismatch, not a search
mapping. It requires the named follow-up below and is not silently treated as
unchanged reasoning truth.

The create reference documents reasoning and structured output separately from
built-in tools. No official specimen or exact response contract was found for
every required combination of `web_search` with `background=true`,
`stream=true`, `store=false`, each model-listed reasoning effort,
provider-native structured output, source inclusion, one reattachment,
retrieve, cancel, delete, detachment, and restart reconciliation.

The absence of a composed specimen is binding. Independent field support is
not promoted to route support.

## Secret-Free Specimens

These secret-free snippets are non-exhaustive illustrative excerpts and
composition sketches. They are intentionally non-live and use placeholders;
S4 and S5 are not schema-complete retrieved or failure Response specimens.
They are not captured provider responses or normative request contracts. Their
digests cover the compact JSON shown, with no trailing newline.

### S1 — candidate create shape

```json
{"model":"gpt-5.6","input":[{"role":"user","content":[{"type":"input_text","text":"<user-input>"}]}],"background":true,"stream":true,"store":false,"max_output_tokens":256,"tools":[{"type":"web_search","external_web_access":true}],"max_tool_calls":1,"tool_choice":"auto","include":["web_search_call.action.sources"]}
```

This is a composition specimen, not an admitted request. It demonstrates the
candidate's exact tool, explicit live-access posture, positive total-call
bound, optional choice, and source inclusion without adding a public tool map.

### S2 — standard documented search-progress excerpt

```json
{"type":"response.web_search_call.searching","output_index":0,"item_id":"ws_123","sequence_number":0}
```

### S3 — documented completed-search item shape

```json
{"type":"web_search_call","id":"ws_example","status":"completed","action":{"type":"search","query":"<omitted>","sources":[]}}
```

The query placeholder is never a diagnostic or provider payload. Search query,
source contents, and raw provider bodies remain excluded from the portable
activity surface.

### S4 — illustrative retrieved-terminal excerpt

```json
{"id":"resp_example","status":"completed","output":[{"type":"web_search_call","id":"ws_example","status":"completed","action":{"type":"search","query":"<omitted>","sources":[]}},{"type":"message","role":"assistant","content":[{"type":"output_text","text":"<assistant-output>"}]}],"usage":{"input_tokens":0,"output_tokens":0,"total_tokens":0}}
```

S4 is a non-exhaustive illustrative excerpt. It omits other Response fields
and does not claim to be a schema-complete retrieved response.

### S5 — illustrative failure/usage excerpt

```json
{"id":"resp_example","status":"failed","error":{"code":"<provider-error>"},"usage":{"input_tokens":0,"output_tokens":0,"total_tokens":0}}
```

S5 is a non-exhaustive illustrative excerpt. It omits other Response and
failure fields and does not claim to be a schema-complete provider failure.

| Specimen | SHA-256 |
| --- | --- |
| S1 | `97780cbbd6d743ddda75d173b390664816c0e160053ef242af32ea933d52f317` |
| S2 | `441984c8a93e92e7f4dc08cab3afc781f0a248903e9df0c34ff99af94c26bf67` |
| S3 | `e363adfcc8b0cb07780133560d7c9bde6ab8050e5e52647fab345b5619df16f7` |
| S4 | `2e9d53579140151b84cc22815c63648ef93aafd8a2c23a5bdb5d45e98030065e` |
| S5 | `2ed9eea3a994a10d0d42b8b42338e01e2ddec9584e44e9a4993b817377f019b0` |

## Compatibility Classification

| Combination or truth | Disposition | Reason |
| --- | --- | --- |
| `gpt-5.6` plus `web_search`, base text output | evidence-gated | Individual fields are documented, but exact background composition, event ordering, source representation, and account policy are not proven. |
| `reasoning` absent, `none`, `low`, `medium`, `high`, `xhigh`, or `max` with search | evidence-gated | The model and current facade classify reasoning separately; no exact search/background composition is documented for the model-listed vocabulary. |
| `minimal` with search, and the inherited `minimal` reasoning claim without search | named follow-up; not admitted | The exact GPT-5.6 model page omits `minimal`, while the current guide and preparation validator admit it. The search lane does not resolve that production-claim mismatch. |
| Provider-native structured output with search | evidence-gated | Structured output and search are individually supported; their combined stream, retrieved terminal, and source truth are not frozen. |
| Reasoning plus structured output plus search | evidence-gated | No exact composed request and lifecycle proof. |
| `background=true`, `stream=true`, `store=false`, one reattachment, retrieve, cancel, delete, detachment, and restart reconciliation with search | evidence-gated | The search-free lifecycle remains qualified; search-item continuity and terminal truth are not. |
| Search absent, tool-free request path | unchanged for this lane | The current tool-free request bytes and behavior remain the search-free claim. The inherited `minimal` reasoning mismatch is tracked separately. |
| Account/project access and provider policy | evidence-gated | No live account or project inspection is authorized or available. Public model support is not access qualification. |
| `web_search_preview` | withheld | Legacy surface; not the selected new-integration tool. |
| Arbitrary Responses tools, filters, location, context size, return-token budget, image search, MCP, functions, or sibling routes | withheld or not applicable | Outside the named candidate and route boundary. |

## Candidate Disposition

Deliver-now rows: none.

The candidate is evidence-gated, not deliver-now. The evidence does establish
that a positive total-call bound and a typed provider-owned tool are available
in the public Responses vocabulary. It does not establish the exact composed
request, stream/retrieve lifecycle, activity mapping, account policy, or
facade behavior required by Contracts 029, 041, and 044.

No new portable contract is needed to explain the stop: the existing
`ExternalSearchPolicy`, `ExternalSearch` capability, and provider-network
vocabulary already express the intended boundary. The missing proof is
provider composition and route implementation, not a reason to widen those
contracts.

The current opaque facade point
`openai-responses-background-2026-07-21` remains unchanged. No private
behavior revision is assigned because no additive behavior has been admitted.
The search lane does not change the adapter runtime. It retains the current
tool-free request, structured-output, retention, reattachment, cancellation,
deletion, detachment, and reconciliation behavior. It does not certify the
inherited `minimal` reasoning claim, which conflicts with the exact model page.

## Named Follow-Up

`g04.043-R1 — reconcile OpenAI background reasoning vocabulary` is promoted
into g04.044 cards 122-123. They must reconcile the official GPT-5.6 effort
list with the prepared guide, `validate_reasoning`, route-local tests, and the
production facade claim. This research record supplies the evidence but does
not itself alter the guide, preparation code, matrices, or shared claim.

## Decision

Card 119 is complete as an evidence stop. Cards 120 and 121 are blocked and
were not executed. The adapter, fixtures, prepared integration guide, public
API baseline, shared architecture, matrices, and changelog remain unchanged.
After PR 42 merged, the orchestrator promoted the inherited reasoning mismatch
into g04.044 rather than absorbing it into search implementation.
