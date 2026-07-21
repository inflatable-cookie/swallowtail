# 011 Hosted Transport Foundations

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Realize the smallest shared endpoint, credential, direct-run, catalogue, and
provider-evidence foundations before any non-Codex network driver.

## Contracts

- Contract 006: Execution Layer and Access Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary

## Generation Runway Goal

Advance g01 from process-only proof drivers into network-backed harness and
direct-inference coverage without stabilizing the pre-1.0 API.

## Goals

- [x] Refresh provider evidence and promote the hosted-transport contract.
- [x] Bind endpoint and credential grants to scope and audience.
- [x] Remove the fake-workspace requirement from direct structured runs.
- [x] Add mutable catalogue limits and typed provider evidence.
- [x] Prove concrete local endpoint and credential host behavior.

## Execution Plan

- [x] Evidence and contract batch: card 035.
- [x] Runtime authority batch: cards 036-037.
- [x] Concrete host and conformance batch: card 038.

## Cards

- `batch-cards/035-provider-expansion-evidence-and-contract.md` — completed
- `batch-cards/036-scoped-endpoint-and-credential-grants.md` — completed
- `batch-cards/037-direct-run-catalogue-and-provider-evidence.md` — completed
- `batch-cards/038-local-hosted-services-and-conformance.md` — completed

## Acceptance Criteria

- [x] public records expose no raw endpoint or secret
- [x] scope, audience, execution host, access profile, instance, and route stay
      bound before network work
- [x] credential cleanup is awaited
- [x] direct inference needs no placeholder resource
- [x] usage, rate, quota, retry, and billing remain distinct
- [x] default QA uses deterministic fixtures only

## Planning Checkpoint

After card 038, begin the attached OpenCode server proof. Reassess only if the
foundation forces a generic byte transport, provider-specific core types, or a
credential-store policy.

## Stop Condition

Stop if endpoint or secret material enters public formatting, a provider retry
becomes implicit, or concrete network execution requires a global executor.
