---
title: g04.075 Qwen headless Plan mode worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/worktrees/swallowtail-review-pr69/docs/handoffs/20260826-172259-g04-075-qwen-headless-plan-mode.md
base_required: pushed-stacked-planning-branch
tags: [coordination, handoff, worker, pr, stacked]
---

## What This Thread Was Doing

The orchestrator reviewed g04.074 through exact head
`f22f2f9c41c3a4b8d67f4f0e46d5431b61f77560`. That PR is review-clean but
unmerged. Updated-head CI run 32988163157 passed four jobs and failed one
unrelated OpenCode cancellation test on a fixture-server `BrokenPipe`; the
exact failed test passed locally unchanged and the failed job was requeued.

The orchestrator then followed the roadmap continuation rule, reassessed the
remaining per-route feature inventory, and selected Qwen headless fixed-
argument Plan as the next serial evidence-first lane. It compiled g04.075,
cards 207-209, Research 222, the programme/front-door updates, triage
disposition, and compilation log on a planning branch stacked over PR 73.

This is one bounded manual implementation thread. Start from this file without
a copied transcript or a second prompt. Do not spawn internal agents; the
operator owns parallelism in their harness.

## Why It Matters

`qwen.headless` hardcodes `--safe-mode --approval-mode default`. Exact
maintained Qwen Code points also parse `plan`, and the route already owns each
run or turn child, one read-only working resource, explicit read-tool and
write/process/tool filters, model/reasoning/budgets, cancellation, deadline,
and joined cleanup.

