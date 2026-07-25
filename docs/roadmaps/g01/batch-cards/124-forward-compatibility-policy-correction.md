# 124 Forward Compatibility Policy Correction

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../041-qualified-support-and-newer-version-execution.md`

## Objective

Correct the hard latest-qualified ceiling before further compatibility work.

## Scope

- define qualified support as the guaranteed window
- define permitted unverified-newer execution separately
- retain exact version identity and visible support posture
- retain hard rejection for known-incompatible and unordered unknown points
- update installed-executable discovery classification
- recompile the active roadmap before implementation

## Acceptance Criteria

- [x] qualified and unverified support claims cannot be confused
- [x] no upstream patch requires an immediate Swallowtail release
- [x] consumer warning or rejection policy remains downstream
- [x] exact exclusions and runtime drift remain closed
- [x] one sole ready implementation card remains

## Validation

- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

Yes. Continue into the provider-neutral assessment after the contract and
roadmap are coherent.

## Outcome

Contract 029 now separates guaranteed qualified support, permitted
unverified-newer execution, and known incompatibility. Contract 032 preserves
the same three-way posture in installed-executable observations. Roadmap 041
owns the core and first adapter proof before provider-coverage selection
resumes.
