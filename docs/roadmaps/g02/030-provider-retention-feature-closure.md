# 030 Provider Retention Feature Closure

Status: completed
Owner: Tom
Created: 2026-07-28
Depends on: g02.029 currentness gate
Vision tags: exact lifecycle, provider breadth, destructive authority
Contract refs: 009, 014-017, 025, 029, 037-038
Planning state: cards 099-102 completed

## Problem

The current matrix retains 75 `No` cells across provider archive, restore,
delete, and operation-owned remote cleanup:

- 19 provider-session archive gaps
- 19 provider-session restore gaps
- 18 provider-session delete gaps
- 19 owned-remote-resource-cleanup gaps

These columns mix user-directed durable-session management with cleanup of
resources created for one operation. Similar provider verbs do not make the
authority or lifecycle equivalent.

## Goals

- [x] Revalidate every retained provider-retention `No` against the exact
      selected route and current maintained evidence.
- [x] Detect false negatives and honest non-applicability.
- [x] Keep archive, restore, deletion strength, local close, native close, and
      operation-owned cleanup separate.
- [x] Select only a contract-ready, consumer-useful tranche.
- [x] Freeze exact version, authority, effect, cancellation, and cleanup
      corpora before production work.
- [x] Re-audit all 75 starting cells and retain honest absence.

## Non-Goals

- deleting consumer threads or consumer-owned state
- treating close, abort, disconnect, history-list removal, or archive as
  deletion
- using filesystem deletion behind a harness interface
- borrowing lifecycle authority from a sibling route
- adding implicit provider, account, credential, endpoint, topology, or
  version fallback
- consumer edits, live authentication, publication, or release mutation

## Execution Plan

### Batch 30.1 — Exact Currentness Audit

- [x] Execute card 099.
- [x] Classify all 75 starting cells exactly once.
- [x] Separate conversion, contract gap, upstream absence, and
      non-applicability.

### Batch 30.2 — Contract And Corpus Gate

- [x] Execute card 100 only after card 099 selects exact routes.
- [x] Promote only evidence-required shared distinctions.
- [x] Freeze deterministic effect and failure corpora.

### Batch 30.3 — Representative Implementation

- [x] Execute card 101 only for contract-ready routes.
- [x] Preserve exact destructive authority and cleanup ownership.

### Batch 30.4 — Matrix Closeout

- [x] Execute card 102.
- [x] Prove package truth, re-audit counts, and select retained execution next.

## Acceptance Criteria

- [x] all 75 starting cells have current exact-route dispositions
- [x] every changed cell maps to a public prepared path
- [x] deletion strength and affected scope never strengthen silently
- [x] user-directed management and operation-owned cleanup remain distinct
- [x] unsupported and not-applicable remain distinct
- [x] no provider effect occurs during audit or default validation

## Decision Gates

- Ask the operator if equally valid route choices would set product policy.
- Stop when official lifecycle wording cannot establish effect strength.
- Do not infer lifecycle authority from an application UI or another
  transport.
- Keep an honest `No` when the selected operation owns no qualifying resource.

## Next Planning Checkpoint

Roadmap 031 and card 103 begin the 59-cell retained-execution and recovery
currentness audit. Stay in g02.
