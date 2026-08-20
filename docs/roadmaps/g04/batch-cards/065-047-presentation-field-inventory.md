# 065 047 Presentation Field Inventory

Status: planned
Owner: Tom
Created: 2026-08-20
Milestone: `../023-047-presentation-metadata.md`
Depends on: completed g04.022

## Goal

Classify which optional fields may enter 047 versus overlay versus
consumer chrome.

## Scope

1. Preferred direction: optional already-stored 057 instance labels onto
   the snapshot.
2. Overlay hide, ordinal, consumer-default, and favourite stay overlay.
3. Accent color stays consumer-owned.
4. Do not change 047 yet.

## Out Of Scope

- contract amendment (card 066)
- realization (card 067)
- overlay marker redesign

## Acceptance Criteria

- [ ] a named field set or an explicit none
- [ ] Ready/NotReady formula impact is none
- [ ] no 047 type changes in this card

## Validation

- `effigy qa:docs:index:logs`
- `git diff --check`

## Auto-Continuation

Yes, into card 066 unless the field set is still forked.

## Stop Conditions

- Stop and ask if accent color or overlay markers are proposed as 047
  fields.
- Stop if emails, tokens, or targets would enter the snapshot.
