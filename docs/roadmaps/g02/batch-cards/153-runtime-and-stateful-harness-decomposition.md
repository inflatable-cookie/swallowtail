# 153 Runtime And Stateful Harness Decomposition

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../045-error-level-structural-health-stabilization.md`

## Goal

Remove runtime, Claude Agent, Gemini, and Kimi error-level findings without
changing cross-transport records or harness behavior.

## Scope

1. Split runtime roles, provider-session management, and typed harness user
   input by record, validation, and execution responsibility.
2. Split Claude Agent validation and test support by operation family.
3. Split Gemini catalogue and headless test cases.
4. Split Kimi local-server activity and WebSocket protocol concerns.

## Acceptance Criteria

- [x] all ten assigned error findings are removed
- [x] runtime public declarations remain stable
- [x] ACP, callback, lifecycle, and activity semantics remain unchanged
- [x] focused runtime and adapter tests pass
- [x] focused warnings-denied clippy passes

## Validation

- focused runtime, Claude Agent, Gemini, and Kimi tests
- warnings-denied clippy for touched crates
- public-API and doctor delta checks

## Stop Conditions

- Stop if concurrent typed user-input work lacks a stable focused baseline.
- Stop if a split changes callback, session, or cleanup authority.
- Do not redesign common runtime roles.

## Auto-Continuation

Yes. Continue to card 154 after focused validation.

## Evidence

- Runtime roles retain every public declaration in `roles.rs`. Private request
  state, forwarding implementations, and driver-trait item macros move into
  focused fragments without changing the declaration baseline.
- Provider-session management and harness-input tests move behind their
  unchanged runtime modules.
- Claude Agent validation, scripted ACP support, and headless tests are split
  by operation family.
- Gemini catalogue helpers and headless cases are split by private transport,
  validation, and operation concern.
- Kimi local-server activity projection and WebSocket event decoding are split
  from their stable records and entry points.
- Focused validation passed 168 tests: runtime 91, Claude Agent 17, Gemini 31,
  and Kimi 29.
- Warnings-denied clippy and the 24-crate public-API declaration baseline
  passed.
- Doctor now reports 148 findings: 141 warnings, seven high errors, and no
  runtime, Claude Agent, Gemini, or Kimi error finding.
