---
title: g05.002 Qoder effective skill roster evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-205241-g05-002-qoder-effective-skill-roster-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, research, qoder, skills]
---

## What This Thread Owns

Execute g05.002 card 004 only. Determine whether Qoder headless `1.1.25`
provides a complete prompt-free effective skill roster through init `skills`
and `plugins`. Produce Research 256 and one reviewable PR.

Start from this file without a copied transcript or second prompt. Do not spawn
internal agents. The operator owns parallelism in their harness.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `7a6fbc584c6bb22449bcf5d950aa850b3302dc62`
- **Worker branch:** `worker/g05-002-qoder-effective-skill-roster`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-002-qoder-effective-skill-roster`
- **Worktree command:** `git worktree add -b worker/g05-002-qoder-effective-skill-roster /Users/tom/Dev/worktrees/swallowtail-g05-002-qoder-effective-skill-roster origin/main`
- **Roadmap:** `docs/roadmaps/g05/002-effective-harness-skill-visibility-proof.md`
- **Ready card:** `docs/roadmaps/g05/batch-cards/004-qoder-effective-skill-roster-evidence.md`
- **Research:** `docs/research/256-qoder-effective-skill-roster-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g05-002-qoder-effective-skill-visibility.md`
- **Contract:** `docs/contracts/058-effective-harness-skill-visibility.md`
- **Parallel lanes:** cards 007 and 008; no shared mutable files
- **Inherited doctor baseline:** `scan.god-files` 381 findings, including 46 errors; stale graph; one generated-in-src warning
- **Required validation:** `effigy validate:focused swallowtail-adapter-qoder`; `effigy qa:northstar`; `git diff --check`
- **Merge authority:** not authorized

## Boundaries

- **Allowed files:** card 004, Research 256, assigned log, and unique Qoder
  fixture/evidence files under `crates/swallowtail-adapter-qoder`.
- **Out:** production code/API, Contract 058 edits, cards 005-006, shared
  roadmap/index/front-door/triage/guide/matrix files, provider prompts, live
  auth, release, merge, or continuation.
- Official docs, exact package source, and existing fixtures are allowed.
- Do not install or run `npx skills`. Trace that workflow from frozen source or
  official evidence and prove how its result reaches Qoder's selected run.
- No login, credentials, paid work, model prompt, ambient home/project scan,
  install/update, or host mutation.
- File presence, manifest membership, and help text are not effective-roster
  evidence.
- An honest empty deliver-now set is complete. It keeps cards 005-006 planned.

## Evidence Questions

1. What exact code populates init `skills` and `plugins`?
2. Are the collections complete for the selected model/run or display-only?
3. Which distribution, global, project, plugin, and unknown sources feed them?
4. Can the frame be obtained before prompt acceptance and model inference?
5. Does initialization require auth, durable provider allocation, or mutation?
6. What are bounds, malformed behavior, ordering, duplicates, freshness, and
   empty semantics?
7. Do model, session, cwd, configuration posture, and route identity bind the
   same observation?

## Completion Protocol

1. Use the clean non-`main` worktree supplied by the launcher. Stop if it is
   dirty or on `main`; do not stash, reset, or clean user work.
2. Fetch origin. Require the planning base to be an ancestor and this handoff
   to exist in `HEAD`.
3. Read AGENTS.md, the roadmap, card, Research 255-256, Contract 058, Qoder
   route evidence, and the assigned log.
4. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor result without repairing it.
5. Freeze sources with exact URL/version/date/digest and separate parsed,
   configured, returned, visible, complete, and inferred truth.
6. Complete card 004, Research 256, and the lane log honestly. Do not edit the
   shared batch index.
7. Run required validation, push the worker branch, and open a PR against
   current `main` linking all evidence and stop gates.
8. Report the PR URL. Do not merge or start card 005.
