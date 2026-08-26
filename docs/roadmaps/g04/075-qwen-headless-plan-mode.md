# g04.075 Qwen Headless Plan Mode

Status: complete
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Depends on: g04.041, g04.051, g04.069; per-route feature completion programme
Vision tags: harness mode, fixed process argument, route-local controls
Contract refs: 011, 012, 023, 029, 033, 034, 037, 040, 052
Research: 081, 082, 173, 189, 198, 216, 222

## Problem

Production route `qwen.headless` always launches with `--safe-mode
--approval-mode default`. Exact maintained Qwen Code points also parse
`--approval-mode plan`, but Swallowtail exposes no caller-selectable Plan
posture on this route.

The route is a credible fixed-argument candidate: it owns one structured child
per run or turn, fixes an explicit read-tool allowlist and write/process/tool
denylist, binds one read-only working resource, and already validates model,
reasoning, budgets, version, lifecycle, and replacement before process work.
Parser presence and a restrictive tool list are not enough. Exact evidence
must prove that Qwen Plan is behaviorally equivalent to portable
`HarnessMode::Plan`, remains fixed for each bounded child, and does not conflict
with safe mode, stream-json control exchange, continuation, or fresh
replacement.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind portable
`HarnessMode::Plan` on `qwen.headless` through canonical
`--approval-mode plan`. Preserve omission as exact `default`, the selected
safe-mode/tool policy, read-only working resource, ambient configuration and
isolation truth, current reasoning and budgets, and one-child-per-turn
lifecycle.

## Goals

- [x] freeze exact `0.21.15`, `0.22.0`, and `0.22.1` parser, precedence,
      mode application, tool policy, control exchange, continuation, output,
      terminal, persistence, and lifecycle truth
- [x] distinguish requested, planned, argv-dispatched, parser-accepted,
      applied, effective, and observed approval mode
- [x] determine whether any exact point admits portable `HarnessMode::Plan`
      without provider prompting or ambient configuration mutation
- [x] promote Research 222 with an exact deliver-now table or honest empty set
- [x] conditionally bind only admitted Plan rows through prepared input,
      immutable plan/evidence, driver, and canonical child argv
- [x] prove omission preserves exact `--approval-mode default` across runs,
      turns, resume, and fresh replacement
- [x] preserve `--safe-mode`, exact tool filters, model/reasoning/budgets,
      read-only working-resource policy, `AmbientHost`, cancellation, and
      joined cleanup

## Non-Goals

- provider `default`, `auto-edit`, `auto`, or `yolo` as new public values;
  raw flags, strings, booleans, or a Qwen-specific public approval enum
- permission bypass, writable operation profiles, tool-policy selection,
  provider sandbox claims, shell/process/network authority, or new callbacks
- changing the existing read-tool allowlist or write/process/tool denylist
- model, reasoning, turn/tool-budget, catalogue, session-id, attachment,
  schema, search, currentness, or credential work
- treating Plan, safe mode, or tool filtering as filesystem, process, network,
  sandbox, or descendant containment
- install, login, account inspection, provider prompt, paid work, release,
  merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `qwen.headless`, driver
`swallowtail.qwen.headless`, axis `qwen-code.package`, exact maintained points
`0.21.15`, `0.22.0`, and `0.22.1`, structured runs and turn-scoped interactive
continuation, delegated-harness access, one read-only filesystem working
resource, and one host deadline per child.

Card 207 must bind findings to the exact artifacts and source identities
already frozen by Research 173 and 216. Current official docs may corroborate
but cannot amend those packages. Trace the shared `APPROVAL_MODES` parser and
`--approval-mode plan` through settings precedence, config construction,
client/tool registration, safe-mode suppression, stream-json initialization,
user-message dispatch, resume, provider session-id reuse, terminal output,
cleanup, and retained state.

The only candidate public value is portable `HarnessMode::Plan`. A prepared
operation may expose it only when its immutable plan contains
`HarnessModeSelection(Plan)` and Research 222 proves behavior equivalent to
Contract 034. The mapping is one fixed child argument, not an ACP-negotiated
option. Every new run, continued turn child, resumed provider session, and
fresh replacement must reapply the same selected mode or fail before spawn.

