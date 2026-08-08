# 176 Runtime Provider History Page

Status: done
Owner: Tom
Created: 2026-08-08
Milestone: `../057-paged-provider-session-history.md`
Depends on: Contract 054

## Goal

Land the portable provider-session history page vocabulary, plan/request/
response types, cursor rules, totals, and driver role in runtime so adapters
can expose newest-first pages without touching load or reconciliation
semantics.

## Scope

1. Add history-page agreement, request, response, older-cursor, and total
   cardinality (`Exact` / `AtLeast` / `Unknown`) types in runtime.
2. Add a read-only driver role/default unsupported method distinct from
   `load_session`, import, catalogue, and reconciliation.
3. Reuse `SessionReplayItem` projection; document page item order and
   cross-page older traversal.
4. Share bound/tail helpers with reconciliation where practical without
   changing `replay_complete` meaning.
5. Prove cursor/plan mismatch, empty history, bound overflow, and metadata
   honesty in runtime fixtures.

## Out Of Scope

- Codex or other adapter emissions
- guide text beyond rustdoc
- native provider pagination wire work
- live provider work

## Acceptance

- [x] Contract 054 runtime acceptance items for page shape, cursors, totals,
      and authority boundary are covered by tests
- [x] existing load and reconciliation APIs remain behavior-compatible
- [x] focused runtime validation passes
- [x] `effigy package:api` disposition matches the active candidate baseline
      policy for the additive surface

## Stop Conditions

- stop if the role would have to return a live session handle to be useful
- stop if totals cannot stay honest without inventing provider counts

## Auto-Continuation

Continue to card 177 when this card’s acceptance is met.

## Closeout

Log: `docs/logs/2026-08-08-runtime-provider-history-page.md`
