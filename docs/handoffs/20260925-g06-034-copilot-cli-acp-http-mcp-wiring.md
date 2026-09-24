---
title: g06.034 — Copilot CLI ACP consumer HTTP MCP wiring
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/034-copilot-cli-acp-consumer-http-mcp-wiring.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'Agreed, continue' on the ACP HTTP MCP lane, whose plan was one wiring task per direct-http-candidate route. Provider-free; no live work."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.034: wire the consumer-supplied streamable-HTTP MCP entry into
`copilot-cli.acp` production `session/new`.

## Why It Matters

Research 351 shows the provider accepts it. Wiring is the step before a live
gate can prove `copilot-cli.acp` reaches Longhorn's `agent-control` MCP directly.

## Current State

`copilot-cli.acp` sends `mcpServers: []` today. Research 351 cites the accepted forms,
header path and gates at `1.0.80`. g06.019 wired the same shape for
`opencode.acp` in `crates/swallowtail-adapter-opencode/src/acp/mcp.rs`.

## Boundaries

Follow `docs/roadmaps/g06/034-copilot-cli-acp-consumer-http-mcp-wiring.md` and its owned paths. Do not edit the task card's prose or this
handoff. No contract, other route, core vocabulary change, live session,
release or tag.

## Important Context

Emit `http` with a route-owned unique name. The provider rejects stdio client entries and skips names that collide with agent-configured servers, so a colliding name must be a typed refusal or a reserved-name guarantee, never a silent skip. Emission is not honouring; the matrix cell stays gated on a live gate.

## Suggested Next Move

Read Research 351's `copilot-cli.acp` evidence and g06.019's encoder, then build the
encoder, the prepared-facade input, and the omission, refusal and redaction
tests before touching guides and matrices.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-copilot-cli -- --check`, `effigy validate:focused swallowtail-adapter-copilot-cli`,
`effigy package:verify-affected swallowtail-adapter-copilot-cli`, `effigy qa:routes`, `effigy
qa:docs`, and `git diff --check`. Open one PR for independent exact-head
review. Report the facade shape and each provider gate's typed outcome.
