---
title: g06.006 Command Code 1.54.0 paid-model live gate
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/006-command-code-1-54-0-paid-model-live-requalification.md
queue_dispatch: northstar-queue
queue_approval: "Tom subscribed to a Command Code account tier on 2026-09-17 and authorized one paid-model gate with 'Go for it', after the free-model attempt reached the provider and failed the route decoder."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g06.006: one authorized live gate for the exact
`command-code.headless` `1.54.0` point on `deepseek/deepseek-v4-flash`, to
settle the feature and activity cells that depend on live evidence.

## Why It Matters

g05.069 stopped twice. The account first refused every request, including the
advertised-free models, with an account-level `insufficient credits` response.
Tom's subscription cleared that, but the free model
`meituan/LongCat-2.0:free` then reached the provider and failed the frozen
route's decoder with `swallowtail.command_code.headless.malformed_stream`.
That points at the free backend's event grammar rather than the route.
`deepseek/deepseek-v4-flash` is the CLI's own default and the model Research
116 and 118 probed on `1.15.1`, so it is the right test of whether the
malformed stream is a free-backend artefact.

## Current State

The account works. The host carries exact `1.54.0` with
`dist/index.mjs` SHA-256
`157feefa0140e78f060ef2c1f9c50d10de702196ea229dc73db6bbcc39a0bcbb`, and
`~/.commandcode/updates.json` has auto-update disabled. The route starts
correctly on this host. Official stable has moved to `1.54.2`, which is an
observation only.

## Boundaries

Follow `docs/roadmaps/g06/006-command-code-1-54-0-paid-model-live-requalification.md`.
One exact version, one model, one attempt. No second attempt, rerun, fallback,
or model substitution; no install, update, login, host update, consumer
mutation, or catalogue operation. Research 116 and 118 stay immutable and bound
to `1.15.1`.

## Important Context

Frozen-executable hazard, learned in g05.069: this CLI self-updates, and a
model-listing call performed an update that moved the host from `1.54.0` to
`1.54.2`. Before any provider operation, verify `command-code --version` is
exactly `1.54.0` and the entrypoint digest matches. Do not run
`command-code update`, `/update`, or `--list-models`. The adapter already passes
`--no-auto-update`. If either check drifted, stop and report rather than
repairing it. The typed credit-failure path is already observed on this exact
point and is excluded.

## Suggested Next Move

Perform the version-and-digest check, then run the single gate once and record
the exact model string in the observation identity. Retain no raw provider
stream, account identifier, prompt, session id, or private path.

## Completion Protocol

Run the focused Command Code tests, route and activity/feature matrix QA, the
named docs gates, and `git diff --check`. Open one PR for independent exact-head
review. Return head, validation, review, merge, and closeout, with the exact
provider evidence. On acceptance reconcile only the live-derived cells; on a
typed stop preserve the version-bound gate and record the reason.
