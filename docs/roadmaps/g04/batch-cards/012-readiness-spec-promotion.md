# 012 Readiness Spec Promotion

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../004-readiness-admission-contract-promotion.md`
Depends on: card 011

## Goal

Promote Spec 011 into the canonical surfaces and leave implementation planned
until 057 is active.

## Scope

1. Archive Spec 011 after 057 and the seam amendments exist.
2. Update contract index, summaries, and contracts README reading order.
3. Point the planned connection-lifecycle architecture note at 057.

## Out Of Scope

- compiling implementation roadmaps
- production code
- tag mutation
- live provider or login work

## Acceptance Criteria

- [x] Spec 011 is archived and listed as promoted
- [x] Contract 057 appears in the index and summaries
- [x] architecture still records the lifecycle as planned, now citing 057
- [x] no implementation card is ready

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Compile implementation roadmaps only after g04.004 closes.

## Stop Conditions

- Stop if Spec 011 still holds an unsettled product decision.
- Stop if architecture would claim the facade is realized.

## Evidence

Spec 011 archived at
`docs/specs/archive/011-route-readiness-and-connection-admission.md`.
Contract index, summaries, and README include 057. Planned connection
lifecycle cites 057 and remains unrealized. No implementation card is ready.
