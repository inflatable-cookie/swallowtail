---
title: g05 Card 154 v0.4.4 final candidate preparation
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: merged
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

## Handoff Closeout

Card 154 completed with PR 300. The exact reviewed candidate head was
`1fb5b16ceccbc1451c6b044dc2b8fabd13a194c6`; review comment `5601625469`
accepted it. The PR merged as `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`.
The base tree was `4dc9be748adc09cfe1d1d01ff9fb710ff44bb18a`; the candidate
and merge tree was `1a9db12742b68e839dfebe42f0633f5d1aa0265b`.

Read-only status inferred patch `0.4.4`; the one authorized prepare passed all
cheap gates; hosted run `34345060452` passed all 11 jobs at the exact candidate
tree; and the preparation receipt digest was
`04a89847e14fc351bbcfdef2b48cb1d8c90282b3ea638db48043d1e8f6ab8feb`.
The worker reported approximately 47 minutes of wall clock from 12:08 to
12:55 +0100 through candidate handoff. A later same-SHA rerun
(`34348374964`) failed only the Pinned MSRV floor test twice. The qualifying
run and local validation stayed green, so that nondeterministic validation
failure is deferred without changing the candidate or consuming another
prepare transaction.

No tag or release publication followed. The dependent Desktop acceptance gate
must use merge SHA `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`; tag creation still
requires a separate exact-SHA operator decision.
