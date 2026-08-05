# 090 Cursor And Grok ACP Load/Replay Qualification

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../035-acp-continuation-recovery-expansion.md`
Depends on: card 089

## Goal

Decide independently whether Cursor Agent ACP and Grok Build ACP satisfy exact
continuation-recovery load and replay requirements.

## Scope

1. Revalidate maintained exact source and fixtures for each route.
2. Freeze `session/load` request, response, update ordering, replay completion,
   readiness, identity, resource, option, and cleanup behavior.
3. Cover foreign session updates, early response, replay after readiness,
   malformed identity, drift, overflow, cancellation, disconnect, and cleanup.
4. Record an independent supported or blocked decision for each route.
5. Use no credential, account, prompt, provider mutation, or live session.

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-cursor swallowtail-adapter-grok`

Passed: 154 tests and focused package checks.

## Stop Conditions

- stop a route if exact evidence cannot prove replay completion before readiness
- stop a route if a saved binding cannot prove exact cwd and attachment identity
- do not borrow behavior from another ACP agent or unqualified version

## Auto-Continuation

Continue to card 091 only for routes selected by this card. If neither route
passes, close g03.035 negatively and advance to g03.036.

## Outcome

- Cursor is blocked: both exact source bundles suppress whole-history and
  per-turn replay failures before returning load success.
- Grok is blocked: all four exact stripped binaries expose incomplete-replay
  paths, while no deterministic load transcript or inspectable control flow
  proves client-visible completeness.
- frozen corpora enumerate every unqualified negative load case
- no production driver, facade, binding, capability, or public API changed
- no authenticated work or provider session ran

Advance to card 093. Cards 091-092 are superseded.
