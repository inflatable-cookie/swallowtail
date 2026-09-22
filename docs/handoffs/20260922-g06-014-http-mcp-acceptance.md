---
title: g06.014 — Consumer-supplied HTTP MCP acceptance per route
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/014-consumer-supplied-http-mcp-acceptance-per-route.md
queue_dispatch: northstar-queue
queue_approval: "Tom directed Swallowtail to match the Longhorn 2026-09-22 production MCP boundary, and Longhorn accepted the stdio-carrier requirement and gated its acceptance evidence on this per-route list. Operator direction 2026-09-22.",
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.014: establish per route whether a consumer-supplied
streamable-HTTP MCP server entry is accepted, and publish the route list that
decides which harnesses connect directly to the consumer's production MCP and
which need Longhorn's stdio carrier.

## Why It Matters

Longhorn's `7ea44d23` makes the Contract 022 `agent-control` server the
production MCP and withdraws the registered-tool bridge's production role, so
Swallowtail's production role narrows to harness-side MCP client configuration.
Longhorn is building a stdio carrier for that single server instance, in `0.2.0`
scope, dispatched as queue task `34c0ed5c-c9a3-4005-8fd8-d6ba64343b56`. Its
worker validates against the routes this lane names. The list therefore decides
what the carrier has to satisfy, and it is wanted as soon as it is real.

## Current State

Contract 063 and Spec 014 now record the withdrawn production role, the Contract
022 production MCP, and Swallowtail's narrowed role, at merge `6176c851`. The
only MCP client surfaces Swallowtail admits today are two Swallowtail-owned
stdio couriers — `claude-agent.sdk` and `grok-build.acp` — both non-production.
Every other route's `client_mcp_servers` cell is a provider limitation. No route
is proven to accept a consumer-supplied HTTP entry.

## Boundaries

Follow `docs/roadmaps/g06/014-consumer-supplied-http-mcp-acceptance-per-route.md`.
Evidence and matrix truth only. Do not implement a connector, carrier, client,
listener, registry or lease. Do not change a route's admission, Contracts 060 or
063 semantics, or Longhorn code. No provider prompt, login, install, host
update, release, or tag.

## Important Context

The question is acceptance of a consumer-supplied entry, not the existence of an
MCP config seam: a route may document `mcpServers` yet honour only stdio, or
ignore an HTTP entry silently. Classify by what the frozen artifacts and
published documentation actually prove, and record a typed gap where acceptance
cannot be settled provider-free rather than inferring it.

`subscriptions/listen` and the `longhorn://agent-control/...` resources are
deliberately not claimed in this pass; record them as typed unsupported with a
reopen condition unless a named route needs them. Longhorn asked to be told if
any route does.

The two existing Swallowtail-owned stdio couriers are not evidence of HTTP
acceptance and must be classified separately.

## Suggested Next Move

Start from the seam inventory in Contracts 012, 017, 041 and 063 plus each
route's prepared guide, then classify transport acceptance from frozen provider
artifacts. Publish the route list before reconciling matrix cells, so the list
can be handed to Longhorn without waiting on the rest.

## Completion Protocol

Run the named docs gates (`effigy qa:docs`, `effigy qa:routes`), the focused
selectors for any crate whose guide or cells change, and `git diff --check`.
Open one PR for independent exact-head review. Return the exact-head route list
in the ready-for-review summary so Chatterbox can relay it to Longhorn without
reading the diff.
