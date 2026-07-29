# 2026-07-29 Shared ACP Activity Projection

## Changed

- Added bounded typed decoding for every selected stable ACP session update.
- Preserved message and thought deltas, plan and tool replacement snapshots,
  tool terminal state, usage, modes, commands, configuration, session
  information, and content blocks.
- Added bounded unknown namespaces without retaining raw payloads.
- Added configurable aggregate, collection, and identifier bounds.
- Added malformed, oversize, contradictory, additive-field, content-shape,
  redaction, and semantic tests.
- Closed card 126 and made card 127 ready.

## Current State

- `swallowtail-protocol-acp` owns shared wire semantics only.
- Provider identity, access, model and mode policy, callbacks, runtime
  emission, lifecycle, and transport selection remain outside the decoder.
- Raw input, raw output, metadata payloads, and uninterpreted public JSON do
  not cross the stable record boundary.
- Stdio and remote transports preserve the same semantic payload without
  becoming the same transport identity.

## Evidence

- 78 complete ACP protocol tests
- 8 remote ACP transport tests
- strict protocol-crate clippy
- repository Rust, documentation, formatting, API, and package gates

## Next

Card 127 maps the shared records through exact Claude Agent, Gemini CLI, and
Kimi Code prepared activity profiles.
