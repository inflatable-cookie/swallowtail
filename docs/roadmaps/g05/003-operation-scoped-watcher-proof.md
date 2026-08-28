# g05.003 Operation-Scoped Watcher Proof

Status: ready
Owner: Tom
Created: 2026-08-28
Depends on: completed g05.001; Contract 059
Vision tags: process watchers, host authority, consumer activity
Contract refs: 009, 010, 012, 013, 023, 041, 044, 059
Research: 257 promoted
Planning state: cards 007-008 complete; card 009 ready; cards 010-011 planned

## Problem

Provider activity does not give Swallowtail watcher control. The portable core
can be built independently, but no production harness may claim support until
one route proves a model-facing tool seam and same-turn completion
interception. Terminal-only failure is insufficient.

## Goal

Build the provider-neutral watcher lifecycle and qualify Claude Code headless
as the only first-route candidate. Add host execution and route binding only
after their independent gates close.

## Execution Plan

### Batch 3.1 — Parallel Core And Route Evidence

- [x] execute ready card 007 for exact Claude MCP, hook, skill, and completion
      evidence
- [x] execute ready card 008 for provider-neutral records, ownership, state
      transitions, control roles, and activity projection

### Batch 3.2 — Host Registry

- [ ] execute ready card 009 after the core lands
- [ ] bind host-authorized start, bounded status and summaries, wait, model and
      operator stop, cancellation, deadline, and joined cleanup

### Batch 3.3 — Conditional Claude Proof

- [ ] execute card 010 only after Research 257 admits the complete seam and
      card 009 lands
- [ ] execute card 011 for fail-closed same-turn acceptance and docs

## Acceptance Criteria

- [ ] watcher ids never become PIDs or provider task ids
- [ ] model and operator controls reach one host-owned registry
- [ ] explicit wait pauses until terminal and joined state
- [ ] early completion returns control to the same model turn
- [ ] consumer events expose status and bounded redacted summaries only
- [ ] every terminal path stops and joins turn-owned work

## Stop Conditions

- Claude `-p` stop hooks cannot block terminal and return control to the model
- watcher MCP or skill injection requires ambient configuration mutation
- the route can only fail after irreversible provider completion
- host start needs arbitrary executable, shell, or PID authority in public data
- raw output is required for correctness

## Batch Cards

- [007 Claude Code Watcher Seam Evidence](batch-cards/007-claude-code-watcher-seam-evidence.md)
- [008 Portable Watcher Lifecycle Core](batch-cards/008-portable-watcher-lifecycle-core.md)
- [009 Host-Local Watcher Registry](batch-cards/009-host-local-watcher-registry.md)
- [010 Claude Code Watcher Bridge](batch-cards/010-claude-code-watcher-bridge.md)
- [011 Watcher Acceptance And Consumer Projection](batch-cards/011-watcher-acceptance-and-consumer-projection.md)

## References

- [Contract 059 Operation-Scoped Process Watchers](../../contracts/059-operation-scoped-process-watchers.md)
- [Research 255 Production Harness Census](../../research/255-production-harness-skill-and-watcher-surface-census.md)
- [Research 257 Claude Code Watcher Seam](../../research/257-claude-code-watcher-seam-evidence.md)
