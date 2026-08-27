# 247 Gemini CLI ACP Sandbox Evidence

Status: done
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.087 Fourth Parallel Per-Route Feature Qualification](../087-fourth-parallel-per-route-feature-qualification.md)
Depends on: g04.034; g04.083; g04.085; g04.086 closeout
Research: [244 Gemini CLI ACP Sandbox Evidence](../../../research/244-gemini-cli-acp-sandbox-evidence.md)

## Goal

Freeze exact Gemini CLI ACP sandbox version, backend, spawn, configuration,
activation, confirmation, authority, and lifecycle truth, then promote
Research 244 with a closed deliver-now table or an honest empty set.

## Work

1. [x] Keep route `gemini-cli.acp`, exact qualified `0.51.0..=0.56.0`,
       enterprise Developer API-key access, ACP v1 stdio, current read/read-
       write profiles, Plan selection, and lifecycle unchanged.
2. [x] Reuse Research 239 only as sibling headless sandbox evidence. Prove the
       ACP spawn and child lifecycle independently; do not copy its conclusion.
3. [x] Freeze official sandbox/ACP documentation plus exact tagged parser,
       config precedence, backend selection, platform gates, ACP entry/spawn,
       re-exec behavior, initialization/session frames, tests, and defaults.
4. [x] Build a closed version/platform/backend/value table. Determine whether
       ACP can select sandboxing process-privately and confirm backend activation
       before session readiness without installing or starting a backend.
5. [x] Separate requested, argv/environment/settings encoded, parent parsed,
       backend started, ACP child connected, accepted, effective, contained,
       and observed truth.
6. [x] Prove unsupported version/platform/backend/value rows reject before
       credentials, resource authority, provider work, or partial ownership.
7. [x] Prove omission retains current ACP argv/environment and makes no sandbox,
       backend, isolation, filesystem, network, or process-containment claim.
8. [x] Audit prepared inputs/evidence, plan/request agreement, ACP fixtures,
       guide, matrices, and API baseline without changing production surfaces.
9. [x] Promote Research 244 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [x] exact ACP version/platform/backend table or honest empty set is recorded
- [x] a non-empty row proves process-private precedence and ACP-child activation
      before readiness without flattening selection into containment
      (vacuous: empty set; gates recorded)
- [x] headless evidence is used only as route-local contrast
- [x] omission and enterprise API-key access stay exact
- [x] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Outcome

Honest empty deliver-now set. Research 244 promoted. Frozen evidence under
`crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-0.56.0-sandbox/`.

Ambient `GEMINI_SANDBOX` still overrides argv/settings. Sandbox re-exec and
optional non-TTY stdin drain happen in the parent before `runAcpClient`.
Initialize/`session/new` expose no sandbox field. Selection is not containment.

## Validation

```sh
effigy validate:focused swallowtail-adapter-gemini
effigy qa:northstar
git diff --check
```

## Stop Conditions

- ambient environment/settings can override the caller selection
- activation cannot be confirmed prompt-free or requires backend startup
- proof conflates backend selection with containment or access authority
- proof needs consumer OAuth, login, credentials, provider prompt, paid work,
  install/update, host mutation, production code, or shared-contract changes

## Out Of Scope

Gemini headless or Live, thinking, output limits, consumer-account login,
portable containment guarantees, production binding, live provider work,
currentness, release, shared closeout, rollover, or g04 closure.
