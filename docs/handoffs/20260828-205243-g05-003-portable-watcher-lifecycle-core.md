---
title: g05.003 portable watcher lifecycle core worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-205243-g05-003-portable-watcher-lifecycle-core.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, implementation, runtime, watchers]
---

## What This Thread Owns

Execute g05.003 card 008 only. Implement the provider-neutral Contract 059
identity, lifecycle, ownership, model/operator control, activity, and pure
state-machine core. Do not start processes or select a provider route.

Start from this file without a copied transcript or second prompt. Do not spawn
internal agents. The operator owns parallelism in their harness.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `7a6fbc584c6bb22449bcf5d950aa850b3302dc62`
- **Worker branch:** `worker/g05-003-portable-watcher-core`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-003-portable-watcher-core`
- **Worktree command:** `git worktree add -b worker/g05-003-portable-watcher-core /Users/tom/Dev/worktrees/swallowtail-g05-003-portable-watcher-core origin/main`
- **Roadmap:** `docs/roadmaps/g05/003-operation-scoped-watcher-proof.md`
- **Ready card:** `docs/roadmaps/g05/batch-cards/008-portable-watcher-lifecycle-core.md`
- **Lane log:** `docs/logs/2026-08-28-g05-003-portable-watcher-core.md`
- **Contract:** `docs/contracts/059-operation-scoped-process-watchers.md`
- **Parallel lanes:** cards 004 and 007; no shared mutable files
- **Inherited doctor baseline:** `scan.god-files` 381 findings, including 46 errors; stale graph; one generated-in-src warning
- **Required validation:** `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`; `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-testkit`; `git diff --check`
- **Merge authority:** not authorized

## Boundaries

- **Allowed files:** card 008, assigned log, and files under
  `crates/swallowtail-core`, `crates/swallowtail-runtime`, and
  `crates/swallowtail-testkit`. Package-local Cargo metadata may change only if
  required by this implementation.
- **Out:** host-local process execution, adapters, Claude/Qoder evidence,
  injected skill or tool transport, shared contracts/architecture/roadmaps/
  indexes/guides/matrices, consumer UI, release, merge, or continuation.
- Do not add a new crate or umbrella registry.
- Public records must not contain executable paths, commands, arguments,
  environment, PIDs, raw output, or provider payloads.
- Keep model and operator requester identity distinct over one registry state.
- The core represents wait and completion gating; it does not invent an
  executor, auto-wait policy, or provider interception.
- Avoid a god trait. Follow existing object-safe role and optional host-service
  patterns.

## Required Shape

1. Turn-scoped opaque watcher identity and activity correlation.
2. Bounded accepted, running, terminal, and joined state with monotonic
   revisions and exact terminal causes.
3. Bounded redacted summaries and safe default formatting.
4. Pure transitions for accepted start, running, completion, failure,
   cancellation, timeout, stopped, joined, repeated stop, and races.
5. Separate model and operator control roles with foreign/stale-id rejection.
6. Object-safe runtime host-service registration and no registered-service
   behavior.
7. Existing ordered turn activity projection for host-owned watcher state.
8. Testkit assertions covering bounds, ownership, races, wait representation,
   cancellation, deadlines, and join truth.

## Completion Protocol

1. Use the clean non-`main` worktree supplied by the launcher. Stop if it is
   dirty or on `main`; do not stash, reset, or clean user work.
2. Fetch origin. Require the planning base to be an ancestor and this handoff
   to exist in `HEAD`.
3. Read AGENTS.md, the roadmap, card 008, Contract 059, and related Contracts
   009, 010, 012, 023, 041, and 044.
4. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; retain the
   inherited doctor result.
5. Inspect existing identity, lifecycle, host-service, activity, and testkit
   patterns before editing. Keep the batch cohesive.
6. Complete card 008 and the assigned log. Do not edit the shared batch index
   or ready card 009.
7. Run required validation, push, and open a PR against current `main` with a
   concise public-surface and lifecycle summary.
8. Report the PR URL. Do not merge or start card 009.
