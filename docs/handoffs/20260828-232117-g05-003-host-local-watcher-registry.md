---
title: g05.003 host-local watcher registry worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-232117-g05-003-host-local-watcher-registry.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, implementation, host-local, watchers]
---

## What This Thread Was Doing

The orchestrator closed the provider-neutral Contract 059 core through PR 115
and advanced g05.003 to its sole ready implementation card. This worker owns
card 009 only: turn the pure watcher state machine into a host-owned local
lifecycle with real start, output draining, wait, stop, deadline, cancellation,
process-tree cleanup, and joined truth.

This is one bounded implementation handoff. Start from this file without a
copied transcript or second prompt. Do not spawn internal agents; the operator
owns parallelism in their harness.

## Why It Matters

Watcher instructions and provider activity are not dependable process control.
Card 009 supplies the host-owned enforcement layer that later lets Claude's
qualified private MCP/Stop-hook seam expose truthful background-process state
without arbitrary commands, PIDs, raw logs, or work surviving its turn.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `464d45c39fc97a277a754ea6aec1190b9552fd8e`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `464d45c39fc97a277a754ea6aec1190b9552fd8e` before this handoff commit
- **Planning checkout:** clean `main`; only this card clarification and handoff
  are added by the dispatch commit
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** Contract 059, Research 257,
  completed card 008, and ready card 009
- **Worker branch:** `worker/g05-003-host-local-watcher-registry`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-003-host-local-watcher-registry`
- **Worktree creation command:** `git worktree add -b worker/g05-003-host-local-watcher-registry /Users/tom/Dev/worktrees/swallowtail-g05-003-host-local-watcher-registry origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path and branch;
  do not create another worktree for that reason. If the current context is
  unusable, use the named worktree when it matches. Only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree and branch under that container from `origin/main`.
  Ask the operator if the file or key is absent. Never use `/tmp`, `TMPDIR`, or
  a guessed path.
- **Required sibling worktree links:** none
- **Active spec lane:** none; promoted Contract 059 is canonical
- **Roadmap milestone:** `docs/roadmaps/g05/003-operation-scoped-watcher-proof.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/009-host-local-watcher-registry.md`
- **Allowed runway:** card 009 only
- **Remaining card budget:** one card; stop after a reviewable PR
- **Dispatch topology:** serial
- **Parallel safety check:** cards 010-011 depend on this work; the Qoder lane
  is gated by Research 256's honest empty result
- **Canonical refs:** `docs/architecture/system-architecture.md`;
  `docs/contracts/009-async-operation-lifecycle.md`,
  `docs/contracts/010-execution-host-services-and-inputs.md`,
  `docs/contracts/023-harness-operation-isolation-and-native-boundary.md`,
  `docs/contracts/041-input-callback-and-provider-tool-admission.md`,
  `docs/contracts/044-observable-agent-activity-and-disclosure.md`, and
  `docs/contracts/059-operation-scoped-process-watchers.md`
- **Evidence ref:** `docs/research/257-claude-code-watcher-seam-evidence.md`
- **Prior implementation log:** `docs/logs/2026-08-28-g05-003-portable-watcher-core.md`
- **Worker log to create:** `docs/logs/2026-08-28-g05-003-host-local-watcher-registry.md`
- **Model capability profile:** capable coding model, medium reasoning; pause
  for frontier review if concurrency, process-tree ownership, or public API
  ambiguity escapes the card
- **Tool/runtime restrictions:** no subagents, live provider prompts, adapter
  wiring, arbitrary shell/PID authority, release work, or merge
- **Inherited doctor baseline:** 381 god-file findings: 334 warnings and 47
  errors; one generated-in-src warning; do not add a finding
- **Required validation:** `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`;
  `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`;
  `effigy package:api`; `git diff --check`
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and PR review
- **Merge authorisation:** not authorized

## Boundaries

- **In scope:** card 009; its host-local registry, runtime/core correction,
  testkit proof, public API baselines, worker log, and honest roadmap closeout.
- **Allowed implementation surfaces:** `crates/swallowtail-core`,
  `crates/swallowtail-runtime`, `crates/swallowtail-host-local`,
  `crates/swallowtail-testkit`, their unreleased API baselines, card 009, the
  assigned log, and the g05/front-door status files needed to make card 010
  ready after successful closeout.
- **Out of scope:** Claude adapter/MCP/skill/hook wiring, card 010 execution,
  consumer UI, skill discovery, provider-native background-task promotion,
  persistence/recovery, durable daemons, arbitrary process attachment, raw-log
  streaming, release work, or merge.
- Correct the pre-1.0 start seam before binding it: caller input is bounded,
  redacted operation data interpreted under host policy. It grants no launch
  authority. A caller-supplied `WatcherSummary` must not become consumer truth;
  only host-selected progress or terminal summaries may be projected.
- Public records and diagnostics must not carry executable paths, commands,
  arguments, environment, working paths, raw stdout/stderr, PIDs, provider
  payloads, or secrets.
- Reuse the existing `ProcessService`, `ProcessHandle`, `ScopedTaskService`, and
  `TimeService` boundaries. Strengthen local ownership when necessary, but do
  not create a shell, ambient executable search, or global executor.
- Every accepted watcher and its owned descendants must reach terminal and
  joined truth. Dropping a handle or observing root-child exit alone is not
  descendant cleanup evidence.
- Keep state turn-scoped. Foreign, stale, reused, post-terminal, and racing IDs
  fail closed and never stop unrelated work.
- Do not invent architecture, change Contract 059, widen the roadmap, or choose
  provider/consumer policy. Stop on any card stop condition and report it.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  checkout or discard another checkout's dirty state.
- Do not merge the PR. Merge remains a separate operator-authorized action.

## Important Context

