---
title: g05.069 Command Code 1.54.0 live requalification
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom re-confirmed on 2026-09-17 that the separately authorized Command Code exact-1.54.0 live gate still stands. It covers one exact version and one attempt only."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.069: run the authorized live requalification for the exact
`command-code.headless` `1.54.0` point and settle the feature and activity
cells that depend on live evidence.

## Why It Matters

g05.068 advanced the exact `QualifiedOnly` point to `1.54.0` provider-free, but
Research 116 and 118 stay bound to `1.15.1`, so the completed/tool/usage/credit
and private-continuation cells remain version-gated rather than proven on the
shipped point.

## Current State

The exact `1.54.0` point, the byte-identical entrypoint, and the 67-hop ledger
are frozen. Tom re-confirmed the provider-operation authority on 2026-09-17.
The gate itself has not run.

## Boundaries

Follow `docs/roadmaps/g05/069-command-code-1-54-0-live-requalification.md`. One
exact version, one attempt, no rerun. No broad provider exploration, install,
login, host update, consumer mutation, or public catalogue, import, export, or
resume operation outside the gate. Research 116 and 118 are immutable
comparison evidence and must not be rewritten.

## Important Context

Reproduce the accepted `1.15.1` evidence shape: authenticated structured
completion, tool lifecycle, usage accounting, the credit-failure path, and the
Contract 043 two-turn private exact-id continuation. Keep `QualifiedOnly`, the
exact tuple, and private exact-id resume unchanged. On a typed stop, preserve
the existing version-bound gate and record the reason.

## Suggested Next Move

Fix the exact executable, account, model, prompt, and cleanup evidence before
any provider operation, then run the single gate once.

## Completion Protocol

Run the focused Command Code tests, route and activity/feature matrix QA, and
the named docs gates. Open one PR for independent exact-head review. Return
head, validation, review, merge, and closeout, with the provider evidence
attached. No provider work beyond the one authorized gate.
