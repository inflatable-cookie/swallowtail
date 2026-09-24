---
title: g06.029 — Command Code 1.65.0 live requalification
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/029-command-code-1-65-0-live-requalification.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: 'OpenCode and Command Code work - go for it' — one live attempt on exact command-code 1.65.0 with deepseek/deepseek-v4-flash, including the pinned install of 1.65.0."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.029: repeat the g06.006 live gate on exact `command-code.headless` `1.65.0` with `deepseek/deepseek-v4-flash`.

## Why It Matters

g06.021 moved the exact point to `1.65.0`; live-derived cells stay gated until this gate runs.

## Current State

Host `command-code` is `1.54.0` at `/Users/tom/.local/bin/command-code`. Research 339 holds the `1.65.0` digests. The probe is `tests/live_installed_probe.rs` behind the `live-probes` feature.

## Boundaries

Follow `docs/roadmaps/g06/029-command-code-1-65-0-live-requalification.md` and its owned paths. Do not edit the
task card's prose or this handoff. The provider authority is exactly the one
gate the card describes: one attempt, the named tuple, no rerun or
substitution. No login, contract change, release or tag.

## Important Context

Two earlier operator authorizations were lost to harness bugs, so the harness
proof is a hard precondition, not a formality. Evidence records keep no raw
provider stream, credential, account identifier, session id or private path.

## Suggested Next Move

Install exactly `1.65.0`, verify version and digest, rerun the fake-provider harness tests, then spend the one attempt.

## Completion Protocol

Run `cargo fmt -p swallowtail-adapter-command-code -- --check`, `effigy validate:focused swallowtail-adapter-command-code`,
`effigy package:verify-affected swallowtail-adapter-command-code`, `effigy qa:routes`, `effigy
qa:docs`, and `git diff --check`. Open one PR for independent exact-head
review. Report the exact tuple, the model string, and accepted or typed-stop
outcome.