Plan is provider behavior, not isolation or permission. Existing `--safe-mode`,
explicit tool allow/deny filters, read-only working-resource authority,
`Ambient` configuration, and `AmbientHost` isolation remain independent. A
Plan label, prompt instruction, or provider tool filter cannot widen those
claims.

Omission must retain exact `--approval-mode default` bytes and behavior. An
admitted selection replaces only that value with `plan`; it cannot add another
mode, change tool filters, mutate ambient settings, or alter the current model,
reasoning, budget, session, terminal, cancellation, or cleanup paths.

## Execution Plan

### Batch 75.1 — Exact Qwen Plan Evidence

- [x] Execute card 207.
- [x] freeze exact parser, precedence, Plan semantics, safe-mode/tool
      composition, child/replacement lifecycle, output, and persistence truth
- [x] promote Research 222 with a non-empty exact table or honest empty set

### Batch 75.2 — Conditional Portable Binding

- [x] Execute card 208 only when Research 222 admits a non-empty set.
- [x] bind only admitted exact `HarnessMode::Plan` rows through preparation,
      immutable plan/evidence, validation, and canonical argv

### Batch 75.3 — Route-Local Acceptance

- [x] Execute card 209 only after card 208.
- [x] prove dispatch, omission, rejection, reasoning/budget composition,
      child/replacement consistency, docs, API, and lifecycle truth

## Acceptance Criteria

- [x] only Research 222 deliver-now rows prepare
- [x] request, immutable plan, prepared evidence, driver, and every child argv
      agree on the selected mode
- [x] exact source proves selected Plan behavior is equivalent to portable
      `HarnessMode::Plan`
- [x] omission preserves exact `default` argv and behavior
- [x] unsupported, mismatched, drifting, overrideable, or behaviorally weaker
      rows reject before process work when knowable
- [x] current model, reasoning, budgets, safe mode, tool filters, session
      continuity, activity, terminal, cancellation, deadline, retention, and
      joined cleanup remain exact
- [x] no permission, configuration, filesystem/network, sandbox, shell,
      process, descendant, account, or billing claim widens
- [x] default QA performs no install, login, account inspection, provider
      prompt, tool execution, or paid request

## Lane Runway

- predecessor: g04.074 Cline headless model-selection evidence stop
- this milestone: exact Qwen headless Plan evidence and conditional portable
  binding
- execution topology: one serial worker lane, cards 207-209
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact source cannot prove complete Plan application or behavioral
  equivalence on any named point.
- Stop if Plan is parser-only, prompt-only, ambiently overrideable, conflicts
  with `--safe-mode`, or permits provider-driven widening during one child.
- Stop if runs, later turns, resume, reasoning-control children, or fresh
  replacement cannot reapply one immutable selected mode.
- Stop if deterministic proof needs login, account inspection, provider
  prompting, tool execution, paid work, ambient config mutation, or a live
  session.
- Stop if delivery needs raw configuration, public provider-mode vocabulary,
  writable authority, sibling-route changes, shared contract changes,
  currentness movement, or a breaking API.

## Batch Cards

- [207-qwen-headless-plan-mode-evidence.md](batch-cards/207-qwen-headless-plan-mode-evidence.md)
- [208-qwen-headless-plan-mode-binding.md](batch-cards/208-qwen-headless-plan-mode-binding.md)
- [209-qwen-headless-plan-mode-acceptance.md](batch-cards/209-qwen-headless-plan-mode-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 173 Qwen Headless 0.21.15 Identity](../../research/173-qwen-headless-0-21-15-identity.md)
- [Research 189 Qwen Headless Reasoning Effort](../../research/189-qwen-headless-reasoning-effort-evidence.md)
- [Research 198 Qwen Headless Turn And Tool Budgets](../../research/198-qwen-headless-turn-and-tool-budget-evidence.md)
- [Research 216 Qwen Headless 0.22.1 Identity](../../research/216-qwen-headless-0-22-1-identity.md)
- [Research 222 Qwen Headless Plan Mode](../../research/222-qwen-headless-plan-mode-evidence.md)
- [Contract 012 Interactive Session Options](../../contracts/012-interactive-session-options-and-callback-exchange.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Qwen Headless Prepared Integration](../../guides/qwen-headless-prepared-integration.md)
