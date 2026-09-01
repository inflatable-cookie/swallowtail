---
title: Live-probe temporary-workspace cleanup papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-100306-papercuts-live-probe-cleanup.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Swallowtail's orchestrator is processing the project-owned PAPERCUTS queue
serially. The next entry says failed Claude watcher live-probe assertions can
bypass temporary-workspace cleanup. Current `main` appears to have repaired
that defect already through g05.006 card 019; this lane must verify the repair
is load-bearing, then reconcile the stale papercut honestly.

This dispatches one bounded implementation lane. No transcript or second prompt
is part of the authority chain.

## Why It Matters

An opt-in live probe must not retain temporary state when setup, provider, or
assertion paths fail. The queue must also distinguish open defects from fixes
already proved and merged.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `dc6f180f7c48dded7ffae1e3452eae9dc32db71b`
- **Pushed main verification:** local `main` and `origin/main` both resolved to
  `dc6f180f7c48dded7ffae1e3452eae9dc32db71b` before this handoff commit
- **Planning checkout:** clean
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** g05.006 card 019, its merged
  panic-safe workspace owner and credential-free proof, and the open
  `PAPERCUTS.md` entry dated 2026-08-30
- **Worker branch:** `worker/papercuts-live-probe-workspace-cleanup`
- **Worker worktree:** launcher-generated Paseo worktree
- **Worktree creation command:** Paseo `create_workspace`, worktree branch-off
  from pushed `origin/main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** none; bounded repository papercut reconciliation
- **Roadmap milestone:** `docs/roadmaps/g05/006-watcher-proof-repair.md`
- **Ready cards, in order:** verify the already-complete
  `docs/roadmaps/g05/batch-cards/019-watcher-proof-oracle-and-activity-delivery-repair.md`
  cleanup acceptance, then close or correct the matching PAPERCUTS entry
- **Allowed runway:** the 2026-08-30 live-probe temporary-workspace cleanup
  papercut only
- **Remaining card budget:** one bounded papercut
- **Dispatch topology:** serial; no other worker may edit `PAPERCUTS.md` or
  `docs/logs/README.md` until this lane reaches review or stop
- **Parallel safety check:** Card 032 closeout remains queued because it shares
  log and front-door surfaces
- **Canonical refs:** g05.006, card 019, current live watcher probe, and its
  credential-free watcher proof; Contracts 059-060
- **Review oracle:** cleanup ownership must exist before provider contact and
  before every fallible assertion. The smallest counterexample removes or
  delays the owner so an assertion panic leaves the directory. The required
  proof is a credential-free caught-panic test plus exact source ordering in the
  ignored live probe.
- **Model capability profile:** bounded papercut worker selected from current
  Paseo profile notes
- **Tool/runtime restrictions:** no provider contact, no live Claude command,
  no live-probe execution, no new provider authorization
- **Required validation:** the exact cleanup proof test; compile the ignored
  live-probe target without running it; `effigy validate:focused
  swallowtail-adapter-claude-agent`; warranted docs checks; `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator exact-head review
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** verify current-main cleanup semantics and proof; if the defect
  is already fully fixed, close the PAPERCUTS entry with exact merged evidence.
  If proof or source ordering is incomplete, implement the smallest complete
  repair and prove it.
- **Out of scope:** watcher route admission, fresh live evidence, provider
  contact, Contracts 059-060 changes, activity projection, isolation flags,
  Card 032 closeout, and any other PAPERCUTS entry.
- **Outcome shape:** fix or verified stale-entry reconciliation, with one
  reviewable PR. A diagnostics-only stop is valid only if current authority or
  source contradicts the bounded repair.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Stop if shared mutable scope or a
  hidden dependency appears.
- Work only in the clean worker worktree selected by `Completion Protocol`.
- Do not merge the PR. Merge belongs to the orchestrator.

## Important Context

- **Planning lineage:** prototype `49f2692f` left an empty workspace after a
  failed assertion; g05.006 card 019 selectively repaired the proof and merged
  through PR 126 at `c8691e84` without provider contact.
- **Why this is ready:** current `main` has `TempWorkspace` drop owners in both
  the ignored live probe and credential-free proof, plus
  `temporary_workspace_cleanup_is_established_before_assertions`.
- **Decisions and preferences:** preserve the opt-in/no-contact boundary. Close
  the entry only if the existing proof really falsifies the original defect.
- **Open tensions:** the PAPERCUTS entry remained open after card 019; determine
  whether that is stale bookkeeping or exposes a real remaining path.
- **Report after:** verified disposition and one coherent PR or evidence stop
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then compare the
open entry against g05.006/card 019 and the exact current live-probe/proof code.
Falsify the cleanup owner before editing the queue.

## Completion Protocol

### Before you start

1. This handoff's worker metadata activates worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Accept a clean registered non-`main` launcher worktree. Record its actual
   root and branch; do not create another because generated names differ.
3. If the current context is unusable, inspect `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask when absent. Never use `/tmp` or
   discard dirty state.
4. Fetch origin with a bounded non-interactive SSH command. Confirm selected
   `HEAD == origin/main`, confirm planning base
   `dc6f180f7c48dded7ffae1e3452eae9dc32db71b` is an ancestor, and load this
   tracked handoff from `HEAD`. Stop if the absolute file differs.
5. Required sibling links: none.
6. Read `AGENTS.md`, g05.006, card 019, `PAPERCUTS.md`, and the exact watcher
   cleanup code/proofs.
7. Run cheap repository orientation through Effigy and record actual commands.

### While you work

- Reproduce or falsify the original cleanup defect without provider contact.
- Keep the change surgical. Do not duplicate the existing drop guard or add a
  compatibility shim.
- Stop on contract ambiguity, scope expansion, or a result that changes the
  planning decision.

### When the assigned runway is complete

1. Run the required validation named above.
2. Mutate the cleanup owner or proof locally, confirm the counterexample fails,
   then restore it. Reconcile every exact/negative claim in the PR.
3. Close only this PAPERCUTS entry when the fix is proved. Add a log entry only
   if the repo's existing papercut closeout practice and materiality warrant it.
4. Push the worker branch and open one PR against current pushed `main`.
5. Report exact head/base, disposition, changed files, falsification,
   validation, residuals, and PR URL. Do not merge.

### Review and merge path

The orchestrator reviews the exact head and records its verdict on the PR. If
changes are required, repair only the posted in-bounds findings on this branch.
Requested changes are: none.

- **Closeout refs:** `PAPERCUTS.md`; optional bounded log/index only when
  warranted; g05.006/card 019 remain complete and unchanged unless a real
  contradiction is found.

### Handoff closeout

Leave the papercut honest. Close it only when the original failure is currently
prevented and proved; otherwise record the exact residual and stop.
