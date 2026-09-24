---
title: g06.021 — Command Code current-release qualification
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/021-command-code-current-release-qualification.md
queue_dispatch: northstar-queue
queue_approval: "Tom accepted the Chatterbox recommendation on 2026-09-24 to run Command Code currentness as the next lane. Provider-free; no live gate."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.021: move the exact `command-code.npm` point from `1.54.0` to the
official stable current at run time.

## Why It Matters

Command Code is the largest known currentness gap, and Contract 029 no longer
allows a stop to hold a ceiling.

## Current State

The exact `QualifiedOnly` point is `1.54.0` (Research 317) with paid-model live
acceptance bound to it (Research 330). npm `latest` was `1.65.0` at planning.

## Boundaries

Follow `docs/roadmaps/g06/021-command-code-current-release-qualification.md`
and its owned paths. Do not edit the task card's prose or this handoff. Hash and
read downloaded artifacts only. No login, prompt, install, execution, live gate,
release or tag.

## Important Context

A selected-surface change is adapted in this task, not held. Live-derived cells
stay bound to `1.54.0` and gated at the new point. Escalate only a
consumer-visible narrowing.

## Suggested Next Move

Freeze identity and the shipped-tree ledger for every published stable after
`1.54.0`, then classify per hop against the selected surfaces.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-command-code -- --check`,
`effigy validate:focused swallowtail-adapter-command-code`,
`effigy package:verify-affected swallowtail-adapter-command-code`,
`effigy qa:routes`, `effigy qa:docs`, and `git diff --check`. Open one PR for
independent exact-head review. Report hops classified and the final point.
