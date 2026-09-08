---
title: g05 Card 144 Claude SDK registered-tool open-rejection diagnosis worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-08
updated: 2026-09-08
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom returned the completed Desktop Claude typed-failure capsule and explicitly directed Swallowtail Chatterbox to apply the Card 132 decision tree on 2026-09-08; Card 132 sends typed open failure evidence to the next producer card with no qualification or retry."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, claude-agent-sdk, registered-tools, open-failure, diagnostics, cleanup]
---

## What This Thread Was Doing

Applying Card 132's typed-failure branch after Desktop's single authorized
Claude SDK registered-tool open returned
`open.failed.swallowtail.claude-agent.sdk.open_rejected`. Card 144 owns the
provider-free diagnosis and the missing structured failed-open evidence.

## Why It Matters

The live gate stopped before a turn or tool call, so neither Contract 061 cell
can move. The immutable capsule also cannot distinguish the bounded sidecar
rejection class or positively report cleanup. Another provider attempt without
closing those producer evidence gaps would spend scarce authority on the same
opaque result.

## Current State

Research 296 records Desktop task
`0a594fd5-a966-4192-aeb6-fa98f5493d1d`, PR 170 accepted restacked head
`bd83f6b038151afd7e51ca19b944b22e9db72799`, review comment `5586693748`,
merge `4cf8049762d5e7891d8b1c8c458af0ef85ed97d6`, closeout
`ee47e160ac8e3ec5db1fa5f0444999e5cd630b2d`, capsule SHA-256
`e0460a54776a5644ff2c54bc412a52d81b0d84f04ee34f56971b78181e501433`,
and source-linked Swallowtail
`04e9b2dd9058783b7186a33a6c13dd74882a5925`. The attempt reached open only;
no turn, dispatch, control, or retry occurred. Both cells remain unqualified.

## Boundaries

The card's owned and forbidden paths bind. Provider-free only. Fixture Node
sidecars are allowed; do not run native Claude or contact a provider, touch
credentials/configuration, edit Desktop, qualify a row, change matrix
availability, or perform release work.
Preserve strict MCP, default permission, omission, correlation, and cleanup
contracts.

## Important Context

Swallowtail already retains the bounded sidecar rejection code inside the safe
diagnostic message and internally returns the original open error only after
the open guard reports cleanup complete. Card 132's capsule oracle forbids
inferring either fact, and the generic error exposes no structured receipt.
Card 144 should add an observation-derived route-specific surface rather than
teach consumers to parse prose. Desktop's accepted review hardened future
consumer capture, but it did not diagnose or repair the producer rejection and
did not mutate the immutable capsule.

## Suggested Next Move

Implement Card 144 as one Claude adapter batch. First build the structured
failed-open receipt around the existing open guard and bounded sidecar failure
enum. Then reproduce the exact Card 132 request through frozen fake SDK/sidecar
scenarios. Repair only a concrete deterministic producer defect. If the route
stays irreducibly live-only, return the narrowed boundary without seeking or
spending another provider attempt.

## Completion Protocol

Stop at exact-head independent review. Report exact head, validation, every
provider-free rejection shape, cleanup evidence, and whether a producer defect
was repaired or the cause remains live-only. The queue owns review, merge,
canonical closeout, and blocker return. Do not merge, release, tag, or start a
new live gate from the worker.
