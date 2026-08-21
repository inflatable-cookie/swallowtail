# 066 047 Presentation Contract Amendment

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../023-047-presentation-metadata.md`
Depends on: card 065

## Goal

Amend 047 and 057 for the named optional presentation field set.

## Scope

1. Selection readiness formula is unchanged.
2. Overlay markers stay overlay, not snapshot fields.
3. Architecture records the named fields.

## Out Of Scope

- runtime realization (card 067)
- accent color
- hosted OAuth
- rewriting `public-api-0.3.3`

## Acceptance Criteria

- [ ] 047/057 name the optional fields
- [ ] `Ready` / `NotReady` rules are unchanged
- [ ] overlay markers are still not 047 fields

## Validation

- `effigy qa:docs:index:logs`
- `git diff --check`

## Auto-Continuation

Yes, into card 067.

## Stop Conditions

- Stop if the amendment changes selection readiness.
- Stop if overlay metadata is copied onto 047 as selectable state.
