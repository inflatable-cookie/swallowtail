# 024 Instance Update Observation

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../008-readiness-refresh-subject-and-updates.md`
Depends on: card 023

## Goal

Project an instance update affordance from Contract 029 claims and Contract
032 installed-executable observations.

## Scope

1. Derive a safe update observation from an existing compatibility claim and
   an optional installed-executable observation.
2. Do not install, upgrade, downgrade, or authenticate.
3. Do not create a second currentness system.

## Out Of Scope

- overlay projection
- running a version-currentness checkpoint
- rewriting Contract 029 or 032 claims
- live harness installs

## Acceptance Criteria

- [x] observation reuses 029 classification and 032 evidence
- [x] it cannot create a configured instance or start sign-in
- [x] no install or upgrade side effect
- [x] `public-api-0.3.3` stays immutable

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

No. g04.008 closes. g04.009 stays planned until refresh and subject exist.

## Stop Conditions

- Stop if this becomes a second currentness system.
- Stop if observation installs, authenticates, or admits an instance.
