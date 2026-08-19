# 004 Readiness And Admission Contract Promotion

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: g04.003
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 006, 008, 010, 014, 015, 017, 020, 029, 032, 036-037, 047, 056-057
Spec: archived 011
Planning state: cards 010-012 completed

## Problem

Spec 011 is ready to promote. Annotated `v0.3.3` exists at `51d18620`. The
lifecycle in front of Contract 047 still has no contract of its own, so
implementation cannot start.

The new contract owns addable-route catalog, credential-field descriptors,
library-max sign-in through host ports, the persistence port, readiness
refresh, authenticated-subject observation, config-field descriptors, and the
bound model-presentation overlay. Existing contracts change only at the named
seams.

## Generation Runway Goal

Promote the readiness/admission contract after the g04.003 tag. Do not write
facade code.

## Goals

- [x] write Contract 057 from Spec 011
- [x] amend 006, 008, 010, 014, 015, 017, 029, 032, 037, and 047 only at the
      named seams
- [x] archive Spec 011 and leave implementation planned until this contract
      is active

## Non-Goals

- production records, adapters, store implementations, or host-port code
- GitHub Release, registry publication, or tag mutation
- OpenHands production wiring
- live provider, install, or login work
- marking implementation cards ready
- putting emails, tokens, or targets into 047

## Execution Plan

### Batch 4.1 — New Contract

- [x] Execute card 010.
- [x] write Contract 057 as the owner of the lifecycle in front of 047
- [x] keep crate placement and first-proof routes as implementation notes,
      not as realized architecture

### Batch 4.2 — Seam Amendments

- [x] Execute card 011 after card 010.
- [x] amend only the named seams
- [x] keep 047 a selection snapshot and keep ACP authenticate and delegated
      harness activation distinct from login

### Batch 4.3 — Spec Promotion

- [x] Execute card 012 after card 011.
- [x] archive Spec 011, update indexes, and point architecture at 057
- [x] leave implementation roadmaps uncompiled until this milestone closes

## Acceptance Criteria

- [x] Contract 057 is active and owns the named lifecycle surfaces
- [x] 047 remains a selection snapshot without emails, tokens, or targets
- [x] enablement and readiness remain independent
- [x] no facade implementation ships from this roadmap
- [x] Spec 011 is archived after promotion

## Decision Gates

- Stop if promotion would store raw secrets or create a Swallowtail server.
- Stop if 047 would gain emails, tokens, targets, or overlay-changed
  readiness.
- Stop if an amendment widens beyond the named seam.
- Stop if implementation starts before 057 is active.
