# 013 Lifecycle Core Records

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../005-connection-lifecycle-kernel.md`
Depends on: completed g04.004

## Goal

Add portable Contract 057 records to `swallowtail-core` without a store,
sign-in loop, or adapter descriptors.

## Scope

1. Topology group: hosted, installed, local-runtime. Not `ExecutionLayer`.
2. Addable-route descriptor: driver identity, topology, availability
   (available, unavailable, unsupported), credential or sign-in
   requirements.
3. Credential-field and config-field descriptors. Config values stay opaque
   host references.
4. Enablement and optional instance label.
5. Overlay marker record: hide, ordinal, consumer-default, favourite, keyed
   to configured-instance, provider, and model ids.
6. Authenticated-subject observation: email, login, or plan; redacted by
   default.

## Out Of Scope

- store trait or host-local adapters (cards 014-015)
- catalog assembly, admission, sign-in, host ports
- production adapter crates
- 047 snapshot fields

## Acceptance Criteria

- [x] records live in `swallowtail-core` with no runtime or host dependency
- [x] topology is not an `ExecutionLayer` alias
- [x] field descriptors and overlay markers carry no secret bytes or paths
- [x] overlay markers cannot be constructed with an empty model id
- [x] subject records default to redacted
- [x] `PlannedConnectionRolloverPolicy` is untouched

## Validation

- `effigy validate:focused swallowtail-core`
- `git diff --check`

## Auto-Continuation

Yes, into card 014.

## Stop Conditions

- Stop if a record would store raw secrets or a 047 email field.
- Stop if public-api `0.3.3` snapshots would be rewritten; additive items
  belong in `release-baselines/public-api-unreleased/`.
