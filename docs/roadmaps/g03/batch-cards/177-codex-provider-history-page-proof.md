# 177 Codex Provider History Page Proof

Status: done
Owner: Tom
Created: 2026-08-08
Milestone: `../057-paged-provider-session-history.md`
Depends on: card 176

## Goal

Prove `codex.app-server` serves Contract 054 newest-first history pages from a
bounded `thread/read(includeTurns: true)` snapshot without control side
effects, using opaque older cursors under existing replay bounds.

## Scope

1. Wire the history-page role on the Codex prepared/app-server surface for an
   exact durable binding.
2. Project turns through the existing replay helpers; slice newest-first pages.
3. Fail closed on bound overflow using the same family of limits as today’s
   history projection.
4. Fixture-prove first page, older page, empty history, overflow, and absence
   of turn/resume/management dispatch.
5. Keep ordinary `load_session` complete-before-ready unchanged.

## Out Of Scope

- native Codex `initialTurnsPage` / turn-cursor qualification
- other routes
- guide inventory closeout (card 178)
- live Codex work

## Acceptance

- [x] Codex fixtures cover Contract 054 Codex mapping items
- [x] no `turn/start`, `turn/interrupt`, `thread/resume`, archive, restore, or
      delete on the history-page path
- [x] focused `swallowtail-adapter-codex` validation passes

## Stop Conditions

- stop if synthetic paging cannot stay honest about `has_older` / totals
  without pretending native provider cursors exist
- stop if history-page work would require changing load readiness rules

## Auto-Continuation

Continue to card 178 when this card’s acceptance is met.

## Closeout

Log: `docs/logs/2026-08-08-codex-provider-history-page-proof.md`
