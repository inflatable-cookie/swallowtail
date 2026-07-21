# 014 Gemini ACP Proof

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Prove ACP v1 as a shared harness lifecycle through Gemini CLI without treating
the protocol as a universal adapter.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 009: Async Operation Lifecycle
- Contract 011: Runtime Conformance Profiles
- Contract 012: Interactive Session Options and Callback Exchange
- Contract 013: Interactive Session Access Policy
- Contract 015: ACP v1 Negotiation and Client Callbacks

## Goals

- [x] Promote exact ACP version, capability, callback, and extension rules.
- [x] Implement a provider-neutral ACP transport layer plus Gemini mapping.
- [x] Prove session, update, permission, filesystem, cancellation, and cleanup.

## Execution Plan

- [x] ACP authority and fixture batch: card 045.
- [x] Gemini ACP driver batch: card 046.
- [x] Protocol conformance and optional live probe: card 047.

## Cards

- `batch-cards/045-acp-v1-authority-and-fixtures.md` — completed
- `batch-cards/046-gemini-acp-driver.md` — completed
- `batch-cards/047-gemini-acp-conformance.md` — completed

## Acceptance Criteria

- [x] version negotiation and optional capabilities fail closed
- [x] client callbacks use existing host and consumer authority boundaries
- [x] provider extensions remain namespaced
- [x] absent local Gemini CLI blocks only the optional live probe

## Planning Checkpoint

After card 047, compare a second ACP agent only if it tests interoperability or
extension isolation not already covered.

## Stop Condition

Stop if current ACP authority or Gemini behavior cannot settle version,
permission, filesystem, or cancellation semantics.
