# 2026-08-05 Core Rustdoc Closure

## Result

`swallowtail-core` now documents and enforces its complete public surface.

The final review closes session resource and callback authority, provider
session discovery and lifecycle truth, observable activity fidelity, direct
continuation bounds, harness-RPC policy, realtime media, attached-runtime
evidence, remote ACP transport restrictions, and the remaining specialized
policy records.

The documentation preserves the boundaries encoded by the types. Provider
requests gain answer authority only through explicit namespace exchange.
Session discovery never becomes configuration implicitly. Lifecycle effects
keep confirmed, pre-effect failure, partial, and post-effect uncertainty
distinct. Observable activity describes maximum route fidelity without adding
control authority. Remote ACP grants no implicit redirect, retry, reconnect,
fallback, replay, pooling, or multiplexing.

Crate-root `deny(missing_docs)` now covers every feature. The final tranche
removes 385 warnings and the complete core review removes 971. Workspace
warnings fall from 4,243 to 3,858 without suppression. Six of 27 packages are
closed under the release documentation gate.

## Validation

- all-feature denied-missing-doc Rustdoc passed
- focused core validation passed 65 tests and warnings-denied clippy
- extracted core package proof passed
- workspace all-feature Rustdoc completed with 3,858 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with `swallowtail-runtime`, then review adapter package
families before the workspace documentation gate.
