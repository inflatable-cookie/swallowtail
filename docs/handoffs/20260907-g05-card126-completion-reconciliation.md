---
title: g05 Card 126 completion reconciliation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-07
updated: 2026-09-07
handoff_path: /Users/tom/.paseo/worktrees/2ee7rnl8/g05-card126-closeout-reconciliation/docs/handoffs/20260907-g05-card126-completion-reconciliation.md
base_required: pushed-main
tags: [coordination, handoff, worker, claude, selected-skill]
---

## Objective

Finish Card 126 from current truth. PR 279 already merged its implementation;
verify every acceptance criterion, repair only missing completion surfaces, and
close the card and matrix accurately without duplicating runtime code.

## Current State

Current main contains `ClaudeAgentSdkSessionProfile::with_selected_skill_bundle`,
driver/preparation bindings, sidecar labelled input, fixtures, guide, changelog,
and additive API baseline from merge `3984f23b`. Card status and feature-matrix
cell remain stale.

## Scope

Verify present/absent/tampered/oversize/foreign-reference behavior and the
Contract 061 projection. Update Card 126 status/result, batch index, and the
route's selected-skill matrix/cross/reason surfaces only where current source
proves availability. Runtime edits are allowed only for a concrete unmet Card
126 acceptance criterion. No live provider, unrelated route, tag, or release.

## Acceptance

All Card 126 criteria are evidenced on current main; absence remains unchanged;
labelled input is not user text, ambient instructions, workspace write, or raw
path; selected-skill row and matrix truth agree; exact-head independent review.

## Stop Conditions

Stop if merged code cannot satisfy the card without new product policy or a
live provider call. Report the exact missing seam; do not duplicate PR 279.

## Validation

Run Card 126 focused/affected/API/northstar/docs/routes and diff checks.

## Completion Protocol

Open one reviewable PR. Create one independent cross-model reviewer in this
same workspace. Worker never merges.
