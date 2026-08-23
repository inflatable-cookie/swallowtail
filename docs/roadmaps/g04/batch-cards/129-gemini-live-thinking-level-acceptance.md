# 129 Gemini Live Thinking-Level Acceptance

Status: conditional; awaiting card 128
Owner: Tom
Created: 2026-08-23
Milestone: [g04.046 Gemini Live Thinking Levels](../046-gemini-live-thinking-levels.md)
Depends on: card 128

## Goal

Prove exact Gemini Live thinking-level dispatch, preserved omission, and
continuity behavior, then produce one review-ready route-local closeout.

## Scope

1. Add deterministic prepared-facade and protocol coverage for every Research
   193 deliver-now value and every rejected portable value.
2. Assert exact model, facade point, private behavior revision, capability,
   constraint, plan, evidence, request, initial setup, and resumed setup.
3. Assert omission retains the current exact `MINIMAL` initial and resume
   fixture bytes without claiming caller reasoning selection.
4. Prove one planned rollover and fresh working-state restoration keep the
   selected level while preserving handle privacy and connection-state truth.
5. Prove invalid value, capability omission, constraint mismatch, request
   drift, facade/model drift, provider failure, malformed event, deadline,
   cancellation, disconnect, rollover failure/exhaustion, and cleanup behavior.
6. Preserve manual PCM formats, output transcription, activity projection,
   usage, active-response interruption, no provider storage, and the exact
   project authorization API-key boundary.
7. Update the realtime prepared-integration guide, Research 193, cards 127-129,
   g04.046, the reserved route-local closeout, examples, and package-specific
   unreleased API baselines when applicable.
8. Record the exact required architecture, route/feature matrix, programme,
   index, changelog, matrix-assertion, and Next Task delta in the closeout and
   PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [ ] every admitted and rejected value has deterministic coverage
- [ ] initial, rollover/resume, fresh restoration, and omission bytes are exact
- [ ] default QA performs no credential, account, external request, provider
      prompt, or paid work
- [ ] docs distinguish requested, planned, and dispatched from accepted and
      effective reasoning
- [ ] no thought-summary disclosure, numeric budget, sibling-route behavior,
      or compatibility point is inferred
- [ ] closeout records PR/head truth without claiming merge
- [ ] worker changes stay inside the named code and route-local docs boundary
- [ ] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-runtime -p swallowtail-adapter-gemini -p swallowtail-adapter-openai
effigy validate:focused swallowtail-runtime swallowtail-adapter-gemini swallowtail-adapter-openai
effigy package:verify-affected swallowtail-runtime swallowtail-adapter-gemini swallowtail-adapter-openai
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

- exact value, setup, continuity, lifecycle, or cleanup truth cannot be proved
- docs would need to infer provider acceptance, effectiveness, or thought
  disclosure
- another route, control family, currentness lane, contract, or release enters
  scope

## Out Of Scope

- live provider verification, publication, merge, shared front-door edits, or
  later feature selection

