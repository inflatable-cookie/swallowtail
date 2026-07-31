# 049 g02 Generation Closeout And g03 Cutover

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g02.048
Vision tags: compatibility maintenance, consumer proof, generation discipline
Contract refs: 001, 002, 005, 011, 029, 036-045
Planning state: cards 165-167 compiled

## Purpose

Close g02 at its intended upper range, preserve its one externally gated Pi
continuity lane, and open g03 around compatibility maintenance and
consumer-proven hardening.

## Generation Runway

Roadmap 049 takes g02 to 49 numbered roadmaps. The stabilization, facade,
feature-matrix, observable-activity, compatibility, structural-health, and
Claude elicitation programmes are complete. Another provider implementation
inside g02 would invent work after the sequencing baseline has expired.

## Goals

- [x] inventory every unfinished g02 roadmap and card
- [x] preserve Pi RPC continuity behind its exact upstream gate
- [x] close g02 without hiding consumer-owned adoption work
- [x] open g03 with an explicit compatibility-maintenance programme
- [x] compile a bounded evidence-first first milestone
- [x] leave one sole ready next task

## Non-Goals

- provider, transport, protocol, host, or consumer implementation
- external currentness research or live provider probes
- publication, tag, push, candidate replacement, or registry mutation
- provider-session binding persistence
- warning-only structural cleanup
- changing the Pi attachment or resource-binding requirement

## Execution Plan

### Batch 49.1 — Inventory And Deferred Disposition

- [x] Execute card 165.
- [x] audit roadmap, card, spec, backlog, and front-door state
- [x] move Pi continuity to shared backlog evidence

### Batch 49.2 — Generation Closure And Cutover

- [x] Execute card 166.
- [x] mark g02 complete at 49 roadmaps
- [x] open g03 as the sole active generation
- [x] refresh front-door and generation authority

### Batch 49.3 — First g03 Runway

- [x] Execute card 167.
- [x] compile g03.001 and cards 001-003
- [x] make only the repository-local inventory card ready
- [x] retain provider selection behind current authoritative evidence

## Acceptance Criteria

- [x] every non-completed g02 surface has an explicit disposition
- [x] paused Pi evidence remains linked and recoverable
- [x] no active spec or consumer handoff silently governs g03
- [x] g03 names a long-horizon programme, not a one-turn queue
- [x] publication and binding persistence remain outside the active lane
- [x] docs QA, Northstar QA, Doctor review, and diff checks pass
- [x] `docs/roadmaps/README.md` contains the sole active Next Task

## Decision

The operator approved g02 closure and a g03 compatibility-maintenance and
consumer-proven-hardening programme on 2026-07-31.

## Evidence

- 49 g02 roadmaps inventoried: 48 completed roadmaps including this closeout,
  plus roadmap 029 moved to backlog evidence
- cards 097-098 moved with the Pi lane; no ready g02 card remains
- Active Specs is empty
- g03.001 and cards 001-003 provide the visible evidence-first runway
- docs QA and Northstar QA passed
- Doctor remains warning-only at 147 findings and zero errors
- `git diff --check` passed
