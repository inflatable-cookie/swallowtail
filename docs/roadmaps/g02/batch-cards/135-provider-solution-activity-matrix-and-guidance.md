# 135 Provider Solution Activity Matrix And Guidance

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../040-provider-wide-activity-acceptance-and-consumer-handoff.md`
Depends on: card 134

## Goal

Publish one machine-readable provider-solution activity inventory and minimal
consumer guidance.

## Scope

1. Add a provider-solution activity matrix covering:
   - assistant intermediate and final content
   - reasoning summaries
   - plans
   - tool lifecycle, display input, output, and correlation
   - command output
   - file changes
   - search and image activity
   - tasks, hooks, and subagents
   - lifecycle and disclosure fidelity
   - unknown-event posture
2. Account for every production solution and operation shape.
3. Link every positive value to a public prepared path and conformance test.
4. Link every unavailable or not-applicable value to exact evidence.
5. Document safe projection, sensitive content, and consumer ownership.
6. Machine-check sorting, allowed values, counts, and route identities.

## Out Of Scope

- consumer database schema
- UI component design
- raw provider payload viewer
- implementation changes

## Acceptance Criteria

- [x] every production solution is represented
- [x] every activity column has explicit semantics
- [x] lifecycle and disclosure strength are not flattened to yes/no
- [x] no positive cell requires provider-native parsing downstream
- [x] no negative cell silently means unknown research
- [x] matrix and route identities are machine-checked

## Validation

- activity matrix checker
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy package:api`

## Stop Conditions

- Stop if one value cannot distinguish unavailable from not applicable.
- Do not publish a positive cell without a prepared facade proof.

## Evidence

- `docs/guides/provider-solution-activity-matrix.csv` records 55 exact
  route-operation rows: 32 available and 23 not applicable.
- `docs/guides/provider-solution-activity-matrix.md` defines every value and
  the safe projection, sensitive-content, and consumer-ownership boundary.
- `scripts/check-provider-activity-matrix.py` checks row identity, order,
  vocabulary, counts, production and auxiliary route coverage, prepared
  entries, conformance tests, and exact inventory references.
- `effigy qa:routes`, `effigy qa:docs`, and `effigy package:api` pass.

## Auto-Continuation

Continue to card 136 after the matrix and public guidance pass.
