# 023 Provider Session Import Acceptance And Handoff

Status: planned
Owner: Tom
Created: 2026-08-01
Depends on: g03.022
Vision tags: provider-wide truth, package acceptance, consumer adoption
Contract refs: 002, 005-006, 011, 017, 029, 036-038, 046
Planning state: cards 061-063 planned

## Problem

The first three implementations need provider-wide classification, consistent
prepared guidance, extracted-package proof, and a consumer handoff that does
not make Swallowtail own Nucleus persistence or UI.

## Goals

- [ ] classify every harness route against the complete import profile
- [ ] distinguish supported, discovery-only, attachment-only, blocked, and not
  applicable routes
- [ ] assess Kimi local server, Claude, Cursor, and Pi without silent widening
- [ ] reconcile route and feature matrices plus integration guidance
- [ ] prove the selected packages assemble together
- [ ] hand Nucleus a bounded browse-select-import-load adoption contract

## Execution Plan

### Batch 23.1 — Remaining Route Classification

- [ ] Execute card 061.
- [ ] audit each harness route for catalogue, lookup, history, load, resume,
  resource binding, activity truth, and exact version support
- [ ] implement no additional route unless all evidence is already complete
- [ ] preserve explicit promotion gates for partial routes

### Batch 23.2 — Public Truth And Package Acceptance

- [ ] Execute card 062 after classification settles.
- [ ] update provider route and feature matrices
- [ ] document the separate catalogue, import, load, resume, and management
  operations
- [ ] assemble and compile the affected common, Codex, Kimi, and OpenCode
  packages through one extracted target

### Batch 23.3 — Nucleus Adoption Handoff

- [ ] Execute card 063 after package acceptance.
- [ ] define the consumer-owned thread mapping and import workflow
- [ ] specify replay persistence, duplicate detection, stale-candidate UX, and
  unsupported-route presentation as Nucleus responsibilities
- [ ] leave Nucleus implementation to its repository

## Boundaries

- no Nucleus, Soundcheck, or other consumer edit
- no Swallowtail thread database, repository scan, UI, routing, or sync daemon
- no background polling or bidirectional merge
- no management-binding persistence promotion
- no capability promotion from provider family or alternate transport
- no live provider effect, publication, or broad workspace suite unless a card
  is amended with explicit operator authority

## Acceptance Criteria

- [ ] every harness route has an evidence-backed import classification
- [ ] only complete list, revalidation, replay, and continuation routes report
  support
- [ ] public guidance prevents raw-id attachment and automatic synchronization
- [ ] selected packages compile independently with common conformance
- [ ] one Nucleus handoff preserves consumer database and UI ownership
- [ ] deferred routes retain explicit unblock evidence
- [ ] the sole Next Task pointer returns to the g03 evidence gate

## Next Planning Checkpoint

After card 063, Nucleus may adopt independently. Swallowtail returns to g03
compatibility maintenance unless adoption exposes a portable defect.
