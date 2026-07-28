# Alibaba And DeepSeek Structured Runs

Date: 2026-07-27
Card: `../roadmaps/g02/batch-cards/072-alibaba-and-deepseek-structured-runs.md`

## Changed

- Added independent `StructuredRun` registration, requirements, validation,
  low-level drivers, and prepared operations to Alibaba Model Studio and
  DeepSeek.
- Alibaba now sends one streamed `store=false` Responses request without
  conversation or previous-response identity.
- DeepSeek now sends one streamed, tool-free Chat Completions request with
  explicit high reasoning, output-token bound, and unmanaged-cache acceptance.
- Added request, output, usage, cancellation, terminal, joined cleanup, and
  local plus remote-authoritative fixture coverage.
- Kept Alibaba conversation cleanup and DeepSeek consumer-owned continuation
  unchanged.
- Changed the two solution-matrix structured-run cells from `No` to `Yes`.

## Current State

The matrix now has ten remaining structured-run gaps from the original audit:
six definite harness or connection projections, one retained Kimi local-server
projection, one owned-serving `Not applicable`, and two realtime-media `No`
values.

Kimi thread deletion remains unsupported. Retained Kimi local-server runs will
require explicit `DurableAllowed` policy and will not claim deletion on close.

Card 073 is ready for the xAI one-response WebSocket proof.

## Validation

- focused Alibaba and DeepSeek test suites pass
- strict Clippy passes for both adapters and all targets
- docs and provider-route checks pass
- no live provider access, credential, account, consumer edit, or release
  mutation occurred
