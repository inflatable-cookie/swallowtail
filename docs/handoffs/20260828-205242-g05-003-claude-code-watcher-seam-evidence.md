---
title: g05.003 Claude Code watcher seam evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-205242-g05-003-claude-code-watcher-seam-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, research, claude, watchers]
---

## What This Thread Owns

Execute g05.003 card 007 only. Decide whether qualified Claude Code headless
can receive an operation-private watcher skill and tool channel, intercept an
early final completion, and return active-watcher state to the same `-p` model
turn. Produce Research 257 and one reviewable PR.

Start from this file without a copied transcript or second prompt. Do not spawn
internal agents. The operator owns parallelism in their harness.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `7a6fbc584c6bb22449bcf5d950aa850b3302dc62`
- **Worker branch:** `worker/g05-003-claude-watcher-seam`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-003-claude-watcher-seam`
- **Worktree command:** `git worktree add -b worker/g05-003-claude-watcher-seam /Users/tom/Dev/worktrees/swallowtail-g05-003-claude-watcher-seam origin/main`
- **Roadmap:** `docs/roadmaps/g05/003-operation-scoped-watcher-proof.md`
- **Ready card:** `docs/roadmaps/g05/batch-cards/007-claude-code-watcher-seam-evidence.md`
- **Research:** `docs/research/257-claude-code-watcher-seam-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g05-003-claude-code-watcher-seam.md`
- **Contract:** `docs/contracts/059-operation-scoped-process-watchers.md`
- **Parallel lanes:** cards 004 and 008; no shared mutable files
- **Inherited doctor baseline:** `scan.god-files` 381 findings, including 46 errors; stale graph; one generated-in-src warning
- **Required validation:** `effigy validate:focused swallowtail-adapter-claude-agent`; `effigy qa:northstar`; `git diff --check`
- **Merge authority:** not authorized

## Boundaries

- **Allowed files:** card 007, Research 257, assigned log, and unique frozen
  Claude evidence under `crates/swallowtail-adapter-claude-agent`.
- **Out:** production command/code/API, watcher MCP implementation, skill or
  hook injection, Contract 059 edits, cards 009-011, shared planning/index/
  guide/matrix files, live provider work, release, merge, or continuation.
- Use official docs and exact `2.1.220..=2.1.241` package/source evidence.
- No login, credentials, paid work, model prompt, install/update, ambient
  configuration mutation, or watcher process.
- Current Swallowtail passes `--mcp-config {"mcpServers":{}}` plus
  `--strict-mcp-config`. Omission must remain exact.
- Hook observation is insufficient. The route must prove pre-terminal blocking
  and return control to the same model turn.
- An honest empty set is complete and blocks cards 010-011.

## Evidence Questions

1. Can one private MCP server be supplied inline or through an opaque
   operation-scoped host reference without persistent config?
2. Can the watcher instruction asset be delivered without writing user or
   project skill folders?
3. Which Stop or equivalent hook runs before terminal in `claude -p`?
4. Can its response block completion and become model-visible input in the
   same turn? What prevents infinite rejection?
5. How do hook failure, MCP failure, cancellation, deadline, output closure,
   and process join order?
6. Are Claude-native background tasks distinct from the private host watcher?
7. What exact version milestones and omission checks are required?

## Completion Protocol

1. Use the clean non-`main` worktree supplied by the launcher. Stop if it is
   dirty or on `main`; do not stash, reset, or clean user work.
2. Fetch origin. Require the planning base to be an ancestor and this handoff
   to exist in `HEAD`.
3. Read AGENTS.md, the roadmap, card, Research 255/257, Contract 059, Claude
   route evidence, and the assigned log.
4. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; retain the
   inherited doctor result.
5. Freeze decisive sources with exact version/date/digest. Separate parsed,
   dispatched, applied, model-visible, blocking, terminal, and cleanup truth.
6. Complete card 007, Research 257, and the lane log honestly. Do not edit the
   shared batch index.
7. Run required validation, push, and open a PR against current `main`.
8. Report the PR URL. Do not merge or start card 010.
