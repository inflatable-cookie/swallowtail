# 189 OpenCode HTTP Web Search Acceptance

Status: blocked; card 188 blocked
Owner: Tom
Created: 2026-08-26
Milestone: [g04.067 OpenCode HTTP Web Search](../067-opencode-http-web-search.md)
Depends on: card 188

## Goal

Prove exact web-search policy and session-permission dispatch, preserved
disabled omission, callback/operation composition, and lifecycle behavior, then
produce one review-ready route-local closeout.

## Scope

1. Add deterministic prepared-facade, session-request, protocol, and driver
   coverage for every Research 214 deliver-now row and rejection boundary.
2. Assert exact version behavior, provider/backend evidence, operation policy,
   plan/evidence, permission ordering/action, prompt path, and route claim.
3. Assert disabled omission retains the prior session-create JSON and claims
   no search authority.
4. Prove incompatible policy pairs, profiles, versions, providers/backends,
   missing availability evidence, and plan/session drift reject before effects.
5. Prove callback-free and admitted callback-enabled composition without
   converting wildcard permission mediation into search authority.
6. Prove existing reasoning, schema, image, resource, provider failure,
   malformed SSE, activity/usage, cancellation, deadline, disconnect, terminal,
   private-session deletion, and joined cleanup truth.
7. Update the OpenCode guide, Research 214, cards 187-189, g04.067,
   route/feature matrices where truth changes, closeout, programme, triage,
   indexes, and sole Next Task. Do not select or compile the next route family.
8. Update examples and package-specific unreleased API baseline only when the
   public shape warrants it.

## Acceptance Criteria

- [ ] every admitted row and rejected boundary has deterministic coverage
- [ ] policy, permission, profile, and version remain exact through dispatch
- [ ] omission, callback composition, activity, usage, cancellation, terminal,
      deletion, and cleanup behavior are exact
- [ ] default QA performs no credentials, login, account/backend probe,
      provider prompt, hosted search, external network, or paid work
- [ ] docs do not infer provider acceptance, result quality, model use,
      entitlement, fallback, or billing
- [ ] closeout records PR/head truth without claiming merge
- [ ] named gates pass and doctor findings do not increase

## Validation

```sh
cargo fmt -p swallowtail-adapter-opencode
effigy validate:focused swallowtail-adapter-opencode
effigy package:verify-affected swallowtail-adapter-opencode
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
effigy doctor
git diff --check
```

Auto-continuation: No.

## Stop Conditions

- exact availability, permission, policy, event, or lifecycle truth cannot be
  proved
- docs would infer provider acceptance, effective search, account/backend
  availability, entitlement, fallback, or billing
- another route/control, currentness lane, shared contract, release, rollover,
  or g04 closure enters scope

## Out Of Scope

- live provider/search verification, publication, merge, later feature
  selection, generation rollover, or g04 closure
