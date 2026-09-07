---
title: g05 Card 132 Claude real-route packet worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-07
updated: 2026-09-07
handoff_path: /Users/tom/.paseo/worktrees/2ee7rnl8/g05-card132-claude-live-gate-packet/docs/handoffs/20260907-g05-card132-packet-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, claude, live-gate, packet]
---

## Objective

Produce Card 132's missing Desktop hand-off packet. Swallowtail defines the
frozen tuple, commands, capsule, stop conditions, and outcome mapping; Desktop
alone schedules and runs the live gate.

## Current State

Card 125's callable provider-spawned mediated-stdio seam is merged. Its Contract
061 row remains `Unqualified / real_route_gate_pending`. No Card 132 packet
exists, so Desktop cannot schedule the bounded gate.

## Scope

Add one tracked packet under `docs/handoffs/` using the Card 128 packet shape.
Include every tuple component and digest/version authority named by Card 132 and
Contract 063, courier feature build, exact commands, redacted capsule fields,
stop conditions, and pass/failure disposition. No provider call, credentials,
runtime, claim, matrix, tag, release, or machine-specific path.

## Acceptance

Packet is executable by Desktop without inference; pass can qualify only the
exact tuple and two named cells; typed failure moves nothing; authority and
cleanup stop conditions are explicit; docs validation and independent review
pass.

## Stop Conditions

Stop if any exact tuple identity, protocol version, digest, command, or capsule
field cannot be derived from frozen repository evidence. Do not guess or run
the route.

## Validation

Run Card 132 docs/routes/link and diff checks. No live or release selector.

## Completion Protocol

Open one docs-only PR and create one independent cross-model reviewer in this
same workspace. Worker never merges. On merge, notify Chatterbox for Desktop
relay; do not execute the packet.
