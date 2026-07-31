# 044 Local Host Deadline Convenience

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../017-prepared-facade-multi-consumer-usability.md`
Depends on: card 043

## Goal

Move repeated local nanosecond deadline arithmetic behind the concrete host
composition without choosing a timeout policy.

## Scope

1. Add `LocalHostServices::deadline_after(Duration)`.
2. Derive from the composition's monotonic origin.
3. Convert duration nanoseconds with bounded integer conversion.
4. Saturate duration or instant overflow at `u64::MAX`.
5. Add deterministic pure conversion coverage plus composition evidence.

## Acceptance Criteria

- [x] an explicit duration derives one deadline after the local host's current
  monotonic instant
- [x] conversion and addition overflow saturate
- [x] the method does not wait, cancel, or choose a default duration
- [x] no runtime-wide clock-unit assumption is introduced
- [x] focused host-local validation passes
- [x] card 045 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-host-local`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 045 after focused host-local acceptance.

## Evidence

- `LocalHostServices::deadline_after` derives from the concrete local monotonic
  clock
- deterministic conversion and both overflow paths pass
- focused host-local validation: 31 passed
