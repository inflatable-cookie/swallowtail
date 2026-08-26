---
title: g04.076 Cursor headless provider sandbox worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/worktrees/swallowtail-review-pr69/docs/handoffs/20260826-200621-g04-076-cursor-headless-provider-sandbox.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reviewed and fast-forward merged PR 74 at exact head
`b841c947568c5d6308ae5fb0c5f7451be7d1b8d6`, verified all five CI jobs green,
then resumed the sole roadmap continuation. It reassessed the remaining
per-route feature inventory and selected Cursor headless provider sandboxing
as the next material route-local gap.

The orchestrator compiled g04.076, cards 210-212, Research 223, programme and
front-door updates, triage disposition, and the compilation log. The planning
base was validated and fast-forwarded to `main` at
`9afc07eede0ab175a3c8fc4b834448043c356f9b`.

This is one bounded manual implementation thread. Start from this file without
a copied transcript or a second prompt. Do not spawn internal agents; the
operator owns parallelism in their harness.

## Why It Matters

`cursor-agent.headless` always prepares `HarnessIsolation::AmbientHost` and
omits `--sandbox`. All four exact qualified Cursor builds expose
`--sandbox enabled|disabled`, and the route already owns one explicit-model
structured child with exact access, Plan-mode, deadline, cancellation,
activity, terminal, durable-state, and cleanup truth.

This is not yet a containment claim. Cursor documentation describes native
filesystem, network, and subprocess restrictions, but also says sandboxing
applies to supported terminal commands, incompatible commands may move toward
approval, and ambient path/network policy can alter the boundary. Exact
build/platform/configuration evidence must settle whether any row satisfies
Contract 023 without silent outside-sandbox execution, fallback, provider
work, or authority widening.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning base:** `main`
- **Planning commit before this handoff:** `9afc07eede0ab175a3c8fc4b834448043c356f9b`
- **Planning branch:** `orchestrator/g04-076-cursor-headless-sandbox`
- **Planning publication:** planning commit is exact `origin/main` before this
  handoff commit
- **Planning checkout:** clean dedicated orchestrator worktree before this
  handoff file
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates worker-only worktree preflight
- **Planning artifacts:** g04.076, cards 210-212, Research 223 reservation,
  compilation log, programme/triage/index updates, and sole Next Task
- **Worker branch:** `worker/g04-076-cursor-headless-provider-sandbox`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-076-cursor-headless-provider-sandbox`
- **Worktree creation command:** `git worktree add -b worker/g04-076-cursor-headless-provider-sandbox /Users/tom/Dev/worktrees/swallowtail-g04-076-cursor-headless-provider-sandbox origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent; never use `/tmp`, `TMPDIR`,
  or a guessed path for a worktree.
- **Active spec lane:** per-route feature completion programme
- **Roadmap milestone:** `docs/roadmaps/g04/076-cursor-headless-provider-sandbox.md`
- **Ready cards, in order:**
  `210-cursor-headless-provider-sandbox-evidence.md`, then conditional
  `211-cursor-headless-provider-sandbox-binding.md`, then conditional
  `212-cursor-headless-provider-sandbox-acceptance.md`
- **Allowed runway:** execute card 210 and promote Research 223; continue to
  cards 211-212 only for a non-empty exact Cursor
  `HarnessIsolation::ProviderEnforced` row whose full boundary is
  preflight-bindable
- **Remaining card budget:** three cards; stop after card 210 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one reviewable PR; no internal
  agents or subagents
- **Parallel safety check:** serial because evidence decides whether binding
  and acceptance exist and every card touches the same Cursor route
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 023, 029, 033, 037, 040, and 052
- **Model capability profile:** exact artifact/source research plus route-local
  Rust implementation and deterministic conformance
- **Tool/runtime restrictions:** exact official source/artifact evidence and
  secret-free local parser/source work only; no install, update, login, account
  inspection, authenticated catalogue, provider prompt, paid work, model run,
  tool execution, ambient config mutation, or sibling-route work
- **Required validation:** card 210 checks first; if delivery proceeds,
  `cargo fmt -p swallowtail-adapter-cursor`,
  `effigy validate:focused swallowtail-adapter-cursor`,
  `effigy package:verify-affected swallowtail-adapter-cursor`,
  `effigy check:examples`, `effigy package:api`, `effigy qa:northstar`, named
  research/log/roadmap/card/next-action checks, `effigy doctor`, and
  `git diff --check`
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings (334
  warnings, 46 errors); `scan.generated-in-src` reports one warning. Existing
  papercut records cover the structural baseline; record drift and do not add
  duplicates or repair unrelated findings.
- **PR base:** `main`
- **PR head:** worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised; operator must explicitly request it

## Boundaries

Keep this run inside the named runway:

- **In scope:** exact qualified Cursor build/platform/access
  `--sandbox enabled` evidence; conditional
  `HarnessIsolation::ProviderEnforced` preparation, immutable plan/evidence,
  platform/preflight validation, canonical argv, deterministic acceptance,
  route-local docs/matrices/API truth, Research 223, closeout, and Next Task
- **Out of scope:** `--sandbox disabled`, raw sandbox/config strings,
  `sandbox.json` editing, network-mode or extra-path selection, host sandbox
  construction, `--force`, `--yolo`, `--auto-review`, approval response,
  MCP/plugin/cloud-worker/worktree features, Cursor ACP/catalogue, another
  route feature, currentness, release, generation rollover, g04 closure, or
  merge
