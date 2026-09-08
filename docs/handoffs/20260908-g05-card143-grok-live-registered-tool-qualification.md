---
title: g05 Card 143 Grok live registered-tool qualification worker handoff
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
queue_approval: "Tom returned accepted Desktop Grok capsules and explicitly directed Swallowtail Chatterbox to apply the Card 128 decision tree on 2026-09-08; that tree sends accepts_client_mcp to producer implementation without another operator decision."
tags: [coordination, handoff, worker, grok, acp, mcp, registered-tools, selected-skill]
---

## What This Thread Was Doing

Applying Card 128's `accepts_client_mcp` branch after Desktop returned accepted
live capsules for exact Grok Build `1.0.4` and `1.0.5`. Card 118 already built
the callable route-local seam; Card 143 converts the completed live gate into
exact route qualification and settles the separate selected-skill gap.

## Why It Matters

Grok MCP/tools/skills are the remaining Grok producer obligation in the
operator's three-route Desktop release scope. The route now has real admission,
discovery, invocation, terminal, and cleanup evidence. Leaving the projection
`Unqualified` would discard accepted evidence; treating the echo call as skill
delivery would overclaim it.

## Current State

Research 295 records Desktop task
`10bdda2b-d395-4dc1-baaa-5e625a6286e5`, PR 169 accepted head
`77bced726a40b3a5dc74917aa4e4b518c8e15972`, review comment `5586043120`,
merge `6b22c82946fcd04d418ee2a810e73970373b7888`, closeout
`c495b6a60976c4618ab8f50c820265849bfe3735`, and source-linked Swallowtail
`04e9b2dd9058783b7186a33a6c13dd74882a5925`. Both exact segments returned
`accepts_client_mcp`; hashes and version receipts are in Research 295. Card 118
is complete at `98543d0b`, but its projection still deliberately returns
`Unqualified` with `real_route_gate_pending`.

## Boundaries

The card's owned and forbidden paths bind. No consumer edit, live route run,
credential/configuration mutation, version extension, tag, release, parity, or
unrelated matrix change. Preserve the separate provider-owned permission
channel and Contract 063 kernel. Omission remains byte-identical.

## Important Context

The live capsules prove MCP admission, `tools/list`, `tools/call`, a completed
`end_turn`, and joined cleanup. No stale callback arrived, so
`stale_callback_rejected: false` is not a failed rejection assertion. The
capsules did not carry a selected skill. The runtime already distinguishes
`RegisteredToolPermissionStrength::NotRepresented`,
`RegisteredToolProgressMode::NoProgress`, and selected-skill delivery. Use
those exact dimensions rather than borrowing Grok's provider-owned one-shot
permission exchange.

## Suggested Next Move

Implement Card 143 as one reviewed Grok-adapter batch. Qualify the existing
courier from Research 295, then inspect frozen ACP/Grok evidence for a distinct
selected-skill input. Implement it only if it stays separate from user text and
ambient files; otherwise freeze the limitation while still landing the three
independently proved MCP/tool cells. Run every validation named by the card.

## Completion Protocol

Stop at exact-head independent review. Report exact head, validation, the three
live-evidence cell dispositions, and the independent selected-skill result.
The queue owns review, merge gate, canonical closeout, and any blocker return
to Chatterbox. Do not merge, tag, release, or touch Desktop from the worker.
