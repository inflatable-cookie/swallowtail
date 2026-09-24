---
title: g06.030 — Ollama runtime current-release qualification
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/030-ollama-runtime-current-release.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: approved an Ollama currentness run to v0.34.4. Provider-free."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.030: raise `ollama.runtime` from `0.34.2` to the official GitHub
stable current at run time.

## Why It Matters

Ollama is the last family behind official after today's currentness sweep.

## Current State

Research 342 froze `0.34.2`. Official GitHub `ollama/ollama` latest was
`v0.34.4` at planning. The Maintained window is `0.14.0..=0.34.2` excluding
`0.32.2` and `0.32.10`.

## Boundaries

Follow `docs/roadmaps/g06/030-ollama-runtime-current-release.md` and its owned
paths. Do not edit the task card's prose or this handoff. No install, model
pull, prompt, binary execution, release or tag.

## Important Context

Research numbers collided three times today. Take the next free number and
recheck it against current `main` immediately before push.

## Suggested Next Move

Reproduce Research 342's `0.34.2` hashes, then freeze and classify each later
published hop.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-ollama -- --check`,
`effigy validate:focused swallowtail-adapter-ollama`,
`effigy package:verify-affected swallowtail-adapter-ollama`,
`effigy qa:routes`, `effigy qa:docs`, and `git diff --check`. Open one PR for
independent exact-head review. Report hops classified and the final ceiling.
