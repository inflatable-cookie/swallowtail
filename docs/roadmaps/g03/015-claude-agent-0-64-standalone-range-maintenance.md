# 015 Claude Agent 0.64 Standalone Range Maintenance

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.014
Vision tags: maintained compatibility, ACP, installed harnesses, consumer stability
Contract refs: 006, 011, 015, 029, 032, 037-041, 044-045
Planning state: cards 039-041 completed

## Problem

Claude Agent `0.62.0..=0.64.0` has exact source evidence but remained coupled
to a Gemini range extension that no longer belongs in the active queue. The
three releases contain one unchanged point and two private behavior milestones.

## Goal

Guarantee Claude Agent ACP through exact `0.64.0` without changing its
baselines, access profiles, lifecycle strength, callback subset, or portable
authority.

## Goals

- [x] freeze exact package and selected-source evidence through `0.64.0`
- [x] add explicit `0.63.0` and `0.64.0` behavior revisions
- [x] retain the unpublished `0.58.0` exclusion and every prior baseline
- [x] prove installed exact `0.63.0` and artifact exact `0.64.0`
- [x] preserve visible unverified-newer execution above the new ceiling
- [x] accept through focused and extracted-package evidence

## Execution Plan

### Batch 15.1 — Exact Range Corpus

- [x] Execute card 039.
- [x] freeze package, dependency, source, access, and selected delta evidence
- [x] keep the production claim at `0.61.0` during corpus acceptance

### Batch 15.2 — Claims And Route Conformance

- [x] Execute card 040.
- [x] extend the claim through `0.64.0` with exact private milestones
- [x] prove lifecycle, elicitation, activity, structured, and interactive truth

### Batch 15.3 — Installed And Package Acceptance

- [x] Execute card 041.
- [x] classify installed `0.63.0` and current `0.64.0` artifact separately
- [x] reconcile public truth and return to the maintenance checkpoint

## Boundaries

- no Gemini implementation
- no provider prompt, authentication mutation, session creation, or deletion
- no nested-transcript or host-owned steering opt-in
- no Claude Code headless claim change
- no implicit credential, endpoint, billing, model, route, or sandbox fallback
- no consumer edit or publication

## Acceptance Criteria

- [x] `0.53.0..=0.64.0` is maintained except explicit `0.58.0`
- [x] selected behavior milestones remain distinct
- [x] local subscription and public API-key access remain separate
- [x] later stable versions remain permitted and visibly unverified
- [x] focused and extracted-package validation pass
- [x] one sole Next Task pointer remains

## Next Planning Checkpoint

Return to g03 maintenance after card 041. Gemini remains deferred until its
backlog promotion gate is satisfied.
