# 153 Doc-Policy And MSRV Tasks Into CI

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../051-validation-machinery-and-index-closure.md`
Depends on: card 152

## Goal

Run the doc-policy, MSRV, and release-floor gates in CI so drift cannot land
on main undetected.

## Scope

1. Add `qa:docs`, `qa:northstar`, `package:msrv`, `package:release-floor`,
   and `validate:selectors:test` to the appropriate CI workflow
   (`.github/workflows/ci.yml`), keeping them separate from the tagged release
   jobs where the workflow already distinguishes them.
2. Confirm the lockfile-sync gate inside `check-release-floor.sh` passes in
   CI.
3. Keep live probes out of CI; they remain operator-gated on demand.

## Out Of Scope

- changing what the gates check
- release-tag or publication automation

## Acceptance

- [ ] all five task families run in CI and pass
- [ ] a deliberately broken link or forbidden token fails the docs job
- [ ] MSRV drift between the baseline env and CI is caught

## Stop Conditions

- stop if a gate cannot run in the CI environment for a reason other than the
  gate itself

## Auto-Continuation

Yes, to card 154 after acceptance.

## Validation

- CI run on the card branch plus `effigy qa:docs`, `effigy qa:northstar`
