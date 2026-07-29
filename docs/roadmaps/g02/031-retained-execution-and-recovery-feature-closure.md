# 031 Retained Execution And Recovery Feature Closure

Status: completed
Owner: Tom
Created: 2026-07-28
Depends on: g02.030
Vision tags: exact lifecycle, provider breadth, retained execution
Contract refs: 009, 014, 016, 021-022, 029, 037, 039, 042
Planning state: cards 103-106 completed

## Problem

The current matrix retains 59 `No` cells across three related but independent
provider-lifecycle features:

- 20 retained-background-execution gaps
- 19 stream-reattachment gaps
- 20 provider-managed-recovery gaps

These columns mix operation survival, stream attachment, and provider-owned
retry or recovery. A provider may retain a result without allowing stream
reattachment, or allow transport reattachment without assuming recovery
authority. Most one-shot, harness, realtime, and attached-runtime operations
may simply be non-applicable.

## Goals

- [x] Revalidate every starting `No` against the exact selected route and
      current maintained evidence.
- [x] Detect false negatives and operation-shape non-applicability.
- [x] Keep retained execution, retrieval, stream reattachment, transport
      reconnect, provider-managed recovery, and consumer retry separate.
- [x] Select only a contract-ready, consumer-useful tranche.
- [x] Freeze exact version, state, cursor, access, cancellation, deadline, and
      cleanup corpora before production work.
- [x] Re-audit all 59 starting cells and retain honest absence.

## Non-Goals

- treating durable provider sessions as retained background operations
- treating a new stream or request retry as reattachment
- granting Swallowtail authority to retry prompts or choose recovery policy
- inferring support from an application UI or sibling provider route
- adding implicit provider, model, endpoint, credential, topology, version, or
  support-authority fallback
- consumer edits, live authentication, publication, or release mutation

## Execution Plan

### Batch 31.1 — Exact Currentness Audit

- [x] Execute card 103.
- [x] Classify all 59 starting cells by exact route and operation shape.
- [x] Rank conversions by consumer value and architectural information.

### Batch 31.2 — Contract And Corpus Gate

- [x] Execute card 104 only after card 103 selects exact routes.
- [x] Promote only evidence-required shared distinctions.
- [x] Freeze deterministic lifecycle and failure corpora.

### Batch 31.3 — Representative Implementation

- [x] Execute card 105 only for contract-ready routes.
- [x] Preserve exact provider state, cursor, recovery, and cleanup truth.

### Batch 31.4 — Matrix Closeout

- [x] Execute card 106.
- [x] Prove package truth, re-audit counts, and select the next matrix family.

## Acceptance Criteria

- [x] all 59 starting cells have current exact-route dispositions
- [x] every changed cell maps to a public prepared path
- [x] retained execution does not imply stream reattachment
- [x] reattachment does not imply provider-managed recovery
- [x] consumer retry policy remains downstream
- [x] no provider effect occurs during audit or default validation

## Decision Gates

- Ask the operator if equally valid route choices would set product priority.
- Stop when official lifecycle wording cannot distinguish retrieval,
  reattachment, reconnect, and recovery.
- Do not borrow lifecycle capability from a sibling route.
- Keep an honest `No` when the selected operation has no qualifying retained
  execution lifecycle.

## Next Planning Checkpoint

Roadmap 032 and card 107 begin the 31-cell working-resource and bounded-write
currentness audit. Stay in g02.
