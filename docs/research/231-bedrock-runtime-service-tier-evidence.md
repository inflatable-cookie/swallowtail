# 231 Bedrock Runtime Service-Tier Evidence

Status: promoted; empty deliver-now set
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.082 / 230

## Question

Which exact `bedrock.runtime` `ConverseStream` `performanceConfig.latency` or
`serviceTier` rows can be selected without flattening account-, region-,
capacity-, or model-dependent behavior into a generic Fast control?

## Method And Boundary

Current official AWS API and user-guide pages plus generated
`aws-sdk-bedrockruntime` sources were retrieved on 2026-08-27. Retrievals were
read-only and used no AWS credential, account, region entitlement, catalogue
inspection, provider request, or paid operation. Digests below identify the
fetched HTML bodies on that date; they are not compatibility guarantees.

The route is `bedrock.runtime`, driver `swallowtail.amazon-bedrock.direct`,
operation `ConverseStream`, axis `amazon-bedrock.runtime-rust-sdk`, current
facade point `bedrock-converse-stream`, and private behavior
`bedrock.runtime-text-v1`. The prepared route binds one exact model id, one
exact region, delegated cloud identity, and one positive output-token bound per
attempt. It does not fix a single production model or region at route scope.

The adapter implementation, fixtures, and guide were inspected without
changing production surfaces. Frozen corpus:
`crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-runtime-service-tier-evidence/`.

No live AWS operation, credential work, account inspection, quota lookup,
model-card enumeration, or paid inference was used. The JSON specimens below
are secret-free documentation-shape specimens, not captured provider responses.

## SDK Version Reconciliation

| Surface | Claimed constant | Cargo lock | Tier-field delta |
| --- | --- | --- | --- |
| Runtime SDK | `SDK_VERSION = 1.136.0` | `aws-sdk-bedrockruntime = 1.139.0` | none for audited fields |

Research 127 and 159 already record this mismatch. For card 230 the decisive
evidence point is the locked dependency `1.139.0` because that is what the
adapter links. The public constant, fixture protocol
`bedrock-runtime-1.136.0`, and guide still claim `1.136.0`. This lane records
both and changes neither.

Byte-identical between `1.136.0` and `1.139.0` for the audited generated
sources:

- `PerformanceConfigLatency`: `standard`, `optimized`
- `ServiceTierType`: `default`, `flex`, `priority`, `reserved`
- `ConverseStream` request serialization keys `performanceConfig` and
  `serviceTier`
- `ConverseStreamMetadataEvent` response keys `performanceConfig`,
  `serviceTier`, and `metrics.latencyMs`

