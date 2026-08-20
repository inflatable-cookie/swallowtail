# 023 Authenticated Subject Observation

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../008-readiness-refresh-subject-and-updates.md`
Depends on: card 022

## Goal

Expose optional provider-disclosed subject observation on the connection
facade: email, login, or plan, redacted by default and revealable.

## Scope

1. Make `SubjectDisclosure::Absent` constructible so adapters can report
   that a field was not disclosed.
2. Observe subject without storing it as a configured-instance id.
3. Keep it out of 047, default diagnostics, and routing keys.
4. `Debug` still redacts revealed values.

## Out Of Scope

- overlay projection
- blur/unblur UI
- putting emails on `AdmittedInstanceRecord` as identity
- live provider identity probes

## Acceptance Criteria

- [x] a field can be Absent, Redacted, or Revealed
- [x] default observation is not revealed
- [x] revealed email does not appear in `Debug`
- [x] 047 types still have no email, login, or plan fields

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Yes, into card 024.

## Stop Conditions

- Stop if subject becomes an instance id, 047 field, or default diagnostic.
- Stop if Absent remains unrepresentable.
