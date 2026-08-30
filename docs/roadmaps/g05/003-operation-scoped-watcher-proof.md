# g05.003 Operation-Scoped Watcher Proof

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-30
Depends on: completed g05.001; Contract 059
Vision tags: process watchers, host authority, consumer activity
Contract refs: 009, 010, 012, 013, 023, 041, 044, 059, 060
Research: 257, 259 promoted; 260 boundary promoted; hard-containment gate superseded
Planning state: cards 007-009 and 014-016 complete; cards 010-011 remain planned behind the orchestrator checkpoint and provider gates

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

- [x] repair and restack PR 117 through revised card 009
- [x] bind host-authorized lifecycle coordination without inferring process
      containment from the default local process service
- [x] require an injected containment backend before process-backed start

The final item records the implementation delivered by card 009. The operator
subsequently rejected hard containment as outside the watcher feature; card 014
owns the pre-1.0 repair.

### Batch 3.3 — Host-Process Supervision Repair

- [x] execute ready card 014 against the ordinary host-local process service
- [x] replace containment-only admission with honest managed-process lifecycle,
      cleanup, and join semantics

### Batch 3.4 — Claude Bridge Transport Evidence

- [x] execute card 015 to settle the host-owned MCP/IPC bridge,
      current-version segment, and live same-turn acceptance gate

### Batch 3.5 — Provider-Neutral HTTP Bridge Core

- [x] execute ready card 016 for the Contract 060 host service, private
      authority, closed HTTP/MCP surface, terminal barrier, and joined cleanup
- [x] return one PR; do not continue into Claude wiring

### Batch 3.6 — Conditional Claude Proof

- [ ] execute card 010 only after card 016 lands and the current Claude segment
      and separately authorized live same-turn gate close
- [ ] execute card 011 for full fail-closed route acceptance and docs

The operator promoted HTTP as the smallest bridge on 2026-08-30. Contract 060
owns the boundary. Card 016 landed the provider-neutral host bridge. No current
Claude version segment or live same-turn proof is qualified yet. Do not start
cards 010-011 until the orchestrator checkpoint and their current-version and
live-provider gates close.

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
- route binding would advertise watcher support without owned, joined host
  process supervision

## Batch Cards

- [007 Claude Code Watcher Seam Evidence](batch-cards/007-claude-code-watcher-seam-evidence.md)
- [008 Portable Watcher Lifecycle Core](batch-cards/008-portable-watcher-lifecycle-core.md)
- [009 Host-Local Watcher Registry](batch-cards/009-host-local-watcher-registry.md)
- [010 Claude Code Watcher Bridge](batch-cards/010-claude-code-watcher-bridge.md)
- [011 Watcher Acceptance And Consumer Projection](batch-cards/011-watcher-acceptance-and-consumer-projection.md)
- [014 Host-Process Watcher Supervision](batch-cards/014-host-process-watcher-supervision.md)
- [015 Claude Code Watcher Bridge Transport Evidence](batch-cards/015-claude-code-watcher-bridge-transport-evidence.md)
- [016 Operation-Scoped Watcher HTTP Bridge Core](batch-cards/016-operation-scoped-watcher-http-bridge-core.md)

## References

- [Contract 059 Operation-Scoped Process Watchers](../../contracts/059-operation-scoped-process-watchers.md)
- [Contract 060 Operation-Scoped Watcher HTTP Bridge](../../contracts/060-operation-scoped-watcher-http-bridge.md)
- [Research 255 Production Harness Census](../../research/255-production-harness-skill-and-watcher-surface-census.md)
- [Research 257 Claude Code Watcher Seam](../../research/257-claude-code-watcher-seam-evidence.md)
- [Research 260 Claude Code Watcher Bridge Transport](../../research/260-claude-code-watcher-bridge-transport.md)
