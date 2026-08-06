# 2026-08-05 Core Admission Rustdoc Tranche

## Result

The second `swallowtail-core` public-API tranche now documents and locally
enforces the provider-neutral admission boundary.

Access documentation keeps credentials, entitlement, endpoint authorization,
runtime readiness, and support authority independent. It does not introduce an
aggregate readiness boolean. Capability documentation distinguishes a named
feature from its exact parameter constraints and keeps provider extensions
namespaced.

Operation requirements now document every route, topology, host-service,
policy, and bounded-feature input preflight evaluates. Preflight remains a pure
admission step: success freezes exact evidence into an immutable plan and does
not perform host or provider side effects. Stale-plan validation continues to
reject any material binding change.

Module-local denied-missing-doc lints cover `access`, `capability`,
`requirement`, and `preflight`. The tranche removes 311 warnings.
`swallowtail-core` has 537 remaining; the workspace falls from 4,706 to 4,395
without suppression. The crate is not yet closed under the package-wide gate.

## Validation

- focused core validation passed 65 tests and warnings-denied clippy
- extracted core package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 4,395 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with core route, instance, registration, event, and failure
records, then close the remaining specialized policy modules.
