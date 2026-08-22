# 118 Cline Thinking Control Acceptance

Status: conditional on card 117
Owner: Tom
Created: 2026-08-22
Milestone: [g04.042 Cline Thinking Controls](../042-cline-thinking-controls.md)
Depends on: card 117

## Goal

Prove exact Cline thinking dispatch for every admitted transport and produce a
review-ready route-local closeout.

## Scope

1. Add deterministic prepared, command, and lifecycle tests for every
   Research 190 deliver-now `(route, value)` row.
2. Assert portable input, plan constraint, prepared evidence, request/session
   policy, configured driver, and exact child argv agree.
3. Assert reasoning absence retains exact current ACP and headless arguments
   and behavior.
4. For admitted ACP rows, assert first and later turns retain one child-spawn
   value and fresh context-losing replacement repeats it on the new child.
   Do not claim provider-session resume.
5. For admitted headless rows, assert the selected value applies to exactly
   one run child and does not persist into an absent-selection run.
6. Assert unsupported values, route/package/behavior drift, and
   plan/evidence/driver mismatches fail before process start. Test exact
   rejection rather than output-based inference.
7. Preserve ACP framing/session lifecycle, headless NDJSON parsing,
   working-resource, cancellation, terminal, and joined-cleanup proofs.
8. Update both Cline route guides where applicable, Research 190, cards
   116-118, g04.042, the pre-indexed route-local closeout log, and the
   package-specific public API baseline.
9. Record the exact required architecture, route/feature matrix, changelog,
   programme, index, matrix-assertion, and Next Task delta in the closeout log
   and PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [ ] exact deliver-now rows and failure classes have deterministic coverage
- [ ] default QA performs no install, login, credential, account, or prompt
- [ ] ACP and headless docs state their capability and lifetime separately
- [ ] docs distinguish dispatch, CLI acceptance, provider-effective, and
      observed reasoning truth
- [ ] no sibling Cline route or value gains the capability by inference
- [ ] closeout records PR/head truth without claiming merge
- [ ] worker changes stay inside the route-local boundary
- [ ] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-cline
effigy validate:focused swallowtail-adapter-cline
effigy package:verify-affected swallowtail-adapter-cline
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

- exact route argv or ACP/headless lifetime cannot be proved
- docs would need to infer provider acceptance or effective reasoning
- another Cline feature, route, currentness family, or contract enters scope

## Out Of Scope

- Cline model/plan/config/permission/tool/search/timeout/retry controls
- live provider verification, install, release, publication, or consumer work
- merge or restack authority
- `CHANGELOG.md`; shared architecture; route/feature matrices;
  programme/front doors/indexes; matrix assertions; shared package lists
