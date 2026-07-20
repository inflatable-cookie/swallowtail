# Contract Kernel Vocabulary

Date: 2026-07-19
Status: recorded

## Outcome

Created `swallowtail-core` and `swallowtail-testkit`. Core now exposes pure
records for adapter identity, explicit capabilities, model identity and
metadata, opaque session and run references, event envelopes, provider
extensions, and safe diagnostics.

## Boundary Evidence

- no external crate dependencies
- `swallowtail-testkit` depends only on `swallowtail-core`
- no Nucleus, Soundcheck, Tauri, Tokio, or Codex references in crate sources
- no execution traits, process behavior, transport, credentials, or
  serialization commitment
- opaque provider data and internal diagnostic detail are redacted by default

## Validation

- `effigy check:rust`
- `effigy lint:rust`
- `effigy test` — seven tests passed
- `cargo doc --workspace --no-deps`
- crate-tree and coupling scans
