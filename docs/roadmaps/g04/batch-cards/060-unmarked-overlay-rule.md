# 060 Unmarked Overlay Rule

Status: planned
Owner: Tom
Created: 2026-08-20
Milestone: `../021-unmarked-overlay-rows.md`
Depends on: card 059

## Goal

Realize the chosen unmarked-row overlay rule.

## Scope

1. Either overlay keys absent-provider rows by instance plus model, or
   architecture records unmarked as durable.
2. Mixed gateway rows stay consumer assembly.
3. 047 `Ready` / `NotReady` is unchanged.

## Out Of Scope

- adapter catalogue identity repairs that invent `provider_id`
- 047 presentation metadata
- hosted OAuth
- rewriting `public-api-0.3.3`

## Acceptance Criteria

- [ ] the chosen rule is realized in overlay or recorded as durable
- [ ] no invented catalogue `provider_id`
- [ ] overlay still cannot mark `NotReady` selectable

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

Yes, into card 061.

## Stop Conditions

- Stop if overlay invents a provider id.
- Stop if overlay changes `Ready` / `NotReady`.
