# 036 Codex Observable Activity Fidelity

Status: completed
Owner: Tom
Created: 2026-07-29
Depends on: g02.035
Vision tags: Codex, installed harness, observable activity
Contract refs: 009, 012-013, 023, 029, 032-034, 037, 044
Planning state: cards 122-124 completed

## Problem

Codex app-server exposes rich item, plan, command, file, tool, search, review,
compaction, hook, and reasoning-summary events. Swallowtail currently
preserves assistant output and selected progress but discards most work-log
semantics. `codex exec` has a different completion-oriented event shape and
must retain a thinner exact profile.

## Generation Runway Goal

Prove the richest installed-harness activity source and the contrasting
bounded structured transport against one portable kernel.

## Goals

- [x] Freeze activity-schema milestones across the maintained Codex range.
- [x] Map app-server item lifecycle without provider payload leakage.
- [x] Correlate approvals and dynamic tools with visible activity.
- [x] Preserve plans, reasoning summaries, commands, files, tools, search,
      subagents, review, compaction, hooks, and unknown semantic items.
- [x] Map `codex exec` as an honest completion-oriented profile.
- [x] Preserve unverified-newer admission without fidelity widening.

## Non-Goals

- changing Codex access, sandbox, workspace, model, or session policy
- treating exec and app-server as one driver
- exposing raw reasoning blocks
- consumer UI or persistence
- live authentication or provider calls in default validation

## Execution Plan

### Batch 36.1 — Maintained-Range Corpus

- [x] Execute card 122.
- [x] Revalidate current official protocol and exact supported milestones.
- [x] Freeze positive, additive, deprecated, unknown, malformed, and
      disclosure fixtures.

### Batch 36.2 — App-Server Projection

- [x] Execute card 123.
- [x] Add complete lifecycle, typed deltas, request correlation, and exact
      prepared activity profile.

### Batch 36.3 — Exec Projection And Closeout

- [x] Execute card 124.
- [x] Add completion-only activity where the JSONL source supports it.
- [x] Run full Codex range, facade, and package-facing regression.

## Acceptance Criteria

- [x] every qualified app-server semantic item is mapped or namespaced unknown
- [x] command output and file diffs retain exact item ownership
- [x] intermediate and final assistant messages remain distinct
- [x] reasoning summaries never imply hidden reasoning
- [x] approvals, callbacks, tool calls, and results retain separate state
- [x] exec does not claim app-server lifecycle fidelity
- [x] every guaranteed Codex version segment has deterministic evidence
- [x] exact newer unknowns do not silently disappear

## Decision Gates

- Stop on undocumented event identity or unsafe raw-content dependence.
- Keep a thinner profile when an older milestone lacks a newer item field.
- Do not narrow the maintained version range merely to simplify mapping.

## Next Planning Checkpoint

Card 125 revalidates ACP's update-oriented shape against the completed Codex
contrast before production protocol mapping.
