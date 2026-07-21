# Anthropic API Protocol Fixture

Date: 2026-07-20
Roadmap: 013
Card: 042

## Changed

- rechecked official authentication, versioning, Models, Messages, streaming,
  error, and rate-limit surfaces
- froze the provider-supported `api.anthropic.com` API-key subset at
  `anthropic-version: 2023-06-01`
- added a fixture-first Anthropic adapter crate with deterministic Models and
  Messages HTTP/SSE payloads and a bounded fake endpoint
- covered pagination, mutable and absent model limits, exact message request,
  ordered output, cumulative usage, request id, rate headers, HTTP and stream
  errors, unknown events, disconnect, and one-attempt enforcement
- updated Contract 014: provider-required maximum-output bounds are explicit
  consumer inputs, never adapter or catalogue defaults

## Decisions

- first auth proof: Console API key in `x-api-key`
- excluded auth: Workload Identity Federation, Claude subscription OAuth, and
  cloud-platform facades
- top-level unknown SSE event: ignore and continue
- unknown content block or delta: protocol failure
- cancellation: stop local reads, close connection, join work; no provider
  cancel or recovery request
- SDK default retries: excluded
- absolute RFC 3339 rate resets remain unknown unless the driver has a reliable
  reference for a relative observation

## Evidence

- nine focused fixture and fake-endpoint tests pass
- full repository QA passes with 159 tests
- payloads are synthetic; no live credential or authenticated request used

## Continuation

Card 043 is ready. Implement the shared maximum-output request input and the
Anthropic direct driver. Card 044 remains planned and in bounds.
