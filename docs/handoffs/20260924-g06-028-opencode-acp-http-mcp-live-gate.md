---
title: g06.028 — OpenCode ACP HTTP MCP live gate
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/028-opencode-acp-http-mcp-live-gate.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'OpenCode and Command Code work - go for it' — one live attempt proving opencode.acp honours a consumer-supplied streamable-HTTP MCP entry on installed exact 1.18.18."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.028: prove that `opencode.acp` honours a consumer-supplied streamable-HTTP MCP entry: one live attempt on installed exact `1.18.18`.

## Why It Matters

g06.019 wired the entry; this gate proves OpenCode connects and calls a tool, which makes `opencode.acp` a direct-HTTP route to Longhorn's `agent-control` MCP.

## Current State

Host `opencode` is `1.18.18` at `/Users/tom/.opencode/bin/opencode`, byte-identical to the official artifact (Research 337). The production encoder landed in g06.019. No live probe or test MCP server exists yet.

## Boundaries

Follow `docs/roadmaps/g06/028-opencode-acp-http-mcp-live-gate.md` and its owned paths. Do not edit the
task card's prose or this handoff. The provider authority is exactly the one
gate the card describes: one attempt, the named tuple, no rerun or
substitution. No login, contract change, release or tag.

## Important Context

Two earlier operator authorizations were lost to harness bugs, so the harness
proof is a hard precondition, not a formality. Evidence records keep no raw
provider stream, credential, account identifier, session id or private path.

## Suggested Next Move

Build the feature-gated probe and the disposable loopback MCP server, prove the harness against a fake ACP agent in a committed test, confirm the host version and a usable configured model, then spend the one attempt.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-opencode -- --check`, `effigy validate:focused swallowtail-adapter-opencode`,
`effigy package:verify-affected swallowtail-adapter-opencode`, `effigy qa:routes`, `effigy
qa:docs`, and `git diff --check`. Open one PR for independent exact-head
review. Report the exact tuple, the model string, and accepted or typed-stop
outcome.
