# 231 Ollama Attached Think Max Evidence

Status: ready
Owner: Tom
Created: 2026-08-27
Milestone: [g04.082 Parallel Per-Route Feature Qualification](../082-parallel-per-route-feature-qualification.md)
Depends on: g04.036; g04.081 closeout

## Goal

Freeze exact Ollama attached `think: "max"` version, model, template,
dispatch, fallback, response, and lifecycle truth, then promote Research 232
with a non-empty deliver-now table or an honest empty set.

## Work

1. [ ] Keep route `ollama.attached`, native `/api/chat`, maintained
       `0.14.0..=0.32.15` claim with existing exclusions, selected-model detail,
       structured run, and private transcript-replay session unchanged.
2. [ ] Freeze official tagged source and documentation for the `think` request
       type and `max` value at every exact candidate version. Identify when the
       value appeared, parser behavior, template/model support, fallback,
       validation, and response thinking fields.
3. [ ] Determine whether `/api/show` or other already-bound static model detail
       advertises exact level membership rather than only generic `thinking`.
       Do not infer `max` from a family name or generic capability boolean.
4. [ ] Build a closed version/model/value/operation table for structured runs
       and interactive turns independently. Prove unsupported rows reject before
       chat dispatch and that replay/restoration keeps the selected value fixed.
5. [ ] Distinguish requested, encoded, server-accepted, template-applied,
       effective, and observed reasoning. Reasoning output alone is not selected
       level confirmation.
6. [ ] Prove omission and existing `off|low|medium|high` request bytes and
       behavior remain unchanged.
7. [ ] Audit prepared inputs/evidence, plan/request agreement, catalogue/detail
       parsing, validation, protocol encoder/decoder, fixtures, guide, matrices,
       and API baseline without changing production surfaces.
8. [ ] Promote Research 232 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] exact version/model/template membership or honest empty set is recorded
- [ ] structured-run and interactive lifecycle dispositions are explicit
- [ ] no generic thinking boolean is presented as exact `max` membership
- [ ] omission and existing four modes remain exact
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-ollama
effigy qa:northstar
git diff --check
```

## Stop Conditions

- exact `max` membership depends on model/template facts not exposed by the
  already-bound selected-model detail
- the server accepts then clamps, defaults, or substitutes without confirmation
- deterministic proof needs an installed runtime, model pull, prompt, network
  model download, or a shared-contract change

## Out Of Scope

Context size, new reasoning vocabulary, model installation, owned Ollama
runtime, production binding, live inference, currentness, release, merge,
shared closeout, rollover, or g04 closure.
