# 2026-08-05 Public API Semantic Baseline Checkpoint

## Result

The release API gate now compares normalized Rust API evidence rather than
source declarations. The reviewed inventory contains 7,819 entries across all
27 packages and includes Oh My Pi, which the old 26-package hash omitted.

Generation and comparison pin `cargo-public-api 0.52.0` and
`nightly-2026-08-05`, enable every package feature, and omit blanket,
auto-trait, and auto-derived implementation noise. The nightly is analysis
tooling only. Stable and MSRV release compilers are unchanged.

Metadata validation now agrees with the manifests and Contract 036: 27
packages, Oh My Pi's two internal dependencies and `live-probes` feature, Rust
1.90 for non-Bedrock packages, and Rust 1.94.1 for Bedrock.

## Documentation State

Shared macro sources now document opaque identities, host references, driver
roles, and repeated prepared-state accessors. The workspace missing-doc count
fell from 5,897 to 5,519 without suppression.

`swallowtail-host-local` and `swallowtail-transport-acp-remote` are the first
packages at zero and enforce `deny(missing_docs)` locally. Card 126 remains
active while the remaining package families receive the same API review.

## Validation

- semantic public API generation and comparison passed for 27 packages
- package metadata and dependency topology passed for 27 packages
- denied missing-doc builds passed for both closed support packages
- 245 focused core/runtime/support tests passed
- 107 focused Alibaba, Antigravity, and Cursor tests passed
- focused warnings-denied clippy passed for all seven touched packages

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with protocol package API review and Rustdoc, then move
through testkit, core/runtime, and adapter families before the workspace gate.
