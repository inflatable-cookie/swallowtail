# 203 Cline Headless Plan-Mode Acceptance

Status: planned; gated on card 202
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.073 Cline Headless Plan Mode](../073-cline-headless-plan-mode.md)
Depends on: card 202

## Goal

Prove exact portable Plan dispatch, preserved omission, fail-closed rejection,
behavior and isolation separation, and unchanged one-run lifecycle, then
produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, driver, fixture, facade, and
   scenario coverage for every Research 220 deliver-now row and rejection
   boundary.
2. Assert exact package/behavior/mode membership, immutable plan/evidence
   agreement, canonical `--plan` placement, and unchanged prompt secrecy.
3. Assert omission retains exact `--json --auto-approve false -c <cwd>
   <prompt>` argv and provider-default mode behavior.
4. Prove unsupported versions, mode mismatch, plan/request/evidence drift,
   ambient override, and behaviorally weaker rows reject before process work
   when knowable.
5. Prove the selected bounded run cannot mutate to Act, add a later turn, or
   gain a reusable-session lifecycle through the public Swallowtail path.
6. Prove Plan grants no auto-approval, permission, arbitrary tool policy,
   filesystem/network/shell/process restriction, sandbox, descendant
   containment, configuration, credential, provider, model, or account claim.
7. Prove existing activity, malformed JSON, terminal, cancellation, deadline,
   failure, retention, close, and joined cleanup behavior.
8. Update the Cline headless guide, Research 220, cards 201-203, g04.073,
   architecture and route/feature matrices where truth changes, closeout,
   programme, triage, indexes, and sole Next Task. Do not select or compile the
   next route family.
9. Update the example and package-specific unreleased API baseline when the
   public shape warrants it.

## Acceptance Criteria

- [ ] every admitted row and rejected boundary has deterministic coverage
- [ ] preparation, Plan dispatch, omission, and compatibility remain exact
- [ ] provider behavior, mode application, observation, permission, tools,
      resources, and isolation truth stay separate
- [ ] existing route lifecycle, activity, failure, retention, and cleanup
      assertions pass
- [ ] public docs claim only exact frozen Plan behavior and dispatch truth
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

- If Research 220 is empty, stop after card 201, mark cards 202-203 blocked,
  retain current argv and claims, and open the evidence-only PR.
- If the Plan row ships, close cards 201-203 and g04.073 honestly.
- Keep g04 open. Do not compile the next family, roll the generation, merge the
  PR, or close g04.

## Out Of Scope

- another feature/route, live provider acceptance, currentness, release, merge,
  generation rollover, or g04 closure
