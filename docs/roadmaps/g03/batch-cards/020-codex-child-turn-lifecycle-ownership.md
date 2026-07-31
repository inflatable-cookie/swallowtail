# 020 Codex Child Turn Lifecycle Ownership

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../008-codex-child-turn-lifecycle-ownership.md`
Depends on: card 019

## Goal

Repair the consumer-proven Codex child lifecycle rejection with exact,
operation-local child-turn correlation and no root authority widening.

## Scope

1. Classify Codex app-server `0.146.0` child turn start and completion shapes.
2. Accept lifecycle only for a child admitted by earlier spawn topology.
3. Retain the child-local turn id separately from the root provider turn id.
4. Attribute lifecycle and ordinary child activity to that child.
5. Keep child completion, failure, and error observational.
6. Preserve root terminal, callback, provider-request, session, and control checks.
7. Add actual ordering, foreign, cross-operation, mismatch, cleanup, and corpus coverage.

## Acceptance Criteria

- [x] root turn lifecycle behavior is unchanged
- [x] completed spawn followed by child start is accepted
- [x] child activity must match the established child-local turn id
- [x] child completion emits observation without finishing the root
- [x] unknown, stale, cross-operation, and post-terminal children fail closed
- [x] child state remains bounded by the existing 256-child admission limit
- [x] operation termination clears child admission and active child turns
- [x] no authenticated, provider, or consumer effect runs
- [x] Cursor card 013 returns as the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy format:check`
- `git diff --check`
- no broad workspace or authenticated provider suite

## Auto-Continuation

No. Return to Cursor card 013 after deterministic closeout.

## Evidence

- Codex `0.146.0` schema and tagged source identify child-local top-level
  `turn/started` and `turn/completed` around child item activity
- one completed successful spawn admits the child; start binds its separate
  child turn id and completion removes only that child-turn correlation
- child completion, failure, and error do not mutate root terminal state
- lifecycle owner, child-turn mismatch, and post-terminal failures use distinct
  redacted diagnostics
- the frozen corpus carries root spawn completion, child start, child activity,
  and child completion in the consumer-observed ordering
- 142 focused Codex tests passed
- the extracted Codex package compiled
- docs, Northstar, Codex-only format, and scoped diff checks passed
- the workspace format check remains blocked by unrelated concurrent Cursor
  headless files; this card did not rewrite them
- no live provider, authentication, installation, or consumer effect ran