That is a credible fixed-argument path to portable `HarnessMode::Plan`, not a
claim. Exact evidence must prove complete Plan behavior and immutable
reapplication across structured runs, reasoning-control children, later turns,
resume, and fresh replacement. Parser presence, argv dispatch, prompt text, or
tool absence alone is insufficient.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Parent PR:** [PR 73](https://github.com/inflatable-cookie/swallowtail/pull/73)
- **Parent branch:** `t3code/review-headless-model-selection`
- **Parent exact head:** `f22f2f9c41c3a4b8d67f4f0e46d5431b61f77560`
- **Parent review state:** review-clean; not merged; updated-head CI retry
  queued after an unrelated OpenCode cancellation-test `BrokenPipe`
- **Planning branch:** `orchestrator/g04-075-qwen-headless-plan-mode`
- **Planning commit before this handoff:** `c51c3c80`
- **Planning ancestry:** parent PR exact head, then g04.075 compilation
- **Planning checkout:** clean dedicated orchestrator worktree before this
  handoff commit
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates worker-only worktree preflight
- **Planning artifacts:** g04.075, cards 207-209, Research 222 reservation,
  compilation log, programme/triage/index updates, and sole Next Task
- **Worker branch:** `worker/g04-075-qwen-headless-plan-mode`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-075-qwen-headless-plan-mode`
- **Worktree creation command:** `git worktree add -b worker/g04-075-qwen-headless-plan-mode /Users/tom/Dev/worktrees/swallowtail-g04-075-qwen-headless-plan-mode origin/orchestrator/g04-075-qwen-headless-plan-mode`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from the pushed planning
  branch. Ask the operator first if the file or key is absent; never use
  `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** per-route feature completion programme
- **Roadmap milestone:** `docs/roadmaps/g04/075-qwen-headless-plan-mode.md`
- **Ready cards, in order:** `207-qwen-headless-plan-mode-evidence.md`, then
  conditional `208-qwen-headless-plan-mode-binding.md`, then conditional
  `209-qwen-headless-plan-mode-acceptance.md`
- **Allowed runway:** execute card 207 and promote Research 222; continue to
  cards 208-209 only for a non-empty exact Qwen Plan row with complete behavior
  proved across every selected child shape
- **Remaining card budget:** three cards; stop after card 207 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one stacked reviewable PR; no
  internal agents or subagents
- **Parallel safety check:** serial because evidence determines whether the
  binding and acceptance cards exist and every card touches the same Qwen route
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 012, 023, 029, 033, 034, 037, 040, and 052
- **Model capability profile:** exact-source research plus route-local Rust
  implementation and deterministic conformance
- **Tool/runtime restrictions:** exact official source/artifact evidence and
  secret-free local parser/source work only; no install, login, account
  inspection, live catalogue, provider prompt, paid work, tool execution,
  ambient config mutation, live session, or sibling-route work
- **Required validation:** card 207 checks first; if delivery proceeds,
  `cargo fmt -p swallowtail-adapter-qwen`,
  `effigy validate:focused swallowtail-adapter-qwen`,
  `effigy package:verify-affected swallowtail-adapter-qwen`,
  `effigy check:examples`, `effigy package:api`, `effigy qa:northstar`, named
  research/log/roadmap/card/next-action index checks, `effigy doctor`, and
  `git diff --check`
- **Inherited doctor baseline:** 379 findings: 333 warnings and 46 errors,
  plus one generated-in-src finding; record drift, do not repair unrelated
  findings
- **PR base:** `t3code/review-headless-model-selection` while PR 73 remains
  open; restack onto current `main` only after PR 73 lands
- **PR head:** worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised; operator must explicitly request it

## Boundaries

Keep this run inside the named runway:

- **In scope:** exact `0.21.15`, `0.22.0`, and `0.22.1`
  `--approval-mode plan` evidence; conditional portable `HarnessMode::Plan`
  preparation, immutable plan/evidence, validation, every-child argv binding,
  deterministic acceptance, route-local docs/matrices/API truth, Research 222,
  closeout, and Next Task
- **Out of scope:** `default|auto-edit|auto|yolo` as new public values, raw
  approval strings, writable profiles, tool-policy selection, provider sandbox
  claims, model/reasoning/budget changes, catalogue, attachments, schema,
  search, credentials, currentness, release, generation rollover, g04 closure,
  or merge
- Plan is provider behavior, not permission or containment. Preserve and keep
  separate `--safe-mode`, core/excluded tools, read-only working-resource
  authority, `Ambient`, and `AmbientHost`.
- Omission must retain exact `--approval-mode default` bytes and behavior.
- Every structured run, reasoning-control child, first/later turn, explicit
  resume, and fresh replacement must reapply the same immutable selection or
  fail before spawn.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's scope or
  spawn subagents. If shared mutable scope or a hidden dependency appears, stop
  and report it through the operator.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or an unrelated dirty checkout.
- Do not merge either PR. Merge remains a separate operator-authorised action.

## Important Context

- **Exact maintained points:** Research 173 freezes `0.21.15`; Research 216
  qualifies exact `0.22.0..=0.22.1` on the same selected mapped revision.
  Unpublished `0.21.16`, deprecated points, and later `UnverifiedNewer` points
  are not automatically included.
- **Parser lead:** the `0.22.1` corpus records shared `APPROVAL_MODES` values
  `plan|default|auto-edit|auto|yolo`; Swallowtail currently dispatches
  `default`.
- **Current argv:** input text or reasoning-control stream JSON, output
  stream JSON, partial messages, `--safe-mode`, `--approval-mode default`,
  exact model, fixed read tools, explicit deny tools, 60-second wall time, and
  adapter-held turn/tool budgets.
- **Current route truth:** model and route are preflight-bound; reasoning is
  exact and model-qualified where selected; every turn starts a joined child;
  continuation privately reuses only the provider session id; configuration
  remains `Ambient` and isolation remains `AmbientHost`.
- **Open tensions:** Plan may be prompt/policy state rather than a complete
  immutable behavior; safe mode or config precedence may shadow it; provider
  output may not confirm it; later-turn/resume construction may lose it; mode
  switching or slash/workflow/subagent paths may widen behavior.
- **Decisions and preferences:** only portable `HarnessMode::Plan`; no new
  provider-mode enum; omission remains explicit `default`; no authority claim
  from a restrictive tool list; no live provider proof.
- **Report after:** Research 222 and card 207 are complete, or earlier when a
  stop condition fires. If evidence is non-empty, continue through cards
  208-209 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Execute card 207 as one coherent evidence chunk. Start with the exact parser,
approval config, safe-mode/tool construction, and every child command path.
Promote the exact empty or non-empty Research 222 set before touching
production binding.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare it with the placeholders or create
   another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; ask the
   operator if it is absent. Create a unique worktree and branch under that
   container from `origin/orchestrator/g04-075-qwen-headless-plan-mode`. Never
   use `/tmp`, `TMPDIR`, or a guessed path; never clean, reset, stash-over, or
   discard the original checkout's dirty state. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. Fetch origin. Confirm the selected `HEAD` descends from planning commit
   `c51c3c80`, confirm parent PR exact head
   `f22f2f9c41c3a4b8d67f4f0e46d5431b61f77560` is an ancestor, and confirm this
   handoff file exists in selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline rather than repairing unrelated findings.

### While you work

- Execute cards 207-209 in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- Do not spawn agents or subagents. The operator coordinates parallel work in
  their harness.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, Plan behavior remains open, immutable
  reapplication fails, intent is ambiguous, or scope expands.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the required final validation named above and in the active cards.
2. Update Research 222, cards, roadmap, log, programme, triage, indexes, guide,
   matrices, API baseline, and sole Next Task required by the actual
   disposition.
3. Push the selected worker branch.
4. Open one reviewable stacked PR against
   `t3code/review-headless-model-selection` while PR 73 remains open. The PR
   must include the g04.075 planning and handoff commits plus the worker lane.
   If PR 73 lands before PR creation, rebase/restack onto fetched `main` and
   target `main` instead. Never merge a stale base into the worker branch.
5. Link the parent PR, milestone, cards, Research 222, changed surfaces,
   evidence, validation, and unresolved items in the PR body.
6. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator reviews the PR against canonical refs, exact parent/head,
diff, and checks. When PR 73 lands, the worker PR must be fast-forward
restacked onto current `main` before merge review.

When orchestrator and worker share a GitHub identity, formal self-approval is
unavailable. The orchestrator posts the evidence-backed verdict as a PR
comment; that comment is the canonical review record. If changes are
requested, make only those changes on this branch, push, and report through the
operator. The operator must explicitly authorise any merge.

- **Closeout refs:** Research 222; cards 207-209; g04.075;
  compilation/closeout log; Qwen headless guide; route/feature matrices where
  truth changes; programme, triage, indexes, and sole Next Task

### Handoff closeout

Before calling the runway complete, leave card, roadmap, log, and next-task
state honest. If blocked, record the blocker and stop rather than making the
handoff look more complete than it is.
