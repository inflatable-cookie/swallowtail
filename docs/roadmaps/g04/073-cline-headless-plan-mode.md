# g04.073 Cline Headless Plan Mode

Status: complete
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Depends on: g04.042; per-route feature completion programme
Vision tags: harness mode, fixed process argument, route-local controls
Contract refs: 011, 012, 023, 029, 033, 034, 037, 052
Research: 144, 147, 190, 220

## Problem

Production route `cline.headless` owns one exact Cline JSON child but always
launches `cline --json --auto-approve false -c <cwd> <prompt>`. Consumers
cannot select Cline's official headless `-p` / `--plan` behavior.

Exact qualified `cline@3.0.55` source is a stronger lead than flag
advertisement alone. It parses an explicit Plan value before persisted global
settings, carries the resolved mode into the one-run config and system prompt,
tags the turn with that mode, selects a Plan tool preset, and registers a
pre-approval command guard. The same root flag is discarded by the ACP early
return, so this lane is headless-only.

Source visibility is not yet a compatibility claim. Exact evidence must prove
that the complete `3.0.55` Plan path is behaviorally equivalent to portable
`HarnessMode::Plan`, remains fixed for the bounded run, and cannot be weakened
by ambient settings, tool construction, mode-switch behavior, or another
execution seam.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind `HarnessMode::Plan` on the
existing `cline.headless` route through one fixed process argument. Preserve
the route's current omitted behavior, explicit `--auto-approve false`, ambient
configuration and isolation truth, one-run lifecycle, and exact version
ceiling.

## Goals

- [x] freeze exact `3.0.55` parser, precedence, prompt, tool-preset, command-
      guard, mode-switch, output, terminal, persistence, and lifecycle truth
- [x] determine whether the complete headless behavior is equivalent to
      portable `HarnessMode::Plan` without a live provider prompt
- [x] distinguish requested, planned, argv-dispatched, parser-accepted,
      applied, effective, and observed mode state
- [x] promote Research 220 with an exact deliver-now table or honest empty set
- [x] conditionally bind only `HarnessMode::Plan` through prepared input,
      immutable plan/evidence, driver, and canonical child argv
- [x] prove omission preserves exact current argv and provider-default mode
- [x] retain `--auto-approve false`, read-only working-resource policy,
      `AmbientHost`, cancellation, terminal, and joined-cleanup truth
- [x] keep ACP, model, thinking, timeout, permissions, and mode switching out

## Non-Goals

- `cline.acp` Plan, thinking, provider/model selection, model catalogues, or
  selected-model work
- provider `act`, `yolo`, or `zen` as public values; raw flags, strings,
  booleans, generic provider configuration, or a new harness-mode vocabulary
- runtime Plan-to-Act mutation, automatic continuation, `switch_to_act_mode`
  as a consumer operation, or reusable sessions
- `--auto-approve true`, `--yolo`, permission bypass, arbitrary tool policy,
  tool execution, compaction, retries, timeout, teams, hub, worktree, or resume
- treating prompt policy, a tool preset, or the command blacklist as complete
  filesystem, network, shell, process, sandbox, or descendant containment
- changing `Ambient` configuration posture, `AmbientHost` isolation, local
  account access, durable-state truth, or the exact compatibility ceiling
- install, login, account inspection, provider prompt, paid work, currentness,
  release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `cline.headless`, driver
`swallowtail.cline.headless`, behavior `cline.headless.stdio-json-v1`, axis
`cline.package`, exact qualified npm package `3.0.55`, one bounded structured
run, local-account access, one read-only filesystem working resource, and one
host process deadline.

Card 201 must bind its findings to annotated tag `cli-v3.0.55`, commit
`ad442cbb6a81d21773ceabc1398ea5eb58170718`, and the published wrapper identity
already frozen by Research 147. It must trace `-p` / `--plan` from Commander
through `resolveStartupMode`, `Config.mode`, system-prompt construction,
mode-tagged user input, runtime tool preset and extensions, every write-capable
tool seam, any `switch_to_act_mode` or continuation path, result envelopes,
cleanup, and retained state.

The only candidate public value is portable `HarnessMode::Plan`. A prepared
run may expose it only when its immutable plan contains
`HarnessModeSelection(Plan)` and exact source proves behavior equivalent to
Contract 034's Plan posture. The mapping is a fixed process argument, not ACP
negotiation. No effective-value observation is synthesized when the selected
JSON wire does not report one.

