# 241 Cline ACP Plan-Mode Evidence

Status: done
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.085 Parallel Per-Route Feature Qualification III](../085-third-parallel-per-route-feature-qualification.md)
Depends on: g04.073; g04.084 closeout
Research: [240 Cline ACP Plan-Mode Evidence](../../../research/240-cline-acp-plan-mode-evidence.md)

## Goal

Freeze exact Cline ACP Plan-mode version, selection, application, confirmation,
permission, and lifecycle truth, then promote Research 240 with a closed
deliver-now table or an honest empty set.

## Work

1. [x] Keep route `cline.acp`, exact qualified package `3.0.55`, ACP v1 stdio,
       caller working resource, observational permissions, no auto-approve, and
       current load/resume/cleanup behavior unchanged.
2. [x] Reuse Research 220 only as a sibling-route contrast: exact root
       `--plan` is applied by headless but discarded by the ACP early-return.
       Do not promote that row onto ACP.
3. [x] Freeze exact tagged ACP source, schemas, initialize/session frames,
       session options, config methods, commands, tests, and official docs for
       any Plan/Act selection and confirmation surface.
4. [x] Determine whether initialize, session creation, a config option, an ACP
       command, or another exact pre-prompt operation lets the caller select
       Plan and confirms the applied value without ambient settings authority.
5. [x] Build a closed version/value/operation table for new, follow-up, load,
       resume, and fresh replacement. Separate portable `HarnessMode::Plan`
       from Cline naming when Contract 034 does not authorize the mapping.
6. [x] Separate requested, negotiated, command/config-selected, dispatched,
       accepted, effective, returned, and observed mode truth. Plan is provider
       behavior, not read-only access, sandboxing, or permission authority.
7. [x] Prove unsupported rows reject before process, resource, credential, or
       provider work. Prove omission retains current ACP frames and no Plan claim.
8. [x] Audit prepared inputs/evidence, plan/request agreement, ACP codec and
       session state, fixtures, guide, matrices, and API baseline without
       changing production surfaces.
9. [x] Promote Research 240 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [x] exact ACP selection/confirmation seam or honest empty set is recorded
- [x] exact version/value/lifecycle table is complete
- [x] headless argv evidence is not promoted onto ACP
- [x] omission, permissions, working resource, and lifecycle stay exact
- [x] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-cline
effigy qa:northstar
git diff --check
```

## Stop Conditions

- Plan exists only on the discarded root argv or ambient provider settings
- no pre-prompt selected-value confirmation exists on the exact ACP path
- mapping would widen permission, access, sandbox, or containment claims
- proof needs login, credential, provider prompt, paid work, install/update,
  host configuration mutation, or a shared-contract change

## Out Of Scope

Cline headless, thinking, model selection, Act/Yolo/Zen, auto-approve,
production binding, live provider work, currentness, release, merge, shared
closeout, rollover, or g04 closure.

## Outcome

Research 240 promotes one deliver-now row: exact `cline.acp` `3.0.55`
`HarnessMode::Plan` via `session/set_config_option` `{ configId: "mode",
value: "plan" }` after `session/new`, confirmed by response
`mode.currentValue`, applied before the first `session/prompt`. Root `--plan`
stays headless-only. Binding remains a later card.
