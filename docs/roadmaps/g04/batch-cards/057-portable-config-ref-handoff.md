# 057 Portable Config-Ref Handoff

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../020-config-ref-prepare-handoff.md`
Depends on: card 056

## Goal

Realize a portable handoff from admitted config refs to Contract 037
prepare inputs.

## Scope

1. Host still resolves refs. Swallowtail never stores values.
2. 037 remains after admission and still binds an exact target.
3. Amend 057 only if the handoff is a durable rule.
4. Additive API in `public-api-unreleased` if public types are added.

## Out Of Scope

- wiring all six adapters (card 058)
- overlay or 047 fields
- hosted OAuth
- rewriting `public-api-0.3.3`

## Acceptance Criteria

- [x] a consumer can hand an admitted instance to prepare without a
      second copy of the target identity
- [x] portable records still expose no path, URL, or env body
- [x] 047 still has no targets

## Evidence

- `AdmittedInstanceRecord::{credential_ref,config_ref}` supplies exact stored
  field lookup.
- Opaque retyping is provided by `InstanceTargetRef::from_config_field`,
  `ExecutableRef::from_config_field`, and
  `EnvironmentRef::from_config_field`.
- Contract 057 records the durable host-resolution rule; Contract 037 remains
  the post-admission exact-target boundary.
- `effigy validate:focused swallowtail-core swallowtail-runtime
  swallowtail-host-local`
- `effigy package:api`
- `git diff --check`

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

Yes, into card 058.

## Stop Conditions

- Stop if values leak into diagnostics, the store, or 047.
- Stop if admission prepares.
