# 014 Cursor Agent 2026.07.23 Range Maintenance

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.013
Vision tags: maintained compatibility, installed harnesses, ACP, structured execution
Contract refs: 011, 015, 020, 029, 032, 037, 039, 044
Planning state: cards 036-038 completed

## Problem

The maintained ACP registry moved Cursor Agent to
`2026.07.23-e383d2b`. Swallowtail guaranteed only exact
`2026.07.01-41b2de7`, and its parser enforced an opaque build revision only for
the latest qualified date. Moving the date constant alone would have weakened
the older milestone.

## Goal

Guarantee both exact Cursor builds across catalogue, ACP, and headless routes,
retain their selected behavior, and enforce build identity per qualified date.

## Execution Plan

### Batch 14.1 — Exact Artifact And Delta Corpus

- [x] Execute card 036.
- [x] freeze current registry, archive, executable, initialize, and chunk identity
- [x] classify selected and unselected deltas

### Batch 14.2 — Claims And Build Gates

- [x] Execute card 037.
- [x] add a second exact milestone to all three claims
- [x] enforce the opaque build revision for both qualified dates
- [x] preserve visible unverified-newer execution above the ceiling

### Batch 14.3 — Package And Public Acceptance

- [x] Execute card 038.
- [x] run focused and extracted-package proof
- [x] reconcile route truth, architecture, and currentness pointers

## Boundaries

- no continuous calendar range
- no provider prompt, authenticated catalogue, session creation, or mutation
- no new ACP lifecycle, MCP, callback, model-selection, or sandbox authority
- no installation, update, consumer edit, or publication

## Acceptance Criteria

- [x] exact current artifact identity is frozen
- [x] both qualified dates require their exact build suffix
- [x] all three routes support both exact milestones
- [x] the calendar gap remains unsupported
- [x] later dates remain permitted and visibly unverified
- [x] focused, extracted-package, route, docs, and Northstar checks pass

## Next Planning Checkpoint

Return to the g03 compatibility-maintenance checkpoint. Keep standalone Claude
and Gemini range work paused.
