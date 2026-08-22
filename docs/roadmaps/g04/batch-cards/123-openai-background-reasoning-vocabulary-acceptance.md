# 123 OpenAI Background Reasoning Vocabulary Acceptance

Status: planned after card 122
Owner: Tom
Created: 2026-08-23
Milestone: [g04.044 OpenAI Background Reasoning Vocabulary Correction](../044-openai-background-reasoning-vocabulary-correction.md)
Depends on: card 122

## Goal

Prove the corrected exact GPT-5.6 reasoning vocabulary and produce one
review-ready route-local closeout with an explicit next-minor delta.

## Scope

1. Add deterministic table coverage for `none`, `low`, `medium`, `high`,
   `xhigh`, and `max` across preparation, plan constraint, evidence, request
   policy, driver validation, and exact request bytes.
2. Add explicit `minimal` rejection coverage. Assert the stable safe diagnostic
   and zero endpoint, credential, request, and provider effects.
3. Prove absent reasoning retains the current request shape and lifecycle.
4. Prove old/new facade, plan, evidence, request-policy, driver, model, and
   route drift fail closed at the earliest knowable boundary.
5. Preserve structured-output, output-bound, retained lifecycle, stream,
   cancellation, deletion, detachment, reconciliation, usage, rate-limit, and
   cleanup proofs.
6. Update the OpenAI background guide, Research 191 follow-up disposition,
   g04.043 named follow-up, cards 122-123, g04.044, the reserved route-local
   closeout, and package-specific unreleased API baseline only when the
   semantic API tool requires it.
7. Record the exact required architecture, route/feature matrix, changelog,
   Contract 036/release-note, programme, index, matrix-assertion, and Next Task
   delta in the closeout and PR body. Do not edit those shared surfaces on the
   worker branch.

## Acceptance Criteria

- [ ] all six admitted values and failure classes have deterministic coverage
- [ ] `minimal` cannot prepare, dispatch, or reach provider work
- [ ] no alias, clamp, default, retry, fallback, or sibling route appears
- [ ] global `ReasoningMode` and other route-qualified values remain unchanged
- [ ] exact facade and behavior revisions are visible and consistently bound
- [ ] default QA performs no credential, account, external request, or paid work
- [ ] route-local docs describe the corrected exact set and breaking status
- [ ] closeout records PR/head truth without claiming merge or release
- [ ] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-openai
effigy validate:focused swallowtail-adapter-openai
effigy package:verify-affected swallowtail-adapter-openai
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

- exact request or facade truth cannot be proved without live provider work
- the correction leaks into another route or global value syntax
- docs would need to claim provider-effective reasoning depth
- release selection or another feature family enters scope

## Out Of Scope

- OpenAI web search or any other new feature
- live provider verification, currentness, release, publication, or consumer work
- merge or restack authority
- contracts, `CHANGELOG.md`, release notes and versions, shared architecture,
  route/feature matrices, programme/front doors/indexes, matrix assertions,
  and shared package lists
