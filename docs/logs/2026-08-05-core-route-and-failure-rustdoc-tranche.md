# 2026-08-05 Core Route And Failure Rustdoc Tranche

## Result

The third `swallowtail-core` public-API tranche now documents and locally
enforces configured route identity, discovery, portable event, diagnostic, and
failure contracts.

Configured instances remain host-admitted selections rather than discovery
results. Model routes carry a second capability scope and optional separate
provider identity. Driver descriptors expose static claims; discovery outcomes
observe candidates without promoting them into configuration.

Common event envelopes preserve provider extensions as opaque, redacted bytes
under explicit preserve-or-reject policy. Internal diagnostic detail remains
excluded from default formatting. Portable failure origin, kind, and recovery
stay independent and additive to exact route diagnostic codes.

Module-local denied-missing-doc lints cover `instance`, `registration`,
`event`, `diagnostic`, and `failure`. The tranche removes 152 warnings.
`swallowtail-core` has 385 remaining; the workspace falls from 4,395 to 4,243
without suppression. The crate is not yet closed under the package-wide gate.

## Validation

- focused core validation passed 65 tests and warnings-denied clippy
- extracted core package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 4,243 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the remaining core session, activity, transport, and
specialized policy modules, then enforce denied missing docs at the crate root.
