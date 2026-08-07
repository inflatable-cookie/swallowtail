# 155 Retired-Script And Constant Consolidation

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../051-validation-machinery-and-index-closure.md`
Depends on: card 154

## Goal

Retire or re-home stale release scripts and consolidate release constants
where cheap.

## Scope

1. Archive or delete the retired candidate scripts with a recorded
   disposition in `scripts/README.md`:
   - `verify-packages-local.sh` (holds stale `1.93.0` MSRV at line 216 and
     re-implements archive.sh's audits inline)
   - `verify-candidate-provider-lifecycle.sh` (asserts the stale 26-row
     lifecycle posture at lines 125-128)
   - `verify-candidate-provider-facades.sh` (stale 26-route proof count)
   - `verify-candidate-consumers.sh`, `verify-candidate-provider-runtime.sh`
   - `check-muse-code-corpus.py` (fully unwired)
2. Re-source `verify-affected-packages.sh` and `verify-packages-local.sh` from
   `scripts/validation/archive.sh` where they still have active consumers,
   deleting the inline copies.
3. Consolidate the MSRV constant: consume `1.95` only from the authoritative
   baseline (`release-baselines/rust-toolchains-0.2.0.env`) in CI and the
   check scripts, and record the tag constant as derived from the workspace
   version where the check scripts allow it.

## Out Of Scope

- changing release authority or the tagged release line
- provider, public API, or consumer behavior changes

## Acceptance

- [ ] every retired script is archived or deleted with a disposition entry
- [ ] the active release path (`config/release.toml` gates through effigy)
      is unaffected
- [ ] MSRV appears once as a consumed value in the active gate surface

## Stop Conditions

- stop if consolidation changes what the active gates validate

## Auto-Continuation

Yes, to card 156 after acceptance.

## Validation

- `effigy qa`, `effigy package:check`, and the CI run on the card branch
