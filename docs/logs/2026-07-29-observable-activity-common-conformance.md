# 2026-07-29 Observable Activity Common Conformance

## Context

Cards 119-120 added portable activity records and exact prepared route
profiles. Card 121 closes the common kernel before any provider adapter maps
native events.

## Changes

- Added reusable activity trace fixtures for complete,
  update-and-completion, completion-only, unavailable, callback, direct-tool,
  intermediate assistant, final assistant, reasoning-summary, and unknown
  semantic cases.
- Added one public assertion over an exact `ObservableActivityProfile` and the
  existing ordered `RuntimeEvent` stream.
- Enforced per-kind lifecycle, content stream, disclosure, correlation, and
  unknown-event posture in the shared assertion.
- Required callback and direct-tool correlation ids to match their separate
  exchange events.
- Proved final assistant activity and final operation output remain distinct
  events even when their task content agrees.
- Added bounds, redaction, ordering failure, safe unknown rejection, and
  unverified-newer non-widening coverage.
- Migrated common ordered-event conformance to carry activity through the
  existing stream.
- Added public one-stream usage guidance.

No production adapter gained a positive activity profile. No provider effect,
live authentication, consumer edit, package publication, release-candidate
replacement, or external release mutation occurred.

## Validation

- `cargo test -p swallowtail-core` — 54 passed
- `cargo test -p swallowtail-testkit` — 67 passed
- `effigy format:check`
- `effigy check:rust` — every workspace crate and target compiled
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy package:api` — 23 public-API baselines passed after the intentional
  additive testkit refresh
- `effigy doctor` — unchanged 111 oversized-file findings: 83 warnings and
  28 errors

## Continuation

Roadmap g02.035 is complete. Card 122 is ready: revalidate the maintained Codex
range and freeze exact app-server and exec activity corpora before production
mapping. Cards 059, 097, and 098 remain paused and in bounds.
