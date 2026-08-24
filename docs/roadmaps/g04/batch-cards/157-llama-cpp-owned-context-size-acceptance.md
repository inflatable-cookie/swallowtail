# 157 llama.cpp Owned Context-Size Acceptance

Status: complete
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.056 llama.cpp Owned Context Size](../056-llama-cpp-owned-context-size.md)
Depends on: card 156

## Goal

Prove the exact admitted owned-serving context-size boundary and publish
route-local guidance without overstating effective allocation, model fit,
inference capacity, output, quality, latency, cost, or billing truth.

## Scope

1. Add deterministic preparation, immutable-state, driver, launch, readiness,
   failure, stop, and cleanup coverage for every Research 203 deliver-now
   state, omission, and rejected boundary.
2. Prove exact selected-value agreement across input, immutable start evidence
   or specification, driver, and launch arguments.
3. Prove caller omission retains the current command with no `--ctx-size`
   member and explicit zero cannot silently stand in for omission.
4. Prove only the Research 203 requested, dispatched, accepted, effective, and
   observed states. Where no exact surface confirms the applied value, docs
   must say so.
5. Prove model artifact/alias, loopback port zero, offline, no-UI, no-agent,
   readiness, access, host deadline, endpoint authority, and one-child
   lifecycle do not change.
6. Prove launch rejection, early exit, readiness timeout, build/route mismatch,
   cancellation, stop, endpoint invalidation, and artifact release retain
   joined ordering.
7. Update the llama.cpp prepared guide, Research 203, cards 155-157, g04.056,
   reserved route-local closeout, examples, fixtures, and package-specific
   unreleased API baseline when applicable.
8. Record the exact architecture, Contract 029, route/feature matrix,
   programme, indexes, changelog, and sole Next Task delta in the closeout and
   PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [x] every admitted state and rejected boundary has deterministic coverage
- [x] omission and selected dispatch truth remain exact and distinct
- [x] docs do not infer effective allocation, model fit, inference capacity,
      output, quality, latency, cost, or billing from dispatch or readiness
- [x] default QA performs no download, install, model load, server launch,
      external request, credential, or paid work
- [x] closeout records PR/head truth without claiming merge
- [x] worker changes stay inside named code and route-local docs
- [x] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-llama-cpp
effigy validate:focused swallowtail-adapter-llama-cpp
effigy package:verify-affected swallowtail-adapter-llama-cpp
effigy check:examples
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy package:api
git diff --check
```

Auto-continuation: No.

## Stop Conditions

- exact launch, application-state, readiness, lifecycle, cancellation, or
  cleanup truth cannot be proved deterministically
- docs would infer effective allocation, model fit, inference capacity,
  output, quality, latency, cost, or billing
- another route/control family, currentness lane, contract, release,
  generation rollover, or g04 closure enters scope

## Out Of Scope

- live model verification, shared front-door edits, publication, merge,
  generation rollover, or g04 closure
