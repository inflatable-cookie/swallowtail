# 242 Cline ACP Plan-Mode Binding

Status: ready
Owner: Tom
Created: 2026-08-27
Milestone: [g04.086 Cline ACP Plan Mode](../086-cline-acp-plan-mode.md)
Depends on: card 241; promoted Research 240 with one exact deliver-now row

## Goal

Bind only Research 240's exact Cline ACP Plan row through typed prepared input,
immutable plan/evidence/request state, exact option negotiation, and selected-
value confirmation before readiness.

## Scope

1. Extend `ClineSessionProfileInput` with only optional portable
   `HarnessMode::Plan`. Expose no raw string, provider enum, generic config,
   Act selection, or runtime mode-change operation.
2. Advertise `HarnessModeSelection(Plan)` only on exact `cline.acp` `3.0.55`.
   Bind the selection through capability constraint, preflight requirements,
   immutable evidence, `SessionOptions`, request, and driver validation.
3. Preserve the qualified `cline.acp.stdio-v1` behavior revision. Do not
   backfill headless `--plan` or mint a sibling route/facade.
4. After `session/new`, parse one bounded snapshot with unique `plan`
   membership and current mode truth. Reject missing, malformed, duplicate,
   foreign, or ambiguous mode/config rows.
5. For explicit Plan, send one correlated `session/set_config_option` request
   for `mode=plan`. Require the response's unique mode config option to report
   `currentValue = plan` before returning the session handle.
6. Keep omission byte-for-byte on the current initialize/session-new path. It
   sends no mode request and makes no selected or default mode claim.
7. Reject unsupported values and request/plan/evidence/driver mismatch before
   process work. Join process, connection, task, resource, and owned state when
   negotiation fails after provider-session allocation.
8. Preserve read-only working resource, `Ambient` configuration,
   `AmbientHost`, local-account access, observational permission handling,
   no auto-approve, active-turn cancellation, and current terminal semantics.
9. Carry the immutable selection through fresh context-losing replacement.
   Do not add load/resume or post-readiness mutation.

## Acceptance Criteria

- [ ] only the Research 240 exact Plan row prepares
- [ ] input, capability, plan, evidence, request, driver, snapshot, request,
      and confirmation agree exactly
- [ ] no ready handle or first prompt precedes exact confirmation
- [ ] malformed, missing, duplicate, rejected, or mismatched option truth
      fails closed and joins owned work
- [ ] omission retains existing frames and no Plan/default claim
- [ ] fresh replacement reuses the immutable selected request
- [ ] permission, access, resource, configuration, isolation, model, account,
      lifecycle, and cleanup claims do not widen

## Validation

```sh
cargo fmt -p swallowtail-adapter-cline
effigy validate:focused swallowtail-adapter-cline
effigy package:verify-affected swallowtail-adapter-cline
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 243 only when preparation, negotiation, confirmation,
omission, rejection, replacement, and cleanup proof passes.

## Stop Conditions

- the existing prepared-session boundary cannot carry Plan without generic or
  breaking configuration
- snapshot membership or confirmation cannot be exact before readiness
- omission changes or replacement can drift
- implementation needs shared runtime/contract changes, live proof,
  post-readiness mutation, or authority widening

## Out Of Scope

Shared closeout and Next Task, Cline headless, another Cline feature, live
provider work, currentness, release, merge, rollover, or g04 closure.
