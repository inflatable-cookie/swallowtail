# 218 llama.cpp Owned Reasoning Controls Acceptance

Status: complete
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.078 llama.cpp Owned Reasoning Controls](../078-llama-cpp-owned-reasoning-controls.md)
Depends on: card 217

## Goal

Prove exact admitted reasoning argv, preserved omission and context-size
behavior, fail-closed runtime/model/template rejection, and unchanged owned
server lifecycle, then produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, driver, fixture, facade, and
   scenario coverage for every Research 225 deliver-now row and rejection
   boundary.
2. Assert exact runtime/model-template/value membership, immutable
   prepared/driver agreement, canonical argv, and unchanged model-path
   handling.
3. Prove omission dispatches no reasoning arguments and every qualified
   context-size value retains exact ordering, bounds, and behavior.
4. Prove unqualified models/templates, invalid combinations, unqualified
   runtime points, stale evidence, raw values, duplicate intent, and
   mismatched low-level use reject before process work.
5. Keep requested, prepared, dispatched, parser-accepted, applied, effective,
   and observed truth separate in tests and docs. Claim only the evidence level
   Research 225 admits.
6. Assert host/port selection, readiness, working-resource authority,
   configuration, `AmbientHost`, provider state, activity, cancellation,
   deadline, terminal, failure, process ownership, and joined cleanup remain
   unchanged.
7. Update the llama.cpp guide, relevant route/feature matrices, changelog,
   Research 225, roadmap/card state, programme, triage, logs, indexes, and sole
   Next Task.
8. Regenerate and review the API baseline only when the public surface changes.
9. Run the complete named validation once for the batch. Record inherited
   doctor findings and exact drift.

## Acceptance Criteria

- [x] every admitted exact row dispatches canonical arguments
- [x] omission, context size, and unsupported rows retain or reject exact
      prior behavior
- [x] prepared, driver, runtime, model/template, and argv mismatches fail
      before process work
- [x] no inference effectiveness, isolation, resource, configuration,
      retention, or lifecycle claim widens
- [x] docs report no stronger reasoning state than exact evidence supports
- [x] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-llama-cpp
effigy validate:focused swallowtail-adapter-llama-cpp
effigy package:verify-affected swallowtail-adapter-llama-cpp
effigy check:examples
effigy package:api
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy doctor
git diff --check
```

## Stop Conditions

- any selected row cannot prove exact immutable dispatch and claimed behavior
- omission, context size, readiness, activity, or lifecycle regresses
- acceptance requires model download/load, prompting, inference, paid work,
  ambient config mutation, authority widening, or unrelated repair

## Out Of Scope

- another feature/route, portable reasoning work, currentness, publication,
  merge, generation rollover, or g04 closure
