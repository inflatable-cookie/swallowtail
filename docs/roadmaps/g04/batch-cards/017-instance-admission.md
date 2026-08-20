# 017 Instance Admission

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../006-addable-catalog-admission-and-config-fields.md`
Depends on: card 016

## Goal

Admit a configured instance from one addable route plus host-owned
configuration, written through the store port.

## Scope

1. Admission of one addable route into a configured instance.
2. Several instances of one family as distinct ids.
3. Persist through the 057 store.
4. Do not prepare, select a model, or change 047 readiness.

## Out Of Scope

- Contract 037 preparation
- sign-in loop
- overlay projection
- production adapter crates

## Acceptance Criteria

- [x] admission writes a `ConfiguredInstance` through the store
- [x] two instances of one family remain distinct ids
- [x] a discovered candidate still cannot execute
- [x] 047 snapshots are unchanged by admission

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `git diff --check`

## Auto-Continuation

Yes, into card 018.

## Stop Conditions

- Stop if admission prepares or mutates 047.
- Stop if instance ids collide for one family.

## Evidence

`admit_instance` writes `AdmittedInstanceRecord` through
`ConnectionLifecycleStore`. Two instances of one family keep distinct ids.
A discovered candidate id is `RouteAbsent`. Admission does not write access
status or change 047 `Ready` / `NotReady`.
