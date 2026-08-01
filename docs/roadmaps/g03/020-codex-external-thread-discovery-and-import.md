# 020 Codex External Thread Discovery And Import

Status: active
Owner: Tom
Created: 2026-08-01
Depends on: g03.019
Vision tags: Codex continuity, external thread import, consumer stability
Contract refs: 010, 017, 029, 032-034, 037-038, 044-046
Planning state: card 052 ready; cards 053-054 planned

## Problem

Codex app-server exposes thread listing, metadata and history reads, and resume,
but the Swallowtail facade only operates on bindings already created through
Swallowtail. Current main-branch documentation does not prove when each method
entered the maintained `0.80.0..=0.146.0` range.

## Goals

- [ ] freeze exact Codex catalogue, read, history, status, and resume milestones
- [ ] expose a resource-scoped prepared thread catalogue
- [ ] revalidate one selected thread before issuing an imported binding
- [ ] load ordered Codex history through the existing replay phase
- [ ] preserve read-only and bounded-workspace profiles without fallback
- [ ] close with deterministic, package, and public guidance evidence

## Execution Plan

### Batch 20.1 — Exact Codex Range Corpus

- [ ] Execute card 052.
- [ ] freeze `thread/list`, `thread/read`, history, filters, pagination, status,
  and `thread/resume` at every maintained behavior milestone
- [ ] preserve legacy points where catalogue import is unavailable
- [ ] record later-stable and experimental fields separately

### Batch 20.2 — Driver And Prepared Facade

- [ ] Execute card 053 after the corpus passes.
- [ ] implement bounded list and exact read-only revalidation
- [ ] issue imported resume bindings only for exact matching working resources
- [ ] route imported load through existing Codex replay projection

### Batch 20.3 — Conformance And Acceptance

- [ ] Execute card 054 after card 053 passes.
- [ ] prove local and remote-authoritative topology, pagination, stale target,
  active-status observation, cancellation, deadlines, and joined cleanup
- [ ] update prepared guidance and exact route truth
- [ ] compile the extracted Codex package

## Boundaries

- no direct rollout-file or Codex state-database access
- no account-wide listing in the first production profile
- no experimental thread fork, rename, compaction, or item-pagination claim
- no implicit archive, restore, delete, or provider lifecycle effect
- no model, approval, sandbox, access, or working-resource fallback
- no Nucleus or Soundcheck edit
- no authenticated prompt or broad workspace suite

## Acceptance Criteria

- [ ] the guaranteed catalogue segment is exact and does not widen legacy support
- [ ] one approved cwd/resource scope reaches only matching interactive threads
- [ ] title and preview content are bounded and diagnostic-safe
- [ ] a stale, missing, mismatched, or unsupported thread issues no binding
- [ ] successful import loads history before readiness and resumes without replay
- [ ] ordinary Swallowtail-created Codex sessions remain unchanged
- [ ] focused Codex and affected-package validation pass

## Next Planning Checkpoint

After card 054, continue to g03.021. Live Codex import remains separately gated
until deterministic and extracted-package evidence pass.
