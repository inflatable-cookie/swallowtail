# Runtime Contract and Implementation Runway

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Promote the cross-adapter runtime decisions into durable contracts and compile
a bounded implementation runway.

## Inputs

- completed cards 006-008
- `docs/specs/002-cross-adapter-runtime-decisions.md`
- the explicit cross-shape conformance matrix in Spec 002

## Scope

- divide the accepted runtime decisions into the smallest durable contracts
- settle concrete record boundaries for configured instances, access status,
  requirements, parameterized capabilities, host services, operation handles,
  terminal outcomes, cleanup, attachments, and schemas
- choose async trait syntax, object-safety posture, executor boundary, and
  dependency policy through minimal compile probes
- define conformance fixture APIs before runtime implementation
- compile bounded implementation cards for core record additions, runtime
  skeleton, testkit fixtures, and the first proof drivers
- keep provider and consumer behavior out of the initial runtime skeleton

## Acceptance Criteria

- contracts reject unsupported capability, host-service, instance, route,
  access, ownership, and topology combinations before provider side effects
- lifecycle rules cover one-shot CLI, long-lived RPC or ACP, hosted API,
  attached self-hosted, and owned self-hosted fixtures
- event ordering, backpressure, terminal outcomes, cancellation, deadlines,
  cleanup, credentials, attachments, schemas, diagnostics, and extensions have
  explicit acceptance cases
- public trait choices are backed by compile evidence rather than preference
- implementation cards are small enough to validate independently but grouped
  into meaningful batches

## Stop Condition

Do not mark runtime implementation ready until the contract can reject
unsupported capability, transport, instance, and topology combinations before
provider work.

## Closeout

- Contracts 008-011 govern registration/preflight, async lifecycle, host
  services/inputs, and cross-shape conformance.
- Rust 1.96.0 compile probes select explicit boxed `Send` futures and dynamic
  role traits; native async trait methods are not dyn compatible.
- `swallowtail-runtime` may depend publicly on core and `futures-core` only in
  the first implementation boundary.
- g01 roadmaps 004-005 sequence records, preflight, runtime roles, scoped
  lifecycle, host services, synthetic profiles, and validation.
- g01 roadmap 006 sequences the first real process host and separate Codex
  exec/app-server proof drivers.
