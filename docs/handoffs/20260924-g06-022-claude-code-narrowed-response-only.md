---
title: g06.022 — Claude Code 2.1.281 narrowed response-only qualification
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/022-claude-code-2-1-281-narrowed-response-only.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'Yeah, go' — accepting the narrowed response-only guarantee after g06.018 branch 3 and dispatch of g06.022. Provider-free; no live work."
queue:
  capability: complex
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.022: qualify both Claude Code stream-JSON axes through current
official, with response-only under the narrowed built-in-hook guarantee.

## Why It Matters

Claude Code has been held at `2.1.278` since `2.1.280`. Tom ruled that holding
is not acceptable and accepted the narrowed guarantee now recorded in Contract
039.

## Current State

Research 341 froze `2.1.280` and `2.1.281` identity, a nine-plugin built-in
hook ledger at
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.281/builtin-hook-ledger.json`,
and parser findings. Both ceilings are `2.1.278`. The response-only route
already accepts an optional `Read` project location where `CLAUDE.md`
discovery resolves upward (Card 108).

## Boundaries

Follow `docs/roadmaps/g06/022-claude-code-2-1-281-narrowed-response-only.md`
and its owned paths. Do not edit the task card's prose or this handoff. No
contract, watcher, Claude Agent ACP or SDK change. Hash and read downloaded
artifacts only — no execution, login, install, prompt or live session. No
release or tag.

## Important Context

The ruling is in Contract 039 under "Built-in provider hooks — operator ruling
2026-09-24". Swallowtail guarantees what its arguments control; provider
built-ins are disclosed provider surface. Headless moves on its own evidence;
g06.018 wrongly held it with response-only.

## Suggested Next Move

Re-probe latest, test the `instructionFiles` and telemetry opt-out candidates
against the frozen parsers, settle the launch directory, then compile headless
first and the response-only milestone second.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-claude-agent -- --check`,
`effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy qa:routes`, `effigy qa:docs`, and `git diff --check`. Open one PR for
independent exact-head review. Report the ceilings reached, switches pinned or
rejected, and the launch-directory result.
