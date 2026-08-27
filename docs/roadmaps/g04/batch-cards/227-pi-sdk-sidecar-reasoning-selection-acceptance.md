# 227 Pi SDK Sidecar Reasoning Selection Acceptance

Status: blocked pending card 226
Owner: Tom
Created: 2026-08-27
Milestone: [g04.081 Pi SDK Sidecar Reasoning Selection](../081-pi-sdk-sidecar-reasoning-selection.md)
Depends on: card 226

## Goal

Prove exact Pi SDK-sidecar reasoning selection, preserved omission, fail-closed
clamp/substitution handling, persistent-session lifecycle, and unchanged route
cleanup, then produce one review-ready closeout.

## Scope

1. Add deterministic preparation, protocol, driver, fixture, new/load/resume,
   replay, restoration, and scenario coverage for every Research 228 row and
   rejection boundary.
2. Assert exact provider/model/value membership, immutable
   option/capability/plan/evidence agreement, canonical `thinkingLevel`, and
   effective bootstrap/state confirmation.
3. Prove omission emits no `thinkingLevel`, retains existing Pi default/stored
   behavior, and advertises no portable reasoning selection.
4. Prove unsupported values/models/lifecycles, stale axes, malformed state,
   missing state, clamp, substitution, model fallback, stored-state conflict,
   and request-plan drift reject before provider work or readiness.
5. Prove new, load, resume, replacement, and fresh restoration retain exact
   session-option and durable-state meaning, including bounded replay before
   load readiness and replay-free resume.
6. Prove `thinking_level_changed` setup/rebind behavior cannot leak as an
   unexpected post-ready selection or silently mutate the chosen mode.
7. Preserve reasoning activity projection without presenting thought text as
   effective-selection evidence.
8. Prove attachments, steer/follow-up, cancellation, deadline, terminal
   ordering, resource/credential release, close/join, and durable provider
   state remain unchanged.
9. Update the Pi prepared guide, route/feature matrices, changelog, Research
   228, roadmap/card state, programme, triage, logs, indexes, and sole Next
   Task. Regenerate and review the API baseline if the public surface changes.
10. Run the complete named validation once for the batch. Record inherited
    doctor findings and exact drift.

## Acceptance Criteria

- [ ] every admitted row dispatches and confirms its exact effective value
- [ ] omission and every existing lifecycle retain prior behavior
- [ ] unsupported or mismatched rows fail before effects or readiness
- [ ] reasoning output remains separate from selection evidence
- [ ] durable sessions, replay, attachments, cancellation, and cleanup remain
      correct
- [ ] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-pi
effigy validate:focused swallowtail-adapter-pi
effigy package:verify-affected swallowtail-adapter-pi
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

- any admitted row cannot prove exact immutable selection and state agreement
- omission, replay, attachment, restoration, or cleanup regresses
- acceptance requires live provider work, credential/account inspection, or
  ambient configuration mutation

## Out Of Scope

- runtime mode changes, cycling, model switching, raw Pi settings, `pi.rpc`,
  newer SDK currentness, release, merge, generation rollover, or g04 closure
