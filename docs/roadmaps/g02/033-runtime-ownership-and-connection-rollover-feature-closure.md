# 033 Runtime Ownership And Connection Rollover Feature Closure

Status: completed
Owner: Tom
Created: 2026-07-28
Depends on: g02.032
Vision tags: runtime ownership, connection continuity, provider breadth
Contract refs: 004, 009, 018, 023, 026-027, 029, 031, 037
Planning state: cards 111 and 114 complete; cards 112-113 superseded by the
negative tranche

## Problem

The current matrix retains 40 `No` cells across two independent tail
features:

- 20 owned-runtime-lifecycle gaps
- 20 planned-connection-rollover gaps

Most provider routes do not own a model runtime, and most operation shapes do
not keep a connection that can roll over. The remaining cells must distinguish
non-applicability from exact selected-surface absence before any
implementation is selected.

## Goals

- [x] Revalidate all 40 starting cells against exact route and operation
      shape.
- [x] Keep runtime ownership, harness child-process ownership, provider
      service ownership, and attached runtime identity separate.
- [x] Keep active-turn reattachment, background retrieval, provider session
      resume, transport reconnect, and planned rollover separate.
- [x] Detect false negatives and current contract-ready candidates.
- [x] Freeze exact lifecycle, version, topology, failure, and cleanup evidence
      before implementation.
- [x] Close the matrix family with exact machine-checked dispositions.

## Non-Goals

- treating every Swallowtail-owned process as an owned model runtime
- absorbing Monkey or consumer serving responsibilities
- relabelling provider-managed services as Swallowtail-owned runtimes
- turning ordinary reconnect into continuity
- implicit provider, model, endpoint, credential, topology, version, or
  support-authority fallback
- consumer edits, live credentials, provider effects, publication, or release
  mutation

## Execution Plan

### Batch 33.1 — Exact Currentness Audit

- [x] Execute card 111.
- [x] Classify every starting cell exactly once.
- [x] Rank only exact, consumer-useful candidates.

### Batch 33.2 — Contract And Corpus Gate

- [x] Supersede card 112 because card 111 selected no exact route.
- [x] Confirm existing contracts already settle every classification.
- [x] Add no speculative corpus.

### Batch 33.3 — Representative Implementation

- [x] Supersede card 113 because no implementation route was selected.
- [x] Preserve ownership, continuity, and cleanup truth.

### Batch 33.4 — Matrix Closeout

- [x] Execute card 114.
- [x] Re-audit all 40 cells and select the next matrix family or programme
      checkpoint.

## Acceptance Criteria

- [x] all 40 starting cells have current exact-route dispositions
- [x] every changed cell is a category correction, not a capability claim
- [x] owned runtime lifecycle never widens attached or provider-owned routes
- [x] rollover never implies replay, reattachment, resume, or recovery
- [x] no provider effect occurs during audit or default validation

## Decision Gates

- Ask the operator if equally useful routes would set product priority.
- Keep an honest `No` when a route has no exact ownership or continuity
  mechanism.
- Stop when current evidence cannot distinguish reconnect from continuity.

## Next Planning Checkpoint

Roadmap 034 audits the 61 cells still lacking a completed feature-family
classification. Stay in g02.