- Native isolation remains separate from `Read|ReadWrite`, `--mode plan`,
  `--trust`, permissions, tools, `.cursorignore`, working-resource authority,
  and host isolation.
- Omission must retain exact no-flag argv, `AmbientHost`, and current ambient
  configuration truth.
- An admitted profile adds only `--sandbox enabled`. It cannot silently fall
  back, approve an outside-sandbox command, widen paths/network, or infer a
  platform/backend fact after spawn.
- Do not invent architecture, change contracts, widen the roadmap, or choose
  an unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's scope or
  spawn subagents. If shared mutable scope or a hidden dependency appears,
  stop and report it through the operator.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Exact qualified points:** `2026.07.01-41b2de7`,
  `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and
  `2026.08.11-e8db854`. Calendar gaps and later `UnverifiedNewer` points do not
  inherit the result.
- **Exact host:** `/Users/tom/.local/bin/cursor-agent --version` reports
  `2026.08.04-aaa8809`. Its current exact help exposes
  `--sandbox <mode>` with `enabled|disabled`. The prompt-free version/help
  observation is authorized; a model run is not.
- **Current argv:** `--print --output-format stream-json --model <exact>
  --trust`; `Read` adds `--mode plan`; `ReadWrite` adds no mode. Prompt content
  goes through stdin. No sandbox, force, yolo, or partial-output flag is sent.
- **Current route truth:** every run is one owned child; model parameters are
  exact Research 183 rows; configuration is `Ambient`; isolation is
  `AmbientHost`; durable provider state may remain without management
  authority.
- **Current official lead:** Cursor CLI docs name `--sandbox enabled|disabled`
  and say the CLI flag overrides configured mode. Current Run Modes docs
  describe macOS Seatbelt, Linux Landlock/Bubblewrap, subprocess-tree
  restriction, default network policy, ambient `sandbox.json`, and commands
  that cannot run sandboxed moving to approval. These pages are mutable
  corroboration, not exact-build proof.
- **Primary current sources:** `https://cursor.com/docs/cli/overview`,
  `https://cursor.com/docs/agent/security/run-modes`,
  `https://cursor.com/docs/cli/reference/configuration`, and
  `https://cursor.com/blog/agent-sandboxing`.
- **Exact source authorities:** Research 077, 087, and 135 plus their frozen
  installed/official bundles and fixture identities. Research 183 governs
  model-parameter composition.
- **Open tensions:** ambient network/path configuration may widen the boundary;
  exact older bundles may differ from current docs; print mode may deny,
  request, or implicitly handle outside-sandbox commands; backend absence may
  degrade or fall back; prepared evidence may lack an exact platform fact.
- **Decisions and preferences:** only typed `ProviderEnforced`; no provider
  sandbox enum; no raw configuration; no live proof; an empty Research 223 set
  is a valid and expected outcome if any boundary remains open.
- **Report after:** Research 223 and card 210 are complete, or earlier when a
  stop condition fires. If evidence is non-empty, continue through cards
  211-212 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Execute card 210 as one coherent evidence chunk. Begin with the exact bundles'
CLI option parser and sandbox/configuration modules, then freeze platform
backend selection, command approval/escape paths, and the current production
preflight facts. Promote an exact empty or non-empty Research 223 set before
touching production binding.

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
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; ask
   the operator if it is absent. Create a unique worktree and branch under that
   container from `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path
   for a worktree; never clean, reset, stash over, or discard the original
   checkout's dirty state. If the launcher supplied a dirty or `main` worktree,
   stop and report it instead of silently creating a second worktree.
4. Fetch origin. Confirm selected `HEAD` descends from planning commit
   `9afc07eede0ab175a3c8fc4b834448043c356f9b`, confirm this handoff file exists
   in selected `HEAD`, and confirm the worker branch targets current `main`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline rather than repairing unrelated findings.

### While you work

- Execute cards 210-212 in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- Do not spawn agents or subagents. The operator coordinates parallel work in
  their harness.
- Artifact download and extraction may use a disposable `mktemp -d` directory;
  never install or replace the host executable.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, isolation truth remains open, immutable
  platform/configuration binding fails, intent is ambiguous, or scope expands.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the required final validation named above and in the active cards.
2. Update Research 223, cards, roadmap, log, programme, triage, indexes, guide,
   matrices, API baseline, and sole Next Task required by the actual
   disposition.
3. Push the selected worker branch.
4. Open one reviewable PR against `main`. Link the milestone, cards, Research
   223, changed surfaces, exact evidence, validation, and unresolved items.
5. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator reviews the PR against canonical refs, exact base/head, diff,
and checks. When orchestrator and worker share a GitHub identity, formal
self-approval is unavailable. The orchestrator posts the evidence-backed
verdict as a PR comment; that comment is the canonical review record. If
changes are requested, make only those changes on this branch, push, and report
through the operator. The operator must explicitly authorise any merge.

- **Closeout refs:** Research 223; cards 210-212; g04.076;
  compilation/closeout log; Cursor prepared guide; route/feature matrices where
  truth changes; programme, triage, indexes, and sole Next Task

### Handoff closeout

Before calling the runway complete, leave card, roadmap, log, and next-task
state honest. If blocked, record the blocker and stop rather than making the
handoff look more complete than it is.
