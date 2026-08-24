# 149 Qoder Headless Maximum-Turn Binding

Status: blocked; card 148 claim reconciliation pending
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.053 Qoder Headless Maximum Turns](../053-qoder-headless-max-turns.md)
Depends on: card 148; promoted Research 200

Card 148 has not closed. Research 200 admits no deliver-now set and pauses for
operator reconciliation of contradicted qualified-route max-turns claims. No
typed maximum-turn input, prepared plan, driver argv selection, or behavior
revision is admitted by this lane.

## Goal

Bind only Research 200's exact Qoder caller-decreasing maximum-turn subset
through typed run input, immutable plan/evidence, driver agreement, and argv.

## Method

1. Add the smallest typed adapter-local positive bound. The planned candidate
   is `1..=8`; Research 200 is authoritative.
2. Carry the selection through `QoderHeadlessRunProfileInput`, prepared run,
   immutable evidence/plan truth, driver, and command construction.
3. Preserve omission as exact `--max-turns 8`; never omit the native flag.
4. Reject invalid values, route/version drift, and selection mismatch before
   process start or prompt.
5. Preserve stream-json, `dont_ask`, no-session-persistence, workdir, local
   access, deadline, cancellation, terminal mapping, and cleanup.
6. Add focused unit and prepared-facade proofs without creating a new god file.

## Acceptance Criteria

- [ ] public type constructs only the exact Research 200 deliver-now set
- [ ] selected, prepared, planned, driven, and emitted values agree
- [ ] omission emits exact current `--max-turns 8`
- [ ] knowable invalid/mismatched values reject before effects
- [ ] no shared `Capability`, generic provider map, or Contract 040
      `OutputTokenLimit` is added
- [ ] all fixed Qoder route boundaries remain unchanged

Not executed. Research 200 admits no deliver-now value and requires operator
claim reconciliation before card 148 can close. Binding argv `--max-turns N`
would overstate AgentLoop enforcement on exact `1.1.25`.

## Validation

- `effigy validate:focused swallowtail-adapter-qoder`
- `effigy package:verify-affected swallowtail-adapter-qoder`
- `effigy check:examples`
- `git diff --check`

## Stop Conditions

- Do not start unless card 148 records a non-empty exact set.
- Stop if binding requires a contract/currentness change, shared control, or
  altered permission/session/lifecycle behavior.
