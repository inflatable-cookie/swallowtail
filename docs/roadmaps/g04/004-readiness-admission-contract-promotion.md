# 004 Readiness And Admission Contract Promotion

Status: planned
Owner: Tom
Created: 2026-08-19
Depends on: g04.003
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 006, 008, 010, 014, 015, 017, 020, 029, 032, 036-037, 047, 056
Spec: 011
Planning state: cards 010-012 ready

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

- [ ] write Contract 057 from Spec 011
- [ ] amend 006, 008, 010, 014, 015, 017, 029, 032, 037, and 047 only at the
      named seams
- [ ] archive Spec 011 and leave implementation planned until this contract
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

- [ ] Execute card 010.
- [ ] write Contract 057 as the owner of the lifecycle in front of 047
- [ ] keep crate placement and first-proof routes as implementation notes,
      not as realized architecture

### Batch 4.2 — Seam Amendments

- [ ] Execute card 011 after card 010.
- [ ] amend only the named seams
- [ ] keep 047 a selection snapshot and keep ACP authenticate and delegated
      harness activation distinct from login

### Batch 4.3 — Spec Promotion

- [ ] Execute card 012 after card 011.
- [ ] archive Spec 011, update indexes, and point architecture at 057
- [ ] leave implementation roadmaps uncompiled until this milestone closes

## Acceptance Criteria

- [ ] Contract 057 is active and owns the named lifecycle surfaces
- [ ] 047 remains a selection snapshot without emails, tokens, or targets
- [ ] enablement and readiness remain independent
- [ ] no facade implementation ships from this roadmap
- [ ] Spec 011 is archived after promotion

## Decision Gates

- Stop if promotion would store raw secrets or create a Swallowtail server.
- Stop if 047 would gain emails, tokens, targets, or overlay-changed
  readiness.
- Stop if an amendment widens beyond the named seam.
- Stop if implementation starts before 057 is active.
