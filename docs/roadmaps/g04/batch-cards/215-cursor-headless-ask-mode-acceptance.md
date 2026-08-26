# 215 Cursor Headless Ask-Mode Acceptance

Status: planned; conditional on card 214
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.077 Cursor Headless Ask Mode](../077-cursor-headless-ask-mode.md)
Depends on: card 214

## Goal

Prove exact Ask dispatch, preserved Plan/no-mode defaults, fail-closed access
and version rejection, model-parameter composition, and unchanged one-child
lifecycle, then produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, driver, fixture, facade, and
   scenario coverage for every Research 224 deliver-now row and rejection
   boundary.
2. Assert exact build/access/mode membership, immutable prepared/driver
   agreement, canonical `--mode ask`, and unchanged prompt secrecy.
3. Prove existing `Read` still dispatches exactly one `--mode plan` and
   `ReadWrite` still omits `--mode`.
4. Prove Ask plus `ReadWrite`, unqualified builds, stale prepared state,
   unsupported values, raw strings, duplicate mode intent, and mismatched
   low-level use reject before process work.
5. Compose Ask with every qualified Cursor model-parameter row without
   changing its membership, rendering, capability constraints, or one-model-
   argument dispatch.
6. Keep requested, prepared, dispatched, parser-accepted, applied, effective,
   and observed truth separate in tests and docs. Claim only the evidence level
   Research 224 admits.
7. Assert working-resource authority, ambient configuration, `AmbientHost`,
   trust, tools, permissions, provider retention, activity, usage,
   cancellation, deadline, terminal, failure, and joined cleanup remain
   unchanged.
8. Update the Cursor guide, relevant route/feature matrices, changelog,
   Research 224, roadmap/card state, programme, triage, logs, indexes, and sole
   Next Task.
9. Regenerate and review the API baseline only when the public surface changes.
10. Run the complete named validation once for the batch. Record inherited
    doctor findings and exact drift.

## Acceptance Criteria

- [ ] every admitted exact row dispatches canonical `--mode ask`
- [ ] Plan/no-mode defaults and unsupported rows retain or reject exact prior
      behavior
- [ ] prepared, driver, version, access, and argv mismatches fail before
      process work
- [ ] Ask composes with all qualified model parameters without semantic drift
- [ ] no isolation, permission, tool, resource, configuration, account,
      retention, or lifecycle claim widens
- [ ] docs report no stronger mode state than exact evidence supports
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

- any selected row cannot prove exact immutable Ask dispatch and claimed
  behavior
- Plan/no-mode defaults, model parameters, access, activity, or lifecycle
  regress
- acceptance requires provider prompting, tool execution, paid work, ambient
  config mutation, authority widening, or unrelated repair

## Out Of Scope

- another feature/route, portable mode work, currentness, publication, merge,
  generation rollover, or g04 closure
