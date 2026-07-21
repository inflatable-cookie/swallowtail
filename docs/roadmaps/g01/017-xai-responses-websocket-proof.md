# 017 xAI Responses WebSocket Proof

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Prove a connection-scoped direct-model interactive session over provider-
supported WebSocket transport, including exact provider-billed cost evidence.

## Generation Runway

Advance g01 coverage expansion with the first long-lived direct-inference
connection. Preserve the later second-ACP, owned-serving, and SDK checkpoints
without adding similar HTTP/SSE providers first.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary
- Contract 016: Connection-Scoped Direct Sessions And Billed Cost

## Goals

- [x] Freeze the supported xAI Responses WebSocket subset in deterministic
      fixtures.
- [x] Realize resource-free direct sessions and provider-billed-cost records.
- [x] Implement one serial, store-disabled xAI WebSocket session driver.
- [x] Prove cancellation, lifetime, disconnect, redaction, and joined cleanup.

## Execution Plan

- [x] Protocol and contract fixture batch: card 052.
- [x] Runtime records and xAI driver batch: card 053.
- [x] Direct-session conformance and closeout batch: card 054.

## Cards

- `batch-cards/052-xai-websocket-protocol-fixtures.md` — complete
- `batch-cards/053-xai-websocket-session-driver.md` — complete
- `batch-cards/054-xai-websocket-conformance.md` — complete

## Acceptance Criteria

- [x] direct-inference sessions open without a working resource
- [x] one approved WebSocket endpoint and API credential remain session-bound
- [x] first and chained turns preserve exact provider correlation and order
- [x] cancellation, deadline, or disconnect cannot imply provider resume
- [x] billed cost remains exact per-turn evidence and does not become policy
- [x] deterministic default QA requires no provider credential or network

## Planning Checkpoint

Research 005 remains current for this checkpoint. Kimi Code ACP portability
ranks next because its load/resume, replay, write-callback, and delegated-login
boundaries add more information than concrete owned lifecycle for an already
proven llama.cpp facade. Roadmap 018 carries the evidence and contract gate.
Owned llama.cpp remains next after that. Do not start an SDK bridge without a
maintained Rust surface or a separately authorized language-boundary contract.

## Stop Condition

Stop if current provider evidence no longer supports the documented WebSocket
route, if the proof needs stored provider state, or if a WebSocket library
would bypass host-approved endpoint or cleanup authority.
