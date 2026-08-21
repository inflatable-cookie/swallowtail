# 062 Remaining Addable Surface Inventory

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../022-further-addable-inventory.md`
Depends on: completed g04.021

## Goal

Map remaining production routes onto the proved 057 shapes without
writing adapter descriptors.

## Scope

1. Start from the production route list. The six current addable rows
   are already proved.
2. Classify hosted API-key, installed, and local-runtime candidates.
3. Write a research note. Do not compile implementation cards.

## Out Of Scope

- adapter-local descriptors
- hosted OAuth
- OpenHands production wiring
- live provider, install, or login work

## Acceptance Criteria

- [ ] remaining production routes have a shape, skip, or gated reason
- [ ] sibling rows such as owned, headless, and response-only are named
- [ ] no production code changes

## Validation

- research note and named docs indexes
- `effigy qa:docs:index:research`
- `git diff --check`

## Auto-Continuation

Yes, into card 063.

## Stop Conditions

- Stop if inventory would require Swallowtail to store secrets or run a
  server.
- Stop if every remaining route is about to be marked addable.
