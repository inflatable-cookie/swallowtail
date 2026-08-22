# 115 Qwen Headless Reasoning Effort Acceptance

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.041 Qwen Headless Reasoning Effort](../041-qwen-headless-reasoning-effort.md)
Depends on: card 114

## Goal

Prove exact Qwen headless reasoning dispatch and produce the review-ready
route-local closeout.

## Scope

1. Add deterministic prepared, command/configuration, and lifecycle tests for
   every Research 189 deliver-now model/value row.
2. Assert input, request, plan constraint, evidence, configured driver, and
   exact child transport agree.
3. Assert absent reasoning retains current argv/environment and behavior.
4. Assert structured runs, first turns, resumed later turns, and fresh
   context-losing replacement receive the same prepared value.
5. Assert unsupported values, model/package drift, and plan/evidence/driver
   mismatches fail before process start; assert ambient override and control
   substitution failures after child startup but before the user
   message/provider prompt.
6. Preserve Qwen stream parsing, session-id resume, model/version validation,
   working-resource, cancellation, terminal, durable-session, and joined
   cleanup proofs.
7. Update the Qwen guide, Research 189, cards 113-115, g04.041, the pre-indexed
   route-local closeout log, and package-specific public API baseline.
8. Record the exact required architecture, route/feature matrix, changelog,
   programme, index, matrix-assertion, and Next Task delta in the closeout log
   and PR body. Do not edit those shared surfaces on the worker branch; the
   operator-authorized Qwen package-list registration is the sole exception
   required by the package API gate.

## Acceptance Criteria

- [x] exact deliver-now rows and failure classes have deterministic coverage
- [x] default QA performs no install, login, credential, account, or prompt
- [x] docs distinguish dispatched, accepted, effective, and observed truth
- [x] no sibling Qwen surface or model gains the capability
- [x] closeout records PR/head truth without claiming merge
- [x] worker changes stay inside the route-local boundary
- [x] named gates pass

## Acceptance Result

Deterministic fixtures cover both admitted models across all five values,
structured runs, first turns, resumed turns, control substitution/ambient
override rejection, unsupported package/model/value/provider rows, and
plan/request mismatch before process start. Fresh replacement retains the
prepared request and repeats the same child handshake. Control substitution is
rejected after child startup but before the user message/provider prompt;
provider-effective and observed reasoning remain unclaimed.

All named route-local and package API gates pass. The operator-authorized
Qwen registration in `release-baselines/public-api-unreleased/packages.txt`
allows the checker to select the package-specific unreleased baseline.

## Validation

```sh
cargo fmt -p swallowtail-adapter-qwen
effigy validate:focused swallowtail-adapter-qwen
effigy package:verify-affected swallowtail-adapter-qwen
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

- exact child transport or cross-child lifetime cannot be proved
- docs would need to infer provider acceptance or effective effort
- another Qwen feature, route, currentness family, or contract enters scope

## Out Of Scope

- other Qwen settings, tools, permissions, search, usage, or model discovery
- live provider verification, install, release, publication, or consumer work
- merge or restack authority
- `CHANGELOG.md`; `docs/architecture/system-architecture.md`; route/feature
  matrices; programme and roadmap front doors; shared indexes and matrix
  assertions; package-list changes other than the operator-authorized Qwen
  registration required by this card
