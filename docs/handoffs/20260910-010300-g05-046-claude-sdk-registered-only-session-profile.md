---
title: g05.046 Claude SDK registered-only session profile
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the separate strict registered-tool editing route on 2026-09-10. Chatterbox ruled from that settled direction that no native remainder is acceptable. This prerequisite is provider-free."
queue:
  dependsOn: [0c79d680-4a5e-42cc-a377-7c4e63311b56]
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.046: make a Claude SDK session with registered tools and zero
native SDK tools representable, with explicit working-resource access.

## Why It Matters

Desktop g02.051 cannot build its strict editing route on `v0.4.4`. Every
representable native remainder violates the before-dispatch oracle, while the
current registered carrier is additive and cannot request `ReadWrite` alone.

## Current State

Desktop task `1e7b9baf-9b97-4e49-a6d4-17aa24bbcb5c` is blocked cleanly with
no edits, PR, or provider contact. g05.045 is already reconciling the native
limitation and must land first because it owns the same integration guide.

## Boundaries

Follow `docs/roadmaps/g05/046-claude-sdk-registered-only-session-profile.md`.
Keep ordinary native profiles and additive registration unchanged. Add only the
route-specific structural binding, access propagation, sidecar rendering,
provider-free tests, guide/API docs, and closeout. Do not change versions,
Desktop, credentials, releases, tags, or provider state.

## Important Context

Do not make generic `RegisteredToolEffectPosture` mean filesystem access.
Registered-only must carry explicit `Read` or `ReadWrite`, require a non-empty
qualified registered selection, admit only its carrier spellings, and disallow
all seven native SDK tools. Empty native admission without that binding remains
an early typed failure.

## Suggested Next Move

Start with the invalid-state boundary across profile, preparation, and driver.
Then propagate exact access and prove the sidecar options structurally before
touching documentation.

## Completion Protocol

Run `effigy validate:focused swallowtail-adapter-claude-agent`, `effigy
package:verify-affected swallowtail-adapter-claude-agent`, the exact sidecar
asset tests named by the task, `effigy qa:docs`, `effigy qa:northstar`, and
`git diff --check`. Open one PR for independent exact-head review. Return head,
validation, review, merge, and closeout. Zero provider calls. No candidate,
release, tag, Desktop repin, or live acceptance follows.
