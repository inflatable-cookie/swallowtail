# v0.3.0 Local Source Candidate

Date: 2026-08-08
Generation: g03

## Outcome

Selected `v0.3.0` as the next coordinated source release. The minor bump is
required because `codex_cli_binding` and `ollama_runtime_binding` now return
`Option<InterfaceVersionBinding>`. Consumers must handle invalid observed
version text explicitly.

The candidate keeps 28 packages, 34 production routes, Rust `1.95.0`, Apple
Silicon macOS verification, `publish = false`, and annotated-Git-tag-only
distribution.

## Local Evidence

- all 11 Effigy release gates passed at commit
  `cffe5ff831ac3d3662d252e9bcc2d1dd19f3728a`
- Nextest passed 1,509 tests across 147 binaries; 13 opt-in tests skipped
- all-features and no-default-features Clippy passed with warnings denied
- semantic API, missing-docs, metadata, security, release-floor, docs, route,
  and external source-consumer gates passed
- release simulation stopped only because `[Unreleased]` was empty and could
  not derive a next version; no release mutation ran

## Next

Prepare the `0.3.0` version and candidate state, commit and push the exact
candidate, then require canonical CI before separate tag authorization.
