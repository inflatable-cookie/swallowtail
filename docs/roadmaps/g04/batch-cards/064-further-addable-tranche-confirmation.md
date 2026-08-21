# 064 Further Addable Tranche Confirmation

Status: completed
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

- [x] the next implementation roadmap after g04.023 is named
- [x] later named routes stay planned, not started
- [x] no adapter crate changes

## Validation

- `effigy qa:docs:index:roadmaps:g04`
- `git diff --check`

## Evidence

Roadmap g04.024 names `kimi-platform.chat` as the first post-g04.023
implementation tranche. Its implementation cards are not compiled or ready;
g04.023 remains the immediate next work. Research 171 leaves the later
installed candidates and gated routes planned behind it.

Validation passed: `effigy qa:docs:index:roadmaps:g04`; `git diff --check`.

## Auto-Continuation

No. Compile g04.023 after this inventory. Named addable implementations
wait until 023 closes.

## Stop Conditions

- Stop if the tranche would compile hosted OAuth or OpenHands.
