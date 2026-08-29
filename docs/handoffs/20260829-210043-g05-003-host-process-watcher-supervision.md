---
title: g05.003 host-process watcher supervision worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-29
updated: 2026-08-29
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260829-210043-g05-003-host-process-watcher-supervision.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, rust, watchers, process-supervision]
---

## What This Thread Was Doing

PR 117 landed the provider-neutral and host-local watcher registry behind a
hard process-containment gate. The operator then clarified the product intent:
the agent starts an ordinary host process through an injected watcher skill and
the consuming application sees its lifecycle. This is not a sandbox or daemon
containment feature.

The orchestrator revised Contracts 010 and 059, retained Research 259 only as
an explicit detached-process non-claim, withdrew the unlaunched Docker research
card/handoff, and compiled replacement g05.003 card 014. This worker owns that
host-local implementation repair only.

## Why It Matters

The portable registry already supports stable watcher identity, model and
operator controls, bounded summaries, wait, stop, terminal state, and join.
Today the default local composition still rejects every process-backed start
unless an injected `ProcessContainmentBackend` is supplied. That gate prevents
the actual feature and promises a stronger security boundary than the operator
wants.

Card 014 must connect the registry to Swallowtail's existing host-approved
process lifecycle. A watched process runs in the normal host environment while
the app receives truthful state. The managed root, cooperative process group,
output readers, watcher monitor, and supervisor must be stopped or completed
and joined before turn completion. Deliberately daemonized or `setsid`-escaped
work remains outside the claim.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `43a266dd9a0a907a2913e2479ce88ba100e3bf17`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  the planning base before this handoff commit
- **Planning checkout:** clean after the planning repair commit
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** revised Contracts 010/059 and
  guardrails; revised Research 259; replacement card 014; corrected g05.003
  runway; host-process direction log; withdrawn Docker card, triage note, and
  handoff
- **Worker branch:** `worker/g05-003-host-process-watcher-supervision`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-003-host-process-watcher-supervision`
- **Worktree creation command:** `git worktree add -b worker/g05-003-host-process-watcher-supervision /Users/tom/Dev/worktrees/swallowtail-g05-003-host-process-watcher-supervision origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, regardless of generated path
  or branch. If unusable, use the named worktree; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique fallback there. Never use `/tmp` or a guessed path.
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contracts 010 and 059 are canonical
- **Roadmap milestone:** `docs/roadmaps/g05/003-operation-scoped-watcher-proof.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/014-host-process-watcher-supervision.md`
- **Allowed runway:** card 014 implementation, tests, public API baseline,
  package docs, card/log evidence, and directly affected front doors
- **Remaining card budget:** one implementation card; stop at one reviewable PR
- **Dispatch topology:** serial
- **Parallel safety check:** card 010 depends on this repair and touches adjacent
  watcher/runtime seams; do not run either Claude wiring or card 011 in parallel
- **Canonical refs:** `docs/architecture/product-guardrails.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/009-async-operation-lifecycle.md`,
  `docs/contracts/010-execution-host-services-and-inputs.md`,
  `docs/contracts/059-operation-scoped-process-watchers.md`;
  `docs/research/259-process-containment-backend-evidence.md`;
  `docs/contracts/rust-quality-profile.json` and its named deviations file
- **Primary code surfaces:** `crates/swallowtail-host-local/src/containment.rs`,
  `host.rs`, `services.rs`, `process.rs`, `process_exit.rs`, `child.rs`,
  `watcher.rs`, and `watcher/`; host-local watcher integration tests and public
  API baselines
- **Current implementation fact:** default `LocalHostServices` installs the
  watcher registry with no containment backend; `accept_start_now` returns
  `swallowtail.local_watcher.containment_unavailable` before consulting the
  approved operation
- **Reusable process fact:** `LocalProcessHost` already binds an approved
  `ProcessRequest`, starts an owned process group on Unix, owns graceful/force
  stop, joins output readers, waits for the root, cleans the cooperative group,
  and completes its supervisor before `ProcessHandle::wait` resolves
- **Model capability profile:** capable Rust implementation model, medium
  reasoning; escalate any public-API or lifecycle ambiguity to the orchestrator
- **Tool/runtime restrictions:** no subagents, containers, Docker/OCI work,
  provider prompts, live provider probes, workflow edits, card 010/011 wiring,
  release work, or merge
- **Inherited doctor baseline:** 385 god-file findings: 338 warnings and 47
  errors; stale graph; one generated-in-src warning. Do not add a finding.
