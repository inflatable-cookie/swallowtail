---
title: Swallowtail flattened-task switchover
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom supplied the Northstar flattened-task switchover authority on 2026-09-09; it explicitly authorizes bounded documentation, planning, instruction, template, and local-checker migration."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Migrate Swallowtail once from milestone-plus-batch-card planning to Northstar's
installed generation-plus-task model under canonical task `g05.038`.

## Why It Matters

Northstar now has one executable planning unit: `gNN.NNN`. Swallowtail still
has four expanded closed generations, active milestone wrappers, 154 nested
cards, legacy templates/checkers, and duplicated frontier state.

## Current State

Swallowtail `main` was clean and synchronized at planning start. All submitted
Swallowtail old-format queue records are done. Desktop Card 323 remains a
separate active record bound to candidate merge `49c9e3b2` and tree
`1a9db127`; it owns no Swallowtail path and may continue unchanged. New
old-format dispatch is suspended.

Canonical task: `docs/roadmaps/g05/038-flattened-task-switchover.md`. It fixes
the closed-generation classifications, preservation boundary, unresolved-card
mapping, mutable paths, review oracle, validation, and stops.

## Boundaries

Do not edit Rust/product/runtime code, provider credentials or calls, queue
plugin state, Desktop Card 323, candidate identity, release state, tags,
publication, consumer pins, or generation numbering. Do not retain aliases or
dual planning authority. Do not modernize logs, closed handoffs, immutable
queue records, or archive prose merely to replace historical nouns.

## Important Context

Use the currently installed Northstar project-refresh, lifecycle-maintenance,
and compile-roadmaps procedures. Freeze the preservation manifest and active
old/new map before deletion. `g01`–`g04` are safely closed; `g05` remains
active. Preserve former cards `005`, `006`, `130`, and `134` as `g05.039`–
`g05.042`. Keep `g05.036` waiting on Desktop exact-tree acceptance without
claiming tag authority.

## Suggested Next Move

Recheck the Swallowtail queue and integration checkout. Freeze the manifest,
then compact closed generations before flattening `g05`. Repair current
instructions/templates/checkers in the same semantic batch and run the full
docs validation only after the coherent migration is assembled.

## Completion Protocol

Meet every `g05.038` acceptance row. Commit and push one reviewable migration,
open one PR, obtain independent exact-head review, merge through the queue, and
synchronize `main`. Return the manifest, mapping, diff inventory, checks,
review/PR/merge identities, retained exceptions, new frontier, and whether
normal dispatch resumed. No tag or product continuation follows automatically.
