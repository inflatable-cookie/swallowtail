# Debug Observation Guide And Example

Date: 2026-08-08
Roadmap: g03.055
Card: 171

## Outcome

Host opt-in wiring for Contract 053 is documented and exemplified.

`docs/guides/debug-observation.md` covers registration, kinds, redaction,
and non-interference. Portable failure handling and key concepts point at it.
`crates/swallowtail-runtime/examples/debug_observation_host.rs` shows a
compiling recording observer. Contract 052 feature-family coverage stays
unchanged; debug observation is an operator sink, not a matrix feature.

Milestone g03.055 is complete. The generation returns to its evidence gate.

## Local Validation

- `effigy check:examples`: passed
- `effigy qa:docs`: passed
- `effigy package:api`: unchanged at the v0.3.0 candidate baseline

## Boundaries

No Nucleus/Soundcheck commits, tag, or release.