The mismatch therefore does not force an empty set. Model, region, inference
profile, entitlement, capacity, and returned-state dependence do.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [ConverseStream API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ConverseStream.html) | request/response field names, optional posture, metadata stream shape | 2026-08-27 | `73af6d8dc1d4008b6b63dcda8139f7bffbaa810aac9e03b6ea2a266dcb8ef193` |
| [PerformanceConfiguration API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_PerformanceConfiguration.html) | `latency` enum `standard \| optimized` | 2026-08-27 | `ead55e16ec0409aea15ea6ed8d1d5ad806cb52bc579a974f80c8c67adef38911` |
| [ServiceTier API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ServiceTier.html) | nested `type` enum `priority \| default \| flex \| reserved` | 2026-08-27 | `20c74518430f1e71ca6469cc2b83ef5550d781cb6f6707b46dddceb73fa95aa4` |
| [Latency optimized inference](https://docs.aws.amazon.com/bedrock/latest/userguide/latency-optimized-inference.html) | preview feature, quota fallback, model/region/profile table | 2026-08-27 | `74c540b2fcae5321c9d1ed500e28a8277bdce963f12ba663d6dea7a689baf05c` |
| [Service tiers](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html) | tier semantics, omission default, quota sharing, returned tier visibility | 2026-08-27 | `ef7b8b4246519bb312a8814fd8dbba62a611c63ddeee9797492c82b59f23e0a2` |
| [Use the Converse API](https://docs.aws.amazon.com/bedrock/latest/userguide/conversation-inference.html) | Converse `serviceTier` object shape and returned-tier note | 2026-08-27 | `7777d2764f6022fa5e121666c958e8c2aa1565b28f54668e06744ec74b4c054b` |

## Frozen Official Semantics

### Request fields on `ConverseStream`

Both fields are optional on the request body.

`performanceConfig` is an object with one documented member:

- `latency`: `standard` or `optimized`
- official text: set `optimized` to use a latency-optimized model version

`serviceTier` is an object with one required member when present:

- `type`: `priority`, `default`, `flex`, or `reserved`

Official Converse user-guide example:

```json
"serviceTier": {
  "type": "reserved" | "priority" | "default" | "flex"
}
```

InvokeModel-family operations expose the same enums through flat parameters or
response headers. This route uses ConverseStream only; the nested object shape
above is the authoritative request encoding for it.

### Omission and documented defaults

Service tiers user guide:

- missing `service_tier` routes to Standard tier
- explicit `default` also selects Standard tier

Latency optimized inference user guide:

- missing latency configuration routes through `standard`
- explicit `standard` selects standard inference

These defaults mean omission already selects Standard latency and Standard
service tier. They are not separate adapter-local control rows for this lane.

### Returned and effective state

ConverseStream returns a terminal `metadata` event that may include:

- `performanceConfig.latency` — served latency configuration
- `serviceTier.type` — served tier
- `metrics.latencyMs` — observed latency metric

Official sources also state:

- latency optimization quota exhaustion can fall back to Standard while billing
  at Standard rates; served latency is visible in the response
- on-demand quota for a model is shared across `priority`, `default`, and
  `flex`
- CloudWatch exposes `ResolvedServiceTier` as the actual tier that served a
  request, which may differ from the requested tier
- Reserved tier requires account-team enablement and is not a per-request toggle
  for ordinary on-demand callers

None of these returned, resolved, billed, or observed facts may become static
capability claims in Swallowtail.

### Model, region, profile, and account dependence

Latency optimization is preview, subject to change, and documented only for a
closed model/region/inference-profile table. Examples from the current guide:

| Provider | Model ID | Cross-region profile regions |
| --- | --- | --- |
| Amazon | `amazon.nova-pro-v1:0` | `us-east-1`, `us-east-2` |
| Anthropic | `anthropic.claude-3-5-haiku-20241022-v1:0` | `us-east-2`, `us-west-2` |
| Meta | `meta.llama3-1-405b-instruct-v1:0` | `us-east-2` |
| Meta | `meta.llama3-1-70b-instruct-v1:0` | `us-east-2`, `us-west-2` |

Additional guide bounds:

- Llama 3.1 405B optimized latency supports total input+output up to 11K tokens;
  larger requests fall back to standard mode

Service-tier support is model-specific. Official guidance directs readers to
Models at a glance and per-model cards rather than a route-wide enum. Example
model-card posture for one embedding model shows Standard only and Priority,
Flex, and Reserved unsupported; that matrix is not transferable to arbitrary
Runtime models.

Because the prepared Runtime route accepts any exact model id and region at
attempt preparation time, a closed deliver-now table would need either:

- a fixed production model and region pair with frozen per-model tier support,
  or
- a generic enum claim that every model and region accepts every tier/latency
  value

Neither is authorized by the evidence. The first depends on mutable catalogue
and account facts this lane cannot bind. The second would flatten model-,
region-, capacity-, and entitlement-dependent behavior into a generic control.

## Truth Separation

| Kind | Requested / SDK-built | Service accepted | Returned / effective | Billed / observed | Current adapter |
| --- | --- | --- | --- | --- | --- |
| Latency mode | optional `performanceConfig.latency` | not proven without live call | `metadata.performanceConfig.latency`; may differ after quota/token fallback | Standard rates on fallback; CloudWatch model-id+latency-optimized metrics | not sent; omission retains current builder |
| Service tier | optional `serviceTier.type` | not proven without live call | `metadata.serviceTier.type`; CloudWatch `ResolvedServiceTier` may differ | tier-specific pricing; shared on-demand quota across priority/default/flex | not sent; omission retains current builder |
| Latency metric | n/a | n/a | `metadata.metrics.latencyMs` | not a control | ignored by decoder |

A product label such as Fast is not evidence for either field.

## Frozen Repository Evidence

Current driver invocation in `src/sdk.rs` sends only:

- `model_id`
- one user `messages` entry
- `inference_config.max_tokens`

It does not call `.performance_config(...)` or `.service_tier(...)`.

`StreamDecoder` accepts the qualified text-only event sequence and reads token
usage from `metadata`. It does not observe returned `performanceConfig`,
`serviceTier`, or `metrics.latencyMs`.

Prepared evidence exposes exact SDK crate/version constants from `SDK_VERSION`
(`1.136.0`) even though Cargo links `1.139.0`. Runtime fixtures under
`tests/fixtures/bedrock-runtime-1.136.0/` remain the historical protocol corpus
for stream/error typing; they do not encode tier controls.

Omission therefore retains the exact current SDK builder call and request
bytes. No adapter-local enum for latency or tier exists yet.

## Exact Deliver-Now Disposition

| Control | Value | Model / region / profile | Request disposition | Returned-state disposition | Deliver-now |
| --- | --- | --- | --- | --- | --- |
| `performanceConfig.latency` | `optimized` | must match official latency table and profile regions | document shape only | served mode may fall back; not observed | no |
| `performanceConfig.latency` | `standard` | route-open model/region | duplicates omission default | not observed | no |
| `serviceTier.type` | `priority` | model-specific support required | document shape only | resolved tier may differ; not observed | no |
| `serviceTier.type` | `flex` | model-specific support required | document shape only | resolved tier may differ; not observed | no |
| `serviceTier.type` | `default` | route-open model/region | duplicates omission default | not observed | no |
| `serviceTier.type` | `reserved` | account-team capacity; not ordinary per-request toggle | document shape only | overflow to Standard documented | no |
| omission | absent both fields | any prepared model/region | current production bytes | usage only | not a tier/latency control row |

No row is deliver-now. The empty set is because exact official sources expose
optional request fields whose eligibility, acceptance, effective tier, billing,
and observed latency all depend on model, region, inference profile, quota,
capacity, or account facts that preparation cannot close for the route-open
Runtime facade. It is not because the SDK lacks the fields, not because the
`1.136.0`/`1.139.0` mismatch hides them, and not because omission fails to
retain current behavior.

## Promotion

Research 231 promotes an empty deliver-now set.

Card 230 completes with the same disposition. A later binding lane may reopen
this family only when an exact qualified model route, region, inference profile,
and preparation-time evidence source can close every Contract 037/040 row
without live AWS work, account inspection, or flattening returned tier, billing,
or latency observation into dispatch claims.

Thinking fields, tools, guardrails, catalogue route selection, SDK currentness
repair, and production binding remain out of scope.

## Primary Sources

- frozen corpus under
  `crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-runtime-service-tier-evidence/`
- [Bedrock SDK prepared integration guide](../guides/bedrock-sdk-prepared-integration.md)
- Research 013, 127, 159
- `crates/swallowtail-adapter-bedrock/src/sdk.rs`
- `crates/swallowtail-adapter-bedrock/src/stream.rs`
