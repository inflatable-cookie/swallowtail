# 153 Runtime And Stateful Harness Decomposition

Status: ready
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

- [ ] all ten assigned error findings are removed
- [ ] runtime public declarations remain stable
- [ ] ACP, callback, lifecycle, and activity semantics remain unchanged
- [ ] focused runtime and adapter tests pass
- [ ] focused warnings-denied clippy passes

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