Plan mode is behavior, not isolation. The existing read-only working-resource
policy and `--auto-approve false` remain independent requirements. The route
stays `AmbientHost` and `Ambient`; source-visible command blocking may support
the provider behavior finding but cannot become a containment claim. Any
write-capable bypass, automatic Plan-to-Act transition, ambient override after
explicit selection, or mode-dependent lifecycle expansion must stop delivery
unless exact bounded behavior can still meet the portable contract.

Omission must retain exact current argv and route behavior. The selected Plan
value must place canonical `--plan` before `-c <cwd> <prompt>`, remain immutable
for the child lifetime, and validate against request, plan, prepared evidence,
driver, and exact version before process work. No other Cline mode or root flag
is admitted by this lane.

## Execution Plan

### Batch 73.1 — Exact Headless Plan Evidence

- [x] Execute card 201.
- [x] freeze exact package, parser, precedence, Plan semantics, tool/guard,
      mode-switch, lifecycle, output, and persistence truth
- [x] promote Research 220 with a non-empty exact table or honest empty set

### Batch 73.2 — Conditional Portable Binding

- [x] Execute card 202 only when Research 220 admits a non-empty set.
- [x] bind only exact `3.0.55` `HarnessMode::Plan` through the prepared
      structured-run surface and canonical argv

### Batch 73.3 — Route-Local Acceptance

- [x] Execute card 203 only after card 202.
- [x] prove dispatch, omission, rejection, mode separation, lifecycle, docs,
      API, and compatibility truth

## Acceptance Criteria

- [x] only Research 220 deliver-now rows prepare
- [x] request, immutable plan, prepared evidence, driver, and child argv agree
- [x] exact source proves the selected fixed argument is behaviorally
      equivalent to portable `HarnessMode::Plan`
- [x] omission preserves prior argv and provider-default mode behavior
- [x] unsupported, mismatched, drifting, overrideable, or behaviorally weaker
      rows reject before process work when knowable
- [x] Plan mode grants no permission, configuration, filesystem/network,
      sandbox, shell, process, descendant, model, or account authority
- [x] existing one-run activity, terminal, cancellation, deadline, failure,
      retention, and joined cleanup remain exact
- [x] default QA performs no install, login, account inspection, provider
      prompt, tool execution, or paid request

## Lane Runway

- predecessor: g04.072 Grok Build ACP subagents-disabled evidence stop
- this milestone: exact Cline headless fixed-argument Plan evidence and
  conditional portable binding
- execution topology: one serial worker lane, cards 201-203
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact `3.0.55` source cannot prove the complete headless Plan path or
  its equivalence to portable `HarnessMode::Plan`.
- Stop if `--plan` is parser-only, ACP/TUI-only, ambiently overrideable,
  advisory without a bounded enforcement path, or ignored by the one-run
  runtime.
- Stop if the model can transition to Act or reach a write-capable tool path
  during the selected bounded run without a new consumer-authorized operation.
- Stop if deterministic proof needs login, account inspection, a provider
  prompt, arbitrary tool execution, paid work, or ambient config mutation.
- Stop if delivery needs a raw flag, generic configuration, sibling-route
  changes, shared contract changes, currentness movement, or a breaking API.

## Batch Cards

- [201-cline-headless-plan-mode-evidence.md](batch-cards/201-cline-headless-plan-mode-evidence.md)
- [202-cline-headless-plan-mode-binding.md](batch-cards/202-cline-headless-plan-mode-binding.md)
- [203-cline-headless-plan-mode-acceptance.md](batch-cards/203-cline-headless-plan-mode-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 144 Primary Wave Source And Route Gate](../../research/144-primary-wave-source-and-route-gate.md)
- [Research 147 Cline Headless Identity](../../research/147-cline-headless-3-0-55-identity.md)
- [Research 190 Cline Thinking Controls](../../research/190-cline-thinking-control-evidence.md)
- [Research 220 Cline Headless Plan Mode](../../research/220-cline-headless-plan-mode-evidence.md)
- [Contract 012 Interactive Session Options](../../contracts/012-interactive-session-options-and-callback-exchange.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Cline Headless Prepared Integration](../../guides/cline-headless-prepared-integration.md)
- [Exact Cline `3.0.55` CLI parser](https://github.com/cline/cline/blob/ad442cbb6a81d21773ceabc1398ea5eb58170718/apps/cli/src/commands/program.ts)
- [Exact Cline `3.0.55` startup settings](https://github.com/cline/cline/blob/ad442cbb6a81d21773ceabc1398ea5eb58170718/apps/cli/src/utils/startup-settings.ts)
- [Exact Cline `3.0.55` headless dispatch](https://github.com/cline/cline/blob/ad442cbb6a81d21773ceabc1398ea5eb58170718/apps/cli/src/main.ts)
