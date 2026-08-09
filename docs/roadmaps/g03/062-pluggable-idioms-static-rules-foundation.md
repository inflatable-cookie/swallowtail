# 062 Pluggable Idioms Static-Rules Foundation

Status: planned
Owner: Tom
Created: 2026-08-09
Depends on: Research 117, Spec 006, Contract 055
Vision tags: learned preferences, idioms, consumer mechanism
Contract refs: 003, 010, 036, 052-053, 055

## Problem

Consumers have no provider-neutral mechanism to learn, store, select, and
deliver behavioral preferences ("idioms") from interaction signals. Contract
055 fixes the boundary; nothing realizes it yet.

## Generation Runway

Advance g03's consumer-facing mechanism breadth without touching adapter
routes. The static-rules proof sequences first; any learned backend stays
behind the trait seam per Contract 055. Candidate lane — not queued against
g03.060/061 until the operator selects it.

## Execution Plan

- [ ] card 189: realize the `swallowtail-idioms` records foundation — idioms
      records, confidence decay, merge, and lint with deterministic fixtures
- [ ] card 190: realize the engine trait — bounded selection, fail-soft
      signal sink, and testkit conformance
- [ ] card 191: realize the static-rules backend and session-preparation
      `IdiomSet` delivery
- [ ] card 192: realize the registry-client merge surface without transport,
      then package, guide, matrix, and acceptance evidence

## Goals

- [ ] one separately selectable `swallowtail-idioms` package through the
      Contract 036 architecture/package review
- [ ] deterministic confidence decay under fixture clocks
- [ ] deterministic merge outcomes: new, raised, lowered, unchanged
- [ ] bounded scope- and confidence-ordered selection
- [ ] fail-soft signal sink; a missing sink is a no-op and never fails an
      operation
- [ ] registry pull/push merge without HTTP or transport authority
- [ ] no prompt composition, no permission authority, no learned-model
      dependency

## Boundaries

- no prompt text composition, editing, or injection
- no permission or trust enforcement
- no Monkey or learned-model dependency in the crate graph
- no HTTP client or raw network authority in the crate
- no adapter or harness route changes
- no version bump, tag, GitHub Release, or registry mutation in this
  milestone

## Acceptance Criteria

- [ ] conformance fixtures cover merge, decay, lint, selection ordering,
      boundedness, and fail-soft sink behavior
- [ ] headless routes remain static-rules-only
- [ ] the crate graph stays acyclic and free of learned-model dependencies
- [ ] focused and extracted-package validation pass without provider or
      consumer work
- [ ] guide, example, matrix, architecture, and package contract remain
      mutually honest
- [ ] operator approves lane selection and the Contract 036 package review
      before release-set entry

## Planning Checkpoint

After card 192, reassess learned-backend selection and the consumer-proven
correction-loop proxy through Soundcheck with exact evidence before any
second tranche.
