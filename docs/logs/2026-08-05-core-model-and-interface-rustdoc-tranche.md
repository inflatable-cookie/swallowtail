# 2026-08-05 Core Model And Interface Rustdoc Tranche

## Result

The first `swallowtail-core` public-API tranche now documents and locally
enforces its model-catalogue and interface-version contracts.

The review preserves stable adapter-owned model identity separately from
mutable presentation and source-bound catalogue observations. Unknown provider
values stay bounded and carry their integration source. Missing source evidence
remains `None` rather than becoming a false negative.

Interface claims keep three different outcomes: qualified behavior evidence,
explicitly permitted but unverified stable-newer evidence, and incompatibility.
Exact exclusions and opaque-version equality remain visible in the API docs.

Module-local denied-missing-doc lints cover `model`, `model_catalog`, and
`interface_version`. The tranche removes 123 warnings. `swallowtail-core` has
848 remaining; the workspace falls from 4,829 to 4,706 without suppression.
The crate is not yet closed under the package-wide documentation gate.

## Validation

- focused core validation passed 65 tests and warnings-denied clippy
- extracted core package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 4,706 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the core access, capability, requirement, and preflight
contract family before moving to runtime execution records.
