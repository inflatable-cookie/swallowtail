# 2026-08-05 Testkit Rustdoc Closure

## Result

`swallowtail-testkit` now documents and enforces its complete public surface.

The review retained its exported assertions, scenario enums, fixture builders,
prepared-evidence helpers, and inspection accessors. These are the reusable
conformance vocabulary downstream adapter packages need; none was public only
to support crate-internal implementation.

The documentation now names the invariant or rejected condition behind every
scenario rather than repeating item names. The crate root denies missing public
documentation. Workspace all-feature warnings fall by 353 from 5,182 to 4,829
without allowances or suppression. Five of 27 packages are closed under the
release documentation gate.

## Validation

- all-feature denied-missing-doc Rustdoc passed
- focused validation passed 83 tests and warnings-denied clippy
- extracted package proof passed
- workspace all-feature Rustdoc completed with 4,829 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the provider-neutral `swallowtail-core` and
`swallowtail-runtime` API families before reviewing adapter packages.
