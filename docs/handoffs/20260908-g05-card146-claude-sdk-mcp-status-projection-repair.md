---
title: g05 Card 146 Claude SDK MCP-status projection diagnosis and repair worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-08
updated: 2026-09-08
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom explicitly approved promotion and queue dispatch of the provider-free MCP-status diagnosis after the Card 145 return on 2026-09-08."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, claude-agent-sdk, registered-tools, mcp-status, provider-free]
---

## What This Thread Was Doing

Applying Card 145's unexpected bounded `mcp_status_invalid` return. Card 146
owns the provider-free diagnosis and repair before any credited live suite.

## Why It Matters

The exhausted-credit diagnostic stopped at MCP-status validation before
provider readiness, so it observed neither quota behavior nor registered-tool
support. Topping up and replaying now could spend the live budget on the same
producer contradiction.

## Current State

Research 297 records Desktop task
`9dc9b50f-5bb2-4912-be7d-a6159e8c758d`, PR 172 accepted head
`8ac46df322bec428e744adb5d22fba3a6ad9a7cc`, review comment `5589754115`,
merge `117e09e02dafd505d9f1d6e2b1380b56bbb22a19`, closeout
`5d7f1681222222d6f1586176c05733c6f1daf7c3`, capsule SHA-256
`81cb0fd6c5a717e9ca66b73c7a746e66870219ebf2c1330081dfd76549608569`,
and exact Swallowtail source
`6a93f1d916945aa2b402df7329dc994570e005c8`. One open ran; no prompt, tool,
control, retry, reconnect, or respawn occurred; cleanup was confirmed.

## Boundaries

Provider-free only. The card's owned and forbidden paths bind. Do not run
native Claude, contact a provider, touch credentials/accounts, edit Desktop,
qualify either cell, alter matrix availability, or perform release work.
Preserve strict MCP configuration, required-server connectivity, mediation,
omission, correlation, and joined cleanup.

## Important Context

The frozen SDK declaration permits optional `serverInfo`, `error`, `config`,
`scope`, and `tools` on `McpServerStatus`. Current sidecar projection rejects
some optional fields, while the fake SDK emits only `name` and `status`. This
is the leading deterministic mismatch, not a settled root cause. Safe repair
may discard declared metadata; it must never expose it or accept an unknown
shape.

## Suggested Next Move

Inspect the exact `0.3.259` declaration and shipped implementation, then build
faithful fake-SDK fixtures for all declared row variants. Prove or falsify the
projection mismatch before editing behavior. If confirmed, make the smallest
safe projection repair and run the card's complete provider-free validation.

## Completion Protocol

Stop at exact-head independent review. Report the exact frozen artifact
evidence, root cause, accepted and rejected row shapes, validation, and whether
the Card 145 tuple is deterministically repaired. The queue owns review,
merge, canonical closeout, and blocker return. Do not launch a live gate, merge,
release, or tag.
