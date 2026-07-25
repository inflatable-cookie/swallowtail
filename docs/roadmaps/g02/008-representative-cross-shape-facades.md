# 008 Representative Cross-Shape Facades

Status: completed
Owner: Tom
Created: 2026-07-25
Depends on: g02.007
Vision tags: harness breadth, direct inference, local runtime
Contract refs: 014-017, 020, 023-024, 029, 031-037
Planning state: cards 020-023 completed

## Problem

A Codex-only facade can accidentally encode installed-process assumptions.
The shared shape needs pressure from different access, topology, version, and
lifecycle boundaries before broad rollout.

## Goals

- [x] Prove Kimi Code ACP as the installed persistent-harness representative.
- [x] Prove Anthropic Messages as the hosted direct representative.
- [x] Prove Ollama as the attached local-runtime representative.
- [x] Review the shared surface before copying it across adapters.

## Execution Plan

### Batch 8.1 — Installed Harness

- [x] Execute card 020 for Kimi Code ACP.

### Batch 8.2 — Hosted Direct

- [x] Execute card 021 for Anthropic Messages.

### Batch 8.3 — Attached Runtime

- [x] Execute card 022 for Ollama native.

### Batch 8.4 — Cross-Shape Review

- [x] Execute card 023 after all three proofs pass.

## Acceptance Criteria

- [x] persistent ACP load, replay, resume, callbacks, and optional isolation
      stay explicit
- [x] endpoint, credential, catalogue, and streaming direct inference stay
      explicit
- [x] attached runtime inventory and residency stay separate from ownership
- [x] shared facade records need no provider-specific variants
- [x] no route selection or fallback appears

## Evidence Requirements

- deterministic adapter conformance under supported host topologies
- exact compatibility and drift failures
- credential-free and model-free fixtures
- shared API review and change classification

## Decision Gate

Card 023 either accepts the shared surface for breadth rollout or returns a
specific missing durable rule to contracts.

Card 023 accepted the shared surface. No missing durable rule blocks breadth
rollout.
