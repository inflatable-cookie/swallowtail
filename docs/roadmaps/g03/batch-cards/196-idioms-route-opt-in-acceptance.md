# 196 Idioms Route Opt-In Acceptance

Status: ready
Owner: Tom
Updated: 2026-08-09

## Goal

Close the route-path idioms lane with guide, example, architecture, and
release-baseline evidence.

## Scope

- guide and example for the route-path opt-in
- architecture note for the host ports, session option, and fold rule
- release-baseline handling: capability inventory, feature truth, and the
  internal patch set
- closeout evidence and the correction-loop checkpoint

## Out Of Scope

- learned backend and the correction-loop proxy
- version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- guide, example, matrix, architecture, and contracts stay mutually honest
- `effigy qa:docs` passes
- focused and extracted-package validation pass

## Validation

- `effigy qa:docs`
- `effigy validate:focused swallowtail-idioms swallowtail-runtime swallowtail-testkit swallowtail-adapter-codex`
