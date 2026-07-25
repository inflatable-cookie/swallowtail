# 002 Prepared Consumer Integration Boundary

Status: completed
Owner: Tom
Created: 2026-07-24
Depends on: completed g02.001
Vision tags: reusable substrate, consumer usability, release discipline
Contract refs: 001, 002, 004, 008, 009, 010, 013, 029, 032, 033, 036, 037
Planning state: completed; card 008 ready under g02.003

## Problem

Swallowtail's low-level contracts preserve exact authority and lifecycle truth,
but normal consumers must reproduce adapter-owned setup. Nucleus needed a
multi-file repair for exact Codex version, configuration posture, and request
policy. Soundcheck still compiles with plans that fail those runtime checks.

The unpublished release candidate proves consumer compilation, not usable
runtime preparation.

## Goals

- [x] Promote one durable prepared-integration boundary.
- [x] Remove unrelated implicit defaults from plan-echoed request state.
- [x] Add safe staged preparation and discovery diagnostics.
- [x] Provide reusable joined local task and host-service composition.
- [x] Keep low-level records and role traits available.
- [ ] Hold publication until the full consumer-usability runway closes.

## Non-Goals

- [ ] Do not implement a provider facade before Contract 037 is active.
- [ ] Do not add an umbrella crate or generic `send_prompt`.
- [ ] Do not select credentials, model, endpoint, billing, or topology.
- [ ] Do not add a global executor or detached task.
- [ ] Do not edit Nucleus or Soundcheck in this milestone.
- [ ] Do not publish, tag, push, or mutate a registry.

## Contract Coverage

- [x] Contracts 004 and 008-010 preserve ownership, preflight, lifecycle, and
      host authority.
- [x] Contracts 013, 032, and 033 preserve explicit access, exact executable
      observation, and harness configuration.
- [x] Research 034 identifies consumer duplication and compile-only evidence
      failure.
- [x] Spec 005 selects a layered prepared-integration shape.
- [x] Contract 037 defines preparation, plan derivation, access provenance,
      diagnostics, and the low-level escape hatch.

## Execution Plan

### Batch 2.1 — Contract And Architecture Promotion

- [x] Execute card 005.
- [x] Promote the prepared-integration layer without changing existing
      operation shapes or provider authority.
- [x] Archive Spec 005 only after all durable decisions move to canonical
      surfaces.

### Batch 2.2 — Plan-Derived Runtime Requests And Diagnostics

- [x] Execute card 006 after Contract 037 is active.
- [x] Make request construction derive immutable plan echoes or require them
      explicitly.
- [x] Add safe typed preparation stages without exposing raw process data.

### Batch 2.3 — Joined Local Host Composition

- [x] Execute card 007 after shared runtime records settle.
- [x] Add a reusable per-host scoped task service and `HostServices`
      composition.
- [x] Prove explicit target selection remains separate from installed
      discovery.

## Acceptance Criteria

- [x] prepared integration has a durable owner and dependency direction
- [x] implicit request defaults cannot contradict an immutable plan
- [x] diagnostics identify preparation stage safely
- [x] access provenance cannot be silently fabricated
- [x] local convenience preserves remote-authoritative topology
- [x] no provider-specific type enters core or runtime
- [x] card 008 can implement Codex without a new planning decision

## Risks And Mitigations

- Risk: convenience becomes hidden routing. Mitigation: prepared setup accepts
  one explicit driver, target, host, access profile, and operation profile.
- Risk: a local bundle becomes a global executor. Mitigation: keep ownership
  per host and require joined task handles.
- Risk: access simplification manufactures readiness. Mitigation: require
  observed status or visible caller assertion.
- Risk: preserving the old constructor retains the footgun. Mitigation: make a
  clean pre-release API change; add no compatibility shim.

## Evidence Requirements

- Research 034 and Spec 005 promotion map
- public API diff and dependency-direction review
- provider-neutral request-plan and diagnostic fixtures
- local and remote-authoritative host conformance
- compile-tested low-level and prepared examples
- doctor delta and full focused validation

## Decision Gate

Cards 005-007 are complete. Contract 037, plan-derived request agreement, safe
preparation diagnostics, honest access provenance, joined local tasks, exact
host-service composition, and opaque target approval are realized. Card 008 is
ready under g02.003. Publication remains held.
