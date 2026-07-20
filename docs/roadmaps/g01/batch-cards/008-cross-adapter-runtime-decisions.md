# Cross-Adapter Runtime Decisions

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Settle host ports, async posture, cancellation, event delivery, runtime-instance
ownership, credential references, access status, attachments, schemas, and
diagnostics against the completed multi-integration inventory.

## Inputs

- completed cards 006 and 007
- Contracts 004-007
- `docs/research/003-integration-surface-and-open-weight-route-inventory.md`
- at least three materially different adapter surface shapes

## Decision Set

- shared driver discovery and configured-instance boundary
- host ports for process, network, clock, credential reference, and event sinks
- async and task ownership without forcing process supervision
- one-shot process, long-lived RPC, ACP, HTTP service, SDK, and local-serving
  lifecycle differences
- cancellation, steering, permission callbacks, tool exchange, attachments,
  schemas, and resume as negotiated capabilities
- model-artifact and serving deployment discovery without download or fleet
  policy
- safe diagnostics and no cross-layer, access, billing, support, or topology
  fallback

## Acceptance Criteria

- decisions work against at least one one-shot CLI, one long-lived RPC or ACP
  harness, one hosted direct API, and one self-hosted serving runtime
- host and consumer ownership remains consistent with Contract 004
- no public trait assumes every driver owns a child process or session
- capability absence is representable before execution
- remaining provider-specific behavior has a namespaced extension path

## Stop Condition

Do not draft public traits while ownership or lifecycle semantics remain
ambiguous or while the evidence is only Codex-shaped.

## Closeout

- Spec 002 settles async posture, scoped task and resource ownership, driver
  discovery, host services, operations, events, cancellation, callbacks,
  attachments, schemas, credentials, diagnostics, and extensions.
- The decisions fit one-shot CLI, long-lived RPC or ACP, hosted direct API,
  attached self-hosted, and owned self-hosted shapes.
- A cross-shape conformance matrix now supplies card 009 acceptance cases.
- Concrete Rust traits, dependencies, and runtime code remain excluded.
