# 067 047 Presentation Fields

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../023-047-presentation-metadata.md`
Depends on: card 066

## Goal

Realize the named optional 047 presentation fields.

## Scope

1. Additive API in `public-api-unreleased`.
2. `public-api-0.3.3` stays immutable.
3. Snapshot still has no emails, tokens, or targets.

## Out Of Scope

- overlay hide/favourite as 047 fields
- accent color
- hosted OAuth

## Acceptance Criteria

- [ ] named fields project onto 047 without changing `Ready` / `NotReady`
- [ ] overlay markers remain overlay
- [ ] `public-api-0.3.3` stays immutable

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api`

## Auto-Continuation

No. Named addable implementations from g04.022 wait until this
milestone closes.

## Stop Conditions

- Stop if `Ready` / `NotReady` changes.
- Stop if overlay markers become 047 snapshot fields.
