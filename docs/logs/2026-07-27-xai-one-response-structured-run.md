# xAI One-Response Structured Run

Date: 2026-07-27
Card: `../roadmaps/g02/batch-cards/073-xai-one-response-structured-run.md`

## Changed

- Added independent `StructuredRun` registration, requirements, validation,
  low-level execution, and prepared operation to the xAI Responses WebSocket
  adapter.
- Added one-response transport execution with `store=false`, no
  previous-response id, no retry, no reconnect, and no continuation escape.
- Preserved streamed output, usage, billed cost, cancellation, deadlines,
  provider failure, connection invalidation, and joined credential cleanup.
- Added deterministic rejection-before-effects, cancellation, local-host,
  remote-authoritative-host, prepared-facade, and provider-neutral hosted-run
  coverage.
- Kept the existing interactive private-continuation path unchanged.
- Changed the xAI solution-matrix structured-run cell from `No` to `Yes`.

## Current State

Roadmap g02.022 is complete. Alibaba, DeepSeek, and xAI now prove Contract
039 across stateless HTTP/SSE and connection-scoped WebSocket transports.

The remaining conversion lane starts with Claude Agent ACP, then Pi RPC,
OpenCode HTTP, Gemini CLI headless, and the separately qualified Kimi routes.
Kimi thread deletion remains unsupported.

## Validation

- all 26 xAI adapter tests pass
- strict xAI all-target Clippy passes
- docs and provider-route checks pass
- the 21-row, 45-column CSV parses, remains provider-sorted, and reports xAI
  structured execution as `Yes`
- no live provider access, credential, account, consumer edit, or release
  mutation occurred
