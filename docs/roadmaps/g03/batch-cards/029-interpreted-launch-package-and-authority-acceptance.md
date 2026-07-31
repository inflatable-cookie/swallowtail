# 029 Interpreted Launch Package And Authority Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../011-host-approved-interpreted-executable-launch.md`
Depends on: card 028

## Goal

Accept the interpreted-launch boundary across affected packages and current
authority surfaces, then close g03.011.

## Scope

1. Verify affected local-host and Pi packages from extracted archives.
2. Confirm launcher values remain absent from stable records and diagnostics.
3. Reconcile architecture, research, front doors, roadmap state, and batch-card
   index.
4. Record one meaningful closeout log and one bounded next task.

## Acceptance Criteria

- [x] affected package archives compile independently
- [x] native and interpreted launch tests pass together
- [x] public docs distinguish host launch machinery from harness version truth
- [x] Research 084 is accepted and linked to realized contracts
- [x] roadmap g03.011 and cards 027-029 close honestly
- [x] the sole Next Task pointer returns to the g03 checkpoint

## Validation

- `effigy package:verify-affected swallowtail-host-local swallowtail-adapter-pi`
- `effigy qa:docs`
- `effigy qa:northstar`
- `cargo fmt --all --check`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

No. Return to the g03 compatibility-maintenance checkpoint after closeout.

## Evidence

- extracted 37-file `swallowtail-host-local` and 85-file
  `swallowtail-adapter-pi` packages compiled together in six seconds
- deterministic host-local and Pi suites passed with the native and interpreted
  paths present together
- exact installed Pi `0.83.0` discovery passed through the host-private recipe
- Contracts 010 and 032, architecture, guide, research, front doors, roadmap,
  and closeout log now agree
