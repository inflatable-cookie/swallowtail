# 109 xAI Responses Control Acceptance

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.039 xAI Responses Reasoning And Output Bounds](../039-xai-responses-reasoning-output-bounds.md)
Depends on: card 108

## Goal

Prove exact xAI reasoning and output-bound dispatch across one-response and
connection-local continuation profiles, then produce a review-ready route
closeout for orchestrator integration.

## Scope

1. Add deterministic prepared, protocol, and driver tests for every Research
   187 deliver-now model/value/profile/control row.
2. Assert input, plan constraints, evidence, configured driver,
   `reasoning.effort`, and `max_output_tokens` agree exactly.
3. Assert absent controls preserve the current fixture request body.
4. Assert aliases, unsupported models/values/bounds, multi-agent semantics, raw
   values, and mismatches fail before network work.
5. Prove one fixed session selection across first response, later turns, failed
   turns, connection loss, and fresh replacement.
6. Preserve model/facade, store-disabled continuation, endpoint, credential,
   cancellation, failure, usage, billed-cost, and cleanup proofs.
7. Update the xAI prepared guide, Research 187, cards 107-109, g04.039, reserved
   route closeout log, and package-specific public API baseline.
8. Record the exact required architecture, route/feature matrix, changelog,
   programme, index, and Next Task delta in the closeout log and PR body. Do not
   edit those shared surfaces on the worker branch.
9. Do not select or compile the next route family.

## Acceptance Criteria

- [x] every deliver-now value, bound, profile, and failure class is covered
- [x] continuation and fresh replacement preserve exact fixed selection
- [x] default QA sends no provider request
- [x] docs distinguish request, plan, dispatch, acceptance, and effectiveness
- [x] no sibling xAI route or model gains a capability by inference
- [x] closeout records PR/head truth without claiming merge
- [x] worker changed files stay inside the route-local boundary
- [x] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-xai
effigy validate:focused swallowtail-adapter-xai
effigy package:verify-affected swallowtail-adapter-xai
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

- exact agreement or continuation invariants cannot be proved
- docs would need to infer provider acceptance, effective reasoning, or exact
  output length
- another model, route, search/tool surface, or currentness family enters scope

## Out Of Scope

- live provider verification, release, publication, or consumer changes
- next-route planning or implementation
- merge or restack authority
- `CHANGELOG.md`; `docs/architecture/system-architecture.md`; route/feature
  matrices; programme and roadmap front doors; shared indexes; `packages.txt`

## Closeout

Research 187's exact rows are covered by prepared-input, capability-plan,
evidence, driver, and protocol tests. The tests prove independent reasoning
and output controls, absent-control compatibility, exact first/later/fresh
session dispatch, unsupported model/value/bound rejection, and request-plan
drift rejection before network or credential effects. No provider acceptance,
effective reasoning depth, or exact text-length claim is made.
