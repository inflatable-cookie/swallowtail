# 026 Overlay Refusals

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../009-model-presentation-overlay.md`
Depends on: card 025

## Goal

Refuse overlay operations that invent a model, copy a model from another
instance, or make `NotReady` selectable.

## Scope

1. Fail closed on unknown model ids.
2. Fail closed on markers whose instance id does not match the catalogue
   instance.
3. Fail closed if overlay would make a `NotReady` instance selectable.

## Out Of Scope

- first-proof catalogues
- 047 presentation metadata
- live provider catalogue calls

## Acceptance Criteria

- [x] unknown model ids are rejected
- [x] cross-instance markers are rejected
- [x] overlay cannot change selection readiness to `Ready`

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

No. Compile first-proof routes after g04.009 closes.

## Stop Conditions

- Stop if overlay can mark `NotReady` selectable.
- Stop if a model can be copied from another instance.
