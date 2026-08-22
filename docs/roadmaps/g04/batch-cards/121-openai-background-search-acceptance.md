# 121 OpenAI Background Search Acceptance

Status: blocked; card 119 evidence stop
Owner: Tom
Created: 2026-08-22
Milestone: [g04.043 OpenAI Background Hosted Search](../043-openai-background-hosted-search.md)
Depends on: card 120

## Goal

Prove bounded OpenAI background provider-owned search across request,
streaming, retained lifecycle, and route-local documentation, then produce one
review-ready closeout.

## Scope

1. Add deterministic corpus and prepared-facade tests for every Research 191
   deliver-now request/control combination.
2. Assert exact tool type, positive total-call maximum, tool choice, source
   inclusion, model, facade, policy, capabilities, and request bytes.
3. Assert the absent path remains byte-for-byte tool-free.
4. Prove initial stream, one reattachment, retrieved terminal response,
   malformed/duplicate/foreign events, provider rejection, incomplete status,
   cancellation, deadline, deletion, detachment, and reconciliation behavior.
5. Prove search progress/activity fidelity and keep enablement, invocation,
   sources/citations, usage, billing, assistant output, and completion separate.
6. Preserve reasoning, structured-output, output-bound, request-correlation,
   usage, rate-limit, checkpoint, and cleanup proofs.
7. Update the OpenAI background guide, Research 191, cards 119-121, g04.043,
   the reserved route-local closeout, and the package-specific unreleased API
   baseline when applicable.
8. Record the exact required architecture, route/feature matrix, changelog,
   programme, index, matrix-assertion, and Next Task delta in the closeout log
   and PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [ ] every admitted request and failure class has deterministic coverage
- [ ] default QA performs no credential, account, external request, or paid work
- [ ] docs distinguish selected, invoked, sourced, completed, and observed truth
- [ ] no generic tool, network, search-option, or sibling-route surface appears
- [ ] closeout records PR/head truth without claiming merge
- [ ] worker changes stay inside the route-local boundary
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

- exact wire, activity, source, or retained-lifecycle truth cannot be proved
- docs would need to infer invocation, model use, citation delivery, or billing
- another Responses tool, route, currentness family, or contract enters scope

## Out Of Scope

- live provider verification, release, publication, or consumer work
- merge or restack authority
- shared architecture, route/feature matrices, programme/front doors/indexes,
  matrix assertions, `CHANGELOG.md`, and shared package lists

## Closeout

Not executed. Card 120 produced no binding because Research 191 has an empty
deliver-now set. There is no admitted request, stream, reattachment, retrieve,
activity, failure, or cleanup behavior to exercise. The adapter, fixtures,
guide, and public API baseline remain unchanged; the shared-surface delta is
recorded in the route-local closeout.
