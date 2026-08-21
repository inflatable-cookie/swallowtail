# 064 Further Addable Tranche Confirmation

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../022-further-addable-inventory.md`
Depends on: card 063

## Goal

Confirm the next addable implementation roadmap after g04.023.

## Scope

1. Name one first implementation tranche on a proved shape.
2. Leave later named routes planned behind it.
3. Do not start adapter wiring.

## Out Of Scope

- addable descriptor code
- hosted OAuth
- g04.023 field work

## Acceptance Criteria

- [ ] the next implementation roadmap after g04.023 is named
- [ ] later named routes stay planned, not started
- [ ] no adapter crate changes

## Validation

- `effigy qa:docs:index:roadmaps:g04`
- `git diff --check`

## Auto-Continuation

No. Compile g04.023 after this inventory. Named addable implementations
wait until 023 closes.

## Stop Conditions

- Stop if the tranche would compile hosted OAuth or OpenHands.
