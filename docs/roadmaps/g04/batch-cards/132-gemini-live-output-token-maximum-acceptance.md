# 132 Gemini Live Output-Token-Maximum Acceptance

Status: done
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.047 Gemini Live Output-Token Maximum](../047-gemini-live-output-token-maximum.md)
Depends on: card 131

## Goal

Prove exact Gemini Live output-maximum dispatch, preserved omission, reasoning
composition, and continuity behavior, then produce one review-ready route-local
closeout.

## Outcome

Deterministic coverage proves domain bounds `1`, `1024`, and `65536`, rejection
of `65537`, omission byte preservation, reasoning composition, one planned
rollover, and fresh restoration. Route-local guide, example, Research 194,
cards, milestone, and closeout record the delivery; shared matrices remain
deferred to orchestrator merge.

## Scope

1. Add deterministic prepared-facade and protocol coverage for the Research
   194 minimum, representative, maximum, and rejected domain boundaries.
2. Assert exact model, facade point, private behavior revision, capability,
   constraint, plan, evidence, request, initial setup, and resumed setup.
3. Assert omission retains the current exact initial and resume fixture bytes
   without claiming an output-token maximum.
4. Prove one planned rollover and fresh working-state restoration keep the
   selected maximum while preserving handle privacy and connection-state truth.
5. Prove the maximum composed with omitted reasoning and each admitted
   `minimal|low|medium|high` selection, without changing either field.
6. Prove out-of-domain maximum, capability omission, constraint mismatch,
   request drift, facade/model drift, provider failure, malformed event,
   deadline, cancellation, disconnect, rollover failure/exhaustion, and cleanup
   behavior.
7. Preserve manual PCM formats, output transcription, activity projection,
   usage, active-response interruption, no provider storage, and the exact
   project authorization API-key boundary.
8. Update the realtime prepared-integration guide, Research 194, cards 130-132,
   g04.047, the reserved route-local closeout, examples, and package-specific
   unreleased API baselines when applicable.
9. Record the exact required architecture, route/feature matrix, programme,
   index, changelog, matrix-assertion, Contract 029, and Next Task delta in the
   closeout and PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [x] every admitted boundary and rejected value class has deterministic
      coverage
- [x] initial, rollover/resume, fresh restoration, composition, and omission
      bytes are exact
- [x] default QA performs no credential, account, external request, provider
      prompt, or paid work
- [x] docs distinguish requested, planned, and dispatched maximum from accepted
      and effective generated length
- [x] no token-count, truncation, sibling-route, or compatibility point is
      inferred
- [x] closeout records PR/head truth without claiming merge
- [x] worker changes stay inside the named code and route-local docs boundary
- [x] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-gemini
effigy validate:focused swallowtail-adapter-gemini
effigy package:verify-affected swallowtail-adapter-gemini
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

- exact value, setup, continuity, composition, lifecycle, or cleanup truth
  cannot be proved
- docs would need to infer provider acceptance or effective generated length
- another route, control family, currentness lane, contract, or release enters
  scope

## Out Of Scope

- live provider verification, publication, merge, shared front-door edits, or
  later feature selection
