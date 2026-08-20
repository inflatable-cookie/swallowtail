# 044 Second-Proof Tranche Confirmation

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../015-second-proof-addable-inventory.md`
Depends on: card 043

## Goal

Confirm g04.016 hosted API-key DeepSeek continuation as the next
implementation roadmap unless inventory contradicts it.

## Scope

1. Check the 042/043 evidence against the proved hosted API-key shape.
2. Confirm DeepSeek continuation as the first second-proof adapter-local
   descriptor tranche.
3. Leave Claude Agent ACP, llama.cpp attached, and hosted OAuth planned
   behind that first implementation.
4. Do not write g04.016 implementation cards in this card.

## Out Of Scope

- compiling or executing g04.016
- adapter code
- live provider probes

## Acceptance Criteria

- [x] g04.016 remains the named next implementation roadmap, or a
      contradictory inventory fact is written
- [x] Claude Agent, llama.cpp attached, and hosted OAuth stay planned
- [x] no adapter implementation card is marked ready from this inventory
- [x] no production code changes

## Evidence

Research 170 does not contradict hosted API-key DeepSeek continuation as
the first second-proof descriptor tranche.

## Validation

- compiled follow-on pointer in g04.015
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Roadmap g04.015 closes. Compile g04.016 only after this card.

## Stop Conditions

- DeepSeek continuation is about to be skipped without a written reason
- an implementation card is about to be marked ready from this inventory
