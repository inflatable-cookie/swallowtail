# 368 Claude Agent SDK Usage and Context Evidence

Status: complete; exact SDK declaration evidence with provider-free projection
Owner: Swallowtail worker
Created: 2026-10-07
Task: swallowtail#092
Contracts: 011, 014, 029, 061

## Question

Which usage result does the exact `claude-agent.sdk` package expose, what can
the prepared route report through `ProviderObservation::Usage`, and where can
Desktop get context-window evidence without treating token usage as current
context occupancy?

## Answer

The SDK result's `usage` is a per-turn snapshot for the main agent loop in
streaming-input sessions. It excludes Task subagents, sidechains, and
auxiliary model calls. The separate `modelUsage` map is cumulative across
turns and includes query-pipeline subagents, sidechains, compaction and
workflow calls. Results carry a running total; consumers must read its latest
value rather than sum results. The sidecar therefore projects only the
per-turn result `usage` and records `modelUsage` presence without forwarding
that map.

The prepared route maps the known numeric dimensions directly:

| SDK result `usage` field | Provider-neutral observation |
| --- | --- |
| `input_tokens` | input tokens |
| `output_tokens` | output tokens |
| `cache_read_input_tokens` | cache-read input tokens |
| `cache_creation_input_tokens` | cache-write input tokens |

Input and output must be non-negative JavaScript safe integers. A missing
cache field stays absent (`None`); malformed values fail closed. The sidecar
emits one snapshot at the result boundary. Streaming deltas do not carry
usage, and the adapter does not difference cumulative `modelUsage` or add it
to result snapshots. Cost, rate, quota, context occupancy, and token limits
remain separate evidence.

For context, the exact SDK declares `Query.getContextUsage({detail})`. Its
response names the selected model, total and maximum token counts, raw maximum,
percentage, and category rows. `summary` uses the last response plus local
estimates; `full` obtains token counts per category through the token-count
API. This is the documented context-usage access path for code that holds the
SDK `Query` object. The current Swallowtail sidecar neither invokes nor exposes
it.

`ModelUsage.contextWindow` is a per-model window value in the result's
cumulative `modelUsage` map. It is not current occupancy. `supportedModels()`
returns model `value` and optional `resolvedModel` identifiers but does not
publish a window limit. `getContextUsage()` supplies its selected `model`
identifier directly. A direct SDK consumer can correlate that identifier to
the model catalogue's `value` or `resolvedModel`; it must not assume a
cross-version alias mapping. The prepared facade exposes neither this
context response nor occupancy, so Desktop can rely on per-turn usage only.

## Frozen Source

The official npm `@anthropic-ai/claude-agent-sdk@0.3.284` tarball has SHA-256
`4550e830246026133fc1802a2208dd0f3a785cae1eec83f261d114c33d797771`, matching
Research 367 and its frozen identity. Its `package/sdk.d.ts` has SHA-256
`048ae2e6c796cc2aa3c423afaad59a08972cb48c271ffcc9847d910ff65f61b2`.
Neither the tarball nor the platform binary was executed or installed.

The declaration details used here are:

- `SDKResultSuccess` and `SDKResultError` both require `usage` and
  `modelUsage`. Their field comments distinguish main-loop per-turn usage
  from cumulative query-pipeline totals and describe the respective scope.
- `ModelUsage` declares input/output, cache-read/cache-creation,
  `costUSD`, `contextWindow`, and `maxOutputTokens`, plus optional canonical
  model identity.
- `NonNullableUsage` maps the imported Anthropic `BetaUsage` type. The wrapper
  package declares peer `@anthropic-ai/sdk >=0.93.0`, not one exact peer
  version. The sidecar therefore accepts only the four named numeric fields
  it consumes, validates them at runtime, and does not promise a wider
  `BetaUsage` schema.
- `Query.supportedModels()` returns `ModelInfo[]`; model identifiers are
  `value` and optional `resolvedModel`.
- `Query.getContextUsage()` supports `summary` and `full` and returns the
  current context usage response described above.
- `SDKUsageReport` is declared as an optional sibling property on an
  `SDKAssistantMessage` for synthetic `/usage` output. It is not either
  result variant's `usage` snapshot. The sidecar does not map that experimental
  wrapper message; `usage_report` remains an unknown private-wire event.

The auditable declaration excerpts are retained beside the pinned identity in
[`sdk-declarations.d.ts`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.284/sdk-declarations.d.ts)
and their fixture index. The wrapper imports `BetaUsage` from
`@anthropic-ai/sdk/resources/beta/messages/messages.mjs` and declares
`NonNullableUsage` as its non-null mapped form. The wrapper's peer range is
`>=0.93.0`, not an exact peer version; this evidence does not claim a wider
frozen peer schema than the four runtime-validated fields the sidecar reads.

The frozen Research 367 identity fixtures are unchanged. This record adds no
version claim, model limit, provider observation, or live-provider evidence.
