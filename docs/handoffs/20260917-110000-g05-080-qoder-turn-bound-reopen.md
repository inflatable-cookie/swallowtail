---
title: g05.080 Qoder turn-bound reopen
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g05/080-qoder-1-1-52-turn-bound-reopen.md
queue_dispatch: northstar-queue
queue_approval: "Tom ruled on 2026-09-17, having delegated the rule choice to the Chatterbox, that the Qoder route accepts a declared turn bound but not an inherited one; reopening needs an explicit deliberate --max-turns value and the recorded error_max_turns shape. Provider-free."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.080: reopen the exact `qoder.package` point and land one honest
`qoder.headless` claim at the then-current published stable.

## Why It Matters

Qoder sits at exact `QualifiedOnly` `1.1.25` while 27 published stable
successors have moved past it. The route's own `--max-turns 8` argv stopped
being inert at `1.1.30` and became the real AgentLoop ceiling, so the claim
cannot move until that bound is declared deliberately and documented.

## Current State

g05.070 stopped as a typed identity stop and Research 318 froze the exact
baseline plus every successor with a mutation-sensitive ledger. The
turn-binding ruling is recorded in `docs/roadmaps/standing-lanes.md`.
`qodercli` is absent from this host, and no prompt, login, or credential is
needed.

## Boundaries

Follow `docs/roadmaps/g05/080-qoder-1-1-52-turn-bound-reopen.md`. Stay
provider-free and inside `crates/swallowtail-adapter-qoder/**`. Do not touch
Qoder ACP, SDK, TUI, or IDE surfaces, skill visibility, the g05.039/g05.040
gate, or any release surface.

## Important Context

The ruling explicitly refuses to bless the inherited `8`. Choose the value
deliberately and justify it against the route's bounded-limit posture, prove
with a fixture that the argv value is what bounds the AgentLoop, and record the
chosen bound with the `error_max_turns` terminal shape in the prepared guide.
Commit identity before the claim, and keep exactly one exact point: do not
retain `1.1.25` as a second point to hide incomplete evidence.

## Suggested Next Move

Re-probe npm stable and dist-tags, re-freeze identity for the current stable,
re-confirm the `1.1.30` hop, then pin the bound with its fixture before
touching documentation.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-qoder -- --check`, `effigy
validate:focused swallowtail-adapter-qoder`, `effigy package:verify-affected
swallowtail-adapter-qoder`, `effigy qa:routes`, `effigy qa:northstar`, and the
focused Qoder fixtures. Open one PR for independent exact-head review. Return
head, validation, review, merge, and closeout. No provider calls.
