# 2026-08-27 g04.082c Bedrock Runtime Service-Tier Evidence

Status: completed
Card: 230
Research: 231

## Boundary

Evidence only. The worker updated this file, card 230, Research 231, and new
Bedrock-local frozen evidence. Shared planning and production code stay unchanged.

## Outcome

Research 231 promotes an empty deliver-now set for `bedrock.runtime`
`ConverseStream` `performanceConfig.latency` and `serviceTier`.

Exact AWS API and user-guide sources plus generated
`aws-sdk-bedrockruntime` `1.136.0`/`1.139.0` shapes show both fields on the
request and in stream `metadata`, with enums `standard|optimized` and
`default|flex|priority|reserved`. The public SDK constant remains `1.136.0`;
Cargo locks `=1.139.0`; tier-field generated sources are byte-identical across
those versions.

The current driver still sends only `model_id`, `messages`, and
`inference_config.max_tokens`. The stream decoder reads usage from metadata and
ignores returned tier and latency fields. Omission retains that builder call.

No preparation-time row closes because latency optimization and service-tier
support are model-, region-, profile-, quota-, capacity-, and account-dependent,
while the prepared Runtime facade accepts any exact model and region per
attempt. Returned tier, resolved tier, billing, and observed latency remain
withheld.

Frozen corpus:
`crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-runtime-service-tier-evidence/`.

## Validation

- `effigy validate:focused swallowtail-adapter-bedrock` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed

## Next

PR: https://github.com/inflatable-cookie/swallowtail/pull/83

Orchestrator review and serial g04.082 promotion. Production binding and shared
inventory disposition wait for that promotion batch.
