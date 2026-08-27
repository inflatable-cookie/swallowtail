---
title: Papercuts wave 2 CI/path worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-181220-papercuts-wave2-ci-path.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

A fresh papercuts collection still has Swallowtail CI and path bugs that
make unrelated PRs red. The operator approved papercuts wave 2.

You are the Swallowtail implementation worker for this lane. Leave
provider-docs SPA research notes and god-file baseline drift alone.

## Why It Matters

`/var` vs `/private/var` breaks affected-package verification on macOS.
OpenCode's fixture panics on expected BrokenPipe and aborts hundreds of
unrelated tests. Isolated HOME from provider probes steals rustup for
later cargo.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `a647751eb5b0cbbdfa775faa7bd9a74828f0bea4`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Planning artifacts included at the base:** `PAPERCUTS.md`; this handoff.
- **Worker branch:** `worker/papercuts-wave2-ci-path`
- **Worker worktree:** prefer the launcher worktree. Named fallback:
  `/Users/tom/Dev/worktrees/swallowtail-papercuts-wave2-ci-path`
- **Worktree creation command:** only if preflight permits:
  `git worktree add /Users/tom/Dev/worktrees/swallowtail-papercuts-wave2-ci-path -b worker/papercuts-wave2-ci-path origin/main`
- **Worker worktree policy:** use a clean dedicated non-`main` launcher
  worktree. `.agents.local.env` has
  `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`.
- **Active spec lane:** none. Do not continue g04 research cards.
- **Roadmap milestone:** none.
- **Ready work items, in order:**
  1. A `/var` review worktree breaks affected-package path patches
  2. OpenCode cancellation fixture panics on expected broken pipe
  3. Isolated HOME for provider probes steals rustup
  4. `rustfmt --edition 2021` cannot parse this 2024 workspace
- **Allowed runway:** those four items only, one PR.
- **Remaining card budget:** four papercuts.
- **Dispatch topology:** serial inside Swallowtail; parallel with other
  wave-2 repos.
- **Parallel safety check:** no shared files with other wave-2 workers.
- **Canonical refs:** `AGENTS.md`; `PAPERCUTS.md`;
  `scripts/verify-affected-packages.sh`; OpenCode prepared-facade HTTP
  fixture writer.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** use Effigy selectors; do not widen
  god-file baselines; do not change research retrieval method.
- **Required validation:** affected-package verifier with a
  `/var`-aliased path; OpenCode cancellation test does not abort the
  process on BrokenPipe/ConnectionReset; cargo after an isolated HOME
  probe still finds host rustup (or the probe restores HOME); rustfmt
  guidance uses the workspace edition. Focused tests for the fixture.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator review after worker completion
- **Merge authorisation:** absent; do not merge

## Boundaries

- **In scope:** the four items, plus closing them in `PAPERCUTS.md`.
- **Out of scope:** xAI/Copilot/Anthropic/Codex SPA HTML research notes;
  god-file baseline raises from Cline/Gemini/Cursor/Kimi proofs; Pi
  replay hang; DeepSeek stream flake; launcher stale worktree
  registrations (T3); parallel duplicate roadmap cards.
- `/var`: canonicalize generated Cargo patch paths, or reject
  symlink-aliased worktree roots with a clear diagnostic.
- OpenCode: treat expected BrokenPipe / ConnectionReset as success on
  the fixture writer drop path. Do not panic/abort.
- HOME: restore host `HOME` after isolated provider probes, or run later
  cargo/effigy with an explicit host HOME and `env -u GROK_HOME`.
- rustfmt: document or wrap so agents do not pass `--edition 2021` on
  this 2024 workspace.
- Do not merge the PR.

## Important Context

- **Planning lineage:** papercuts wave 2. OpenCode fixture has recurred
  on Stable, not just MSRV.
- **PAPERCUTS.md** on this repo is missing `Possible fix` fields on most
  entries; do not spend the lane reformatting the whole file unless a
  touched entry is easy to complete.
- **Report after:** path canonicalize; OpenCode fixture; HOME restore;
  rustfmt note; then PR.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this file from the top. Run the worktree-safety preflight. Use the
launcher worktree if it is clean, dedicated, and not `main`.

Start with `scripts/verify-affected-packages.sh` path canonicalization.

## Completion Protocol

### Before you start

1. Read this handoff. Then run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it.
3. Only if unusable, use the named worktree, then `.agents.local.env`.
   Never use `/tmp`.
4. Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor a647751eb5b0cbbdfa775faa7bd9a74828f0bea4 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md` and `PAPERCUTS.md` for the four titles.

### While you work

- Commit in meaningful chunks.
- Report through the operator after each item.

### When the assigned runway is complete

1. Run the focused validation named above.
2. Close the four papercuts in `PAPERCUTS.md`.
3. Push the worker branch and open a PR against current pushed `main`.
4. Report the PR URL. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If OpenCode already treats BrokenPipe as non-fatal on this SHA, close
that entry with evidence.
