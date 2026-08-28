---
title: g04.090b Kiro ACP agent-profile evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-180615-g04-090b-kiro-acp-agent-profile-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, kiro]
---

## What This Thread Was Doing

The orchestrator closed g04.089, audited the full feature remainder, and
compiled g04.090 as the final two bounded qualification questions. This lane
owns Kiro ACP agent-profile evidence: card 257, Research 254, its reserved log,
and optional new Kiro-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Official Kiro ACP documentation names optional `--agent`, but the selected
route has no closed profile membership, invalid-name failure, or applied-profile
confirmation. Those must close before selection can become a real feature.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `3d5481590d9c4c7eb087b283856892aedb6ac406`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Planning artifacts included at the base:** g04.089 closeout, remainder audit, g04.090, card 257, Research 254, and reserved lane log
- **Worker branch:** `worker/g04-090b-kiro-acp-agent-profile-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-090b-kiro-acp-agent-profile-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-090b-kiro-acp-agent-profile-evidence /Users/tom/Dev/worktrees/swallowtail-g04-090b-kiro-acp-agent-profile-evidence origin/main`
- **Active programme:** `docs/roadmaps/g04/per-route-feature-completion.md`
- **Roadmap milestone:** `docs/roadmaps/g04/090-residual-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/257-kiro-acp-agent-profile-evidence.md`
- **Research:** `docs/research/254-kiro-acp-agent-profile-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g04-090b-kiro-acp-agent-profile-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set; one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lane A; serial integration Goose then Kiro
- **Parallel safety:** unique card, Research, log, and Kiro package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 006, 014-016, 020, 023, 029, 034, 037, 040, 047, 052; Research 153, 156, 251, and 254; `docs/guides/kiro-acp-prepared-integration.md`
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings: 334 warnings and 46 errors; graph index is stale; one generated-in-src warning
- **Model capability profile:** bounded evidence worker, medium reasoning
- **Tool/runtime restrictions:** frozen official docs and exact qualified package/source artifacts when recoverable plus secret-free local inspection only; no install/update, login, credentials, account inspection, provider prompt, paid work, trust widening, profile mutation, or host mutation
- **Required validation:** `effigy validate:focused swallowtail-adapter-kiro`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 257 exactly; exact `kiro.acp` `2.18.1`; official ACP
  `--agent`; profile namespace and membership, parser, precedence, invalid-name
  failure, application, confirmation, lifecycle, cleanup, omission, Research
  254, and the assigned log.
- **Allowed changed files:** assigned card, Research 254, assigned log, and new
  Kiro-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Kiro chat/headless, effort, cloud
  sessions, model selection, trust-all tools, shared milestone/inventory/
  programme/triage/matrices/indexes/Next Task, currentness, live provider work,
  release, merge, rollover, or g04 closure.
- Do not infer a selected profile from generic `agentInfo`, chat/TUI docs,
  current stable behavior, or unsupported session extensions.
- Do not configure, create, mutate, or persist a host profile to make a row pass.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Production argv is exactly `kiro-cli acp`; official ACP docs show optional
  `kiro-cli acp --agent ...`.
- Exact `2.18.1` archives returned HTTP 403 during Research 251. Treat that as a
  source gate, not permission to substitute current stable `2.20.1`.
- Profile names may be user-owned ambient state. A documented arbitrary string
  is not closed portable membership.
- A non-empty row needs static or observable membership, pre-prompt rejection,
  application, applied-profile confirmation, safe authority, and omission.
- An honest empty set is a valid completion.
- Report after the complete evidence table and source freeze, or immediately on
  a stop condition.

## Suggested Next Move

Start from the frozen official ACP page and exact `2.18.1` identity corpus.
Trace `--agent` parsing, profile resolution, invalid-name behavior, and any ACP
response or update that can confirm the applied profile before first prompt.

## Completion Protocol

### Before you start

1. Read this handoff first. Run `git rev-parse --show-toplevel`, `git branch
   --show-current`, `git status --porcelain`, and `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree supplied by the launcher,
   even if its path or branch differs from the placeholders above. Record the
   actual path/branch and do not create a second worktree for that reason.
3. If the launcher supplied `main` or dirty state, stop and report it. Never
   clean, reset, stash over, or discard user work. Only when the current context
   is otherwise unusable should you inspect the named worktree. If that is also
   unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator if it is absent. Never
   use `/tmp`, `TMPDIR`, or a guessed path.
4. From the selected worktree, fetch origin. Confirm `HEAD == origin/main`,
   `git merge-base --is-ancestor 3d5481590d9c4c7eb087b283856892aedb6ac406 HEAD`
   succeeds, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.090, card 257, Research 254, the lane log, and named refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`.

### While you work

- Execute only card 257 and edit only allowed lane files.
- Freeze sources with final URL/tag, retrieval date, digest, and decisive evidence.
- Separate requested, parsed, configured, dispatched, accepted, effective,
  returned, observed, and persisted truth.
- Stop on missing authority, shared-file need, scope expansion, or a result
  requiring provider work.
- Report meaningful progress through the operator; do not start a nested worker.

### When the assigned runway is complete

1. Run the listed validation.
2. Complete card 257, Research 254, and the assigned log honestly.
3. Push the worker branch and open a reviewable PR against current pushed `main`.
4. Link g04.090, card 257, Research 254, evidence, validation, and unresolved items.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the PR independently and records its verdict on the PR.
If changes are requested, make only those changes on this branch, push, and
report through the operator. The operator must explicitly authorise merge.

- **Closeout refs:** card 257, Research 254, assigned log, g04.090, inventory,
  programme, indexes, and sole Next Task remain orchestrator-owned.

### Handoff closeout

Leave the assigned card, Research record, and lane log honest. If blocked,
record the named blocker and stop rather than making the lane look complete.
