# 217 llama.cpp Owned Reasoning Controls Binding

Status: complete
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.078 llama.cpp Owned Reasoning Controls](../078-llama-cpp-owned-reasoning-controls.md)
Depends on: card 216; promoted Research 225 with a non-empty deliver-now set

## Goal

Bind only Research 225's exact reasoning rows through closed llama.cpp-local
types, immutable prepared state, exact-runtime and model/template validation,
and canonical owned-server argv.

## Scope

1. Add only the adapter-local closed types Research 225 admits. Do not expose
   raw strings/numbers, generic reasoning effort, provider-neutral budget, or
   configuration maps.
2. Preserve existing construction as exact omission. Add a fallible typed
   selection path only for admitted selection, budget, or composed rows.
3. Retain admitted values and their exact model/template evidence immutably in
   the prepared result and low-level owned-driver binding. Keep context-size
   evidence independent and composable.
4. Validate exact runtime revision, model/template applicability, prepared
   state, driver state, and command intent before process work. Reject missing,
   stale, mismatched, unsupported, or evidence-gated rows.
5. Dispatch exactly the canonical arguments Research 225 admits. Never infer
   model capability, accept caller strings, rely on ambient templates, or fall
   back to another reasoning state after rejection.
6. Preserve exact no-flag argv for omission and exact context-size argv for
   every current row. Keep model path, host/port, readiness, working resource,
   configuration, `AmbientHost`, and provider-state posture unchanged.
7. Preserve activity, cancellation, deadline, terminal, failure, process
   ownership, and joined cleanup. Advance an adapter-private behavior revision
   only when exact evidence requires it.

## Acceptance Criteria

- [x] only Research 225 deliver-now rows prepare reasoning controls
- [x] public seams are closed and llama.cpp-local; no portable or raw API
      appears
- [x] prepared state, driver state, exact runtime, model/template evidence,
      and argv agree
- [x] omission and all context-size rows retain exact prior behavior
- [x] unadmitted reasoning values are unconstructible; prepared, access,
      host-service, deadline, and model-route mismatches reject before process
      work; the exact executable build is verified after launch
- [x] docs claim no stronger applied, effective, or observed state than exact
      evidence supports
- [x] serving ownership, readiness, retention, lifecycle, and cleanup do not
      widen

## Realized Boundary

Scope item 4 is realized as far as this route allows, and no further. The
closed one-variant type makes every unadmitted reasoning value unconstructible,
so there is no new pre-dispatch reject class. Prepared, access, host-service,
deadline, and model-route validation keeps rejecting before artifact
acquisition and process start. Exact runtime revision is checked as declared
configuration during preflight; the actual executable build is verified only
after launch through `/props`. Research 225 found no preflight model/template
source, and the admitted row needs none because its applied server state is
template-independent.

## Validation

```sh
cargo fmt -p swallowtail-adapter-llama-cpp
effigy validate:focused swallowtail-adapter-llama-cpp
effigy package:verify-affected swallowtail-adapter-llama-cpp
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 218 only when typed preparation, exact evidence gating,
canonical argv, omission compatibility, rejection, and lifecycle proof pass.

## Stop Conditions

- immutable adapter-local state cannot bind the admitted row without a shared
  plan or breaking public change
- model/template evidence can drift or a value can become inert after
  preparation
- implementation needs raw configuration, live inference, sibling-route work,
  shared contract/runtime change, or authority widening

## Out Of Scope

- portable reasoning promotion, another llama.cpp feature/route, live model
  work, currentness, release, merge, rollover, or g04 closure
