# 172 Kimi Code ACP Plan-Mode Acceptance

Status: ready
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Milestone: [g04.061 Kimi Code ACP Plan Mode](../061-kimi-code-acp-plan-mode.md)
Depends on: card 171

## Goal

Close the route-local family with deterministic exact-version proof for Kimi
ACP plan mode, reasoning composition, failure handling, lifecycle, permission,
isolation, compatibility, and documentation truth.

## Work

1. Add frozen fixtures for every Research 208 delivered row and the immediately
   preceding unsupported boundary.
2. Prove exact `mode` snapshot membership, one set request, provider
   application, effective response/update confirmation, and no prompt before
   readiness.
3. Prove omission and composition with reasoning omission plus every admitted
   `off|on|low|medium|high|xhigh|max` row applicable to the exact version.
4. Prove provider-only `default|auto|yolo` coexistence, unknown rows,
   unsupported values, malformed/duplicate/missing options, wrong category,
   missing confirmation, effective drift, rejection, disconnect, deadline,
   cancellation, and joined cleanup.
5. Prove load/resume harness-mode redeclaration rejects before credential,
   resource, or process effects; import and recovery remain non-mutating.
6. Prove the exact compatibility segment/revision, qualified older behavior,
   and visible `UnverifiedNewer` posture without guarantee inheritance.
7. Prove manual permission and `AmbientHost` isolation remain explicit and
   independent; plan mode is not reported as containment.
8. Update the Kimi prepared guide, feature matrix when warranted, package API
   baseline only if changed, Research 208, milestone/cards, inventory,
   programme, reserved closeout, indexes, and sole Next Task.
9. Run the named package, route, docs, API, example, doctor, and diff gates.

## Acceptance Criteria

- [ ] every delivered exact version/value row has deterministic evidence
- [ ] plan mode never prepares or dispatches without exact version and snapshot
      admission
- [ ] exact effective confirmation is required before readiness
- [ ] reasoning composition retains independent selection and confirmation
- [ ] unsupported, malformed, ambiguous, substituted, rejected, or drifted
      values never reach a prompt or fall back
- [ ] omission preserves prior wire and behavior
- [ ] load/resume/import/recovery remain outside the selection lifecycle
- [ ] no automatic approval, permission widening, or isolation claim appears
- [ ] stable diagnostics disclose no provider payload, credential, prompt,
      output, account identity, endpoint, model alias, or host path
- [ ] guide and matrix distinguish plan mode, permission posture, and isolation
- [ ] no sibling route, currentness, contract, release, or generation state changes
- [ ] `cargo fmt -p swallowtail-adapter-kimi` passes
- [ ] `effigy validate:focused swallowtail-adapter-kimi` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-kimi` passes
- [ ] `effigy check:examples`, `effigy qa:routes`, `effigy qa:northstar`,
      relevant index gates, `effigy package:api`, and `git diff --check` pass
- [ ] `effigy doctor` does not worsen the inherited 378-finding baseline

## Stop Conditions

- exact fixtures contradict Research 208 or confirmation is not effective truth
- plan/reasoning composition changes permission or cannot join partial failure
- acceptance requires live account/provider work, generic config, shared
  contract/runtime change, or currentness movement
- doctor findings increase or package/API gates fail

## Out Of Scope

- another Kimi family, live inference, release, merge, generation rollover, or
  g04 closure
