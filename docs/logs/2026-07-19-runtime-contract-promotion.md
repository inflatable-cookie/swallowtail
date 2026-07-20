# 2026-07-19 Runtime Contract Promotion

Status: complete
Owner: Tom

## Change

Promoted Spec 002 into four durable contracts:

- Contract 008: registration, configured instances, access, requirements, and
  side-effect-free preflight
- Contract 009: object-safe async roles, scoped handles, events, cancellation,
  terminal outcomes, and cleanup
- Contract 010: capability-scoped host services, credentials, resources,
  attachments, schemas, events, and diagnostics
- Contract 011: deterministic conformance profiles

## Compile Evidence

Rust 1.96.0 edition-2024 probes established:

- explicit boxed `Send` futures, boxed dynamic event streams, dynamic role and
  handle traits, and optional service trait objects compile
- native `async fn` on the equivalent driver trait fails dynamic dispatch with
  `E0038`

The runtime public boundary therefore uses explicit boxed futures and
`futures-core` streams without exposing a concrete executor or `async-trait`.

## Roadmap

g01 remains active. Roadmaps 004-005 implement the runtime kernel against
synthetic profiles. Roadmap 006 then implements a local process host and
separate Codex exec and app-server proof drivers. Runtime stability remains
gated on later non-Codex proofs.
