---
title: g06.016 — OpenCode ACP route implementation
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/016-opencode-acp-route-implementation.md
queue_dispatch: northstar-queue
queue_approval: "Tom directed building the OpenCode ACP route on 2026-09-22 as Bovine's likely fourth harness. Implementation against the frozen g06.015 identity; no provider work."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.016: build the `opencode.acp` route in
`swallowtail-adapter-opencode` against the identity frozen by Research 337.

## Why It Matters

Bovine needs a fourth harness and OpenCode is the operator's named candidate.
Research 337 found that OpenCode's ACP `session/new` accepts a client-declared
`mcpServers` entry in `stdio`, `http` and `sse` form, advertises
`mcpCapabilities: { http: true, sse: true }`, and connects an `http`/`sse` entry
over StreamableHTTP forwarding declared headers. That means the production MCP
shape is representable without Longhorn's stdio carrier — the first route where
that is true.

## Current State

Research 337 is merged and freezes official `opencode-ai` identity through
`1.18.32`, proves the installed `1.18.18` binary byte-identical to the official
platform package, enumerates the ACP surface, and proposes route id, axis, claim
id, two behavior revisions, and segments. No `opencode.acp` route, claim, or
matrix cell exists yet. `opencode.http` is separate and untouched.

## Boundaries

Follow `docs/roadmaps/g06/016-opencode-acp-route-implementation.md`. Build the
ACP surface and the **stdio** MCP entry that Contract 063 already admits. Do not
wire a URL-plus-header placement into production: it is representable on the
provider but is a new placement shape no contract admits, so model it behind
that gate and leave the admission to the operator. Do not touch `opencode.http`
or web search, Contracts 060/063 semantics, or Longhorn. No provider prompt,
login, install, host update, live ACP session, release, or tag.

## Important Context

Four frozen findings shape the driver and must not be silently dropped:

- `protocolVersion` is not negotiated — OpenCode always answers `1`, so the
  driver must not infer that its requested revision was accepted.
- `opencode acp` loads host plugins unless `--pure` is passed; pin `--pure`
  unless contract review settles otherwise, since plugin loading is not part of
  the selected surface.
- Per-session declarations map onto a name-keyed instance-scoped registration
  with last-write-wins and no unregister on close. Bound declarations to one
  route-owned name or surface the collision honestly.
- The window's only behavior delta lands at `1.18.31`. If you compile one
  revision, compile `opencode.acp-v1.client-mcp-servers-v2` for
  `1.18.31..=1.18.32` and treat `v1` as accepted-but-older.

Live honouring of a remote entry is **not** proven by the frozen evidence — only
representability, mapping, and a connect attempt. So no matrix MCP cell may move
to `Yes` on this lane's evidence, and a live gate is a separate authorization.

## Suggested Next Move

Add the route and driver first, keeping the HTTP route's tests untouched, then
map the surface, then bind the admitted stdio seam, and only then compile the
behavior revision and reconcile the guide and matrices.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-opencode -- --check`, `effigy
validate:focused swallowtail-adapter-opencode`, `effigy
package:verify-affected swallowtail-adapter-opencode`, `effigy qa:routes`,
`effigy qa:docs`, and `git diff --check`. Open one PR for independent exact-head
review. Return the route surface actually built, the placement gate state, and
any matrix cell you deliberately left gated.
