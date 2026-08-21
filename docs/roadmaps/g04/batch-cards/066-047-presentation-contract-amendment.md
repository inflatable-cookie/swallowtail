# 066 047 Presentation Contract Amendment

Status: completed
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

- [x] 047/057 name the optional fields
- [x] `Ready` / `NotReady` rules are unchanged
- [x] overlay markers are still not 047 fields

## Amendment

Contracts 047 and 057 now name `instance_label` as the sole optional 047
presentation field in this tranche. It is copied from the exact host-owned
label stored on the Contract 057 admitted-instance record. It is not provider
identity, product chrome, a default, routing input, or readiness evidence.

The contracts continue to exclude overlay hide, ordinal, consumer-default,
and favourite; accent color and other consumer chrome; authenticated-subject
values, emails, tokens, and targets. The 047 `Ready` / `NotReady` formula is
unchanged.

Architecture records the same boundary and provenance.

## Evidence

Contracts 047/057 and `docs/architecture/system-architecture.md` were amended
for the card 065 field decision. No runtime realization or public API baseline
changed in this card.

## Validation

- `effigy qa:docs:index:logs`
- `git diff --check`

## Auto-Continuation

Yes, into card 067.

## Stop Conditions

- Stop if the amendment changes selection readiness.
- Stop if overlay metadata is copied onto 047 as selectable state.
