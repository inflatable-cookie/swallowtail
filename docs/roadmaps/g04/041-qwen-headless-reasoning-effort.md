# g04.041 Qwen Headless Reasoning Effort

Status: completed; worker PR pending review
Owner: Tom
Created: 2026-08-22
Depends on: per-route feature completion programme; g04.026
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 011, 020, 029, 033, 037, 040, 050, 052
Research: 017, 081, 082, 159, 173, 189

## Problem

`qwen.headless` binds an exact model and supports both bounded structured runs
and turn-scoped interactive sessions, but it rejects portable reasoning
selection. Current Qwen documentation exposes `model.reasoningEffort` and the
interactive `/effort` command. That surface is not yet evidence that exact
package `0.21.15` offers a process-private headless transport, preserves one
selected value, or avoids provider/model clamping and defaults.

Swallowtail currently runs Qwen with ambient harness configuration. Contract
040 permits an exact operation-private command or child environment mapping;
it does not permit user-config mutation or a synthetic configuration root.
Evidence must settle that boundary before implementation.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind one model-qualified Qwen
headless reasoning selection across the existing prepared run, interactive
turn, resume, and fresh-replacement paths.

## Goals

- [x] freeze official current documentation and exact `0.21.15` package source
      for reasoning values, transport, precedence, defaults, and lifetime
- [x] separate interactive `/effort` from a process-safe headless mapping
- [x] classify `low`, `medium`, `high`, `xhigh`, and `max` per exact model
- [x] prove whether any value survives without clamp, aliasing, or default
      substitution
- [x] prove a process-private mapping that neither mutates ambient settings nor
      requires an unleased synthetic configuration root
- [x] bind only Research 189 deliver-now model/value rows through prepared
      input, immutable plan/evidence, request policy, driver, and child process
- [x] preserve the selected value on structured runs, first and resumed turns,
      and fresh context-losing replacement
- [x] preserve exact current behavior when reasoning is absent
- [x] publish dispatch/acceptance truth without inferring effective reasoning

## Non-Goals

- ambient user or project settings mutation
- a generic Qwen settings, argv, or environment map
- synthetic home/config roots without the Contract 033 host-scoped lease
- `/effort` UI automation, interactive shell state, or per-turn mutation
- model discovery, model defaults, provider selection, or sibling Qwen routes
- tool, permission, search, usage, output-token, or structured-output controls
- a compatibility-ceiling change, install, login, prompt, or live provider work

## Named Scope

The milestone is restricted to `qwen.headless`, package axis
`qwen-code.package`, the existing
`qwen-code.headless.v0.21.0-catalogue-filter` behavior through `0.21.14`, and
the exact `qwen-code.headless.v0.21.15-reasoning-control` behavior revision at
qualified point `0.21.15`. The reasoning revision is a private feature
boundary; it does not retroactively map `0.21.0..=0.21.14`.

Qwen preparation already selects an exact model route. Research 189 must still
qualify every model/value combination independently. A documented global
setting or interactive command is only a lead. The evidence gate must prove
the exact headless process transport, precedence over ambient configuration,
parse/admission behavior, model support, clamp/default behavior, and lifetime.

The mapping may continue only if it is operation-private under Contract 040.
If exact `0.21.15` requires editing user/project settings, creating a synthetic
configuration tree, or relying on an ambient default, the milestone stops
after card 113.

## Execution Plan

### Batch 41.1 — Exact Package And Transport Evidence

- [x] Execute card 113.
- [x] freeze official and exact `0.21.15` configuration specimens and digests
- [x] promote Research 189 with model/value, transport, precedence, clamp,
      default, version, and lifetime dispositions

### Batch 41.2 — Prepared Run And Session Binding

- [x] Execute card 114 after card 113 admitted a useful exact subset.
- [x] bind optional portable reasoning through run/session input, plan,
      evidence, request, driver, and operation-private child configuration
- [x] preserve the absent path; reject plan/evidence drift before process work
      and control/ambient drift after child startup but before the user message

### Batch 41.3 — Lifecycle And Acceptance

- [x] Execute card 115 after card 114; the operator-authorized Qwen package
      registration makes the final API gate pass.
- [x] prove run, first turn, resumed turn, and fresh replacement dispatch
- [x] update route-local guidance and report the deferred shared closeout delta

## Acceptance Criteria

- [x] only Research 189 deliver-now model/value rows prepare
- [x] request, plan constraint, prepared evidence, driver, and child transport
      agree exactly
- [x] ambient configuration cannot override the planned value
- [x] one selected value spans every child in the prepared operation lifetime
- [x] absent reasoning retains current command and behavior
- [x] no raw setting, alias, clamp, default substitution, or model inference
      enters the public mapping
- [x] plan/evidence/driver failures occur before process start; control and
      ambient override failures occur after child startup but before the user
      message/provider prompt
- [x] deterministic QA uses no install, login, credential, account, or prompt
- [x] docs stop at dispatched, accepted, effective, and observed truth actually
      proved by the exact surface

An empty Research 189 deliver-now set is an honest stopped milestone, not a
reason to weaken Contract 033 or 040.

## Lane Runway

- predecessor: g04.040 Copilot CLI ACP evidence stop
- this milestone: Qwen headless reasoning-effort evidence and conditional
  binding
- execution topology: one serial worker lane, cards 113-115
- next route family: selected by orchestrator closeout from the remaining
  per-route feature inventory

## Decision Gates

- Stop if exact `0.21.15` does not expose the documented reasoning surface.
- Stop if headless use exists only through ambient/global settings or `/effort`.
- Stop if the setting requires user-config mutation or a synthetic config root.
- Stop if every value is clamped, substituted, or model-dependent without an
  exact qualified subset.
- Stop if the value cannot remain exact across runs, resumed turns, and fresh
  replacement.
- Stop if support requires a contract change, unresolved public lifecycle, or
  breaking API change.

## Batch Cards

- [113-qwen-headless-reasoning-effort-evidence.md](batch-cards/113-qwen-headless-reasoning-effort-evidence.md) — complete
- [114-qwen-headless-reasoning-effort-binding.md](batch-cards/114-qwen-headless-reasoning-effort-binding.md) — complete
- [115-qwen-headless-reasoning-effort-acceptance.md](batch-cards/115-qwen-headless-reasoning-effort-acceptance.md) — complete

## Closeout

Cards 113-115 are complete in the worker worktree. Research 189 admits only
the exact `0.21.15` DashScope rows for `qwen3.8-max` and
`qwen3.8-max-preview`, all five canonical values. The route-local guide,
fixtures, prepared adapter, deterministic acceptance tests, and
package-specific API baseline are updated. The operator-authorized Qwen line
in the shared unreleased package list makes the package API gate pass. The PR
and reviewed head are recorded in the route-local closeout log after push; no
merge is claimed.

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 173 Qwen Headless 0.21.15 Identity](../../research/173-qwen-headless-0-21-15-identity.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 050 Working-State Restoration](../../contracts/050-working-state-restoration-facade.md)
- [Qwen Headless Prepared Integration](../../guides/qwen-headless-prepared-integration.md)
- [Qwen Headless Mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Qwen Settings](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/settings/)
