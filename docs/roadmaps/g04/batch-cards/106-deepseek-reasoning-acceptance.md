# 106 DeepSeek Reasoning Acceptance

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.038 DeepSeek Continuation Reasoning Controls](../038-deepseek-continuation-reasoning-controls.md)
Depends on: card 105

## Goal

Prove exact DeepSeek reasoning dispatch and continuation invariants, then
produce the review-ready route closeout for orchestrator integration.

## Scope

1. Add deterministic prepared and protocol tests for every Research 186
   deliver-now value, field combination, profile, and attempt shape.
2. Assert input, plan constraint, evidence, configured driver,
   `reasoning_effort`, and `thinking.type` agree exactly.
3. Assert the existing high/enabled fixture path remains byte-for-byte stable
   when no additive selection is used.
4. Assert aliases, unsupported modes, profile combinations, raw values, and
   mismatches fail before network work.
5. For sessions, prove one fixed selection on initial, tool-result, final, later
   turn, failed turn, and fresh restoration paths while private reasoning stays
   undisclosed.
6. Preserve model/facade, tool-loop, output, cache, endpoint, credential,
   cancellation, failure, and cleanup proofs.
7. Update the DeepSeek guide, Research 186, cards 104-106, g04.038, the
   pre-indexed route-local closeout log, and the DeepSeek package API baseline.
8. Record the exact required architecture, route/feature matrix, changelog,
   programme, index, and Next Task delta in the closeout log and PR body. Do not
   edit those shared surfaces on this parallel branch.
9. Do not compile or start xAI from this worker.

## Acceptance Criteria

- every deliver-now value, field combination, and failure class is covered
- continuation replay and restoration preserve exact fixed selection
- default QA sends no provider request
- docs distinguish dispatch, acceptance, effective depth, and private replay
- no sibling DeepSeek route or model gains the capability
- closeout records PR/head truth without claiming merge
- worker changed files stay within the programme's parallel execution boundary
- named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-deepseek
effigy validate:focused swallowtail-adapter-deepseek
effigy package:verify-affected swallowtail-adapter-deepseek
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

## Closeout

Card 106 closes the route-local DeepSeek reasoning-control lane. The existing
fixture tests prove exact `low`, `high`, and `max` request encoding, fixed
`thinking.type=enabled`, exact prepared plan/request/evidence agreement,
high/enabled compatibility, private replay, fresh restoration, and
pre-network rejection of `medium`, `xhigh`, provider aliases, alternate
models, and unaccepted cache posture. No effective reasoning or provider
acceptance claim is made.

The prepared guide and milestone are complete. The exact required changes to
shared architecture, route/feature matrices, changelog, programme, front
doors, indexes, and `packages.txt` are recorded in the reserved route-local
closeout log for orchestrator application after review and merge. The worker
branch does not edit those surfaces.

The named validation gates passed before the review PR: focused package
validation, affected-package verification, examples, route QA, Northstar QA,
all required research/log/roadmap/batch-card index checks, next-action QA,
package API verification, and `git diff --check`. The DeepSeek package test
suite also passed with 42 tests under Effigy validation.

## Stop Conditions

- exact agreement or continuation invariants cannot be proved
- docs would need to infer effective reasoning depth
- another model, route, tool-loop change, or currentness family enters scope

## Out Of Scope

- live provider verification, release, publication, or consumer changes
- xAI planning or implementation
- merge or restack authority
- `CHANGELOG.md`; `docs/architecture/system-architecture.md`; route/feature
  matrices; programme and roadmap front doors; shared indexes; `packages.txt`
