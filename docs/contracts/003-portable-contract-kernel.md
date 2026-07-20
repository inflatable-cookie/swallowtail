# 003 Portable Contract Kernel

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Define the smallest stable vocabulary that provider adapters and consumers can
share before runtime and transport choices are made.

## First Boundary

The first implementation may contain pure records and validation for:

- adapter identity and version
- explicit capability manifests
- model catalog entries and stable model identity
- opaque provider-owned session and run references
- normalized event envelopes with provider extension isolation
- structured errors with safe operator-facing diagnostics

It may include deterministic fixtures and conformance helpers for those
records.

## Required Rules

- provider references are opaque values; consumers cannot parse hidden meaning
- unsupported capabilities fail explicitly before provider work starts
- a model catalog distinguishes stable identity from mutable display metadata
- common event metadata does not require exposing raw provider payloads
- provider extensions are namespaced and optional
- diagnostics separate a safe public message from internal source details
- secrets, tokens, prompt bodies, and raw tool payloads are not included in
  default display or serialization paths
- unknown provider additions can be preserved or rejected deliberately, never
  silently reinterpreted

## Out Of Scope

- async runtime selection
- execution traits and host ports
- process spawning or supervision
- Codex protocol and wire translation
- credential loading
- tool execution policy
- session persistence
- product prompts, task models, schemas, or storage
- consumer dependency changes

## Acceptance

Fixture tests prove:

- capability rejection is explicit and structured
- provider references remain opaque
- safe diagnostics do not expose internal details
- provider extensions do not contaminate the common vocabulary
- records contain no consumer-specific types or concepts

Contracts 008-011 now govern runtime records, object-safe async roles, host
services, and conformance. This contract remains the dependency-light core
record foundation and stays silent on provider transport.
