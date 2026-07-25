# 003 Codex Prepared Integration Facade

Status: completed
Owner: Tom
Created: 2026-07-24
Depends on: g02.002
Vision tags: provider-specific behavior, consumer usability, exact compatibility
Contract refs: 008-013, 023, 029, 032-034, 037
Planning state: completed

## Problem

Codex consumers currently assemble version discovery, configured instances,
capabilities, access, model routes, configuration posture, preflight plans, and
matching runtime requests themselves.

The low-level driver remains correct, but the normal path is too easy to
misconfigure.

## Goals

- [x] Add prepared Codex exec and app-server surfaces inside the existing
      adapter crate.
- [x] Preserve separate catalogue, structured-run, and interactive-session
      lifecycles.
- [x] Bind exact installed version and compatibility assessment once.
- [x] Provide named inspectable catalogue, read-only, bounded-workspace, and
      structured-run profiles.
- [x] Expose one concise compile-tested integration path.
- [x] Preserve every existing low-level capability and conformance profile.

## Non-Goals

- [ ] Do not merge Codex exec and app-server into one operation.
- [ ] Do not choose a model, credential, workspace, network, search, or tools.
- [ ] Do not make writable or ambient-host authority implicit.
- [ ] Do not add provider fallback or hide unverified-newer status.
- [ ] Do not edit consumer repositories.
- [ ] Do not add a new crate unless roadmap g02.002 disproves the selected
      package shape.

## Execution Plan

### Batch 3.1 — Prepared Discovery And Plan Factory

- [x] Execute card 008.
- [x] Prepare one exact Codex target and compatibility assessment.
- [x] Construct consistent operation-independent adapter, instance, access,
      route-factory, and plan-factory inputs.

### Batch 3.2 — Prepared Operation Profiles

- [x] Execute card 009 after the factory passes.
- [x] Add catalogue, read-only session, bounded-workspace session, and
      structured-exec preparation.
- [x] Keep consumer choices explicit and plan-derived echoes internal.

### Batch 3.3 — Conformance And Public Guidance

- [x] Execute card 010.
- [x] Run the prepared paths across qualified, deprecated, unverified-newer,
      failure, cleanup, local, and remote-authoritative fixtures.
- [x] Add concise public examples and migration guidance.

## Acceptance Criteria

- [x] ordinary consumers do not construct Codex adapter-fixed preflight fields
- [x] one exact target and version reach discovery and execution
- [x] configuration, isolation, access, retention, network, and tools remain
      independently visible
- [x] read-only and bounded-workspace requests cannot drift from their plans
- [x] exec and app-server behavior revisions remain separate
- [x] low-level driver entry points remain usable
- [x] deterministic conformance passes without installed Codex or live auth
- [x] card 011 can migrate Nucleus without facade design work

## Risks And Mitigations

- Risk: the facade becomes a Codex-only replacement for core. Mitigation: keep
  it adapter-local and built from public shared preparation primitives.
- Risk: named profiles hide policy. Mitigation: expose their expanded plan and
  assessment.
- Risk: target changes between probe and execution. Mitigation: bind one
  approved canonical target reference for the prepared object's lifetime.
- Risk: preparation repeats probes excessively. Mitigation: reuse an exact
  observation only while its target and instance revision remain unchanged.

## Evidence Requirements

- public API examples
- deterministic exact-version promotion and runtime-drift rejection
- prepared catalogue, exec, read-only, bounded-workspace, tool, cancellation,
  deadline, and cleanup fixtures
- local and remote-authoritative topology matrix
- redaction and staged-diagnostic audit
- focused and full repository QA

## Decision Gate

Contract 037 is active. Cards 008-010 realize the exact-target factory,
separate prepared profiles, deterministic conformance, public guidance, and
exact consumer migration inputs. Roadmap g02.003 is complete.
