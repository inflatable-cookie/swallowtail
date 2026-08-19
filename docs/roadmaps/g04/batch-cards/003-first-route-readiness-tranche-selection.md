# 003 Follow-On Roadmap Confirmation

Status: planned
Owner: Tom
Created: 2026-08-19
Milestone: `../001-route-availability-and-readiness-evidence.md`
Depends on: card 002

## Goal

Confirm g04.002 spec closeout and g04.003 source tag as the next roadmaps, and
keep facade implementation planned until that tag exists.

## Scope

1. Check the inventory against Spec 011 and the compiled g04.002 / g04.003
   files.
2. Amend those later roadmaps only if inventory finds a factual gap.
3. Do not mark persistence, sign-in, overlay, or admission implementation
   cards ready.

## Out Of Scope

- writing the contract
- implementing adapters
- preparing or cutting the source tag
- consumer application edits

## Acceptance Criteria

- [ ] g04.002 and g04.003 remain the next named roadmaps
- [ ] facade implementation stays planned until g04.003 tags
- [ ] inventory contradictions, if any, are written into Spec 011 or a later
      roadmap rather than silently dropped

## Validation

- compiled follow-on roadmaps and cards
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Roadmap g04.001 closes. Continue through g04.002.
