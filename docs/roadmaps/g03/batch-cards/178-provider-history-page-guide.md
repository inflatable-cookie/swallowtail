# 178 Provider History Page Guide

Status: done
Owner: Tom
Created: 2026-08-08
Milestone: `../057-paged-provider-session-history.md`
Depends on: card 177

## Goal

Document the portable history-page feature and Codex first mapping so consumers
can resume/browse without treating pages as load replay or reconciliation.

## Scope

1. Add or extend a guide covering newest-first pages, cursors, totals
   Exact/AtLeast/Unknown, and authority limits.
2. Cross-link Contracts 017, 048, and 054; note load vs resume+page vs
   reconciliation.
3. Update route/feature inventory notes for Codex app-server support.
4. Record the milestone closeout log.

## Out Of Scope

- additional adapter mappings
- Nucleus local transcript paging
- live provider work
- release tagging

## Acceptance

- [x] guide states ordinary apps may resume then page, and must not use pages
      for control flow or transcript authority
- [x] Codex mapping and synthetic-page limits are explicit
- [x] indexes/logs updated
- [x] docs QA for touched guides passes when run for this card

## Stop Conditions

- stop if docs would imply every route already supports history pages

## Auto-Continuation

Close g03.057 and return to the g03 evidence gate.

## Closeout

Log: `docs/logs/2026-08-08-provider-history-page-guide.md`
