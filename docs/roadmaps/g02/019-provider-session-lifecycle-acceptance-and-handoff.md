# 019 Provider Session Lifecycle Acceptance And Handoff

Status: completed
Owner: Tom
Created: 2026-07-26
Depends on: g02.016, g02.017, and g02.018
Vision tags: provider-wide facade, explicit unsupported routes, Nucleus handoff
Contract refs: 005-011, 017, 029, 036-038
Planning state: cards 058 and 060 completed; card 059 superseded by the
broader packaged lifecycle evidence in card 136

## Problem

Applicable implementations do not alone prove a coherent provider-wide
consumer surface. Every production route needs an explicit supported,
unsupported, or not-applicable lifecycle posture before Nucleus adoption.

## Goals

- [x] Classify all 22 production routes without fabricating lifecycle support.
- [x] Publish prepared management guidance and package-level conformance.
- [x] Preserve package and release evidence without publishing.
- [x] Produce a bounded Nucleus adoption handoff under consumer authority.

## Execution Plan

### Batch 19.1 — Route Classification

- [x] Execute card 058 after roadmaps 016-018 close.

### Batch 19.2 — Packaged Acceptance

- [x] Preserve card 059's transient lifecycle evidence and use card 136's
      later, broader extracted-package proof instead of refreshing a retained
      publication candidate.

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

## Closeout

The provider-wide lifecycle matrix, packaged behavior, and Nucleus handoff are
complete. Card 059's candidate-refresh tail is superseded. Registry
publication is outside the active roadmap until the operator explicitly
reopens it after months of consumer usage evidence.
