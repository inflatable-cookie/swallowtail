# 197 Copilot CLI ACP Built-In Tool Allowlist Acceptance

Status: blocked; card 196 blocked
Owner: Tom
Created: 2026-08-26
Milestone: [g04.071 Copilot CLI ACP Built-In Tool Allowlist](../071-copilot-cli-acp-built-in-tool-allowlist.md)
Depends on: card 196

## Goal

Prove exact built-in allowlist dispatch, preserved omission, fail-closed
rejection, immutable replacement, permission separation, and lifecycle behavior,
then produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, driver, fixture, and facade coverage
   for every Research 218 deliver-now row and rejection boundary.
2. Assert exact package/profile/tool membership, canonical argv syntax and
   ordering, immutable plan/evidence agreement, and private behavior claim.
3. Assert omission retains exact `copilot --acp --stdio` argv and existing
   capability truth.
4. Prove unknown, unsupported, empty, duplicate, version-mismatched, ambient,
   and drifting rows reject before process work when knowable.
5. Prove first/later prompt and fresh context-losing replacement retain the
   prepared allowlist without turning it into permission or isolation truth.
6. Prove existing initialize, session, activity, permission observe-and-stop,
   cancellation, malformed transport, terminal outcome, close, and joined
   cleanup behavior.
7. Update the Copilot guide, Research 218, cards 195-197, g04.071, route and
   feature matrices where truth changes, closeout, programme, triage, indexes,
   and sole Next Task. Do not select or compile the next route family.
8. Update examples and package-specific unreleased API baseline only when the
   public shape warrants it.

## Acceptance Criteria

- [ ] every admitted row and rejected boundary has deterministic coverage
- [ ] preparation, command dispatch, omission, and replacement remain exact
- [ ] permission, callback, consumer-tool, MCP, extension, activity, and
      isolation truth stays separate
- [ ] existing route lifecycle and cleanup assertions pass
- [ ] public docs claim only the exact frozen restriction and dispatch truth
- [ ] package API drift is intentional and recorded when applicable
- [ ] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-copilot-cli
effigy validate:focused swallowtail-adapter-copilot-cli
effigy package:verify-affected swallowtail-adapter-copilot-cli
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

- If Research 218 is empty, stop after card 195, mark cards 196-197 blocked,
  retain current argv and claims, and open the evidence-only PR.
- If the admitted subset ships, close cards 195-197 and g04.071 honestly.
- Keep g04 open. Do not compile the next family, roll the generation, merge the
  PR, or close g04.

## Out Of Scope

- another feature/route, live provider acceptance, currentness, release, merge,
  generation rollover, or g04 closure
