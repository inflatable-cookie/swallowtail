# 264: Pin the development toolchain to Rust 1.100.0

- The root `rust-toolchain.toml` pins Rust `1.100.0` with rustfmt and clippy,
  so bare `cargo` commands in the checkout use that compiler. The unified MSRV
  stays `1.95`, and `rust-version`, the MSRV check and the public API baseline
  are unchanged.
- The CI jobs that run Cargo in the repository no longer install a stable
  toolchain that the pin would bypass. The docs and public API job now names
  `nightly-2026-08-05` explicitly for `cargo doc`, and `external-source` keeps
  its stable install because its consumer build runs outside the repository.
- Rust 1.100.0 clippy and rustfmt report no warnings across the workspace, and
  clippy with `-D warnings` still passes on the `1.95.0` floor.
