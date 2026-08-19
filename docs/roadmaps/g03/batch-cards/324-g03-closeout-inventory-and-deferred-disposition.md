# 324 g03 Closeout Inventory And Deferred Disposition

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../106-generation-closeout-and-g04-cutover.md`

## Objective

Prove that g03 can close without losing unfinished authority or pretending
deferred harness work, currentness, or consumer defects are complete.

## Governing Refs

- Contract 001
- generation index and long-term plan
- g03 generation and batch-card indexes
- g03.051, g03.093, g03.094, g03.095
- Research 153, 155, 158, 159
- shared roadmap backlog

## Scope

1. Inventory all g03 roadmap and batch-card statuses.
2. Confirm active and provisional spec state.
3. Repair the g03.051 planned-versus-complete index drift.
4. Rehome Aider headless, Kiro headless, and OpenHands production wiring as
   shared backlog evidence.
5. Leave Gemini requalification, Pi continuity, and binding persistence on
   their existing backlog gates.
6. Treat recurring currentness as a continuing Contract 029 process, not a
   leftover g03 card.

## Acceptance Criteria

- [x] g03 contains 106 roadmaps after the closeout milestone
- [x] g03.095 and cards 295-298 are the only unfinished Aider execution
      evidence
- [x] Kiro headless and OpenHands production wiring each have one shared
      backlog record
- [x] no ready g03 implementation card remains
- [x] Active Specs is empty
- [x] no provider or consumer behavior changes

## Validation

- roadmap and card status inventory
- spec and backlog inventory
- `git diff --check`

## Stop Conditions

- another unfinished g03 commitment lacks a disposition
- a deferred harness gate can no longer be stated from its research record
- a consumer handoff is incorrectly represented as Swallowtail execution

## Auto-Continuation

Yes, into card 325 after every unfinished surface has an explicit home.

## Evidence

- 105 numbered g03 roadmaps plus README inventoried before adding 106
- g03.051 marked completed; its cards 152-155 already closed
- specs 006-007 promoted; specs 008-010 archived
- Aider, Kiro headless, and OpenHands production wiring now have backlog
  records
- Gemini, Pi, and binding persistence retain explicit dispositions