- **Required validation:** `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`;
  `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`;
  `effigy package:api`; `effigy qa:docs`; `effigy qa:northstar`;
  `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and exact head
- **Merge authorisation:** not authorized

## Boundaries

- **In scope:** the complete card 014 repair: diagnose the smallest clean
  pre-1.0 API shape, replace misleading containment terminology, make ordinary
  host-approved process execution the default watcher backend, preserve exact
  lifecycle/control/redaction behavior, add deterministic fixtures, update API
  baselines and closeout evidence, and open one PR.
- **Out of scope:** Claude MCP, watcher-skill injection, stop-hook or completion
  interception, route capability claims, consumer guide/matrix publication,
  generic shell or path authority, detached-daemon management, containers/VMs,
  cgroups, Job Objects, process-table polling, provider work, CI workflows,
  release work, and merge.
- **Outcome shape:** smallest complete contract-valid implementation. Own
  diagnosis, code changes, cleanup of temporary diagnostics, validation,
  evidence, and PR creation. Do not stop at an API proposal.
- Preserve host approval: `WatcherOperationData` resolves to an exact private
  `ProcessRequest`; caller data never becomes an executable, command, path,
  environment, working directory, PID, raw output, or permission grant.
- Preserve lifecycle truth: accepted/running/terminal/joined remain monotonic;
  wait resolves only after the managed process and watcher supervision join;
  model and operator stop remain separate request paths against one registry.
- Treat process groups honestly. They clean up ordinary cooperative children.
  They do not contain a deliberately detached child, and no diagnostic, type,
  method, test, or guide may imply otherwise.
- Remove or rename public containment concepts that now overstate the contract.
  Swallowtail is pre-1.0; do not retain compatibility shims unless a governing
  contract explicitly requires one.
- Work only in the selected worker worktree. Preserve every unrelated checkout
  and change. Do not merge the PR.

## Important Context

- **Planning lineage:** cards 007-009 are complete. Research 257 admits the
  future Claude seam. Card 014 repairs host-local execution. Card 010 then owns
  Claude route binding, and card 011 owns conformance and consumer projection.
- **Why this card is ready:** the operator settled the ambiguous product choice;
  revised Contract 059 now defines lifecycle supervision and its detached-work
  non-claim. Existing code already supplies the ordinary process mechanics.
- **Decisions and preferences:** the process runs on the normal host; the app
  sees lifecycle while the turn remains open; model and operator can inspect
  and stop it; Docker is overkill and is not part of this lane.
- **Open tensions:** choose the smallest honest replacement for
  `ProcessContainmentBackend` / `ProcessContainmentLease` and their builder
  methods. Avoid duplicate stop/wait ownership between the watcher wrapper and
  `ProcessHandle`; preserve idempotence and public API clarity.
- **Report after:** API/ownership shape and default composition compile with
  core fixtures; then full card validation and PR-ready closeout
- **Report to:** the operator, who will relay progress and the PR to the
  orchestrator

## Suggested Next Move

Run the worker preflight first. Then read card 014, revised Contract 059,
Research 259, and the host-local process/watcher surfaces named above. Trace the
existing `ProcessHandle::wait` and stop semantics before choosing the replacement
lease abstraction; do not add a second supervisor when the current process
handle already owns the needed join.

Implement one coherent tranche: honest API terminology, default local watcher
process composition, lifecycle fixtures, and public API updates. Run focused
validation after that batch. If truthful joined state cannot be expressed
without a new product or security decision, stop and report the exact gap.

## Completion Protocol

### Before you start

1. Read this handoff. Its worker metadata activates implementation mode. Before
   broad reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a clean, registered, dedicated non-`main` worktree,
   accept it as launcher-provided. Record its actual root/branch and do not
   create another because names differ from the placeholders.
3. If the launcher supplied a dirty or `main` worktree, stop and report it.
   Only for another unusable context, inspect the named worktree and then
   `.agents.local.env`; require `AGENTS_WORKTREE_CONTAINER_DIR` before a unique
   fallback. Never clean/reset another checkout or use `/tmp`.
4. In the selected worktree, fetch origin. Confirm `HEAD == origin/main`,
   confirm `git merge-base --is-ancestor 43a266dd9a0a907a2913e2479ce88ba100e3bf17 HEAD`,
   and load the tracked handoff with
   `git show HEAD:docs/handoffs/20260829-210043-g05-003-host-process-watcher-supervision.md`.
   If the absolute file differs, stop. The tracked copy is canonical.
5. Required sibling links are `none`.
6. Read `AGENTS.md`, card 014, g05.003, the canonical refs, and the current
   host-local watcher/process implementation. Follow the activated strict
   everyday Rust-quality route before Rust edits.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited baseline but do not run the broad workspace test plan.

### While you work

- Execute card 014 only. Keep implementation and tests in one meaningful batch.
- Use repository-owned focused selectors. Do not substitute broad workspace
  tests for the card's named package scope.
- Preserve the registry's bounded lifecycle and failure semantics. Add no
  arbitrary process authority and no security-containment claim.
- Report the API/default-composition chunk through the operator before final
  closeout. Stop on a contract gap, new public product choice, or scope change.

### When the assigned runway is complete

1. Run every required validation command named in `## Current State`.
2. Update card 014 and its implementation log with exact changed surfaces,
   tests, API change, and validation. Reconcile g05.003, the batch-card index,
   generation index, logs index, and the sole roadmap Next Task. Do not mark
   card 010 ready unless its declared dependencies are actually complete.
3. Confirm no container dependency, provider route claim, raw process data, or
   compatibility shim entered the diff.
4. Push the selected worker branch and open one reviewable PR against current
   pushed `main`.
5. Link card 014, revised Contract 059, Research 259, changed API/process/watcher
   surfaces, fixtures, and validation in the PR body.
6. Report the PR URL and exact head to the operator. Do not merge or start card
   010.

### Review and merge path

The orchestrator will review the exact PR head against Contract 059, card 014,
the code diff, public API baseline, focused validation, and hosted checks. With
the shared GitHub identity, the canonical verdict may be a PR comment rather
than formal self-approval. Requested changes are `none` at dispatch. The
operator must explicitly authorize merge.

- **Closeout refs:** card 014; g05.003 milestone; host-process watcher
  implementation log; g05/batch-card/generation/log indexes; roadmap front door

### Handoff closeout

Leave card 010 behind card 014 and return one PR for review. If the existing
host process boundary cannot support truthful root-process and supervisor join,
record the exact blocker and stop rather than reviving Docker or weakening the
contract silently.
