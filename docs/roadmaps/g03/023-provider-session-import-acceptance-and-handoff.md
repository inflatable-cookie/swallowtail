# 023 Provider Session Import Acceptance And Handoff

Status: completed
Owner: Tom
Created: 2026-08-01
Depends on: g03.022
Vision tags: provider-wide truth, package acceptance, consumer adoption
Contract refs: 002, 005-006, 011, 017, 029, 036-038, 046
Planning state: cards 061-063 completed

## Problem

The first three implementations need provider-wide classification, consistent
prepared guidance, extracted-package proof, and a consumer handoff that does
not make Swallowtail own Nucleus persistence or UI.

## Goals

- [x] classify every harness route against the complete import profile
- [x] distinguish supported, discovery-only, attachment-only, blocked, and not
  applicable routes
- [x] assess Kimi local server, Claude, Cursor, and Pi without silent widening
- [x] reconcile route and feature matrices plus integration guidance
- [x] prove the selected packages assemble together
- [x] hand Nucleus a bounded browse-select-import-load adoption contract

## Execution Plan

### Batch 23.1 — Remaining Route Classification

- [x] Execute card 061.
- [x] audit each harness route for catalogue, lookup, history, load, resume,
  resource binding, activity truth, and exact version support
- [x] implement no additional route unless all evidence is already complete
- [x] preserve explicit promotion gates for partial routes

### Batch 23.2 — Public Truth And Package Acceptance

- [x] Execute card 062 after classification settles.
- [x] update provider route and feature matrices
- [x] document the separate catalogue, import, load, resume, and management
  operations
- [x] assemble and compile the affected common, Codex, Kimi, and OpenCode
  packages through one extracted target

### Batch 23.3 — Nucleus Adoption Handoff

- [x] Execute card 063 after package acceptance.
- [x] define the consumer-owned thread mapping and import workflow
- [x] specify replay persistence, duplicate detection, stale-candidate UX, and
  unsupported-route presentation as Nucleus responsibilities
- [x] leave Nucleus implementation to its repository

## Boundaries

- no Nucleus, Soundcheck, or other consumer edit
- no Swallowtail thread database, repository scan, UI, routing, or sync daemon
- no background polling or bidirectional merge
- no management-binding persistence promotion
- no capability promotion from provider family or alternate transport
- no live provider effect, publication, or broad workspace suite unless a card
  is amended with explicit operator authority

## Acceptance Criteria

- [x] every harness route has an evidence-backed import classification
- [x] only complete list, revalidation, replay, and continuation routes report
  support
- [x] public guidance prevents raw-id attachment and automatic synchronization
- [x] selected packages compile independently with common conformance
- [x] one Nucleus handoff preserves consumer database and UI ownership
- [x] deferred routes retain explicit unblock evidence
- [x] the sole Next Task pointer returns to the g03 evidence gate

## Next Planning Checkpoint

After card 063, Nucleus may adopt independently. Swallowtail returns to g03
compatibility maintenance unless adoption exposes a portable defect.
