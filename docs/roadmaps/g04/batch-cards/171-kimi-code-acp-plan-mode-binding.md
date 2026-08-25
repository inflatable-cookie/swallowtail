# 171 Kimi Code ACP Plan-Mode Binding

Status: ready
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Milestone: [g04.061 Kimi Code ACP Plan Mode](../061-kimi-code-acp-plan-mode.md)
Depends on: card 170; non-empty Research 208 deliver-now table

## Goal

Admit only Research 208's exact Kimi ACP `HarnessMode::Plan` rows through the
existing new-session negotiated-option path with exact plan, request, snapshot,
provider application, and effective confirmation.

## Work

1. Update Kimi ACP compatibility behavior only as Research 208 requires. Split
   a version segment/revision at the proved source milestone when necessary.
2. Advertise `HarnessModeSelection(Plan)` only for admitted versions and accept
   only `SessionOptions::with_harness_mode(HarnessMode::Plan)`.
3. Bind the exact mode to the immutable plan, prepared request, and runtime
   agreement before provider work. Omission keeps the previous request bytes.
4. Parse only one valid current `mode` select option and require exact `plan`
   membership. Known provider-only rows may coexist only as Research 208 allows.
5. Send one exact qualified selection request and require the response or
   correlated update to confirm effective `currentValue = plan` before
   returning a ready session.
6. Compose plan mode with reasoning through separate capability constraints,
   separate requests, and separate confirmations. Preserve the proved order
   and join every partial-failure path.
7. Keep `default|auto|yolo`, unknown values, labels, aliases, generic config,
   model changes, and permission widening private or rejected.
8. Reject harness-mode redeclaration on load and resume before host effects.
   Import and recovery gain no selection path.
9. Preserve access, ambient isolation, resource, retention, callback,
   cancellation, deadline, terminal, cleanup, and diagnostics truth.
10. Add focused fixtures and API/guide changes only where the admitted surface
    requires them.

## Acceptance Criteria

- [ ] only Research 208 exact version/value rows prepare
- [ ] request, immutable plan, runtime agreement, snapshot, and response agree
- [ ] one exact plan request precedes readiness and effective confirmation
- [ ] `auto|yolo` never become public selections or implicit permissions
- [ ] unsupported, missing, malformed, duplicate, substituted, rejected, and
      drifted values fail closed without fallback
- [ ] omission and reasoning-only wire remain unchanged
- [ ] every admitted reasoning mode composes with independent confirmation
- [ ] load/resume/import/recovery gain no harness-mode mutation
- [ ] ambient isolation, manual permission, access, model, resource, and
      provider-state truth remain unchanged
- [ ] no breaking public API or shared contract/runtime change
- [ ] `cargo fmt -p swallowtail-adapter-kimi` passes
- [ ] `effigy validate:focused swallowtail-adapter-kimi` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-kimi` passes
- [ ] `git diff --check` passes

## Stop Conditions

- Research 208 is empty or ambiguous
- implementation needs generic provider config, public `auto|yolo`, permission
  widening, unconfirmed dispatch, shared contract/runtime change, or breaking API
- plan and reasoning cannot compose without shared or inferred confirmation

## Out Of Scope

- other Kimi controls/routes, live OAuth/provider work, currentness, release,
  merge, generation rollover, or g04 closure
