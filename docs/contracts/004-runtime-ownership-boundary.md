# 004 Runtime Ownership Boundary

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Fix ownership before Swallowtail defines runtime traits or host ports.

## Execution Shapes

Swallowtail must preserve two distinct operations:

1. **Interactive session** — multi-turn continuity with optional resume,
   streamed events, callback exchange, active-turn interruption, and explicit
   close or recovery.
2. **Structured run** — one bounded request with progress, deadline,
   cancellation, optional schema and attachments, and one terminal outcome.

Adapters declare support per capability. A host rejects unsupported operation
and option combinations before provider work begins.

Execution shape is independent of the harness or direct-inference layer defined
by Contract 006. A structured run is not necessarily a direct model call, and
an interactive session is not necessarily provider-owned orchestration.

## Execution Host

The authorized execution host owns access to:

- provider binaries, SDKs, servers, and network routes
- resolved credentials and secret injection policy
- working resources, temporary storage, and attachment materialization
- process or connection authority
- clock, deadline, cancellation, and cleanup facilities
- event delivery and diagnostic redaction policy

A renderer or remote client does not gain authority by supplying a filesystem
path, executable path, environment, or secret. It supplies product identifiers
and requested intent; the host resolves authorized resources.

## Swallowtail Ownership

Swallowtail runtime and adapter layers may own:

- discovery, version, auth/readiness, and model-catalog mechanisms
- provider connection or owned-process lifecycle
- translation between provider activity and ordered normalized events
- scoped cancellation, deadline observation, and cleanup mechanics
- opaque provider session, run, turn, item, and callback references
- tool or callback declarations, calls, and response transport
- structured-output descriptors, attachment transport, and terminal result
  envelopes
- safe diagnostics with internal detail behind host policy

Owned versus external runtime mode must remain explicit. Swallowtail does not
decide which host receives execution authority.

## Consumer Ownership

Consumers own:

- prompts, instructions, product tools, and tool authority
- model defaults, budgets, routing policy, and operation scheduling
- product schemas and domain validation
- repair, retry, ranking, and multi-run orchestration policy
- project, task, goal, memory, review, and conversation records
- persistence of opaque Swallowtail/provider references
- approvals, receipts, audit consequences, and UI progress wording
- interpretation and application of terminal results

Provider completion never implies consumer acceptance or product-state
mutation.

## Lifecycle Rules

- request, session, run, turn, callback, and provider ids remain distinct
- cancellation scope is explicit: run, active turn, or whole session
- completion, cancellation, timeout, provider failure, and host failure are
  distinct terminal outcomes
- owned processes or connections have observable cleanup outcomes
- resume never silently substitutes a different provider reference
- progress/events are distinct from the terminal result
- raw provider payloads, stderr, prompts, outputs, and secrets are not safe
  diagnostics by default
- schema transport does not transfer schema meaning or domain validation to
  Swallowtail

## Deferred Decisions

This contract does not choose:

- async traits, pollable handles, or sync worker boundaries
- exact host-port traits
- event buffering, replay, or backpressure strategy
- portable attachment representation
- generic JSON Schema validation ownership
- registry configuration and runtime-instance storage

Spec 002 settles their cross-adapter semantics. Card 009 must promote concrete
record and trait contracts before any `swallowtail-runtime` code is authorized.
