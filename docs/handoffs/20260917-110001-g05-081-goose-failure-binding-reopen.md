---
title: g05.081 Goose failure-binding reopen
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g05/081-goose-1-50-0-failure-binding-reopen.md
queue_dispatch: northstar-queue
queue_approval: "Tom ruled on 2026-09-17, having delegated the rule choice to the Chatterbox, that the Goose route binds the typed provider-authentication semantics; reopening needs a new behavior revision and the typed failure shape recorded in the prepared guide. Provider-free."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.081: reopen the exact `goose.release` point and land one honest
`goose.acp` claim at the then-current published stable.

## Why It Matters

Goose sits at exact `QualifiedOnly` `1.46.0` because provider-authentication
failure became typed at the hop `1.46.0..1.47.0`. The operator ruling now
admits that typed arm, so the route can advance on a behavior revision that
actually covers it.

## Current State

g05.071 stopped as a typed identity stop and Research 319 froze the exact
baseline plus all four published successors. The failure-binding ruling is
recorded in `docs/roadmaps/standing-lanes.md`. `goose` is absent from this
host and no provider operation is needed.

## Boundaries

Follow `docs/roadmaps/g05/081-goose-1-50-0-failure-binding-reopen.md`. Stay
provider-free and inside `crates/swallowtail-adapter-goose/**`. Do not touch
`goose serve`, HTTP/WebSocket/TLS, desktop, TUI, recipes, ACP provider
adapters, or any release surface, and never run a live ACP session.

## Important Context

The ruling rests on the fact that `1.46.0` returned untyped error text plus
`end_turn`, so no consumer could branch on it. Mint the behavior revision that
covers the typed mapping, record that a chained ACP `AuthRequired` on
`session/new` or `session/prompt` surfaces as `auth_required`, and keep the
claim one exact point. Advertised-only `session/delete`, `recipeParameterScopes`,
and effort menus stay independently gated.

## Suggested Next Move

Re-probe the canonical release channel, re-freeze identity, then confirm the
mapped failure surface across every hop with a fixture that fails if the typed
mapping is removed or reshaped.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-goose -- --check`, `effigy
validate:focused swallowtail-adapter-goose`, `effigy package:verify-affected
swallowtail-adapter-goose`, `effigy qa:routes`, `effigy qa:northstar`, and the
focused Goose fixtures. Open one PR for independent exact-head review. Return
head, validation, review, merge, and closeout. No provider calls.
