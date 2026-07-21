# 023 OpenAI Background Responses Proof

Status: completed
Owner: Tom
Updated: 2026-07-21

## Purpose

Prove one provider-owned asynchronous direct run whose remote lifecycle can
outlive an SSE attachment, without making provider retention, reattachment,
subscription access, retry, or route selection implicit.

## Generation Runway

Keep g01 active. It contains 23 numbered roadmaps and remains below the normal
30-50 roadmap rollover range. This lane adds a new operation-lifecycle shape,
not a second generic OpenAI-compatible adapter.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary
- Contract 021: Provider-Owned Background Run And Temporary Retention Boundary

Research 015 exposes the missing provider-operation, stream-reattachment,
native-cancel, and temporary-retention rules. Card 074 must promote Contract
021 before production work.

## Goals

- [x] Promote an explicit provider-background and temporary-retention
      boundary without widening all structured runs.
- [x] Freeze the exact OpenAI Responses create, SSE, retrieve, reattach, and
      cancel corpus without live access.
- [x] Implement one public-API background structured-run driver.
- [x] Prove local and remote-authoritative hosted conformance, unconfirmed
      remote-stop evidence, redaction, and joined cleanup.

## Execution Plan

- [x] Background-operation contract and deterministic fixtures: card 074.
- [x] Production driver, conformance, and closeout: card 075.

## Cards

- `batch-cards/074-background-run-contract-and-fixtures.md` — completed
- `batch-cards/075-openai-background-driver-and-conformance.md` — completed

## Planning Checkpoint

Research 015 selects OpenAI Responses background mode over Cursor Cloud Agents,
remaining language SDKs, experimental Kimi Wire, preview ACP Rust work, and
similar hosted or attached HTTP routes. The proof uses the provider-supported
OpenAI public API with one exact endpoint audience and API-key lease. It does
not reuse ChatGPT, Codex, harness, subscription, or community OAuth access.

The first subset fixes `background=true`, `stream=true`, `store=false`, one
explicit model route, one positive output bound, required temporary provider
retention, one explicit deadline, one inference attempt, bounded cursor
reattachment, and native cancel. It excludes durable detach or resume, tools,
search, files, structured
output, conversation state, webhooks, Batch API, retry, fallback, and route
defaults.

Cursor remains the strongest later cloud-harness candidate. It requires a
separate operator decision on repository integration, provider-owned remote
workspace mutation, artifact handoff, and durable-agent deletion authority.

## Stop Condition

Stop before production if provider retention cannot be selected explicitly,
reattachment can start or replay an inference attempt, provider cancellation
truth cannot be represented, or the route depends on ChatGPT or Codex login
rather than one exact public-API credential.
