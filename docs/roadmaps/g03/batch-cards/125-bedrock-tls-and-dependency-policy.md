# 125 Bedrock TLS And Dependency Policy

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../043-v0-1-0-source-release-readiness.md`
Depends on: card 124

## Goal

Remove the redundant vulnerable Bedrock TLS path and make dependency advisory,
license, and source policy deterministic.

## Scope

1. Remove AWS SDK legacy Rustls feature selection from Bedrock.
2. Prove only the modern qualified TLS path remains.
3. Add a reviewed dependency-policy configuration and Effigy selector.
4. Record direct dependency currentness without unrelated upgrades.

## Validation

- `effigy validate:focused swallowtail-adapter-bedrock`
- `effigy package:verify-affected swallowtail-adapter-bedrock`
- Rust `1.94.1` Bedrock all-target check
- dependency advisory, license, and source policy check

## Completion

- Bedrock no longer enables the AWS SDK's legacy `rustls` feature beside the
  modern default HTTPS client.
- `rustls 0.21`, `rustls-webpki 0.101.7`, legacy Hyper, and their duplicate
  support graph leave the lock file.
- `deny.toml` admits the exact selected license and crates.io source set with
  no advisory exceptions.
- `effigy security:dependencies` passes.
- 28 focused tests, warnings-denied clippy, extracted-package proof, and the
  Rust 1.94.1 all-target check pass.
- no provider, credential, consumer, or release mutation ran.

## Auto-Continuation

No. Card 126 is ready for public API review.
