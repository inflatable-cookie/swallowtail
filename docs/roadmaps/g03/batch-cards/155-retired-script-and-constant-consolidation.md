# 155 Retired-Script And Constant Consolidation

Status: completed
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

- [x] every retired script is archived or deleted with a disposition entry
- [x] the active release path (`config/release.toml` gates through effigy)
      is unaffected
- [x] MSRV appears once as a consumed value in the active gate surface

## Stop Conditions

- stop if consolidation changes what the active gates validate

## Auto-Continuation

Yes, to card 156 after acceptance.

## Validation

- `effigy qa`, `effigy package:check`, and the CI run on the card branch

## Completion Evidence

- archived eight retired registry-candidate scripts as frozen evidence in
  `release-candidates/0.1.0/scripts/` (verify-packages-local,
  verify-release-candidate, verify-candidate-consumers,
  verify-candidate-provider-facades, verify-candidate-provider-lifecycle,
  verify-packaged-consumer-runtime, verify-packaged-provider-runtime, and
  the unwired check-muse-code-corpus.py), with a disposition entry in
  `scripts/README.md`; `release-package-set.sh` stays live because
  `verify-affected-packages.sh` sources its package lists, and
  verify-affected already re-sources `validation/archive.sh`
- MSRV consolidation: `check-package-metadata.sh`, `verify-source-consumer.sh`,
  and the CI `rust-floor` job now consume `SWALLOWTAIL_MSRV` from
  `release-baselines/rust-toolchains-0.2.0.env` (the CI job sources the file
  and uses `cargo +$RUST_MSRV` instead of the pinned YAML value); the only
  remaining `1.95` mentions are prose
- v0.3.0 baseline: the operator authorized aiming the next release at v0.3.0
  after the semantic API gate flagged the sanctioned card-148
  `codex_cli_binding` signature change against the frozen v0.1.0 record;
  created `release-baselines/public-api-0.3.0/` (28 packages, current
  source), re-pointed `check-public-api.sh` at it, and pointed
  `check-package-metadata.sh`'s package list at it; the consumer front door
  keeps the released v0.2.0 baseline because it validates the tagged install
  block
- the active release path is untouched and `config/release.toml` gates are
  unchanged
- `effigy qa` and `effigy package:check` pass in full
