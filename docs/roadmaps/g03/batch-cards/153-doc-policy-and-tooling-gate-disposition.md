# 153 Doc-Policy And Tooling Gate Disposition

Status: completed
Owner: Tom
Created: 2026-08-08
Milestone: `../051-validation-machinery-and-index-closure.md`
Depends on: card 152

## Goal

Verify where each doc-policy and tooling gate actually runs, and record why
none of them needs per-commit CI.

## Scope

1. Confirm `qa:docs` runs at the milestone gate: it is a member of the
   `qa` sequence in `effigy.toml`, which the accepting roadmap card and the
   release flow run. Card 152 wired the index checks into it, so index
   drift is caught at the gate that was previously silent.
2. Record the disposition for each gate the audit had flagged as "not in
   CI":
   - `package:msrv` is already enforced per-commit: the CI `rust-floor` job
     pins `1.95.0` and runs clippy plus tests under it, and the `stable`
     job covers current stable.
   - `package:release-floor` stays release-gated: it syncs `Cargo.lock` to
     the workspace version during `effigy release prepare`, which must not
     run in read-only per-commit CI.
   - `qa:northstar` stays out: its only non-tautological check (the
     forbidden `--repo .` token) already runs inside `qa:docs` as
     `qa:docs:agent-defaults`; the remaining spine checks are
     paths-exist and contains tautologies.
   - `validate:selectors:test` stays out: a tooling break surfaces
     immediately in every focused-validation run; it is not a silent-drift
     risk.
3. The CI `stable` job keeps the pure-Python contract checks
   (route matrix, guide coverage, consumer front door, literal version
   expects), which need no effigy dependency.

## Out Of Scope

- changing what any gate validates
- release-tag or publication automation
- installing effigy into CI

## Acceptance

- [x] each flagged gate has a recorded disposition in this card
- [x] `qa:docs` is confirmed as a member of the `qa` milestone sequence

## Stop Conditions

- stop if a gate must run somewhere it cannot

## Auto-Continuation

Yes, to card 154 after acceptance.

## Validation

- `effigy qa:docs`
- `effigy qa` task listing shows `qa:docs` in the sequence

## Completion Evidence

- the audit's "tasks never run in CI" finding (M5) was re-examined against
  what each task actually does: the two MSRV toolchain legs are already
  exercised by the `rust-floor` and `stable` CI jobs, the release-floor
  gate is release-scoped by design, the northstar spine checks are
  tautological beyond the forbidden token already inside `qa:docs`, and a
  selector-tooling regression is immediately visible to contributors
- the real drift cause was unwired checks, not missing CI: card 152 wired
  the eight index gates plus the version-expects scan into `qa:docs`,
  which is a member of the `qa` milestone sequence; the next milestone gate
  now catches index and front-door drift
- no CI change was made; the `stable` job keeps the effigy-free Python
  contract checks
- `effigy qa:docs` runs all fifteen checks green
