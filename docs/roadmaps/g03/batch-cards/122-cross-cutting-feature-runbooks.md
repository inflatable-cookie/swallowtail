# 122 Cross-Cutting Feature Runbooks

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../042-complete-integration-guide-system.md`
Depends on: card 121

## Goal

Give consumers and operators task-oriented guidance for every portable feature
without repeating route-specific implementation details.

## Scope

1. Add configured-instance, model catalogue, preparation, and route-selection
   guidance.
2. Add ordinary run, session, event, terminal, cleanup, usage, and cancellation
   guidance.
3. Add generation controls, inputs, tools, permissions, questions, resources,
   writes, and search guidance.
4. Add session continuation, management, recovery, and owned-resource guidance.
5. Deepen activity, plan, task-list, subagent, realtime, and failure guidance.

## Validation

- compile consumer examples
- `effigy qa:docs`
- `effigy qa:routes`

## Auto-Continuation

Continue to card 123 when every feature column has one canonical guide owner.

## Completion

- added task-oriented selection/preparation, ordinary lifecycle,
  generation/input authority, and provider-state/resource runbooks
- deepened observable activity with exact plan-mode and task-list replacement
  guidance
- completed failure and validation ownership boundaries
- all 34 feature columns and the named portable configured-instance,
  activity, task-list, subagent, restoration, and failure surfaces now have a
  canonical complete guide owner
- specialist realtime, import, reconciliation, detachment, restoration,
  activity, failure, and validation guides remain canonical rather than being
  duplicated
- `effigy check:examples`, `effigy qa:docs`, and `effigy qa:routes` passed
- no live or authenticated provider work ran
