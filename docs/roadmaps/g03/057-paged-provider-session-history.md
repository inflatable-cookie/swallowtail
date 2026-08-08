# 057 Paged Provider Session History

Status: completed
Owner: Tom
Created: 2026-08-08
Depends on: g03.020, g03.027, g03.056
Vision tags: consumer stability, session continuity, compatibility maintenance
Contract refs: 017, 038, 044, 046, 048, 054
Planning state: cards 176-178 completed

## Problem

Chat UIs need a bound newest history window first, then older pages on
scroll-back, with honest fetched/total metadata. Today `load_session` forces
complete-before-ready full replay, and reconciliation’s `replay_complete` means
snapshot-fit-bounds, not pagination. Consumers lack a portable page API.

## Goals

- [x] land portable history-page plan/request/response, cursors, totals, and
      driver role in runtime under Contract 054
- [x] prove Codex app-server synthetic newest-first pages under existing
      replay bounds without control side effects
- [x] document the feature and Codex mapping for consumers/operators

## Execution Plan

- [x] Execute card 176 (runtime history-page vocabulary and role).
- [x] Execute card 177 (Codex synthetic page proof).
- [x] Execute card 178 (guide and matrix/inventory notes).

## Boundaries

- no weakening of load complete-before-ready
- no folding scroll into reconciliation
- no consumer-transcript store in Swallowtail
- no native Codex turn-pagination qualification in this milestone
- no live provider work unless a card explicitly requires the active API
  baseline check
- no tag or release in this milestone

## Acceptance Criteria

- [x] Contract 054 acceptance items covered by runtime and Codex fixtures
- [x] focused runtime and Codex package validation passes
- [x] public API disposition matches the active candidate baseline policy
- [x] guides state resume/browse vs load, and totals Exact/AtLeast/Unknown

## Next Planning Checkpoint

Additional honest route mappings landed in g03.058. Native Codex turn
pagination remains a later exact qualification.
