# 040 Claude Agent 0.64 Claims And Conformance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../015-claude-agent-0-64-standalone-range-maintenance.md`
Depends on: card 039

## Goal

Extend the production claim through exact `0.64.0` while keeping private
behavior, access, lifecycle, callback, and activity truth explicit.

## Acceptance Criteria

- [x] `0.62.0` retains the `0.61.0` behavior
- [x] `0.63.0` and `0.64.0` use named private behavior revisions
- [x] form custom-answer metadata remains lossless
- [x] unadvertised nested transcripts and steering fallback remain unselected
- [x] lifecycle deletion strength and access authority do not widen
- [x] focused Claude and ACP conformance passes

## Evidence

- six exact behavior segments through `0.64.0`; `0.65.0` remains permitted
  and visibly unverified
- focused validation — 157 passed
- extracted package proof — two packages passed in 13 seconds

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-protocol-acp swallowtail-adapter-claude-agent`
- `git diff --check`

## Auto-Continuation

Yes. Continue to card 041 after focused and extracted-package proof passes.
