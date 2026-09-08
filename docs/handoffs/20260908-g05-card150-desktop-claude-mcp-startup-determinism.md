---
title: g05 Card 150 Desktop Claude MCP startup determinism packet
kind: northstar-handoff
handoff_mode: external-evidence-packet
status: ready
owner: Swallowtail Chatterbox
created: 2026-09-08
updated: 2026-09-08
tags: [claude-agent-sdk, registered-tools, mcp, startup, provider-free]
---

## What This Thread Was Doing

Treating Card 314's `mcp_server_failed` as a material registered-MCP startup
defect rather than ignoring it as a failed quota probe.

## Why It Matters

The release requires Claude registered MCP/tools. Card 314 never reached
provider readiness or submitted its prompt, so removing MCP would evade the
same route that the final paid suite must qualify.

## Current State

Research 299 freezes PR 177, capsule
`f9cd0b6f73c1ab89d322ae6d15d46808121db180937f42328ecc47dc0e428877`,
and the same exact tuple on which Card 312 previously opened successfully.
Card 139 measured an analogous Cargo target-path replacement race; Desktop's
runner still builds and later spawns the shared `target/debug` courier path.
That mechanism is a hypothesis until reproduced here.

## Boundaries

Provider-free diagnosis and repair only. Reproduce under concurrent build
churn, trace the complete courier/startup chain, and publish an immutable
per-run artifact if Desktop owns the failure. Preserve historical capsules and
all MCP admission, redaction, and cleanup rules. Do not contact Claude, mutate
credit, remove MCP, change Swallowtail production code, qualify cells, or touch
release state.

## Important Context

The provider—not Swallowtail—spawns the declared courier. Therefore the
approved executable must remain stable after prepare returns. A successful
build does not guarantee that a shared Cargo target path remains present or
unchanged. If evidence instead points into Swallowtail, stop with exact files,
invariant, reproduction, and the smallest owner handoff.

## Suggested Next Move

Build a target-churn reproduction around the real wrapper path, retain local
startup evidence outside capsules, then move acquisition to a content-addressed
or otherwise immutable per-run path and prove 24+ registered opens.

## Completion Protocol

Open one Desktop PR for exact-head independent review. Return the reproduced
cause or falsification, accepted head, review, merge, closeout, validation, and
confirmation that no provider ran. No automatic live continuation.
