# 019 Provider Session Lifecycle Acceptance And Handoff

Status: paused
Owner: Tom
Created: 2026-07-26
Depends on: g02.016, g02.017, and g02.018
Vision tags: provider-wide facade, explicit unsupported routes, Nucleus handoff
Contract refs: 005-011, 017, 029, 036-038
Planning state: cards 058 and 060 completed; card 059 paused at
canonical-source gate

## Problem

Applicable implementations do not alone prove a coherent provider-wide
consumer surface. Every production route needs an explicit supported,
unsupported, or not-applicable lifecycle posture before Nucleus adoption.

## Goals

- [x] Classify all 22 production routes without fabricating lifecycle support.
- [x] Publish prepared management guidance and package-level conformance.
- [ ] Preserve candidate and release evidence without publishing.
- [x] Produce a bounded Nucleus adoption handoff under consumer authority.

## Execution Plan

### Batch 19.1 — Route Classification

- [x] Execute card 058 after roadmaps 016-018 close.

### Batch 19.2 — Packaged Acceptance

- [ ] Execute card 059 after the matrix and facade surface are exact.

### Batch 19.3 — Consumer Handoff

- [x] Execute card 060 after transient packaged acceptance passed and the
      operator authorized the handoff before retained-candidate replacement.

## Acceptance Criteria

- [x] every production route appears once with supported, unsupported, or
      not-applicable management posture
- [x] Kimi and Gemini ACP remain explicit unsupported without private-route
      substitution
- [x] existing driver-owned cleanup remains distinct
- [x] package artifacts prove every supported operation and unsupported stop
- [x] Nucleus receives separate local and provider action/outcome guidance
- [x] no Nucleus edit, provider call, publication, push, tag, or release occurs

## Decision Gate

Card 060 stops with a consumer handoff. Editing Nucleus or choosing its exact
archive/delete UX requires separate consumer-repository authority.

## Next Planning Checkpoint

After Nucleus adoption evidence, reassess whether provider history browsing,
binding import, export, or active-session management has enough product value
for another contract. None is implied by this lane.
