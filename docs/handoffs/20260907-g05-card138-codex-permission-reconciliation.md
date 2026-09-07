---
title: g05 Card 138 Codex permission exchange reconciliation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-07
updated: 2026-09-07
handoff_path: /Users/tom/.paseo/worktrees/2ee7rnl8/g05-card138-codex-permission-reconciliation/docs/handoffs/20260907-g05-card138-codex-permission-reconciliation.md
base_required: pushed-main
tags: [coordination, handoff, worker, codex, permissions, feature-matrix]
---

## Objective

Execute Card 138. Settle from frozen Codex evidence and adapter source whether
a consumer can answer an approval request and have that answer reach Codex, or
can only observe it. Make the feature matrix and the Desktop reconciliation
statement agree without weakening either route.

## Current State

- Repository base: pushed `origin/main` at `0120231fe362c04a9bf31fbafb31f7fed177bb67` or a clean descendant.
- Worker branch: `g05-card138-codex-permission-reconciliation`.
- Authority: Card 138, g05.035 dispatch row, Contract 061, frozen Codex corpus,
  current adapter source, and this handoff.
- Known conflict: the combined `codex.app-server; codex.exec` matrix row says
  `permission_exchange=No`, while the Desktop capsule says answerable.

## Scope

In scope: frozen-source and fixture audit; exact anchors for request, consumer
answer, callback response, and provider delivery; separate app-server and exec
findings; the Card 138-owned matrix/reason/research or consumer-correction
surfaces; provider-free tests needed to make the finding load-bearing.

Out of scope: live Codex, credentials, config mutation, runtime redesign,
unrelated matrix cells, provider install, release, tag, or publication.

## Acceptance

1. The answerable-versus-observed result is anchored in frozen evidence and
   shipped source, including the complete response path or its exact absence.
2. App-server and exec are evaluated separately; a combined row never reports
   a stronger capability than its weaker route.
3. The matrix and Desktop-facing statement agree, with a bounded reason and
   evidence citation.
4. Named Card 138 validation passes and an independent cross-model reviewer in
   this same workspace posts a durable exact-head verdict.

## Stop Conditions

Stop if the frozen corpus cannot distinguish the two routes, if correction
requires new product policy or a public runtime operation, or if a live Codex
session would be needed. Report the exact missing artifact or policy question;
do not infer support.

## Validation

Run the selectors named by Card 138, including `effigy qa:routes`, applicable
focused Codex evidence tests, docs QA/index checks, and `git diff --check`. Do
not run provider/live or release selectors.

## Completion Protocol

Work only in this registered worktree. Commit the evidence and correction as
one coherent PR, push, and create exactly one independent cross-model reviewer
in this same workspace. The worker never merges. Report the exact head, finding,
changed consumer surface, validation, and durable review URL.
