# 008 Nucleus Interactive-Session Readiness

Status: completed
Owner: Tom
Updated: 2026-07-19

## Purpose

Close the shared interactive-session gaps exposed by Nucleus and prepare a
bounded replacement for its Codex transport behind the existing consumer
facade. Keep Nucleus control, persistence, tools, and product state downstream.

## Generation Runway

This milestone follows the Soundcheck proof in g01. It validates long-lived
sessions, callbacks, and host placement before later non-Codex adapters test the
same runtime vocabulary.

## Contracts

- Contract 002: Repository Authority
- Contract 004: Runtime Ownership Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles
- Contract 012: Interactive Session Options And Callback Exchange

## Goals

- [x] Realize provider-neutral session instructions, model options, tool
      declarations, and correlated callback exchange.
- [x] Expand Codex app-server only for declared dynamic-tool behavior.
- [x] Prove local and remote execution-host placement preserves authority.
- [x] Produce a reversible Nucleus-owned facade replacement plan.

## Execution Plan

- [x] Contract batch: card 026 defines session options, model reasoning
      metadata, tool declarations, callback ids, responses, and wait states.
- [x] Driver batch: card 027 adds Codex dynamic-tool callback transport without
      adding a generic tool executor.
- [x] Topology batch: card 028 proves resource mapping, resume, cancellation,
      callback waits, and cleanup through an authorized execution host.
- [x] Handoff batch: card 029 maps the existing `AgentSessionRuntime` facade to
      Swallowtail and records rollback and consumer validation.

## Cards

- `batch-cards/026-interactive-session-callback-contract.md` — completed
- `batch-cards/027-codex-app-server-tool-callbacks.md` — completed
- `batch-cards/028-nucleus-topology-and-lifecycle-proof.md` — completed
- `batch-cards/029-nucleus-adoption-handoff.md` — completed

## Acceptance Criteria

- [x] Swallowtail transports tool declarations and correlated calls/responses
      but never executes a Nucleus tool or grants product authority.
- [x] Nucleus project/resource ids map through host-owned references; raw
      client paths do not become authority.
- [x] Provider, runtime, callback, session, turn, task, and receipt ids remain
      distinct.
- [x] Session interruption, callback wait, resume, close, and cleanup pass the
      long-lived RPC profile.
- [x] The handoff targets the existing live adapter facade and does not require
      wholesale replacement of Nucleus supervision or persistence.

## Planning Checkpoint

After card 029, choose the first non-Codex harness proof from promoted
integration evidence. Do not stabilize the runtime from two Codex transports.

The handoff is recorded in `nucleus-adoption-handoff.md`. No non-Codex target
is selected yet, so no later implementation card is ready.

## Stop Condition

Stop before implementation if callback authority, remote host placement, or
session-option semantics remain ambiguous in Contracts 008-010. Do not infer
them from Nucleus product code.
