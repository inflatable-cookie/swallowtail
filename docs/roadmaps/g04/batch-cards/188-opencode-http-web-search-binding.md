# 188 OpenCode HTTP Web Search Binding

Status: ready; conditional
Owner: Tom
Created: 2026-08-26
Milestone: [g04.067 OpenCode HTTP Web Search](../067-opencode-http-web-search.md)
Depends on: card 187; promoted Research 214 with a non-empty deliver-now set

## Goal

Bind only Research 214's exact OpenCode HTTP web-search rows through existing
shared search/network policy, immutable prepared evidence, exact session
permissions, and fail-closed driver validation.

## Scope

1. Admit the existing shared `ExternalSearchPolicy::Enabled` only with the
   exact compatible network policy and profile rows selected by Research 214.
2. Bind the selection through prepared structured-run and/or interactive-
   session input, plan/evidence, access policy, and driver state. Do not add a
   generic OpenCode tool or permission surface.
3. Add only the exact `websearch` session permission rule/action admitted by
   Research 214. Preserve explicit deny-first ordering and existing read/glob/
   grep rules.
4. Preserve current policy and session-create JSON byte-for-byte when search is
   disabled. Never enable search from wildcard callback posture alone.
5. Reject incompatible policy pairs, unsupported profiles/versions/providers,
   missing exact availability evidence, and plan/session drift before creation
   or prompt effects when knowable.
6. Keep permission callback authority distinct from external network and
   search authority. Do not auto-approve a permission request outside the
   admitted profile.
7. Preserve model/reasoning/schema/image/resource, retention, activity, usage,
   cancellation, deadline, terminal, private-session deletion, and joined
   cleanup truth.
8. Advance only Research 214's selected private behavior/claim revisions.

## Acceptance Criteria

- [ ] only Research 214 deliver-now rows prepare
- [ ] policy, plan/evidence, permission JSON, driver, and prompt path agree
- [ ] disabled omission preserves the prior request and capability truth
- [ ] unsupported/drifting rows reject before effects
- [ ] permission, network, search, and provider-result claims stay separate
- [ ] no generic settings, attached-server mutation, provider-acceptance,
      result-quality, entitlement, or billing claim enters the API

## Validation

```sh
cargo fmt -p swallowtail-adapter-opencode
effigy validate:focused swallowtail-adapter-opencode
effigy package:verify-affected swallowtail-adapter-opencode
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 189 only when exact preparation, permission JSON,
omission, rejection, callback composition, and lifecycle proof passes.

## Stop Conditions

- existing prepared state cannot express the admitted exact set
- availability or permission truth can drift after preparation without an
  exact fail-closed boundary
- implementation needs ambient config, generic tools/permissions, live proof,
  unplanned contract change, or a breaking API

## Out Of Scope

- shared closeout selection, another OpenCode feature/route, live provider
  work, currentness, release, merge, rollover, or g04 closure
