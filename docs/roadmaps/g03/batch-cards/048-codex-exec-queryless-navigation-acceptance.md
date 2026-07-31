# 048 Codex Exec Queryless Navigation Acceptance

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../018-codex-exec-queryless-navigation-lifecycle.md`
Depends on: card 047

## Goal

Prove the lifecycle-only observation no longer prevents later valid structured
output and close the consumer defect.

## Acceptance Criteria

- [x] the full frozen stream reaches its valid final `agent_message`
- [x] `turn.completed` produces a completed terminal outcome
- [x] structured-output validation is unchanged
- [x] focused Codex validation passes
- [x] the extracted Codex package compiles
- [x] docs, Northstar, format, and diff checks pass
- [x] the fixture and parser fix are committed together
- [x] Soundcheck's ignored Luna/medium review is the explicit next task

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy qa:docs`
- `effigy qa:northstar`
- scoped Codex formatting
- `git diff --check`
- no broad workspace or authenticated provider suite

## Auto-Continuation

No. Return Swallowtail to the g03 evidence gate after recording the Soundcheck
retest handoff.

## Evidence

- the whole-stream fixture retains one content-free search activity identity,
  then exposes the valid JSON proposal and completed terminal outcome
- focused Codex validation: 146 passed
- the extracted Codex package compiled
- docs, Northstar, scoped format, and diff checks passed
- no provider or consumer effect ran
