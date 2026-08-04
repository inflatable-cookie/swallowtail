# 082 Retained Operation Reconciliation Selection

Status: planned
Owner: Tom
Created: 2026-08-04
Milestone: `../032-retained-operation-reconciliation-candidate-gate.md`
Depends on: card 081

## Goal

Select the strongest exact retained-operation reconciliation mapping, or close
the lane honestly when neither candidate passes.

## Scope

1. Compare Gemini and Anthropic evidence against Contract 048.
2. Select only a read-only route with exact durable correlation.
3. Compile a multi-card implementation runway for the selected exact route.
4. Keep rejected routes and their promotion gates explicit in Research 099.
5. Update the g03 checkpoint and sole Next Task.

## Validation

- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- stop implementation planning when both routes fail exact read-only correlation
- stop rather than inventing a generic history scanner or provider router

## Auto-Continuation

Continue into the selected implementation roadmap only when one exact route
passes. Otherwise return to the g03 evidence gate.
