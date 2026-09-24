---
title: g06.037 — Kiro ACP consumer HTTP MCP wiring
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/037-kiro-acp-consumer-http-mcp-wiring.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'Agreed, continue' on the ACP HTTP MCP lane, whose plan was one wiring task per direct-http-candidate route. Provider-free; no live work."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.037: wire the consumer-supplied streamable-HTTP MCP entry into
`kiro.acp` production `session/new`.

## Why It Matters

Research 351 shows the provider accepts it. Wiring is the step before a live
gate can prove `kiro.acp` reaches Longhorn's `agent-control` MCP directly.

## Current State

`kiro.acp` sends `mcpServers: []` today. Research 351 cites the accepted forms,
header path and gates at `2.21.4`. g06.019 wired the same shape for
`opencode.acp` in `crates/swallowtail-adapter-opencode/src/acp/mcp.rs`.

## Boundaries

Follow `docs/roadmaps/g06/037-kiro-acp-consumer-http-mcp-wiring.md` and its owned paths. Do not edit the task card's prose or this
handoff. No contract, other route, core vocabulary change, live session,
release or tag.

## Important Context

Emit `type: "http"` with headers. The provider drops session-injected servers when MCP is governance-disabled and its `initialize` advertisement is unproven, so the route must surface a typed outcome when the entry is dropped and must not claim acceptance from emission. Emission is not honouring; the matrix cell stays gated on a live gate.

## Suggested Next Move

Read Research 351's `kiro.acp` evidence and g06.019's encoder, then build the
encoder, the prepared-facade input, and the omission, refusal and redaction
tests before touching guides and matrices.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-kiro -- --check`, `effigy validate:focused swallowtail-adapter-kiro`,
`effigy package:verify-affected swallowtail-adapter-kiro`, `effigy qa:routes`, `effigy
qa:docs`, and `git diff --check`. Open one PR for independent exact-head
review. Report the facade shape and each provider gate's typed outcome.
