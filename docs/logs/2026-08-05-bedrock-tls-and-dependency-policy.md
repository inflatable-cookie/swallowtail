# 2026-08-05 Bedrock TLS And Dependency Policy

## Result

Bedrock now uses only the AWS SDK's modern default HTTPS client. Removing the
redundant legacy `rustls` feature drops Rustls 0.21, `rustls-webpki 0.101.7`,
legacy Hyper, and the three RustSec findings from the selected graph.

`deny.toml` now denies unknown dependency registries and Git sources, admits
the reviewed license set, and carries no advisory exception. The
`security:dependencies` Effigy selector passes advisory, license, and source
checks.

## Validation

- 28 focused Bedrock tests passed
- warnings-denied Bedrock clippy passed
- extracted 58-file Bedrock package compiled
- Rust 1.94.1 all-target Bedrock check passed
- dependency policy passed without warnings or exceptions

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Card 126 reviews the first supported public API and Rustdoc surface.
