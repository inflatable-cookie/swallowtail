---
title: g06.033 — Claude Agent ACP consumer HTTP MCP wiring
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/033-claude-agent-acp-consumer-http-mcp-wiring.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'Agreed, continue' on the ACP HTTP MCP lane, whose plan was one wiring task per direct-http-candidate route. Provider-free; no live work."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.033: wire the consumer-supplied streamable-HTTP MCP entry into
`claude-agent.acp` production `session/new`.

## Why It Matters

Research 351 shows the provider accepts it. Wiring is the step before a live
gate can prove `claude-agent.acp` reaches Longhorn's `agent-control` MCP directly.

## Current State

`claude-agent.acp` sends `mcpServers: []` today. Research 351 cites the accepted forms,
header path and gates at `0.79.0`. g06.019 wired the same shape for
`opencode.acp` in `crates/swallowtail-adapter-opencode/src/acp/mcp.rs`.

## Boundaries

Follow `docs/roadmaps/g06/033-claude-agent-acp-consumer-http-mcp-wiring.md` and its owned paths. Do not edit the task card's prose or this
handoff. No contract, other route, core vocabulary change, live session,
release or tag.

## Important Context

Emit `{ type: "http", name, url, headers: [{name, value}] }`. The provider also maps `stdio` and `sse`; emit `http` only. Emission is not honouring; the matrix cell stays gated on a live gate.

## Suggested Next Move

Read Research 351's `claude-agent.acp` evidence and g06.019's encoder, then build the
encoder, the prepared-facade input, and the omission, refusal and redaction
tests before touching guides and matrices.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-claude-agent -- --check`, `effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`, `effigy qa:routes`, `effigy
qa:docs`, and `git diff --check`. Open one PR for independent exact-head
review. Report the facade shape and each provider gate's typed outcome.
