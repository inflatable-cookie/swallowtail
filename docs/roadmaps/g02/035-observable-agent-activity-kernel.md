# 035 Observable Agent Activity Kernel

Status: completed
Owner: Tom
Created: 2026-07-29
Depends on: g02.034
Vision tags: portable runtime, consumer usability, observability
Contract refs: 003, 009, 011-012, 029, 037, 044
Planning state: cards 119-121 completed

## Problem

Swallowtail delivers ordered bounded runtime events, but its portable event
vocabulary cannot represent stable activity identity, item lifecycle, content
ownership, disclosure strength, or exact route fidelity. Rich provider events
are therefore flattened into generic progress before consumers can build a
work log.

## Generation Runway Goal

Make the prepared provider-wide surface usable by long-lived applications
without requiring provider-native event parsing.

## Goals

- [x] Add provider-neutral observable activity records.
- [x] Keep the existing run and turn streams as the transport.
- [x] Separate activity fidelity from binary streaming support.
- [x] Expose immutable prepared route activity evidence.
- [x] Preserve callback, direct-tool, terminal, and cleanup boundaries.
- [x] Establish deterministic cross-shape conformance before adapter rollout.

## Non-Goals

- consumer transcript storage or database schema
- chat grouping, collapsed rows, labels, or UI components
- raw provider payload exposure
- hidden reasoning or chain-of-thought
- provider-specific event mapping
- live provider effects, package publication, or consumer edits

## Execution Plan

### Batch 35.1 — Runtime Activity Records

- [x] Execute card 119.
- [x] Add identity, lifecycle, kinds, content streams, status, correlation,
      disclosure, and redacted formatting.
- [x] Integrate activity into the existing semantic event stream.

### Batch 35.2 — Capability And Prepared Evidence

- [x] Execute card 120 after card 119 passes focused validation.
- [x] Add exact activity capability constraints and immutable route profiles.
- [x] Keep preparation default-light and failure-before-effects.

### Batch 35.3 — Common Conformance

- [x] Execute card 121.
- [x] Prove full, completion-only, unavailable, unknown, and
      unverified-newer profiles.
- [x] Close the kernel before provider mappings begin.

## Acceptance Criteria

- [x] portable activity cannot be confused with consumer messages
- [x] every activity observation has exact operation ownership
- [x] no synthetic lifecycle phase is required
- [x] reasoning means provider-visible summary only
- [x] raw provider payloads remain private and formatting is redacted
- [x] route profiles are inspectable without starting an operation
- [x] all existing adapters compile before semantic rollout
- [x] focused core, runtime, testkit, docs, and public-API gates pass

## Decision Gates

- Stop if safe display content cannot remain distinct from diagnostics.
- Stop if the profile requires consumers to enumerate provider-native event
  names.
- Ask the operator before adding durable storage or product presentation.

## Next Planning Checkpoint

The common records and reusable assertion pack cover complete item lifecycle
and completion-oriented structured traces. Start g02.036 with exact Codex
range and corpus evidence; do not promote production activity before it.
