# 196 Idioms Route Opt-In Acceptance

Status: completed
Owner: Tom
Updated: 2026-08-09

## Goal

Close the route-path idioms lane with guide, example, architecture, and
release-baseline evidence.

## Scope

- guide and example for the route-path opt-in
- architecture note for the host ports, session option, and fold rule
- release-baseline handling: capability inventory, feature truth, and the
  internal patch set
- closeout evidence and the correction-loop checkpoint

## Out Of Scope

- learned backend and the correction-loop proxy
- version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [x] guide, example, matrix, architecture, and contracts stay mutually
      honest
- [x] `effigy qa:docs` passes
- [x] focused and extracted-package validation pass

## Validation

- [x] `effigy qa:docs` — all gates pass
- [x] `effigy validate:focused swallowtail-idioms swallowtail-runtime
      swallowtail-testkit swallowtail-adapter-codex` — 467 tests pass
- [x] `effigy package:verify-affected swallowtail-idioms swallowtail-runtime
      swallowtail-testkit swallowtail-adapter-codex` — extracted package
      proof passes (cross-crate example dev-deps avoided; opt-in example
      lives in `swallowtail-runtime`)
- [x] `cargo fmt --check` and warnings-denied clippy pass
- [x] architecture records the realized route-path opt-in; Contract 056
      accepted; capability and host-service kinds registered
