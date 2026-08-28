# g04.091 Generation Closeout And g05 Cutover

Status: completed
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Depends on: g04.090 closeout
Vision tags: generation discipline, harness skills, process observability
Contract refs: 001, 013, 017, 023, 029, 034, 041, 044, 047, 052, 057
Planning state: cards 258-260 completed

## Purpose

Close g04 after the route-readiness and per-route feature programmes exhausted
their active runway. Preserve every parked or standing item. Open g05 with an
evidence-first harness skill visibility and process observability programme.

## Generation Boundary

g04 closes at 91 numbered roadmaps. The 85-item per-route ledger has 83 closed
dispositions, no active qualification or delivery item, and two parked Bedrock
items. Contract 029 currentness remains standing. Hosted OAuth, OpenHands,
Aider, and Kiro headless remain parked outside routine summaries.

## Goals

- [x] prove no active g04 feature or delivery lane remains
- [x] preserve parked and standing work without promoting it
- [x] mark g04 complete and make g05 the sole active generation
- [x] promote the harness skill and watcher triage into an evidence-first lane
- [x] compile g05.001 and cards 001-003
- [x] leave only the prompt-free inventory card ready

## Non-Goals

- watcher implementation or skill injection
- public API, adapter, process, transport, or provider behavior changes
- live provider prompts, credentials, install, login, or ambient home scanning
- arbitrary PID or foreign-process authority
- new routes or parked Bedrock qualification
- release, publication, tag, or consumer-repository mutation

## Execution Plan

### Batch 91.1 — Closeout Inventory

- [x] execute card 258
- [x] reconcile the g04 generation, per-route ledger, backlog, triage, and
      standing-lane dispositions

### Batch 91.2 — Generation Cutover

- [x] execute card 259
- [x] close g04 and open g05 as the sole active generation
- [x] retarget active roadmap and batch-card validation to g05

### Batch 91.3 — First g05 Runway

- [x] execute card 260
- [x] compile g05.001 and cards 001-003
- [x] reserve Research 255 and make only card 001 ready

## Acceptance Criteria

- [x] generation index names exactly one active generation
- [x] g04 contains no ready milestone or active programme
- [x] all unfinished g04 surfaces retain explicit closed, blocked, parked, or
      standing dispositions
- [x] g05 begins with evidence, not a selected watcher architecture
- [x] sole Next Task points to g05.001 card 001
- [x] docs QA, Northstar QA, and diff checks pass

## Decision

The operator authorized rollover on 2026-08-28 after confirming the per-route
list was exhausted. The only open product triage family becomes g05's
evidence-first programme. Its unresolved operator decisions remain behind card
001 evidence and card 002 classification.

## Evidence

- g04.090 closeout at `d63a519b`
- 83 closed per-route items, zero active, two parked
- promoted harness skill discovery and process watcher triage
- g05.001 with one ready evidence card and two planned follow-ons
