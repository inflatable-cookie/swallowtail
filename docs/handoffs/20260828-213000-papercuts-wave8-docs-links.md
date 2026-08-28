---
title: Papercuts wave 8 research link-check worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-213000-papercuts-wave8-docs-links.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

`effigy qa:docs` passed PR 112 while Research 255 linked six nonexistent
contract filenames. `qa:docs:links` only checks a bounded front-door set
and does not inspect the changed research file.

You are the Swallowtail implementation worker. Add changed Markdown, or
the indexed research/log corpus, to a bounded link-check selector. Do
not restore broad child-index churn. Leave SPA research notes and
god-file baselines alone.

## Why It Matters

Promoted evidence can claim canonical authority with broken durable
links, and green CI does not catch it.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `0cf6bd0026f5c45b2c19eb104357b49473c47a09`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave8-docs-links`
- **Worker worktree:** launcher first.
  `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`.
- **Ready work items, in order:**
  1. Docs link QA omits research and lane-log bodies — extend
     `qa:docs:links` (or an adjacent bounded selector) so a changed
     research/log Markdown file is checked. Keep the check bounded
- **Out of scope:** xAI/Copilot/Anthropic/Codex SPA corpus notes;
  god-file warning baselines; launcher stale worktree registrations;
  evidence-download cwd; zsh special variables; Antigravity probes;
  parallel currentness card allocation.
- **Canonical refs:** `PAPERCUTS.md`; `effigy.toml` `qa:docs:links`;
  Research 255 as the motivating miss.
- **Required validation:** a research or lane-log file with a broken
  contract link fails the selector. Front-door `qa:docs` still finishes
  without scanning every child index.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Bounded link coverage, not a restored full-tree docs crawl.
- Do not merge.

## Important Context

- Current `qa:docs:links` is a hardcoded README/AGENTS/CHANGELOG/releases
  list in root `effigy.toml`.
- **Report to:** the operator.

## Suggested Next Move

Read this file, run the worktree preflight, then extend the link selector
to cover changed research/log Markdown.

## Completion Protocol

### Before you start

1. Read this handoff. Run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept a clean dedicated non-`main` registered worktree. Record the
   actual path/branch.
3. Confirm `HEAD == origin/main` and ancestor
   `0cf6bd0026f5c45b2c19eb104357b49473c47a09`.
4. Confirm this handoff exists in `HEAD`.

### When the assigned runway is complete

1. Update `PAPERCUTS.md`. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

Leave SPA and god-file papercuts open.
