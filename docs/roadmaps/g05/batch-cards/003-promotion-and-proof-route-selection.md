# 003 Promotion And Proof-Route Selection

Status: ready
Owner: Tom
Created: 2026-08-28
Milestone: `../001-harness-skill-and-watcher-surface-inventory.md`
Depends on: card 002; recorded operator decisions

## Goal

Promote settled boundaries and compile only the proof routes admitted by
evidence and operator decisions.

## Recorded Decisions

1. Skill discovery targets the effective skill set visible to the selected
   harness session, including distribution-bundled, operator-installed global,
   and project-local skills. Exact harness evidence is required; ambient
   filesystem scanning is not a substitute.
2. Model and operator controls use separate typed operations against one
   host-owned watcher registry.
3. Consumer projection includes lifecycle, status, and bounded redacted output
   summaries. Raw or continuous logs remain out.
4. Explicit watcher wait pauses the agent turn. A fail-closed completion gate
   rejects successful completion while watchers remain active. Cancellation or
   deadline stops and joins all owned watchers before failure.

## Scope

1. Promote realized structure into architecture and durable rules into
   contracts.
2. Stop any family whose authority or enforcement boundary cannot close.
3. Select at most one skill-discovery proof route and one watcher-enforcement
   proof route when their boundaries are independent and testable.
4. Compile implementation cards only after contract promotion.

## Acceptance Criteria

- [ ] no raw triage or research claim becomes execution authority
- [ ] skill discovery and watcher control remain separate when evidence differs
- [ ] any proof route has bounded scope, acceptance, validation, and stop gates
- [ ] sole Next Task names the selected continuation or an honest stop

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Continue only through newly promoted ready cards.
