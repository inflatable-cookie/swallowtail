# 254 Kiro ACP Effort Evidence

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: [g04.089 Sixth Parallel Per-Route Feature Qualification](../089-sixth-parallel-per-route-feature-qualification.md)
Depends on: g03.093-g03.096; g04.088 closeout
Research: [251 Kiro ACP Effort Evidence](../../../research/251-kiro-acp-effort-evidence.md)

## Goal

Freeze exact Kiro ACP effort version, model, value, spawn/session application,
confirmation, lifecycle, and omission truth. Promote Research 251 with a
closed deliver-now table or an honest empty set.

## Work

1. [ ] Keep route `kiro.acp`, exact `2.18.1`, host-approved executable and
       environment, provider-supported host account state, current ACP
       lifecycle, and current permission posture unchanged.
2. [ ] Freeze official `--effort low|medium|high|xhigh|max` documentation plus
       exact package/source parser, precedence, model dependence, ACP spawn,
       session configuration, request use, returned state, and failures.
3. [ ] Determine whether effort belongs before ACP startup, at session open,
       or to a model-selection surface. Do not infer support from an
       unsupported `session/set_model` method.
4. [ ] Build a closed version/model/value/lifecycle table. Separate requested,
       parsed, dispatched, accepted, effective, returned, and observed truth.
5. [ ] Prove unsupported model/value rows reject before provider effects;
       prove omission retains exact `kiro-cli acp` argv and behavior.
6. [ ] Audit prepared sessions, spawn plan/evidence, ACP fixtures, guide,
       matrices, and API baseline without production changes.
7. [ ] Promote Research 251 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] exact version/model/value/lifecycle table or honest empty set exists
- [ ] a non-empty row closes membership, application, confirmation, cleanup,
      and omission
- [ ] headless or interactive CLI evidence is not silently promoted onto ACP
- [ ] unsupported rows reject before provider effects
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-kiro
effigy qa:northstar
git diff --check
```

## Stop Conditions

- effort membership depends on live model/account state
- ACP startup or session bytes do not carry and confirm the selection
- proof needs login, credentials, provider prompts, paid work, install/update,
  host mutation, or shared-contract change

## Out Of Scope

Kiro headless, agent profile, cloud sessions, trust-all tools, model routing,
production binding, live provider work, currentness, release, shared closeout,
rollover, or g04 closure.
