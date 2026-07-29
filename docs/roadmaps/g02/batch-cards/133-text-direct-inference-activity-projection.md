# 133 Text Direct Inference Activity Projection

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../039-direct-inference-activity-truth.md`
Depends on: card 132

## Goal

Map exact assistant, reasoning-summary, and tool activity for selected text
direct-inference routes without fabricating harness work.

## Scope

1. Implement only the mapping gaps selected by card 132.
2. Correlate consumer direct-tool continuation with existing exchanges.
3. Preserve provider-owned tool identity where the selected API exposes it.
4. Publish exact route and operation activity profiles.
5. Keep usage, billed cost, rate, quota, request correlation, cache,
   retention, recovery, and cleanup as separate evidence.
6. Run focused direct and attached-runtime conformance.

## Out Of Scope

- commands, file changes, plans, tasks, hooks, or subagents without source
  evidence
- realtime media
- tool execution
- new API routes

## Acceptance Criteria

- [ ] all selected direct mappings match frozen corpora
- [ ] provider and consumer tool ownership remain distinct
- [ ] reasoning summaries exclude private continuation
- [ ] assistant activity and final output remain explicit
- [ ] no direct route claims harness lifecycle
- [ ] all route access, retention, cancellation, and cleanup tests remain green

## Validation

- selected hosted-direct and attached-runtime adapter tests
- direct continuation conformance
- `effigy check:rust`
- `effigy lint:rust`
- `effigy package:api`

## Stop Conditions

- Stop on ambiguous provider display intent.
- Keep identity-only or unavailable disclosure rather than exposing raw data.

## Auto-Continuation

Continue to card 134 after every selected text route passes.

