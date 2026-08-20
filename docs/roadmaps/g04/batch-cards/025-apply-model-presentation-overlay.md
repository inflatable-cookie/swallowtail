# 025 Apply Model Presentation Overlay

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../009-model-presentation-overlay.md`
Depends on: completed g04.008

## Goal

Apply stored overlay markers to one bound catalogue result without changing
047 readiness.

## Scope

1. Project hide, ordinal, consumer-default, and favourite.
2. Key to exact configured-instance, provider, and model ids.
3. Keep provider catalogue defaults distinct from consumer-default.

## Out Of Scope

- overlay refusals (card 026)
- 047 snapshot fields
- accent color
- first-proof catalogues

## Acceptance Criteria

- [x] markers apply only to matching catalogue identities
- [x] provider default is not rewritten as consumer-default
- [x] 047 `Ready` / `NotReady` is unchanged

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-testkit`
- `git diff --check`

## Auto-Continuation

Yes, into card 026.

## Stop Conditions

- Stop if overlay metadata is added to 047.
- Stop if gateway rows are flattened into one catalogue.
