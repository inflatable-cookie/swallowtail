# 004 Spec Inventory Fold

Status: planned
Owner: Tom
Created: 2026-08-19
Milestone: `../002-route-readiness-spec-and-contract-targets.md`
Depends on: g04.001 card 003

## Goal

Fold the g04.001 research inventory into Spec 011 without changing the four
settled operator decisions.

## Scope

1. Replace remaining spec unknowns with inventory facts.
2. Keep authenticated-subject, library-max sign-in, persistence port, and
   model overlay decisions intact.
3. List only crate-placement and first-proof-route questions as later
   implementation gates.

## Out Of Scope

- contract file creation
- production code
- source-tag mutation

## Acceptance Criteria

- [ ] Spec 011 cites the inventory research note
- [ ] no settled decision is silently reversed
- [ ] remaining questions cannot change product policy

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, into card 005.
