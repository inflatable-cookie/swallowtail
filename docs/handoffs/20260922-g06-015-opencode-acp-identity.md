---
title: g06.015 — OpenCode ACP identity and surface freeze
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/015-opencode-acp-identity-and-surface-freeze.md
queue_dispatch: northstar-queue
queue_approval: "Tom directed building the OpenCode ACP route on 2026-09-22, naming OpenCode as the most likely fourth harness for Bovine outside Codex, Claude, and Grok. Evidence and identity only; no claim or implementation authority."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.015: freeze the identity and selected surface of OpenCode's ACP
server and propose the `opencode.acp` route shape, so the implementation lane
can be compiled against evidence.

## Why It Matters

Bovine needs a fourth harness, and the operator names OpenCode as the most
likely one outside Codex, Claude, and Grok. OpenCode ACP is an already
inventoried, deliberately unflattened candidate that was parked only for want of
a named consumer need, which now exists.

The route also carries the seam this programme cares about: ACP puts
`mcpServers` on `session/new`, so a client can declare an MCP server per
session — the same shape that already works for `grok-build.acp`, and therefore
the same path to the production MCP through Longhorn's stdio carrier.

## Current State

Installed `opencode 1.18.18` at `/Users/tom/.opencode/bin/opencode` exposes
`opencode acp — start ACP (Agent Client Protocol) server`, with `--cwd`,
`--pure`, `--log-level`, and loopback host options. The literal `mcpServers`
occurs in the installed payload, so the seam is plausible — but nothing is
frozen, no handshake is recorded, and no `opencode.acp` route, claim, or matrix
row exists. `opencode.http` is a separate qualified route and must stay
unflattened from this one.

## Boundaries

Follow `docs/roadmaps/g06/015-opencode-acp-identity-and-surface-freeze.md`.
Evidence only: no adapter, driver, or route implementation, and no claim,
matrix, or admission change. Do not touch `opencode.http` or OpenCode web
search, Contracts 060 or 063 semantics, or Longhorn. No provider prompt,
inference, login, install, host update, release, or tag. Running the CLI beyond
`--help` and `--version`, or executing a downloaded artifact, is out of scope.

## Important Context

**Identity before claim.** This lane must land evidence and a proposed claim
shape without moving a claim. The implementation lane, g06.016, is compiled
against whatever this lane freezes, so treat the frozen identity as the
contract for that work.

The single most important result is the `mcpServers` handoff: which transports a
client may declare, whether a stdio entry is representable, and whether it is
per session. That decides whether `opencode.acp` can reach the production MCP
through Longhorn's carrier, so settle it explicitly rather than inferring it
from the presence of the literal, and record a typed gap if it cannot be settled
provider-free. A live ACP handshake needs separate operator authorization.

Keep HTTP and ACP facts strictly separate: an `opencode.http` observation is
never evidence for an ACP surface.

## Suggested Next Move

Freeze official and installed identity first, then enumerate the ACP surface
from the frozen payload and published documentation, and settle `mcpServers`
before proposing the claim shape. Report the `mcpServers` finding explicitly in
the ready-for-review summary — it decides whether the next lane is worth
building as planned.

## Completion Protocol

Run `effigy qa:docs`, `effigy qa:routes` (which must be unchanged by an
evidence-only lane), the research/roadmap/lifecycle index checks, and
`git diff --check`. Open one PR for independent exact-head review. Return the
exact-head identity summary, the `mcpServers` finding, and the proposed claim
shape. No provider work, no claim change, no implementation.
