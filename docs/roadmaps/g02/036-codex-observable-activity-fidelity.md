# 036 Codex Observable Activity Fidelity

Status: planned
Owner: Tom
Created: 2026-07-29
Depends on: g02.035
Vision tags: Codex, installed harness, observable activity
Contract refs: 009, 012-013, 023, 029, 032-034, 037, 044
Planning state: cards 122-124 planned

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

- [ ] Freeze activity-schema milestones across the maintained Codex range.
- [ ] Map app-server item lifecycle without provider payload leakage.
- [ ] Correlate approvals and dynamic tools with visible activity.
- [ ] Preserve plans, reasoning summaries, commands, files, tools, search,
      subagents, review, compaction, hooks, and unknown semantic items.
- [ ] Map `codex exec` as an honest completion-oriented profile.
- [ ] Preserve unverified-newer admission without fidelity widening.

## Non-Goals

- changing Codex access, sandbox, workspace, model, or session policy
- treating exec and app-server as one driver
- exposing raw reasoning blocks
- consumer UI or persistence
- live authentication or provider calls in default validation

## Execution Plan

### Batch 36.1 — Maintained-Range Corpus

- [ ] Execute card 122.
- [ ] Revalidate current official protocol and exact supported milestones.
- [ ] Freeze positive, additive, deprecated, unknown, malformed, and
      disclosure fixtures.

### Batch 36.2 — App-Server Projection

- [ ] Execute card 123.
- [ ] Add complete lifecycle, typed deltas, request correlation, and exact
      prepared activity profile.

### Batch 36.3 — Exec Projection And Closeout

- [ ] Execute card 124.
- [ ] Add completion-only activity where the JSONL source supports it.
- [ ] Run full Codex range, facade, and package-facing regression.

## Acceptance Criteria

- [ ] every qualified app-server semantic item is mapped or namespaced unknown
- [ ] command output and file diffs retain exact item ownership
- [ ] intermediate and final assistant messages remain distinct
- [ ] reasoning summaries never imply hidden reasoning
- [ ] approvals, callbacks, tool calls, and results retain separate state
- [ ] exec does not claim app-server lifecycle fidelity
- [ ] every guaranteed Codex version segment has deterministic evidence
- [ ] exact newer unknowns do not silently disappear

## Decision Gates

- Stop on undocumented event identity or unsafe raw-content dependence.
- Keep a thinner profile when an older milestone lacks a newer item field.
- Do not narrow the maintained version range merely to simplify mapping.

## Next Planning Checkpoint

After card 124, compare the Codex lifecycle proof with ACP's update-oriented
shape before executing g02.037.

