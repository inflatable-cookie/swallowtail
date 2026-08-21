# 065 047 Presentation Field Inventory

Status: completed
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

- [x] a named field set or an explicit none
- [x] Ready/NotReady formula impact is none
- [x] no 047 type changes in this card

## Decision

The named optional 047 field set is one instance-level field:

- `instance_label`: the optional host-owned `InstanceLabel` already stored by
  Contract 057 on the admitted configured-instance record. A consumer may
  copy that value into the 047 configured-instance snapshot for presentation.
  It is not an identity, selection, routing, default, or readiness input.

No model-level presentation fields enter 047 in this tranche. Contract 057
overlay markers remain overlay fields: hide, ordinal, consumer-default, and
favourite. Accent color and other pure UI chrome stay consumer-owned.
Authenticated-subject values, emails, tokens, targets, provider defaults, and
other authority-bearing or consumer-policy fields remain outside the snapshot.

The 047 `Ready` / `NotReady` formula is unchanged. This card makes no 047 type
changes.

## Evidence

Research 171 and Contracts 047/057 leave instance labels as the only named
optional field already stored at the connection-lifecycle boundary. The
field set is not forked, so card 066 may proceed.

## Validation

- `effigy qa:docs:index:logs`
- `git diff --check`

## Auto-Continuation

Yes, into card 066 unless the field set is still forked.

## Stop Conditions

- Stop and ask if accent color or overlay markers are proposed as 047
  fields.
- Stop if emails, tokens, or targets would enter the snapshot.
