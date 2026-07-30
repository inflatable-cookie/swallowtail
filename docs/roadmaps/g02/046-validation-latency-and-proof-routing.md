# 046 Validation Latency And Proof Routing

Status: completed
Owner: Tom
Created: 2026-07-30
Depends on: g02.045
Vision tags: validation discipline, developer feedback, package isolation
Contract refs: 001, 036
Planning state: cards 156-158 completed

## Problem

Swallowtail has strong focused, workspace, package, route, and release proof,
but normal implementation work can trigger duplicated broad gates. Affected
package checks may also rebuild the same dependency graph in separate targets.
Small batches therefore spend more time re-proving unrelated surfaces than
implementing or reviewing the change.

Proof must remain strong. The fix is explicit evidence tiers, focused
selectors, and shared package-proof work, not skipped acceptance.

## Generation Runway Goal

Make normal development validation proportional to change scope while keeping
milestone, candidate, and release acceptance exact and independently runnable.

## Goals

- [x] measure current selector runtime, duplication, and cache boundaries
- [x] classify required proof by focused batch, milestone, package, and release
  tier
- [x] add focused Effigy selectors for changed packages and affected extracted
  package proof
- [x] share package assembly and compilation work where isolation permits
- [x] document when broad workspace and candidate gates are required
- [x] prove unchanged failure and evidence coverage with lower normal-path
  latency

## Non-Goals

- removing tests, warnings-denied lint, route truth, package isolation, or
  release gates
- changing provider behavior or public APIs
- consumer repository edits
- warning-only source decomposition
- publication or retained-candidate replacement

## Execution Plan

### Batch 46.1 — Inventory And Budgets

- [x] Execute card 156.
- [x] map tasks, scripts, duplicated work, cache reuse, and current durations
- [x] freeze evidence tiers and acceptable normal-path budgets

### Batch 46.2 — Focused Selectors

- [x] Execute card 157.
- [x] add changed-package and affected extracted-package selectors
- [x] preserve package isolation and exact command failure

### Batch 46.3 — Acceptance

- [x] Execute card 158.
- [x] compare representative before and after paths
- [x] prove full milestone and release gates remain available and unchanged
- [x] publish concise validation guidance

## Acceptance Criteria

- [x] common adapter batches have one documented focused validation path
- [x] affected package archives compile through one reusable selector
- [x] repeated shared dependency compilation is removed where safe
- [x] broad workspace, candidate, and release gates retain their evidence
- [x] selector failures remain visible and non-zero
- [x] no provider effect, consumer edit, or publication occurs
- [x] one clear next task remains

## Decision Gates

- Stop if latency reduction weakens package isolation or release proof.
- Stop if a selector infers provider or public-API scope from unsafe heuristics.
- Keep explicit package lists until changed-file selection is proven reliable.
- Do not benchmark the full test matrix repeatedly.

## Next Planning Checkpoint

Reassess the next g02 product or provider milestone after concurrent
subagent-topology work closes. Warning-only reduction and publication remain
deferred.
