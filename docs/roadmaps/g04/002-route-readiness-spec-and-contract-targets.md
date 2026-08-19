# 002 Route Readiness Spec And Contract Targets

Status: planned
Owner: Tom
Created: 2026-08-19
Depends on: g04.001
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 005-006, 008, 010, 014, 017, 020, 029, 032, 037, 047
Planning state: cards 004-005 planned

## Problem

Operator decisions for authenticated subject, library-max sign-in, persistence
port, and model-presentation overlay are settled in Spec 011. They are not yet
promoted into architecture or named contract targets, and g04.001 still has to
prove they fit existing records.

## Generation Runway Goal

Close Spec 011 far enough to name the later contract without writing facade
code or selecting a source tag.

## Goals

- [ ] fold g04.001 inventory into Spec 011
- [ ] name contract targets and 047/006/008 amendment bounds
- [ ] keep implementation and the source tag on later roadmaps

## Non-Goals

- production records, adapters, or store implementations
- tag, version, or changelog mutation
- live provider, install, or login work
- marking implementation cards ready

## Execution Plan

### Batch 2.1 — Inventory Into Spec

- [ ] Execute card 004 after g04.001 card 003.
- [ ] replace remaining spec unknowns with inventory facts
- [ ] keep the four settled operator decisions intact

### Batch 2.2 — Contract Targets

- [ ] Execute card 005 after card 004.
- [ ] name the new contract versus amendments
- [ ] record crate-placement and first-proof-route questions that still
      belong to implementation roadmaps

## Acceptance Criteria

- [ ] Spec 011 is ready to promote once the source tag exists
- [ ] 047 remains a selection snapshot
- [ ] no facade implementation card is ready
- [ ] g04.003 remains the source-tag gate before building

## Decision Gates

- Stop if inventory contradicts a settled decision and the operator has not
  re-answered it.
- Stop if promotion would require storing raw secrets or a Swallowtail server.
