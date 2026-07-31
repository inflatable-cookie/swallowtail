# 047 Codex Exec Queryless Navigation Projection

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../018-codex-exec-queryless-navigation-lifecycle.md`
Depends on: card 046

## Goal

Freeze and project the exact queryless Codex navigation lifecycle without
widening malformed completed-search acceptance.

## Scope

1. Add a deterministic `0.146.0` activity-corpus case with one queryless
   started item and the observed queryless completed `action.type == "other"`
   item under the same provider id.
2. Accept missing display content for that completed action only.
3. Assert stable activity identity, start/completion phases, and no content.
4. Assert normal query-bearing search content remains visible.
5. Assert completed `action.type == "search"` without a query remains
   malformed.

## Acceptance Criteria

- [x] the fixture fails before the parser change
- [x] the exact queryless navigation lifecycle projects after the change
- [x] no query or summary is invented
- [x] query-bearing search behavior is unchanged
- [x] malformed completed actual searches remain rejected
- [x] focused activity/parser tests pass
- [x] card 048 becomes the sole ready and next task

## Validation

- targeted Codex activity and exec parser tests
- `git diff --check`
- no live provider or broad suite

## Auto-Continuation

Yes. Continue to card 048 after the narrow projection passes.

## Evidence

- the exact corpus failed at completed projection before the rule change
- `Completed` plus exact `action.type == "other"` is the only new content-free
  case
- query-bearing, deferred-query, and malformed actual-search tests pass
- three targeted queryless lifecycle/parser regressions pass
