# 212 Cursor Headless Provider-Sandbox Acceptance

Status: blocked; card 211 blocked
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.076 Cursor Headless Provider Sandbox](../076-cursor-headless-provider-sandbox.md)
Depends on: card 211

## Goal

Prove exact provider-sandbox dispatch, preserved ambient omission, fail-closed
platform/configuration rejection, access/mode separation, and unchanged
one-child lifecycle, then produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, driver, platform, fixture, facade,
   and scenario coverage for every Research 223 deliver-now row and rejection
   boundary.
2. Assert exact build/platform/access/isolation membership, immutable
   plan/evidence agreement, canonical `--sandbox enabled`, and unchanged prompt
   secrecy.
3. Prove omission retains exact no-flag argv and `AmbientHost` behavior.
4. Prove `Read` still selects `--mode plan`; `ReadWrite` still omits mode;
   neither profile infers isolation from access, trust, permissions, or tools.
5. Prove missing backend/platform/configuration facts, mismatched plans,
   unqualified builds, unsupported values, and fallback shapes reject before
   process work.
6. Compose the admitted sandbox with every qualified Cursor model-parameter
   row without widening its membership or semantics.
7. Assert working-resource authority, ambient configuration and durable state,
   activity, usage, cancellation, deadline, terminal, failure, and joined
   cleanup remain unchanged.
8. Update the Cursor guide, route/feature matrices where truth changes,
   changelog, Research 223, roadmap/card state, programme, triage, logs,
   indexes, and sole Next Task.
9. Regenerate and review the API baseline only when the public surface changes.
10. Run the complete named validation once for the batch. Record inherited
    doctor findings and exact drift.

## Acceptance Criteria

- [ ] every admitted exact row dispatches canonical `--sandbox enabled`
- [ ] omission and unsupported rows retain or reject exact prior behavior
- [ ] request, plan, evidence, driver, platform, and argv mismatches fail before
      process work
- [ ] isolation composes with access, Plan, and model parameters without
      changing their membership or semantics
- [ ] no permission, tool, resource, configuration, host-isolation, account,
      retention, or lifecycle claim widens
- [ ] docs distinguish requested/dispatched/backend-active/enforced/effective/
      observed truth
- [ ] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-cursor
effigy validate:focused swallowtail-adapter-cursor
effigy package:verify-affected swallowtail-adapter-cursor
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

- any selected row cannot prove exact immutable provider-enforced isolation
- omission, access, Plan, model parameters, activity, or lifecycle regresses
- acceptance requires provider prompting, tool execution, paid work, ambient
  config mutation, authority widening, or unrelated repair

## Out Of Scope

- another feature/route, currentness, release, publication, merge, generation
  rollover, or g04 closure
