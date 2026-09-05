---
title: g05.030 Card 090 v0.4.1 compatibility audit worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260905-022300-g05-card090-v0-4-1-compatibility-audit.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, release-readiness]
---

## What This Thread Was Doing

This dispatches g05.030 Card 090, the evidence-only compatibility audit from
immutable v0.4.0 through the promoted v0.4.1 source head. The worker must
produce Research 286, fill the card result, and stop at exact-head review.

## Why It Matters

Card 090 freezes the compatibility evidence required before any candidate
preparation or operator-authorized smoke/tag work. It proves the patch class
instead of assuming it and preserves the feature freeze through Card 092.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `55d66a0e506c92d362a22630daa3c6775c284c03`
- **Pushed main verification:** `HEAD == origin/main == 55d66a0e506c92d362a22630daa3c6775c284c03`
- **Planning checkout:** clean at the promoted base before this handoff commit
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** g05.030 roadmap and Card 090
  manifest, with Card 080 and Card 034 merged prerequisites.
- **Worker branch:** `worker/g05-card090-v0-4-1-compatibility-audit`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/2ee7rnl8/g05-card090-v0-4-1-compatibility-audit`
- **Worktree creation command:** `paseo workspace create --isolation worktree --mode branch-off --new-branch worker/g05-card090-v0-4-1-compatibility-audit --base main`
- **Worker worktree policy:** follow Completion Protocol; use the launcher
  worktree and do not create another worktree for review.
- **Required sibling worktree links:** none
- **Active spec lane:** g05.030 v0.4.1 Release Readiness
- **Roadmap milestone:** `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`
- **Ready cards, in order:** Card 090 only
- **Allowed runway:** Card 090 compatibility audit through one reviewable PR
- **Remaining card budget:** one card; stop after the audit PR
- **Dispatch topology:** one worker and one same-workspace independent reviewer
- **Parallel safety check:** feature and currentness merges are frozen; no
  sibling mutable scope is approved.
- **Surfaces this lane owns:** `docs/research/286-*.md`, one appended line in
  `docs/research/README.md`, this card's `## Result`, and append-only
  `PAPERCUTS.md` entries.
- **Integration ownership:** coordinator owns shared closeout surfaces,
  roadmap/index/log updates, merge, and post-merge reconciliation.
- **Merge ordering:** same-repository PRs merge one at a time; refresh against
  current `main` and re-review if the base advances.
- **Canonical refs:** Contract 036, immutable v0.4.0 at `56f3913a`, and card
  050's compatibility-audit precedent; `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`.
- **Review oracle:** one exact tree supports every compatibility statement;
  the smallest counterexample is a changed public item or guaranteed value
  absent from the ledger, or a break classified as compatible.
- **Model capability profile:** evidence-first audit worker with
  `cargo-public-api` and Contract 036 toolchain discipline; no provider access.
- **Worker provider/model identity:** Codex `gpt-5.6-luna`, full-access,
  xhigh reasoning.
- **Frontier-worker justification:** none; this is a bounded evidence audit.
- **Tool/runtime restrictions:** read-only release status; no credentials,
  provider calls, tags, publication, binaries, installers, or consumer writes.
- **Required validation:** `effigy package:api`; `effigy qa:routes`;
  `effigy qa:docs`; `effigy qa:northstar`; `git diff --check`.
- **PR base/head:** current pushed `main`; worker reports the exact head.
- **PR URL:** pending
- **Review state:** awaiting worker PR and same-workspace independent review
- **Merge path:** orchestrator after accepted exact-head review and green checks

## Boundaries

- **In scope:** Research 286 package/dependency/route/semantic-API/
  guaranteed-behaviour ledgers, immutable-baseline comparison, read-only
  release-status evidence, Card 090 result, and the named research index line.
- **Out of scope:** every `crates/**` path; `Cargo.toml`; `Cargo.lock`;
  `CHANGELOG.md`; `release-baselines/**`; `docs/releases/**`; contracts;
  version claims; tags; publication; providers; consumer repositories.
- **Outcome shape:** one evidence-first audit PR, or a precise stop returned to
  Chatterbox if any breaking change, baseline mutation, open mergeable feature
  PR, or wrong release classification is found.
- Do not invent architecture, change contracts, widen the roadmap, or resolve
  the release-classification decision. Any breaking change returns to
  Chatterbox for the minor-version decision.
- Do not merge the PR; merge belongs to the coordinator after its gate.

## Important Context

- **Planning lineage:** v0.4.0 is immutable; Card 080 and Card 034 are merged;
  Card 090 is the sole ready lane in the feature-frozen g05.030 milestone.
- **Decisions and preferences:** no feature or currentness PR may merge until
  Card 092 stops; Cards 091 and 092 require their separately named authority.
- **Open tensions:** any breaking public API or guaranteed-behaviour change is
  an operator/Chatterbox decision, not a worker classification choice.
- **Report after:** the complete ledgers, immutable-baseline proof, release
  status result, and one reviewable PR or a complete stop capsule.
- **Report to:** the operator, who relays progress to the orchestrator.

## Suggested Next Move

Run the Completion Protocol preflight, then inspect the milestone, Card 090,
AGENTS.md, and Contract 036. Audit the exact v0.4.0 peel against the promoted
head, write Research 286 and its index line, fill the card result, run the
named gates, and open one PR. Stop on any breaking classification.

## Completion Protocol

Use the standard Northstar orchestrator Completion Protocol from the template:
verify the committed handoff and canonical base before broad reads, work only
in the selected clean worker worktree, push one PR, and leave merge/review and
reserved closeout surfaces to the coordinator. The reviewer must use a
different underlying provider/model identity in this exact worker workspace.

Feature freeze is active from the promoted planning commit until Card 092
stops. Cards 081-088 and the remaining Contract 061 work stay queued.
