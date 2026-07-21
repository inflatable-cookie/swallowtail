# 013 Anthropic Direct Inference Proof

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Prove provider-supported direct inference through Anthropic Models and Messages
HTTP/SSE without importing Claude Code subscription authority.

## Contracts

- Contract 006: Execution Layer and Access Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary

## Goals

- [x] Freeze versioned Models and Messages fixtures, including stream errors.
- [x] Implement paginated catalogue and one-attempt structured inference.
- [x] Normalize output, usage, rate, request-correlation, and failures.
- [x] Prove credential scope, release, cancellation, and no-process behavior.

## Execution Plan

- [x] API fixture and exact subset batch: card 042.
- [x] Direct driver implementation batch: card 043.
- [x] Hosted profile and separately gated live check: card 044.

## Cards

- `batch-cards/042-anthropic-api-fixtures.md` — completed
- `batch-cards/043-anthropic-direct-driver.md` — completed
- `batch-cards/044-anthropic-direct-conformance.md` — completed

## Acceptance Criteria

- [x] public API credential and Claude subscription OAuth remain distinct
- [x] every request uses the exact bound endpoint, audience, model, and version
- [x] mid-stream errors fail the run
- [x] no implicit inference retry or provider fallback occurs
- [x] default QA requires no live credential

## Planning Checkpoint

After card 044, audit whether a second direct provider adds a new transport or
only provider mapping. Prefer ACP next unless evidence changes.

## Stop Condition

Stop if public API use depends on Claude Code credential extraction, provider
messages become stable error codes, or usage is treated as cost.
