# 015 llama.cpp Attached Runtime Proof

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Prove an attached self-hosted serving runtime while keeping model artifact,
server process, protocol facade, deployment, and route identities separate.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 007: Model Artifact and Serving Boundary
- Contract 009: Async Operation Lifecycle
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary

## Goals

- [x] Inventory one exact llama.cpp server facade and model fixture.
- [x] Implement attach, readiness, catalogue, and bounded direct inference.
- [x] Prove observed parser/template capabilities and external ownership.

## Execution Plan

- [x] Deployment and fixture batch: card 048.
- [x] Attached runtime driver batch: card 049.
- [x] Self-hosted conformance batch: card 050.

## Cards

- `batch-cards/048-llama-cpp-deployment-fixtures.md` — completed
- `batch-cards/049-llama-cpp-attached-driver.md` — completed
- `batch-cards/050-llama-cpp-attached-conformance.md` — completed

## Acceptance Criteria

- [x] attached cleanup never stops the external server
- [x] model artifact and serving runtime remain distinct
- [x] effective tools, reasoning, and schema support are observed deployment
      capabilities, not family claims
- [x] Monkey responsibilities remain outside Swallowtail

## Planning Checkpoint

After card 050, re-rank owned serving, SDK, WebSocket, and additional provider
coverage from accumulated conformance evidence.

## Stop Condition

Stop if the proof requires Swallowtail to download models, manage artifacts, or
own an externally attached server.
