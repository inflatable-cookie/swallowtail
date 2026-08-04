# 064 Configured Provider Instance Catalogue Admission

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../024-configured-provider-instance-catalogue.md`
Depends on: card 063

## Goal

Implement Contract 047's bounded, provider-neutral configured-instance
catalogue and strict prepared-evidence admission.

## Scope

1. Add safe instance, route, credential-posture, model-catalogue, and catalogue
   records to `swallowtail-runtime`.
2. Validate exact driver, instance, facade, target, host, access, and catalogue
   source agreement during admission.
3. Derive conservative selection readiness from exact evidence.
4. Bound instances, routes, and models; reject duplicates.
5. Add deterministic positive, negative, unavailable, and redaction coverage.

## Out Of Scope

- provider discovery or model-catalogue execution
- provider-specific factories
- routing, selection, default, fallback, refresh, or persistence policy
- consumer edits or live provider work

## Acceptance Criteria

- [x] ready evidence projects as selectable
- [x] every incomplete or negative posture remains visible but non-selectable
- [x] exact model-catalogue source and model/provider identity survive
- [x] mismatched and duplicate evidence fails safely
- [x] credential and target authority are absent from the public projection
- [x] focused runtime validation passes

## Validation

- `effigy validate:focused swallowtail-runtime`
- `effigy package:verify-affected swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Continue to card 065 after deterministic admission and focused validation pass.
