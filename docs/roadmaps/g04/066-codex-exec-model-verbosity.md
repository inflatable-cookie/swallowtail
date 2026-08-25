# g04.066 Codex Exec Model Verbosity

Status: complete
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Depends on: g04.054; per-route feature completion programme
Vision tags: explicit selection, provider truth, installed-route isolation
Contract refs: 011, 020, 029, 033, 037, 040, 044, 052
Research: 160, 172, 201, 213

## Problem

Production route `codex.exec` selects one exact model and already binds
reasoning, search, one image, JSON Schema output, a working resource, and one
owned ephemeral child. It does not expose Codex's `model_verbosity` control.

Current official Codex configuration schema describes `model_verbosity` as an
optional `low|medium|high` Responses API control for GPT-5 models. Official
model metadata separately carries `support_verbosity` and `default_verbosity`.
The control is therefore not safe to add from the config key alone: exact CLI,
model, provider, default, unsupported-model, precedence, and silent-ignore
truth must agree before dispatch.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind the smallest adapter-local
model-verbosity subset on the existing maintained `codex.exec` child without a
generic provider-settings map, ambient-config dependence, model-family
inference, or a claim that dispatched verbosity was provider-accepted or
effective.

## Goals

- [x] freeze exact Codex source, schema, model-metadata, parser, and selected
      Exec command evidence
- [x] identify exact CLI/version, provider, selected-model, supported-value,
      and default rows
- [x] classify config precedence, omission, unknown values, unsupported
      models/providers, silent ignore, warning, fallback, and failure behavior
- [x] distinguish selected, planned, dispatched, provider-accepted, effective,
      and observed verbosity truth
- [x] promote Research 213 with an exact deliver-now table or honest empty set
- [x] expose only a closed adapter-local typed verbosity selection admitted by
      Research 213
- [x] bind the selection through prepared input, immutable evidence, driver,
      and exact child arguments
- [x] preserve reasoning, search, schema, image, access, retention, activity,
      usage, cancellation, deadline, terminal, and joined cleanup truth
- [x] prove omission preserves existing argv and selected-model behavior

## Non-Goals

- a portable verbosity, response-length, style, personality, or reasoning
  capability
- `codex.app-server`, OpenAI Responses, Chat Completions, Realtime, or another
  provider route
- service tier, Fast mode, multi-agent, plan-mode effort, personality, or
  maximum-output controls
- raw `--config`, arbitrary provider settings, model/provider overrides,
  project/user config mutation, or a synthetic Codex home
- inferring support from a `gpt-5` name prefix, a successful text response, or
  current main-branch metadata outside an exact release
- live login, account/catalogue inspection, provider requests, paid inference,
  currentness, release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `codex.exec`, driver `swallowtail.codex.exec`,
axis `codex.cli`, maintained behavior `codex.exec.jsonl-v1`, exact qualified
ceiling `0.149.1`, and model/provider rows admitted by Research 213. No model,
value, version floor, or behavior revision is prequalified here.

The current maintained route owns one `codex exec --json --ephemeral` child,
suppresses user config and rules, fixes read-only sandbox and never approval,
selects one model explicitly, and passes typed configuration overrides. The
only candidate public shape is one optional adapter-local
`CodexModelVerbosity` carried by prepared evidence and the bound driver. It
must not become a shared `Capability` or generic settings surface.

Card 184 must freeze the exact release-tag `config.schema.json`, `models.json`,
model-info types, request construction, CLI config parser, precedence, and
selected command. A schema enum proves syntax, not model support. A
`support_verbosity` row proves official model metadata, not provider acceptance
for the caller's account. Values may proceed only when exact selected model,
provider transport, parser, and fail-closed pre-spawn validation can all be
bound without a live request.

Omission must retain current argv byte-for-byte and must not serialize a
model-default value. Explicit low, medium, and high values remain distinct from
reasoning effort. If the exact CLI ignores, warns, defaults, clamps, substitutes,
or silently accepts verbosity on an unsupported model/provider, that row is
withheld unless Swallowtail can reject it before spawn.

An empty Research 213 deliver-now set is an honest stop.

## Execution Plan

### Batch 66.1 — Exact Verbosity Evidence

- [x] Execute card 184.
- [x] freeze exact release, model, provider, parser, precedence, and command
      truth
- [x] promote Research 213 with a non-empty exact table or honest empty set

### Batch 66.2 — Conditional Adapter-Local Binding

- [x] Execute card 185 only when Research 213 admits a non-empty set.
- [x] bind only exact version/model/value/profile rows through the existing
      prepared Exec child

### Batch 66.3 — Route-Local Acceptance

- [x] Execute card 186 only after card 185.
- [x] prove dispatch, omission, rejection, composition, lifecycle, docs, and
      API truth

## Acceptance Criteria

- [x] only Research 213 deliver-now rows prepare
- [x] input, model route, plan/evidence, driver, and child argv agree exactly
- [x] omission preserves the prior argv and behavior
- [x] unsupported models/providers/versions/values and knowable drift reject
      before process or credential effects
- [x] verbosity remains distinct from reasoning, service tier, personality,
      and output-token bounds
- [x] every existing Exec composition and lifecycle claim remains exact
- [x] docs claim no provider acceptance, effective output length, billing,
      entitlement, or model support beyond frozen evidence
- [x] default QA performs no credential, login, account, provider, or paid work

## Lane Runway

- predecessor: g04.065 Claude Code headless Ultracode evidence stop
- this milestone: Codex Exec model-verbosity evidence and conditional binding
- execution topology: one serial worker lane, cards 184-186
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact release-tag model metadata, parser, provider applicability,
  default, or unsupported-model behavior cannot be closed without inference.
- Stop if support depends on ambient/user config, an unbounded live catalogue,
  account inspection, provider prompting, or model-family prefix matching.
- Stop if selected and default verbosity cannot stay distinct or an unsupported
  value can be silently ignored after spawn.
- Stop if delivery needs a portable capability, generic settings surface,
  sibling-route promotion, shared contract change, currentness movement, or a
  breaking public lifecycle.

## Batch Cards

- [184-codex-exec-model-verbosity-evidence.md](batch-cards/184-codex-exec-model-verbosity-evidence.md)
- [185-codex-exec-model-verbosity-binding.md](batch-cards/185-codex-exec-model-verbosity-binding.md)
- [186-codex-exec-model-verbosity-acceptance.md](batch-cards/186-codex-exec-model-verbosity-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 201 Codex 0.149.1 Identity](../../research/201-codex-0-149-1-identity.md)
- [Research 213 Codex Exec Model Verbosity](../../research/213-codex-exec-model-verbosity-evidence.md)
- [Contract 020 Model Catalogue Observation](../../contracts/020-model-catalogue-observation-and-availability-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 044 Observable Agent Activity](../../contracts/044-observable-agent-activity-and-disclosure.md)
- [Codex Prepared Integration](../../guides/codex-prepared-integration.md)
- [Codex configuration schema](https://github.com/openai/codex/blob/main/codex-rs/core/config.schema.json)
- [Codex model metadata](https://github.com/openai/codex/blob/main/codex-rs/core/models.json)
