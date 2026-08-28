# 251 Cline ACP Model-Selection Evidence

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: [g04.088 Fifth Parallel Per-Route Feature Qualification](../088-fifth-parallel-per-route-feature-qualification.md)
Depends on: g04.042; g04.074; g04.086; g04.087 closeout
Research: [248 Cline ACP Model-Selection Evidence](../../../research/248-cline-acp-model-selection-evidence.md)

## Goal

Determine whether exact qualified Cline ACP `3.0.55` exposes a closed
provider/model row that can be selected and confirmed before the first prompt
without ambient provider drift, open membership, silent fallback, or
unauthorized durable settings mutation. Promote Research 248 with a closed
table or honest empty set.

## Work

1. [ ] Keep route `cline.acp`, exact `3.0.55`, ACP v1 stdio, enterprise/local
       configured access posture, caller working resource, observational
       permissions, no auto-approve, and current lifecycle unchanged.
2. [ ] Use Research 221 only as headless contrast. Do not promote root
       `-m/--model`, provider resolution, request echoes, durable-write
       conclusions, or its empty set onto ACP without exact ACP proof.
3. [ ] Freeze tagged ACP source, schemas, initialize/session frames, model
       picker or config options, model/provider catalogue sources, selection
       methods, returned updates, and official docs.
4. [ ] Build a closed version/provider/model/operation/lifecycle table for new,
       follow-up, load, resume, and fresh replacement. Prove membership and
       route agreement before selection.
5. [ ] Determine whether one pre-prompt request selects the model and whether
       its response or update confirms the exact selected value. Classify
       absent, duplicate, foreign, defaulted, rejected, and drifted rows.
6. [ ] Trace any provider/model settings reads and writes. Separate session-
       private configuration from shared durable state under Contract 033.
7. [ ] Separate requested, advertised, selected, dispatched, accepted,
       effective, returned, observed, persisted, and restored truth. Prove
       omission retains current ACP frames and behavior.
8. [ ] Audit prepared input/evidence, ACP codec/session state, fixtures, guide,
       matrices, and API baseline without production changes.
9. [ ] Promote Research 248 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] exact provider/model/operation/lifecycle table or honest empty set exists
- [ ] a non-empty row closes membership, route agreement, pre-prompt selection,
      exact confirmation, persistence/restoration, failure, and omission
- [ ] headless evidence is not promoted onto ACP
- [ ] unsupported rows reject before provider effects or durable mutation
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-cline
effigy qa:northstar
git diff --check
```

## Stop Conditions

- provider remains ambient or model membership stays live, open, or fallback-prone
- no pre-prompt exact selected-value confirmation exists
- selection unavoidably mutates shared durable settings
- proof needs login, account inspection, live catalogue access, provider prompt,
  paid work, install/update, host mutation, or a shared-contract change

## Out Of Scope

Cline headless, caller provider selection, thinking, Plan delivery, Act/Yolo/
Zen, auto-approve, production binding, live provider work, currentness,
release, shared closeout, rollover, or g04 closure.
