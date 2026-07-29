# 035 Observable Agent Activity Kernel

Status: active
Owner: Tom
Created: 2026-07-29
Depends on: g02.034
Vision tags: portable runtime, consumer usability, observability
Contract refs: 003, 009, 011-012, 029, 037, 044
Planning state: card 119 ready; cards 120-121 planned

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

- [ ] Add provider-neutral observable activity records.
- [ ] Keep the existing run and turn streams as the transport.
- [ ] Separate activity fidelity from binary streaming support.
- [ ] Expose immutable prepared route activity evidence.
- [ ] Preserve callback, direct-tool, terminal, and cleanup boundaries.
- [ ] Establish deterministic cross-shape conformance before adapter rollout.

## Non-Goals

- consumer transcript storage or database schema
- chat grouping, collapsed rows, labels, or UI components
- raw provider payload exposure
- hidden reasoning or chain-of-thought
- provider-specific event mapping
- live provider effects, package publication, or consumer edits

## Execution Plan

### Batch 35.1 — Runtime Activity Records

- [ ] Execute card 119.
- [ ] Add identity, lifecycle, kinds, content streams, status, correlation,
      disclosure, and redacted formatting.
- [ ] Integrate activity into the existing semantic event stream.

### Batch 35.2 — Capability And Prepared Evidence

- [ ] Execute card 120 after card 119 passes focused validation.
- [ ] Add exact activity capability constraints and immutable route profiles.
- [ ] Keep preparation default-light and failure-before-effects.

### Batch 35.3 — Common Conformance

- [ ] Execute card 121.
- [ ] Prove full, completion-only, unavailable, unknown, and
      unverified-newer profiles.
- [ ] Close the kernel before provider mappings begin.

## Acceptance Criteria

- [ ] portable activity cannot be confused with consumer messages
- [ ] every activity observation has exact operation ownership
- [ ] no synthetic lifecycle phase is required
- [ ] reasoning means provider-visible summary only
- [ ] raw provider payloads remain private and formatting is redacted
- [ ] route profiles are inspectable without starting an operation
- [ ] all existing adapters compile before semantic rollout
- [ ] focused core, runtime, testkit, docs, and public-API gates pass

## Decision Gates

- Stop if safe display content cannot remain distinct from diagnostics.
- Stop if the profile requires consumers to enumerate provider-native event
  names.
- Ask the operator before adding durable storage or product presentation.

## Next Planning Checkpoint

After card 121, confirm the common records are sufficient for both complete
Codex item lifecycle and completion-only structured JSONL before starting
g02.036.

