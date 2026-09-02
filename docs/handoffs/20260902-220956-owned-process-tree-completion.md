---
title: Owned process-tree completion evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260902-220956-owned-process-tree-completion.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Implement g05.023 card 057: provider-neutral evidence distinguishing root
process exit from an attested-empty host-owned descendant tree.

## Why It Matters

The Claude Agent SDK route cannot satisfy Contract 019 or merge PR 188 until
the host can prove the entire process tree is gone. Root exit and successful
force-stop requests are not that proof.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `4b722eb2255a2e2319fd55f103d22dbd6456b38e`
- **Pushed main verification:** planning commit containing this handoff must equal `origin/main` before dispatch
- **Planning checkout:** clean before this planning batch
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Planning artifacts included at the base:** g05.023 and cards 057-058
- **Worker branch:** `worker/g05-card057-owned-process-tree-completion`
- **Worker worktree:** launcher-provided branch-off worktree
- **Worktree creation command:** Paseo branch-off from pushed `origin/main`
- **Required sibling worktree links:** none
- **Active spec lane:** Contracts 010 and 019
- **Roadmap milestone:** `docs/roadmaps/g05/023-claude-sdk-shared-lifecycle-prerequisites.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/057-owned-process-tree-completion-evidence.md`
- **Allowed runway:** card 057 only
- **Remaining card budget:** one card
- **Dispatch topology:** sole ready lane; card 058 awaits an operator decision and PR 188 is paused
- **Parallel safety check:** no concurrent mutable overlap; the SDK workspace is preserved but paused
- **Surfaces this lane owns:** runtime process-completion vocabulary, host-local process-tree proof, directly coupled contracts/docs/tests/API baseline, card 057 closeout
- **Integration ownership:** orchestrator owns the later card 058/055 dependency transition
- **Merge ordering:** same-repository PRs merge one at a time
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 010 and 019
- **Review oracle:** card 057
- **Model capability profile:** frontier lifecycle/public-API implementation and falsification
- **Frontier-worker justification:** exceptional reasoning is required to prove process-group emptiness without PID/group reuse or root-exit inference; this is the highest-priority blocker to the operator-requested Claude SDK route and has material lifecycle/public-API consequences
- **Tool/runtime restrictions:** no provider contact, live probes, package installs, release commands, PR 188 edits, or guessed platform claims
- **Required validation:** exact card 057 validation only
- **PR base/head:** current pushed `main` / worker exact head
- **PR URL:** pending
- **Review state:** awaiting implementation and exact-head frontier review
- **Merge path:** orchestrator after accepted review of the current head and passing required checks

## Boundaries

- **In scope:** card 057 only.
- **Out of scope:** close signature/deadline design, Claude SDK adapter/sidecar, Windows job objects, provider work, release work.
- **Outcome shape:** implementation if the concrete mechanism proves the oracle; otherwise evidence-backed stop with no positive tree claim.
- Do not invent architecture, change unrelated contracts, widen the roadmap, or choose the card 058 public-API decision.
- Work only in the clean worker worktree selected by the completion protocol.
- Do not merge the PR.

## Important Context

- **Planning lineage:** PR 188 review comment `5516417399`; g05.022 card 055; Contract 019 descendant-tree invariant.
- **Why this card is ready:** the evidence distinction and stop condition are settled; no operator choice is needed for additive tree evidence.
- **Decisions and preferences:** root exit remains root-only by default; only a concrete host may construct positive tree-empty evidence.
- **Open tensions:** the Unix group-owner implementation may terminate every member without being able to observe emptiness safely; stop if so.
- **Report after:** causal audit plus the first load-bearing positive/negative fixture batch, or the evidence stop.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the worker preflight, read card 057 and Contracts 010/019, then trace the
local host group owner from spawn through termination and final wait before
choosing any public vocabulary.

## Completion Protocol

Before broad reads, run `git rev-parse --show-toplevel`, `git branch
--show-current`, `git status --porcelain`, and `git worktree list --porcelain`.
Accept a clean launcher-provided non-`main` worktree. Fetch origin, require the
planning base to be an ancestor, and verify this handoff's tracked blob matches
the absolute dispatch file.

Read `AGENTS.md`, g05.023, card 057, Contracts 010/019, and the process host
implementation. Work in meaningful batches. Preserve unrelated work. Stop on
a missing proof or scope expansion.

When complete, run the card's validation, falsify the root-exit/descendant-
survives counterexample, update card/log/front doors honestly, push one worker
branch, and open a PR against current pushed `main`. Do not merge. The
orchestrator posts the canonical exact-head review and wakes this same worker
for any revision.
