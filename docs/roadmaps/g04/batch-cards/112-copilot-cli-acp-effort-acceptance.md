# 112 Copilot CLI ACP Effort Acceptance

Status: blocked
Owner: Tom
Created: 2026-08-22
Milestone: [g04.040 Copilot CLI ACP Session Effort](../040-copilot-cli-acp-session-effort.md)
Depends on: card 111

## Goal

Prove exact Copilot CLI ACP startup/session effort dispatch and produce the
review-ready route-local closeout.

## Scope

1. Add deterministic prepared, command, and lifecycle tests for every Research
   188 deliver-now value.
2. Assert input, request, plan constraint, evidence, configured driver, and
   canonical child argv agree exactly.
3. Assert absent effort retains exact `--acp --stdio` argv and current behavior.
4. Assert the first and every later prompt share the one process-fixed value;
   fresh context-losing replacement spawns with the same prepared value.
5. Assert unsupported values, raw aliases, package drift, and plan/evidence/
   driver mismatches fail before task or process effects.
6. Preserve initialize, `session/new`, prompt, permission-stop, callback denial,
   cancellation, terminal, and joined cleanup proofs.
7. Update the Copilot CLI ACP guide, Research 188, cards 110-112, g04.040, the
   pre-indexed route-local closeout log, and package-specific public API baseline.
8. Record the exact required architecture, route/feature matrix, changelog,
   programme, index, matrix-assertion, and Next Task delta in the closeout log
   and PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [ ] exact deliver-now values and failure classes have deterministic coverage
- [ ] default QA performs no login, credential, account, or provider prompt
- [ ] docs distinguish dispatched, accepted, and effective effort
- [ ] no sibling Copilot surface or ACP route gains the capability
- [ ] closeout records PR/head truth without claiming merge
- [ ] worker changes stay inside the route-local boundary
- [ ] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-copilot-cli
effigy validate:focused swallowtail-adapter-copilot-cli
effigy package:verify-affected swallowtail-adapter-copilot-cli
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

- exact process/session inheritance cannot be proved
- docs would need to infer provider acceptance or effective effort
- another Copilot feature, route, currentness family, or contract enters scope

## Out Of Scope

- tool filters, dangerous permissions, TCP, IDE/API, login, model selection
- live provider verification, release, publication, or consumer changes
- merge or restack authority
- `CHANGELOG.md`; `docs/architecture/system-architecture.md`; route/feature
  matrices; programme and roadmap front doors; shared indexes and matrix
  assertions; `packages.txt`

## Closeout

Not executed. There is no dispatch, guide change, or API baseline delta to
prove. Route-local closeout is the Research 188 stop record.
