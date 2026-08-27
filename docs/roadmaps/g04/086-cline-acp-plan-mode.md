# g04.086 Cline ACP Plan Mode

Status: ready
Owner: Tom
Created: 2026-08-27
Depends on: g04.085 closeout; promoted Research 240
Vision tags: harness mode, negotiated session options, route-local controls
Contract refs: 011, 012, 017, 023, 029, 033, 034, 037, 047, 052
Research: 146, 220, 240

## Problem

Research 240 closes one exact `cline.acp` `3.0.55` Plan row. The native ACP
session advertises `plan`, accepts it through `session/set_config_option`, and
returns `mode.currentValue = plan` before the first prompt. The first runtime
manager then builds from that stored mode.

Production still rejects every `harness_mode` on Cline ACP. It opens
`session/new`, ignores the returned mode/config snapshot, and returns a ready
session without selecting or confirming Plan.

## Goal

Bind only Research 240's exact new-session `HarnessMode::Plan` row. Require
advertisement, one correlated set-config request, and exact selected-value
confirmation before returning a usable session. Preserve omission, default Act
bytes, permissions, working-resource authority, lifecycle, and joined cleanup.

## Named Scope

- route `cline.acp`
- driver `swallowtail.cline.acp`
- package axis `cline.package`
- exact qualified package `3.0.55`
- ACP v1 stdio over `cline --acp`
- existing behavior revision `cline.acp.stdio-v1`
- local-account access, read-only working resource, observational permissions
- portable value `HarnessMode::Plan`
- native request `session/set_config_option` with `configId = mode`,
  `value = plan`

Provider `act`, root `--plan`, `session/set_mode`, post-start mode changes,
load/resume mutation, generic config, and Plan-to-Act switching remain out.

## Goals

- [ ] expose one optional typed Plan selection on `ClineSessionProfileInput`
- [ ] bind exact capability, constraint, plan, evidence, request, driver, and
      provider-session agreement
- [ ] require `session/new` to advertise one unambiguous selectable `plan`
- [ ] send one set-config request and require response
      `mode.currentValue = plan` before readiness
- [ ] preserve omission as current frames with no mode request or Plan claim
- [ ] retain the immutable selected mode through later turns and fresh
      context-losing replacement
- [ ] reject load/resume redeclaration and post-readiness mutation
- [ ] preserve permission, access, isolation, activity, terminal, failure,
      cancellation, and joined cleanup truth

## Non-Goals

- Cline headless `--plan`, model selection, thinking, Act/Yolo/Zen, or
  auto-approve
- generic ACP configuration, raw option ids/values, runtime mode mutation, or
  a portable `act` value
- `session/load`, resume, transcript restoration, provider-session deletion,
  or arbitrary retained-session mutation
- read-only, sandbox, filesystem, network, shell, process, descendant, model,
  account, or permission claims derived from Plan
- live provider work, currentness, contract changes, release, merge,
  generation rollover, or g04 closure

## Execution Plan

### Batch 86.1 — Prepared Binding

- [ ] execute card 242
- [ ] add only the exact typed Plan input and capability/plan/request binding
- [ ] negotiate and confirm Plan after `session/new` and before readiness
- [ ] preserve omission and the existing ACP behavior revision

### Batch 86.2 — Route-Local Acceptance

- [ ] execute card 243 after card 242
- [ ] prove positive, omission, drift, failure, turn, replacement, permission,
      lifecycle, docs, matrix, example, and API truth
- [ ] leave shared inventory, programme, indexes, and Next Task for the
      orchestrator after merge

## Acceptance Criteria

- [ ] only exact `cline.acp` `3.0.55` `HarnessMode::Plan` prepares
- [ ] request, capability constraint, plan, evidence, provider snapshot,
      set-config request, and confirmation agree exactly
- [ ] missing, malformed, duplicate, foreign, rejected, or mismatched option
      truth fails before a ready session or prompt
- [ ] omission sends no mode request and retains current default-Act frames
      without claiming a provider default as a selected value
- [ ] same-session turns retain the selected runtime posture; fresh
      context-losing replacement renegotiates the immutable selection
- [ ] load/resume and post-start mutation remain unsupported
- [ ] Plan does not widen permission, auto-approve, resource, configuration,
      isolation, tool, access, model, or account authority
- [ ] deterministic package validation uses no credential, account, provider
      prompt, install, update, or paid work

## Decision Gates

- Stop if the exact `session/new` snapshot cannot establish unique `plan`
  membership before selection.
- Stop if set-config confirmation cannot be tied to the selected session before
  a usable handle or first prompt.
- Stop if omission gains a mode request, selected/default claim, or changed
  provider bytes.
- Stop if later turns or fresh replacement can drift from the immutable Plan
  selection.
- Stop if delivery needs generic provider config, runtime mode mutation, a
  shared runtime/contract change, live provider work, or authority widening.

## Batch Cards

- [242 Cline ACP Plan-Mode Binding](batch-cards/242-cline-acp-plan-mode-binding.md)
- [243 Cline ACP Plan-Mode Acceptance](batch-cards/243-cline-acp-plan-mode-acceptance.md)

## Shared Closeout Deltas (orchestrator only)

After merge, update:

- original inventory item 41 and the active-delivery count
- programme, triage, roadmap/card/log indexes, and sole Next Task
- do not edit those shared surfaces on the worker branch

## References

- [Research 240 Cline ACP Plan-Mode Evidence](../../research/240-cline-acp-plan-mode-evidence.md)
- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Per-Route Feature Inventory](./per-route-feature-inventory.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Cline ACP Prepared Integration](../../guides/cline-acp-prepared-integration.md)