- PR 115 merged the split provider-neutral watcher vocabulary, pure registry,
  model/operator roles, optional `WatcherHostService`, activity projection, and
  testkit pack at `464d45c3`.
- `LocalHostServices` currently composes local process, scoped task, and time
  services but does not register a watcher service. `HostServices::with_watcher`
  is optional and registration already proves that it starts nothing.
- `LocalProcessHost` already owns approved launch resolution, bounded stdio,
  stop/force-stop, exit observation, and handle-drop cleanup. Its current
  supervisor explicitly notes that descendants can retain output pipes. Do not
  confuse bounded reader abandonment with process-tree termination.
- Contract 059 requires start acceptance before work, bounded status, explicit
  wait, two stop paths over one registry, first-terminal-wins races, cancellation
  and deadline cleanup, and no successful turn while work remains unjoined.
  Card 009 owns the host lifecycle; card 010 later owns the Claude same-turn
  completion gate.
- The merged start methods take an optional `WatcherSummary`. That is not the
  final Contract 059 start shape: the contract assigns start operation data to
  host interpretation and output summaries to the host. Card 009 deliberately
  owns this pre-1.0 correction with no compatibility shim.
- A safe summary may be lifecycle/classification/count metadata selected by the
  host. Raw or continuous output is never required. If dependable redaction of
  output content is unavailable, retain private bounded capture and expose no
  content rather than weakening the boundary.
- The relevant triage note is already promoted into Contracts 058-059 and the
  g05 proof lanes. It remains historical context, not separate execution
  authority.
- **Report after:** the start-boundary correction and registry shape are stable;
  then after real local lifecycle/race/cleanup fixtures pass; then at PR-ready
  closeout.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Start by accepting the launcher-provided clean worker worktree and verifying
the tracked handoff as described below. Read card 009 and Contracts 010, 023,
and 059 before editing. Then map the existing `LocalHostServices`,
`LocalProcessHost`, `ProcessHandle`, and `WatcherHostService` seams.

Take the start boundary first: replace caller-authored summary input with the
smallest bounded, opaque, redacted operation-data record that lets host policy
accept or reject without exposing process authority. Once that public shape and
its rejection tests are stable, bind the host-local lifecycle and process-tree
cleanup. If process-tree ownership cannot be made truthful on supported local
hosts inside the current contracts, stop and return that exact blocker instead
of weakening the claim.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   broad repository reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not compare them with the placeholder path/branch
   above or create another worktree merely because they differ.
3. If the launcher supplied a dirty or `main` worktree, stop and report it; do
   not silently create a second worktree. Only when the current context is
   otherwise unusable should you inspect the named worktree. If that cannot be
   used, read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
   ask the operator if it is absent before creating a unique fallback under
   that container. Never use `/tmp`, `TMPDIR`, or a guessed path. Never clean,
   reset, stash over, or discard the original checkout's state.
4. In the selected worktree, record this repository-relative handoff path.
   Fetch origin. Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor 464d45c39fc97a277a754ea6aec1190b9552fd8e HEAD`,
   and confirm the handoff exists in `HEAD`. Load it with
   `git show HEAD:docs/handoffs/20260828-232117-g05-003-host-local-watcher-registry.md`.
   If the absolute file differs from that tracked blob, stop. The committed
   `HEAD` copy is canonical.
5. Required sibling worktree links are `none`; make no sibling-path changes.
6. Read `AGENTS.md`, the milestone, card 009, the named contracts, Research
   257, and the prior core log.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   actual inherited doctor baseline; do not run the broad test plan.

### While you work

- Execute card 009 only. Keep commits aligned with the start/API correction,
  host lifecycle implementation, and coherent closeout rather than model turns.
- Prove rejection before work, output draining/backpressure, natural exit,
  explicit wait, graceful then forced stop where authorized, cancellation,
  deadline, first-terminal-wins races, foreign/stale IDs, descendant cleanup,
  and joined task/process release.
- Keep new Rust modules focused. Compare `effigy doctor` with the inherited
  baseline before PR closeout and split new oversized files rather than adding
  structural debt.
- Report each meaningful chunk through the operator with changed files,
  validation actually run, remaining work, risks, and blockers.
- Stop if a contract is missing, the start data needs a provider- or
  consumer-specific command schema, supported-host descendant cleanup cannot be
  proved, scope expands, or validation changes the plan.

### When the assigned runway is complete

1. Run `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`.
2. Run `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`.
3. Run `effigy package:api` and `git diff --check`.
4. Create or update the assigned log with exact lifecycle, cleanup, race,
   structural-health, and validation evidence.
5. Mark card 009 complete. Make card 010 ready without executing it; keep card
   011 planned. Reconcile the milestone, g05 README, batch-card index,
   generation index, and the single `docs/roadmaps/README.md` Next Task.
6. Push the selected worker branch and open one reviewable PR against current
   pushed `main`. In the PR body link the milestone, card, contracts, log,
   changed public/host surfaces, validation, and unresolved items.
7. Report the PR URL and exact head to the operator. Do not merge or begin card
   010.

### Review and merge path

The orchestrator will review the exact PR head against the contracts, card,
diff, public API, process-tree cleanup evidence, structural baseline, and hosted
checks. Because the orchestrator and worker may share a GitHub identity, the
canonical verdict may be a PR comment rather than formal self-approval.
Requested changes are currently `none`; later review findings replace that
state. The operator must explicitly authorize any merge.

- **Closeout refs:** card 009; g05.003 milestone; assigned log; g05 README;
  batch-card and generation indexes; `docs/roadmaps/README.md`

### Handoff closeout

If card 009 completes, leave card 010 as the sole ready next lane and stop. If
it hits a stop condition, record the exact blocker in the card/log and return
to the orchestrator. Do not make card 010 ready on a partial or weakened host
claim.
