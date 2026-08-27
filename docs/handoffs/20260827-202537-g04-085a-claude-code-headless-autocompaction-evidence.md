---
title: g04.085a Claude Code headless autocompaction evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-202537-g04-085a-claude-code-headless-autocompaction-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, claude-code]
---

## What This Thread Was Doing

The orchestrator compiled g04.085 as four independent evidence-only lanes.
This lane owns Claude Code headless autocompaction: card 238, Research 237, its
reserved log, and optional new Claude-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Claude Code exposes `--autocompact`, but Swallowtail must know exactly what it
selects, where it applies, and whether the headless route can bind it privately
without turning compaction into a false context-size or token-limit claim.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `4861bbe07a1aaa39dbb243fbbc300f3133496475`
- **Pushed main verification:** planning base equalled `origin/main` before the
  handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.085, card 238, Research 237 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-085a-claude-headless-autocompaction-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-085a-claude-headless-autocompaction-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-085a-claude-headless-autocompaction-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-085a-claude-headless-autocompaction-evidence
  origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable, inspect
  the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.085 lane A
- **Roadmap:** `docs/roadmaps/g04/085-third-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/238-claude-code-headless-autocompaction-evidence.md`
- **Research:** `docs/research/237-claude-code-headless-autocompaction-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-085a-claude-code-headless-autocompaction-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes B, C, D; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Claude package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 024, 029, 037, 040, 041, 047, 052; Research 202, 226, and 233;
  `docs/guides/claude-agent-prepared-integration.md`
- **Model capability profile:** bounded exact-source research and route-local audit
- **Tool/runtime restrictions:** official and exact published package sources
  plus secret-free local inspection only; no install/update, login, credential,
  provider prompt, account inspection, paid work, or host settings mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-claude-agent`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 238 exactly; `claude-code.headless` autocompaction version,
  native value meaning, precedence, application, confirmation, lifecycle,
  omission, Research 237, and the assigned log.
- **Allowed changed files:** assigned card, Research 237, assigned log, and new
  Claude-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, response-only, ACP, Fast, Ultracode,
  spend cap, advisor, permission widening, teams, shared milestone/inventory/
  programme/triage/matrices/indexes/Next Task, currentness, live provider work,
  release, merge, rollover, or g04 closure.
- Do not present autocompaction as context size, output limit, or provider
  session continuity.
- Requested, encoded, accepted, effective, compacted, and usage-observed truth
  remain distinct.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- The exact route is qualified at published points in `2.1.220..=2.1.241`,
  runs read-only Plan mode with fixed tools, and disables session persistence.
- Research 226 froze every published binary in that range and treated
  `--autocompact` as an out-of-lane sibling, not a qualified control.
- A non-empty row needs exact native meaning, version membership,
  operation-private precedence, pre-effect rejection, and honest application
  or confirmation evidence.
- An empty set is correct if ambient state can override selection or effective
  compaction cannot be confirmed without a provider prompt.
- Report after Research 237 and card 238 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Freeze the exact parser, native value meaning, and
precedence across the qualified range before deciding whether a deliver-now row
exists.

## Completion Protocol

### Before you start

1. Read this handoff first. Before broad reads run `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree immediately. Record its
   actual path/branch; do not create a second one because placeholders differ.
3. If current context is unusable, inspect the named worktree. Only if needed,
   use `.agents.local.env` and `AGENTS_WORKTREE_CONTAINER_DIR` for a unique
   fallback from `origin/main`. Ask if absent. Never clean/reset/stash-over a
   dirty checkout or use `/tmp`. If the launcher supplied `main` or dirty state,
   stop and report it.
4. Fetch origin. Confirm `HEAD == origin/main`, the planning base is an
   ancestor, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.085, card 238, Research 237, the lane log, and named
   canonical refs. Then run the cheap repo orientation checks.

### While you work

- Execute only card 238 and edit only allowed lane files.
- Record primary sources with final URL/tag, retrieval date, digest, and the
  decisive bounded evidence. Respect source quotation limits.
- Report meaningful progress with changed files, validation, remaining work,
  risks, and blockers.
- Stop on missing authority, shared-file need, scope expansion, or validation
  that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 238, Research 237, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.085, card 238, Research 237, changed evidence, validation, and
   unresolved items. Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the exact diff and checks. If earlier evidence PRs
land first, restack this branch onto current `main` before fast-forward-only
merge. An evidence-backed PR comment is canonical if self-approval is
unavailable. Merge requires explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared g04.085 and Next Task
state belong to the orchestrator after the evidence PR lands.
