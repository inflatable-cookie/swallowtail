---
title: g06.032 — Research number collision check
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/032-research-number-collision-check.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'Agreed, continue' on the ACP HTTP MCP evidence lane and the research-number collision check."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.032: make `qa:docs` and pre-push refuse a research record whose number collides with a different record locally or on remote `main`.

## Why It Matters

Research numbers collided three times on 2026-09-24 across parallel lanes; one needed a post-merge renumber.

## Current State

`scripts/check-roadmap-number-collision.py` already does this for roadmap task numbers, with a test suite under `scripts/tests/roadmap-number-collision/`. The `328` pair is a historical collision to allowlist.

## Boundaries

Follow `docs/roadmaps/g06/032-research-number-collision-check.md` and its owned paths. Do not edit the
task card's prose or this handoff. Tooling only. Do not renumber existing records. No route code or contracts. No release or tag.

## Important Context

Research numbers collided repeatedly today. Take the next free number and
recheck against current `main` immediately before push.

## Suggested Next Move

Extend or mirror the roadmap check, add cases for cross-lane collision, companions and the `328` allowlist, then wire it into `qa:docs` and pre-push.

## Completion Protocol

Run `effigy qa:docs`, the new test script, the existing `bash scripts/tests/roadmap-number-collision.sh`, and `git diff --check`. Open one PR for independent exact-head review.
