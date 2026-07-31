# 008 Codex Child Turn Lifecycle Ownership

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.007
Vision tags: consumer stability, observable child work, exact ownership
Contract refs: 044, 045
Planning state: card 020 completed; Cursor card 013 restored

## Problem

Nucleus reran its authenticated typed-question and subagent acceptance against
the operation-local child activity fix. The completed `spawnAgent` observation
now succeeds and exposes the exact child id, but the next child-local
`turn/started` envelope still enters Codex's root-only lifecycle check and
fails with a provider-session mismatch.

Codex app-server `0.146.0` uses the same top-level turn lifecycle methods for
root and child threads. Child lifecycle must remain observable without gaining
root turn, terminal, callback, provider-request, session, or control authority.

## Contract Delta

Contracts 044 and 045 now permit exact child-local turn start and completion
for an already-admitted operation child. The child turn id is tracked
separately from the root provider turn id. Child completion and failure are
observations only. Unknown, stale, cross-operation, mismatched, and
post-operation ownership still fails closed.

## Goal

Project the real Codex child turn lifecycle and bind ordinary child activity
to it without mutating any root authority.

## Execution

- [x] Execute card 020.
- [x] classify the exact `0.146.0` child lifecycle envelope and ordering
- [x] observe admitted child start and completion with stable child attribution
- [x] bind child item activity to the exact child-local turn id
- [x] keep root lifecycle, terminal, callbacks, requests, and control unchanged
- [x] reject foreign, stale, cross-operation, and post-terminal child lifecycle
- [x] clear child lifecycle state with operation termination
- [x] restore Cursor card 013 as the sole next task

## Boundaries

- no consumer parsing or consumer repository edit
- no child admission without earlier trusted spawn topology
- no root turn-id mutation from child lifecycle or activity
- no root terminal mutation from child completion, failure, or error
- no callback, provider-request, session, or direct-control widening
- no invented parentage or persisted child authority

## Acceptance

- [x] root turn lifecycle remains accepted and terminal
- [x] admitted child start, activity, and completion are observed in order
- [x] child completion does not finish the root operation
- [x] unknown and another operation's child ids fail with a lifecycle-specific diagnostic
- [x] mismatched and post-terminal child lifecycle fails closed
- [x] operation cleanup removes child admission and child-turn correlation
- [x] focused and affected-package Codex validation pass

## Next

After closeout, resume roadmap g03.005 at Cursor card 013. This compatibility
interruption does not alter the Cursor or Antigravity sequence.
