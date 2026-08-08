# 171 Debug Observation Guide And Example

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../055-opt-in-debug-observation-seam.md`
Depends on: card 169

## Goal

Make the opt-in host debug seam discoverable: one guide, one compiling example
or testkit pattern, and a cross-link from portable failure handling.

## Scope

1. Add `docs/guides/debug-observation.md` covering registration, observation
   kinds, redaction, non-interference, and when ordinary apps should skip it.
2. Cross-link from `portable-failure-handling.md` and the guides index.
3. Add or extend a compiling example / testkit recording pattern that shows
   host registration without implying product logging policy.
4. Keep Contract 052 route-guide obligations unchanged; this is a portable
   operator/debug surface, not a new provider route.

## Out Of Scope

- Nucleus or Soundcheck commits
- adapter emission beyond what cards 169-170 already landed
- release notes or tag work

## Acceptance

- [x] guide states observer registration is optional
- [x] guide separates safe diagnostics from debug observations
- [x] example or testkit pattern compiles and demonstrates recording
- [x] docs/route QA gates that cover guides still pass

## Closeout

Added `docs/guides/debug-observation.md`, guides index + key-concepts +
failure-handling cross-links, and
`crates/swallowtail-runtime/examples/debug_observation_host.rs`. Left the
Contract 052 feature-family table unchanged so debug observation stays an
operator opt-in surface rather than a matrix feature column.

## Validation

- `effigy check:examples`: passed
- `effigy qa:docs`: passed
- `effigy package:api`: unchanged at the v0.3.0 candidate baseline
