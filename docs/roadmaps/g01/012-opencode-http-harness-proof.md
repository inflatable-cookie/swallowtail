# 012 OpenCode HTTP Harness Proof

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Prove one attached non-Codex harness over HTTP and SSE with delegated provider
authentication, exact route selection, sessions, abort, and joined cleanup.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 012: Interactive Session Options and Callback Exchange
- Contract 013: Interactive Session Access Policy
- Contract 014: Hosted Transport, Credential, And Evidence Boundary

## Goals

- [x] Freeze the first supported OpenCode server protocol subset in fixtures.
- [x] Implement attached-server discovery, catalogue, and interactive sessions.
- [x] Normalize SSE events, cancellation, failure, and cleanup.
- [x] Prove provider/model identity and delegated-auth boundaries.

## Execution Plan

- [x] Protocol and fixture batch: card 039.
- [x] Adapter implementation batch: card 040.
- [x] Harness conformance and optional installed probe: card 041.

## Cards

- `batch-cards/039-opencode-http-protocol-fixtures.md` — completed
- `batch-cards/040-opencode-http-harness-driver.md` — completed
- `batch-cards/041-opencode-http-conformance.md` — completed

## Acceptance Criteria

- [x] explicit endpoint and observed server version replace default-port
      assumptions
- [x] provider and model ids remain separate route evidence
- [x] provider auth stays delegated to OpenCode
- [x] attached close never stops the external server
- [x] no OpenCode config or credential store is inspected or mutated

## Planning Checkpoint

After the harness proof, compare its shared HTTP/SSE behavior against the
Anthropic direct driver. Promote only genuinely portable deltas.

## Stop Condition

Stop on undocumented permission semantics, required auth mutation, or protocol
drift that cannot be fixed in deterministic fixtures.
