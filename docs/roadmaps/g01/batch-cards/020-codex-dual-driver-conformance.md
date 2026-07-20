# Codex Dual-Driver Conformance

Status: completed
Owner: Tom
Roadmap: 006 Codex Proof Drivers
Updated: 2026-07-19

## Goal

Prove the two Codex surfaces remain separate drivers behind one small runtime
vocabulary.

## Scope

- shared identity, access, model-route, host-service, diagnostic, and fixture
  reuse audit
- capability and lifecycle difference tests
- failure, cancellation, deadline, event, cleanup, and redaction review
- package-boundary review

## Acceptance Criteria

- no runtime branch switches on Codex identity
- exec does not claim interactive-session behavior
- app-server does not inherit structured-output support without evidence
- both drivers pass their selected profiles with no lowered common assertion

## Validation

- focused dual-driver registration, preflight, capability, and profile tests
- crate dependency and provider-identity scans
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Closeout

- both drivers share the `codex` integration family while retaining distinct
  adapter identities, transports, runtime roles, and operation shapes.
- dynamic registration exposes only structured run for exec and only model
  catalog plus interactive session for app-server.
- each driver rejects a plan bound to the other before process work starts.
- their selected Contract 011 profiles retain the same full common assertion
  set while preserving separate process and session lifecycle assertions.
- app-server does not claim or accept structured output. Unsupported turn input
  rejects before `turn/start` provider work.
- core, runtime, testkit, and local-host crates contain no Codex identity branch.
  The adapter depends only on core/runtime plus its private JSON dependency.
