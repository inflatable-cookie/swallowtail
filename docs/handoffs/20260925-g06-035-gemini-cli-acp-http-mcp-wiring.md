---
title: g06.035 — Gemini CLI ACP consumer HTTP MCP wiring
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/035-gemini-cli-acp-consumer-http-mcp-wiring.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'Agreed, continue' on the ACP HTTP MCP lane, whose plan was one wiring task per direct-http-candidate route. Provider-free; no live work."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.035: wire the consumer-supplied streamable-HTTP MCP entry into
`gemini-cli.acp` production `session/new`.

## Why It Matters

Research 351 shows the provider accepts it. Wiring is the step before a live
gate can prove `gemini-cli.acp` reaches Longhorn's `agent-control` MCP directly.

## Current State

`gemini-cli.acp` sends `mcpServers: []` today. Research 351 cites the accepted forms,
header path and gates at `0.59.0`. g06.019 wired the same shape for
`opencode.acp` in `crates/swallowtail-adapter-opencode/src/acp/mcp.rs`.

## Boundaries

Follow `docs/roadmaps/g06/035-gemini-cli-acp-consumer-http-mcp-wiring.md` and its owned paths. Do not edit the task card's prose or this
handoff. No contract, other route, core vocabulary change, live session,
release or tag.

## Important Context

Emit `http`, which the provider maps to `httpUrl`, with headers in `requestInit.headers`. The entry is only honoured after `authenticate` completes; an unauthenticated session must fail typed, not drop the entry. Emission is not honouring; the matrix cell stays gated on a live gate.

## Suggested Next Move

Read Research 351's `gemini-cli.acp` evidence and g06.019's encoder, then build the
encoder, the prepared-facade input, and the omission, refusal and redaction
tests before touching guides and matrices.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-gemini -- --check`, `effigy validate:focused swallowtail-adapter-gemini`,
`effigy package:verify-affected swallowtail-adapter-gemini`, `effigy qa:routes`, `effigy
qa:docs`, and `git diff --check`. Open one PR for independent exact-head
review. Report the facade shape and each provider gate's typed outcome.
