# 200 Grok Build ACP Subagents-Disabled Acceptance

Status: blocked; card 199 blocked
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.072 Grok Build ACP Subagents Disabled](../072-grok-build-acp-subagents-disabled.md)
Depends on: card 199

## Goal

Prove exact disabled-profile dispatch, preserved omission, fail-closed
rejection, immutable replacement, topology separation, and lifecycle behavior,
then produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, driver, fixture, and facade coverage
   for every Research 219 deliver-now row and rejection boundary.
2. Assert exact package/profile/lifecycle membership, canonical argv placement,
   immutable plan/evidence agreement, and private behavior claim.
3. Assert omission retains exact `--no-auto-update agent stdio` argv and
   existing behavior.
4. Prove unsupported, version-mismatched, profile-mismatched, ambiently
   overrideable, and drifting rows reject before process work when knowable.
5. Prove initialize, first/later prompts, operation-private sessions,
   attachment recovery, and fresh child replacement retain the prepared
   restriction.
6. Prove the profile adds no child observation/control, permission authority,
   tool-selection claim, sandbox, filesystem/network restriction, or OS
   descendant-process containment.
7. Prove existing model, activity, permission observe-and-stop, cancellation,
   malformed transport, terminal outcome, close, and joined cleanup behavior.
8. Update the Grok guide, Research 219, cards 198-200, g04.072, route and
   feature matrices where truth changes, closeout, programme, triage, indexes,
   and sole Next Task. Do not select or compile the next route family.
9. Update examples and package-specific unreleased API baseline only when the
   public shape warrants it.

## Acceptance Criteria

- [ ] every admitted row and rejected boundary has deterministic coverage
- [ ] preparation, command dispatch, omission, and replacement remain exact
- [ ] topology restriction, observation, control, permission, tools, and
      isolation truth stay separate
- [ ] existing route lifecycle and cleanup assertions pass
- [ ] public docs claim only exact frozen restriction and dispatch truth
- [ ] package API drift is intentional and recorded when applicable
- [ ] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-grok
effigy validate:focused swallowtail-adapter-grok
effigy package:verify-affected swallowtail-adapter-grok
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

- If Research 219 is empty, stop after card 198, mark cards 199-200 blocked,
  retain current argv and claims, and open the evidence-only PR.
- If the admitted profile ships, close cards 198-200 and g04.072 honestly.
- Keep g04 open. Do not compile the next family, roll the generation, merge the
  PR, or close g04.

## Out Of Scope

- another feature/route, live provider acceptance, currentness, release, merge,
  generation rollover, or g04 closure
