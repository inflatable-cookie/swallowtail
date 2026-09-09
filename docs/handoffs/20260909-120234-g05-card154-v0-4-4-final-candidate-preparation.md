---
title: g05 Card 154 v0.4.4 final candidate preparation
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the consolidated final-candidate qualification batch with 'Go for it' on 2026-09-09."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Prepare the final source-only `v0.4.4` candidate after Card 153 closed the
Claude SDK registered-tool producer gap.

## Why It Matters

The parked candidate `0673541d` predates the accepted Grok and Claude repairs.
The live capsules also proved different source trees, so neither can authorize
a tag. One fresh immutable candidate SHA must exist before the final Desktop
gates run.

## Current State

Canonical `main` is clean and pushed at the planning commit carrying Card 154.
Card 153 is complete. The old candidate stays parked and untagged. Card 154 and
the release playbook define the exact owned release surfaces, one-shot prepare
transaction, hosted gate, review, freeze, and stop conditions.

## Boundaries

Do not edit Rust source/tests, run a provider, touch Desktop, change dependency
support, alter an earlier baseline, publish anything, or create/push a tag.
Use the automatic adequate queue pool and independent review. Preserve the
candidate freeze after merge.

## Important Context

The downstream Desktop gate must link the exact Card 154 merge SHA. Review and
hosted CI must prove that SHA or an identical tree. Queue closeout documentation
may advance `main`; it does not change the candidate identity returned by this
task. No provider capsule transfers from a different tree.

## Suggested Next Move

Verify clean pushed `main`, read the release playbook and Card 154, run read-only
status, reconcile the complete candidate content, then consume the single
authorized prepare transaction.

## Completion Protocol

Meet every Card 154 acceptance check. Open one PR, obtain exact-head independent
review and qualifying hosted CI, and leave the worker tree clean. Report the
accepted head, merge SHA, tree IDs, hosted run ID, preparation receipt digest,
and wall clock. Queue orchestration owns merge and closeout. Stop without a tag.
