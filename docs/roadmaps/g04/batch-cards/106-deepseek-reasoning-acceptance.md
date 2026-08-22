# 106 DeepSeek Reasoning Acceptance

Status: ready after 105
Owner: Tom
Created: 2026-08-22
Milestone: [g04.038 DeepSeek Continuation Reasoning Controls](../038-deepseek-continuation-reasoning-controls.md)
Depends on: card 105

## Goal

Prove exact DeepSeek reasoning dispatch and continuation invariants, then close
the fourth route-local feature milestone.

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
7. Update realized architecture, DeepSeek guide, route and feature matrices,
   changelog, Research 186, cards 104-106, g04.038, programme, logs, and indexes.
8. Keep the sole Next Task on completing/restacking the parallel feature wave
   until g04.036 and g04.037 land. Do not compile or start xAI from this worker.

## Acceptance Criteria

- every deliver-now value, field combination, and failure class is covered
- continuation replay and restoration preserve exact fixed selection
- default QA sends no provider request
- docs distinguish dispatch, acceptance, effective depth, and private replay
- no sibling DeepSeek route or model gains the capability
- closeout records PR/head truth without claiming merge
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

## Stop Conditions

- exact agreement or continuation invariants cannot be proved
- docs would need to infer effective reasoning depth
- another model, route, tool-loop change, or currentness family enters scope

## Out Of Scope

- live provider verification, release, publication, or consumer changes
- xAI planning or implementation
- merge or restack authority

