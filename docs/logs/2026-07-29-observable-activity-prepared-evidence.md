# 2026-07-29 Observable Activity Prepared Evidence

## Context

Card 119 made provider-visible work representable on the existing ordered
runtime stream. Card 120 adds route fidelity before any adapter maps native
events.

## Changes

- Added `ObservableActivity` as a separate capability from ordered
  `StreamingEvents`.
- Added route profiles for portable kinds, lifecycle fidelity, content
  streams, disclosure, correlation, unknown-event posture, and qualified
  interface behavior.
- Added cumulative constraint truth: complete lifecycle satisfies consumers
  requiring completion-only delivery, while the inspected profile remains
  exact.
- Added available, unavailable, and not-applicable profile states.
- Added transport identity and the immutable activity profile to prepared
  operation evidence.
- Required available profiles to remain within bound capability evidence and
  match the qualified interface behavior revision.
- Bound unverified-newer interfaces to the latest qualified behavior revision
  instead of widening from newly observed fields.
- Kept every existing adapter on the default unavailable or not-applicable
  path. No adapter gains a positive activity claim in this batch.

No provider effect, live authentication, consumer edit, package publication,
release-candidate replacement, or external release mutation occurred.

## Validation

- `cargo test -p swallowtail-core` — 53 passed
- `cargo test -p swallowtail-runtime` — 84 passed
- `cargo test -p swallowtail-testkit`
- focused prepared-operation tests — six passed
- `effigy format:check`
- `effigy package:api` — 23 crate baselines pass after the intentional
  additive core and runtime refresh
- `effigy check:rust`
- `effigy lint:rust`

`effigy doctor` remains at the pre-existing 111 oversized-file findings:
83 warnings and 28 errors. The activity profile, prepared validation, and
focused fixtures were split so this batch adds no finding.

## Continuation

Card 121 is ready. It adds reusable full, update-and-completion,
completion-only, unavailable, unknown, correlation, and unverified-newer
conformance before any provider mapping. Cards 059, 097, and 098 remain paused
and in bounds.
