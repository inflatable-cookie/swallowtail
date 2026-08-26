# g04.074 Cline Headless Model Selection

Status: ready
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Depends on: g04.042, g04.073; per-route feature completion programme
Vision tags: model route, provider agreement, fixed process arguments
Contract refs: 005, 008, 020, 023, 029, 033, 037, 040, 052
Research: 144, 147, 190, 220, 221

## Problem

Production route `cline.headless` owns one exact Cline JSON child but selects
neither a provider nor a model. Consumers cannot bind the official
`-m, --model <model-id>` option to an immutable Swallowtail model route.

Exact qualified `cline@3.0.55` source is a credible lead, not a compatibility
claim. Explicit `args.model` wins over persisted and catalogue/default model
state, but the provider independently resolves from explicit argv, ambient
last-used settings, or `cline`. The CLI then attempts to persist the resolved
provider/model selection before the run. Exact evidence must determine whether
any closed provider/model row can be bound without ambient provider drift,
silent model fallback, invented catalogue authority, or unauthorized
configuration mutation.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind a closed Cline headless
model route through canonical fixed process arguments. Preserve current argv
and provider-default behavior on omission, the exact local-account access
boundary, one-run lifecycle, optional Plan composition, and the exact version
ceiling.

## Goals

- [ ] freeze exact `3.0.55` parser, provider/model resolution, membership,
      validation, fallback, persistence, application, output, and lifecycle
      truth
- [ ] determine whether any exact provider/model pair is closed and
      preflight-validatable without a live account, catalogue, or prompt
- [ ] distinguish requested route, planned route, argv dispatch, parser
      acceptance, selected provider/model, applied/effective model, and
      observation
- [ ] promote Research 221 with an exact deliver-now table or honest empty set
- [ ] conditionally bind only Research 221 rows through prepared input,
      immutable `ModelRoute`, plan/evidence, driver, and canonical argv
- [ ] prove omission preserves exact current argv and ambient provider/model
      behavior
- [ ] compose an admitted model route with optional portable Plan without
      widening either feature
- [ ] preserve current access, configuration, isolation, activity,
      cancellation, terminal, retention, and joined-cleanup truth

## Non-Goals

- caller-selectable provider choice, API-key input, OAuth, credential
  discovery, catalogue management, aliases, provider fallback, or defaults
- arbitrary model strings, unbounded slugs, display metadata, pricing,
  entitlement, billing, context-window, modality, or availability claims
- `cline.acp` model selection, ACP `session/set_model`, TUI/hub model pickers,
  `--id` resume, or sibling-route promotion
- thinking delivery, reasoning levels, effective-thinking observation, or
  reopening g04.042 inside this lane
- generic provider configuration, settings-file parsing/writing, temporary
  home/config synthesis, migration, mutation, cleanup, or `HostScoped`
- live install, login, account inspection, catalogue request, provider prompt,
  paid work, currentness, release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `cline.headless`, driver
`swallowtail.cline.headless`, behavior `cline.headless.stdio-json-v1`, axis
`cline.package`, exact qualified npm package `3.0.55`, one bounded structured
run, local-account access, one read-only filesystem working resource, and one
host process deadline.

Card 204 must bind findings to annotated tag `cli-v3.0.55`, commit
`ad442cbb6a81d21773ceabc1398ea5eb58170718`, and the published wrapper identity
already frozen by Research 147. Current documentation/main may corroborate but
cannot amend the exact package. Trace `-m` / `--model` and `-P` / `--provider`
from Commander through provider-settings lookup, `resolveProviderConfig`,
`knownModels`, model selection, session creation, provider dispatch, JSON
events, failure, persistence, cleanup, and retained state.

The only candidate public shape is an exact `ModelRoute` whose provider and
model identities agree with the configured instance, access audience,
immutable plan, and child argv. A provider may be adapter-fixed only when
Research 221 proves that mapping from existing route/access facts. No caller
provider selector is admitted. If exact agreement requires a public provider
choice, ambient last-used state, a live catalogue, or an open model string,
the deliver-now set is empty.

Card 204 must freeze missing, empty, whitespace, repeated, and conflicting
option behavior; canonical spellings and placement relative to `--json`,
`--auto-approve false`, optional `--plan`, `-c`, and the prompt; explicit,
persisted, catalogue-first, and hardcoded-fallback precedence; invalid and
unknown model behavior; provider/model entitlement; and whether the selected
JSON wire confirms the effective pair without unselected `--verbose`.

