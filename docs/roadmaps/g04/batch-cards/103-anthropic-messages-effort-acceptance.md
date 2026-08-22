# 103 Anthropic Messages Effort Acceptance

Status: ready after 102
Owner: Tom
Created: 2026-08-22
Milestone: [g04.037 Anthropic Messages Effort](../037-anthropic-messages-effort.md)
Depends on: card 102

## Goal

Prove exact Messages effort dispatch and produce the review-ready Anthropic
route closeout without obscuring the parallel integration order.

## Scope

1. Add deterministic prepared and protocol tests for every Research 185
   deliver-now value and profile.
2. Assert request, plan constraint, evidence, configured driver, and
   `output_config.effort` agree exactly.
3. Assert absent effort preserves current fixture bodies byte-for-byte.
4. Assert unsupported models, values, profiles, raw strings, and mismatches fail
   before network work.
5. If sessions are admitted, assert every attempt and fresh restoration uses
   the fixed prepared value. If not, document and test rejection.
6. Preserve output-token, search, attachment, model, version, cancellation,
   failure, and cleanup proofs.
7. Update the Anthropic direct guide, Research 185, cards 101-103, g04.037, the
   pre-indexed route-local closeout log, and the Anthropic package API baseline.
8. Record the exact required architecture, route/feature matrix, changelog,
   programme, index, and Next Task delta in the closeout log and PR body. Do not
   edit those shared surfaces on this parallel branch.
9. Do not compile or start xAI from this worker.

## Acceptance Criteria

- exact deliver-now values and failure classes have deterministic coverage
- default QA sends no provider request
- docs distinguish dispatch, acceptance, and effective effort
- no sibling Anthropic route gains the capability
- closeout records PR/head truth without claiming merge
- worker changed files stay within the programme's parallel execution boundary
- named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-anthropic
effigy validate:focused swallowtail-adapter-anthropic
effigy package:verify-affected swallowtail-adapter-anthropic
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

- exact agreement cannot be proved through dispatch
- docs would need to infer effective effort from provider output
- another Anthropic feature, route, or currentness family enters scope

## Out Of Scope

- Messages thinking, newer web-search tool, managed-agent model settings
- live provider verification, release, publication, or consumer changes
- merge or restack authority
- `CHANGELOG.md`; `docs/architecture/system-architecture.md`; route/feature
  matrices; programme and roadmap front doors; shared indexes; `packages.txt`
