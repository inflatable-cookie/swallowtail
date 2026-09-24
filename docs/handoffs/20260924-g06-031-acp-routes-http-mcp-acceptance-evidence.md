---
title: g06.031 — ACP routes HTTP MCP acceptance evidence
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/031-acp-routes-http-mcp-acceptance-evidence.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'Agreed, continue' on the ACP HTTP MCP evidence lane and the research-number collision check."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.031: settle, from frozen artifacts, which remaining ACP routes accept a consumer-supplied streamable-HTTP MCP entry.

## Why It Matters

Each route that does can reach Longhorn's `agent-control` MCP without the stdio carrier; `opencode.acp` proved the pattern (Research 349).

## Current State

Research 336 left `cline.acp`, `copilot-cli.acp`, `gemini-cli.acp`, `goose.acp`, `kiro.acp` and `deepagents.acp` as producer gaps; `claude-agent.acp` sends an empty server list; `grok-build.acp` was stdio-only at `1.0.4`/`1.0.5` and is now maintained through `1.0.41`. Research 337 is the method template.

## Boundaries

Follow `docs/roadmaps/g06/031-acp-routes-http-mcp-acceptance-evidence.md` and its owned paths. Do not edit the
task card's prose or this handoff. Hash and read artifacts only. No route code, no matrix cell moving to available, no login, install or live session. No release or tag.

## Important Context

Research numbers collided repeatedly today. Take the next free number and
recheck against current `main` immediately before push.

## Suggested Next Move

Follow Research 337's method route by route: `mcpCapabilities`, accepted `mcpServers` forms, header forwarding, gating flags. Write one classification row per route with its citation.

## Completion Protocol

Run `effigy qa:docs`, `effigy qa:routes`, and `git diff --check`. Open one PR for independent exact-head review. Report the classification table.
