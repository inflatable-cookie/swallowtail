---
title: g06.019 — OpenCode ACP consumer HTTP MCP wiring
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/019-opencode-acp-consumer-http-mcp-wiring.md
queue_dispatch: northstar-queue
queue_approval: "Tom accepted the Chatterbox recommendations on 2026-09-24, admitting the consumer-supplied URL-plus-header MCP placement (Contract 063) and its opencode.acp wiring. Provider-free implementation; no live work."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.019: wire the Contract 063 consumer-supplied streamable-HTTP MCP
entry into `opencode.acp` production `session/new`.

## Why It Matters

This makes `opencode.acp` the first harness route that connects straight to
Longhorn's Contract 022 `agent-control` MCP with no stdio carrier.

## Current State

g06.016 built the route. `crates/swallowtail-adapter-opencode/src/acp/mcp.rs`
models `OpenCodeAcpRemoteMcpPlacement` and refuses production encoding with
`swallowtail.opencode.acp.mcp_remote_not_admitted`. Contract 063 section
"Consumer-Supplied HTTP MCP Placement — 2026-09-24" now admits the shape and
sets its rules. Research 337 froze the provider side.

## Boundaries

Follow `docs/roadmaps/g06/019-opencode-acp-consumer-http-mcp-wiring.md` and
its owned paths. Do not edit the task card's prose or this handoff. No contract,
`opencode.http`, core vocabulary, or Longhorn change; stop and ask if one is
needed. No live session, release or tag.

## Important Context

Emit `http` only; keep `sse` modelled. One consumer entry per session under the
route-owned name. Values pass verbatim and never surface in failures, `Debug`,
activity, receipts or fingerprints. Emission is not honouring: remote-call
cells stay gated.

## Suggested Next Move

Replace the refusal with the encoder, extend the prepared facade, then write
the refusal and redaction tests before touching guides and matrices.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-opencode -- --check`,
`effigy validate:focused swallowtail-adapter-opencode`,
`effigy package:verify-affected swallowtail-adapter-opencode`,
`effigy qa:routes`, `effigy qa:docs`, and `git diff --check`. Open one PR for
independent exact-head review. Report the facade shape and the cells left gated.
