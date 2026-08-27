# 228 Codex App-Server Model Verbosity Evidence

Status: ready
Owner: Tom
Created: 2026-08-27
Milestone: [g04.082 Parallel Per-Route Feature Qualification](../082-parallel-per-route-feature-qualification.md)
Depends on: g04.066; g04.081 closeout

## Goal

Freeze exact Codex app-server model-verbosity configuration, model membership,
precedence, lifecycle, dispatch, and confirmation truth, then promote Research
229 with a non-empty deliver-now table or an honest empty set.

## Work

1. [ ] Reuse Research 213 only as a source lead. Prove app-server separately;
       do not copy exec argv or its adapter-local binding.
2. [ ] Freeze exact tagged source and generated app-server protocol/config
       schemas at `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1` for
       `model_verbosity`, model metadata, defaults, unsupported behavior, and
       request construction.
3. [ ] Trace the selected route's new, import, load, resume, follow-up, and
       fresh restoration paths. Identify the exact per-session or per-turn
       configuration seam, precedence over ambient config, and whether the
       selected value is confirmed before readiness or turn effects.
4. [ ] Build a closed version/model/value/profile table. Distinguish parser
       acceptance, dispatched Responses `text.verbosity`, provider acceptance,
       effective verbosity, and observed output length.
5. [ ] Prove unsupported models, values, versions, providers, and profiles can
       reject before process, credential, resource, or app-server work.
6. [ ] Prove omission retains current app-server request/config bytes and
       ambient/default behavior without claiming caller selection.
7. [ ] Audit prepared session inputs/evidence, plan/request agreement, RPC
       encoder/decoder, load/resume/import, fixtures, guide, matrices, and API
       baseline without changing production surfaces.
8. [ ] Promote Research 229 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] exact app-server configuration and lifecycle seam is frozen
- [ ] exact version/model/value/profile table or honest empty set is recorded
- [ ] omission and every unsupported row have pre-effect dispositions
- [ ] requested, dispatched, accepted, effective, and observed truth stay separate
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-codex
effigy qa:northstar
git diff --check
```

## Stop Conditions

- selection exists only as ambient/global config or cannot be bound to the
  prepared session lifecycle
- exact model membership or unsupported behavior cannot reject before effects
- effective selection silently defaults, substitutes, or remains unconfirmed
- deterministic proof needs login, credential, provider work, install/update,
  or a shared-contract change

## Out Of Scope

Production binding, exec changes, Fast tier, personality, Plan effort,
multi-agent, live provider work, currentness, release, merge, shared closeout,
rollover, or g04 closure.
