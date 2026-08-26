# 209 Qwen Headless Plan-Mode Acceptance

Status: planned; conditional on card 208
Owner: Tom
Created: 2026-08-26
Milestone: [g04.075 Qwen Headless Plan Mode](../075-qwen-headless-plan-mode.md)
Depends on: card 208

## Goal

Prove exact portable Plan dispatch, preserved omission, fail-closed rejection,
behavior/isolation separation, and unchanged multi-child lifecycle, then
produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, driver, fixture, facade, and
   scenario coverage for every Research 222 deliver-now row and rejection
   boundary.
2. Assert exact package/revision/mode membership, immutable plan/evidence
   agreement, canonical `--approval-mode plan`, and unchanged prompt secrecy.
3. Prove omission retains exact `--approval-mode default` bytes and behavior.
4. Prove selected Plan on structured runs, reasoning-control children, first
   turns, continued turns, explicit resume, and fresh replacement. Include
   version, plan, request, session, and child-argv mismatch rejection before
   process work.
5. Compose Plan with every admitted model, reasoning, and budget row without
   widening any feature. Prove unsupported combinations fail closed.
6. Assert `--safe-mode`, core/excluded tools, read-only working resource,
   ambient configuration/isolation, delegated access, activity, usage,
   cancellation, deadline, terminal, failure, retention, and joined cleanup
   remain unchanged.
7. Update the Qwen prepared guide, route/feature matrices where truth changes,
   architecture only when required, changelog, Research 222, roadmap/card
   state, programme, triage, logs, indexes, and sole Next Task.
8. Regenerate and review the API baseline only when the public surface changes.
9. Run the complete named validation once for the batch. Record inherited
   doctor findings and any exact drift.

## Acceptance Criteria

- [ ] every admitted exact row dispatches canonical Plan on every child shape
- [ ] omission and all unsupported rows retain or reject exact prior behavior
- [ ] request, plan, evidence, driver, replacement, and argv mismatches fail
      before process work
- [ ] Plan composes with model/reasoning/budgets without changing their
      membership or semantics
- [ ] no permission, tool, resource, configuration, isolation, sandbox,
      shell/process/network, account, retention, or lifecycle claim widens
- [ ] docs and matrices distinguish requested/dispatched/applied/effective/
      observed truth
- [ ] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-qwen
effigy validate:focused swallowtail-adapter-qwen
effigy package:verify-affected swallowtail-adapter-qwen
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

- any selected row or child shape cannot prove exact immutable Plan dispatch
- omission, safe mode, tool filters, reasoning, budgets, continuation, or
  replacement regresses
- acceptance requires provider prompting, tool execution, paid work, ambient
  config mutation, authority widening, or unrelated repair

## Out Of Scope

- another feature/route, currentness, release, publication, merge, generation
  rollover, or g04 closure