Exact source currently attempts `saveProviderSettings` with the resolved
provider and `config.modelId` before starting the run. Research 221 must prove
whether an admitted explicit route changes ambient durable state, whether the
write can be disabled or contained by the selected invocation, and whether
the existing `Ambient` posture can truthfully represent the behavior. Contract
033 grants no adapter authority to discover, parse, mutate, migrate, create,
or delete provider configuration. Unavoidable new configuration authority is
a stop, not an implementation detail.

Omission must retain exact current argv and route truth. An admitted row must
dispatch canonical fixed provider/model arguments selected by Research 221,
remain immutable for the child lifetime, and validate request, route, plan,
prepared evidence, driver, exact version, and argv before process work. Plan
composition may add existing canonical `--plan`; it cannot select or alter a
model.

## Execution Plan

### Batch 74.1 — Exact Provider/Model Evidence

- [ ] Execute card 204.
- [ ] freeze exact package, parser, provider/model resolution, membership,
      fallback, persistence, application, output, and lifecycle truth
- [ ] promote Research 221 with a non-empty exact table or honest empty set

### Batch 74.2 — Conditional Model-Route Binding

- [ ] Execute card 205 only when Research 221 admits a non-empty set.
- [ ] bind only exact provider/model rows through prepared input, immutable
      route/plan/evidence, driver, and canonical argv

### Batch 74.3 — Route-Local Acceptance

- [ ] Execute card 206 only after card 205.
- [ ] prove dispatch, omission, rejection, Plan composition, authority
      separation, lifecycle, docs, and API truth

## Acceptance Criteria

- [ ] only Research 221 deliver-now rows prepare
- [ ] configured instance, access audience, `ModelRoute`, immutable plan,
      prepared evidence, driver, and child argv agree exactly
- [ ] omission preserves prior argv and ambient provider/model behavior
- [ ] unsupported, mismatched, drifting, open, fallback-prone, or mutating rows
      reject before process work when knowable
- [ ] model dispatch grants no catalogue, entitlement, billing, credential,
      configuration, reasoning, or availability claim
- [ ] optional Plan remains independently selected and exact
- [ ] current one-run activity, terminal, cancellation, deadline, failure,
      retention, and joined cleanup remain exact
- [ ] default QA performs no install, login, account inspection, catalogue
      request, provider prompt, configuration mutation, or paid work

## Lane Runway

- predecessor: g04.073 Cline headless Plan mode delivery
- this milestone: exact Cline headless provider/model agreement evidence and
  conditional model-route binding
- execution topology: one serial worker lane, cards 204-206
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact `3.0.55` cannot provide a closed provider/model set and
  fail-closed membership before provider effects.
- Stop if provider selection remains ambient, model/provider agreement is
  post-spawn, unknown models silently fall back, or the selected pair cannot be
  tied to the configured instance and access audience.
- Stop if explicit selection necessarily mutates ambient provider settings or
  needs configuration authority absent from Contract 033.
- Stop if proof requires login, account inspection, a live catalogue,
  provider prompt, paid work, or ambient configuration mutation.
- Stop if delivery needs arbitrary strings, caller provider selection, API
  keys, sibling-route work, a shared contract change, currentness movement, or
  a breaking API.

## Batch Cards

- [204-cline-headless-model-selection-evidence.md](batch-cards/204-cline-headless-model-selection-evidence.md)
- [205-cline-headless-model-selection-binding.md](batch-cards/205-cline-headless-model-selection-binding.md)
- [206-cline-headless-model-selection-acceptance.md](batch-cards/206-cline-headless-model-selection-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 147 Cline Headless Identity](../../research/147-cline-headless-3-0-55-identity.md)
- [Research 190 Cline Thinking Controls](../../research/190-cline-thinking-control-evidence.md)
- [Research 220 Cline Headless Plan Mode](../../research/220-cline-headless-plan-mode-evidence.md)
- [Research 221 Cline Headless Model Selection](../../research/221-cline-headless-model-selection-evidence.md)
- [Contract 005 Integration Identity](../../contracts/005-integration-identity-and-transport-diversity.md)
- [Contract 008 Runtime Registration And Preflight](../../contracts/008-runtime-registration-and-preflight.md)
- [Contract 020 Model Catalogue Boundary](../../contracts/020-model-catalogue-observation-and-availability-boundary.md)
- [Contract 023 Harness Operation Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Cline Headless Prepared Integration](../../guides/cline-headless-prepared-integration.md)
- [Exact Cline `3.0.55` CLI parser](https://github.com/cline/cline/blob/ad442cbb6a81d21773ceabc1398ea5eb58170718/apps/cli/src/commands/program.ts)
- [Exact Cline `3.0.55` headless dispatch](https://github.com/cline/cline/blob/ad442cbb6a81d21773ceabc1398ea5eb58170718/apps/cli/src/main.ts)
