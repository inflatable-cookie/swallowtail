# 206 Cline Headless Model-Selection Acceptance

Status: blocked
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.074 Cline Headless Model Selection](../074-cline-headless-model-selection.md)
Depends on: card 205

## Goal

Prove exact model-route dispatch, preserved omission, fail-closed rejection,
Plan composition, authority separation, and unchanged one-run lifecycle, then
produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, driver, fixture, facade, and
   scenario coverage for every Research 221 deliver-now row and rejection
   boundary.
2. Assert exact package/behavior/provider/model membership, configured-instance
   and access-audience agreement, immutable plan/evidence, and canonical argv.
3. Assert omission retains exact current argv and ambient provider/model
   behavior. No implicit provider or model may appear.
4. Prove unsupported versions, unknown models, provider mismatch, route drift,
   request/plan/evidence mismatch, ambient override, fallback-prone rows, and
   unauthorized configuration effects reject before process work when
   knowable.
5. Prove optional `HarnessMode::Plan` composes without changing the model route
   or losing either immutable selection.
6. Prove model dispatch grants no catalogue, entitlement, billing, credential,
   configuration, reasoning, effective-value, or availability claim.
7. Prove existing activity, malformed JSON, terminal, cancellation, deadline,
   failure, retention, close, and joined cleanup behavior.
8. Update the Cline headless guide, Research 221, cards 204-206, g04.074,
   architecture and route/feature matrices where truth changes, closeout,
   programme, triage, indexes, and sole Next Task. Do not select or compile the
   next route family.
9. Update the example and package-specific unreleased API baseline when the
   public shape warrants it.

## Acceptance Criteria

- [ ] every admitted row and rejected boundary has deterministic coverage
- [ ] preparation, exact route dispatch, omission, and compatibility remain
      exact
- [ ] provider/model selection, application, observation, catalogue,
      entitlement, configuration, and reasoning truth stay separate
- [ ] optional Plan and existing route lifecycle remain exact
- [ ] public docs claim only the frozen route and dispatch strength
- [ ] package API drift is intentional and recorded when applicable
- [ ] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-cline
effigy validate:focused swallowtail-adapter-cline
effigy package:verify-affected swallowtail-adapter-cline
effigy check:examples
effigy package:api
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy doctor
git diff --check
```

Focused package verification is required. Broad workspace tests, live probes,
MSRV, release, and consumer checks are not authorized by this card.

## Closeout Boundary

- If Research 221 is empty, stop after card 204, mark cards 205-206 blocked,
  retain current argv and claims, and open the evidence-only PR.
- If exact model-route rows ship, close cards 204-206 and g04.074 honestly.
- Keep g04 open. Do not compile the next family, roll the generation, merge the
  PR, or close g04.

## Out Of Scope

- another feature/route, thinking delivery, live provider acceptance,
  currentness, release, merge, generation rollover, or g04 closure

## Blocked

Blocked by card 205, which is blocked by card 204. Research 221 admits no
deliver-now row, so there is no exact model dispatch, rejection boundary, or
Plan composition to prove. The Cline adapter, fixtures, guide, matrices, and
API baseline stay unchanged.
