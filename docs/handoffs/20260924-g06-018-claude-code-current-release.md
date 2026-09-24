---
title: g06.018 — Claude Code current-release qualification
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/018-claude-code-current-release-qualification.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'We also need to sort out Claude Code version support - stopping at an older version is not acceptable.' Provider-free adaptation and qualification; no live work."
queue:
  capability: complex
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.018: qualify both Claude Code stream-JSON axes through the official
stable current at run time, adapting the response-only route to the `2.1.280`
`--safe-mode` change.

## Why It Matters

g06.017 recorded a stop at `2.1.278`. Tom ruled that stopping at an older
version is not acceptable, and Contract 029 (No Terminal Stop) now makes this
task the stop's owner. Consumers get no maintained Claude Code point until it
lands.

## Current State

Research 338 names the change: at `2.1.280` safe mode keeps `@builtin` plugin
hook registration where `2.1.278` skipped all of it. The response-only route
pins `--safe-mode` in
`crates/swallowtail-adapter-claude-agent/src/claude_code_response_command.rs`;
headless does not. Both ceilings are `2.1.278`. npm `latest` was `2.1.281` at
planning.

## Boundaries

Follow `docs/roadmaps/g06/018-claude-code-current-release-qualification.md` and
its owned paths. Do not edit the task card's prose or this handoff; the
lifecycle block is hook-owned. No watcher, Claude Agent ACP or SDK change; no
contract edit. Hash and read downloaded artifacts only — no execution, login,
install, prompt or live session. No release or tag.

## Important Context

The card's three response-only branches are the decision tree: inert kept
hooks extend the segment; a real disable switch becomes a pinned
adapter-private milestone from `2.1.280`; a hook that affects the turn with no
switch is prepared as a narrowed claim and escalated. Only branch 3 stops.
Headless is classified on its own evidence.

## Suggested Next Move

Freeze identity through current latest, reproduce Research 338, inventory the
`@builtin` plugins and their hooks, then search the option and settings parsers
for a disable switch before choosing the branch.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-claude-agent -- --check`,
`effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy qa:routes`, `effigy qa:docs`, and `git diff --check`. Open one PR for
independent exact-head review. Report the branch taken, the hops qualified per
axis, and any switch pinned.
