---
title: g05.003 card 010 Claude Code watcher binding worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-30
updated: 2026-08-30
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260830-171932-g05-003-card-010-claude-watcher-binding.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, rust, claude-code, watcher]
---

## What This Thread Was Doing

The provider-neutral watcher lifecycle, host-local registry, ordinary process
supervision, and Contract 060 HTTP/MCP bridge have landed. PR 121 then
qualified the existing Claude Code headless and response-only routes through
official `2.1.251` without claiming watcher support.

The post-merge reassessment makes card 010 ready for one exact,
credential-free binding. This worker connects Claude Code `2.1.251` to the
existing bridge with operation-private configuration and deterministic fake
provider fixtures. It does not run Claude, advertise watcher support, or start
card 011.

## Why It Matters

Swallowtail needs one production-harness candidate whose model-facing tool seam
and completion interception reach the same host-owned watcher registry used by
operator controls. Card 010 supplies the testable route binding needed before
a separately authorized live same-turn proof can decide whether the route may
publish watcher support.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `f0c9211addfcd6480e89e6bfd6212860b45ae747`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `f0c9211addfcd6480e89e6bfd6212860b45ae747` before this handoff was created
- **Planning checkout:** clean before the handoff commit
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** ready g05.003 card 010;
  post-currentness reassessment log; reconciled g05, milestone, batch-card,
  generation-index, log, papercut, and sole Next Task surfaces
- **Worker branch:** `worker/g05-003-card-010-claude-watcher-binding`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-003-card-010-claude-watcher-binding`
- **Worktree creation command:** `git worktree add -b worker/g05-003-card-010-claude-watcher-binding /Users/tom/Dev/worktrees/swallowtail-g05-003-card-010-claude-watcher-binding origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even when its generated path
  or branch differs from the placeholders above. Do not create a second
  worktree for a naming difference. If that context is unusable, use the named
  worktree; only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique fallback there. Never
  use `/tmp` or a guessed worktree path
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contracts 059-060 and g05.003 are canonical
- **Roadmap milestone:** `docs/roadmaps/g05/003-operation-scoped-watcher-proof.md`
- **Ready cards, in order:**
  `docs/roadmaps/g05/batch-cards/010-claude-code-watcher-bridge.md` only
- **Allowed runway:** exact Claude Code `2.1.251` credential-free watcher
  binding and deterministic provider-free fixtures from card 010
- **Remaining card budget:** one card; one reviewable PR
- **Dispatch topology:** serial single-card lane
- **Parallel safety check:** the consumer feature/option projection census is
  an independent evidence lane with no shared allowed files. Do not edit or
  promote `docs/triage/2026-08-30-consumer-route-feature-and-option-projection.md`
  or its census artifact. Stop if either lane unexpectedly needs shared
  mutable scope
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/contracts/009-async-operation-lifecycle.md`,
  `docs/contracts/010-execution-host-services-and-inputs.md`,
  `docs/contracts/012-interactive-session-options-and-callback-exchange.md`,
  `docs/contracts/013-interactive-session-access-policy.md`,
  `docs/contracts/023-harness-operation-isolation-and-native-boundary.md`,
  `docs/contracts/041-input-callback-and-provider-tool-admission.md`,
  `docs/contracts/044-observable-agent-activity-and-disclosure.md`,
  `docs/contracts/059-operation-scoped-process-watchers.md`, and
  `docs/contracts/060-operation-scoped-watcher-http-bridge.md`
- **Evidence refs:**
  `docs/research/257-claude-code-watcher-seam-evidence.md`,
  `docs/research/260-claude-code-watcher-bridge-transport.md`, and
  `docs/research/261-claude-code-2-1-251-identity.md`
- **Primary code surfaces:** Claude Code prepared profile/input/command/run and
  cleanup surfaces under `crates/swallowtail-adapter-claude-agent`; existing
  watcher bridge and working-resource interfaces under
  `crates/swallowtail-runtime`; existing Contract 060 implementation under
  `crates/swallowtail-host-local`; focused fixtures and tests. Follow the code
  shape rather than assuming every crate needs production edits
- **Exact admitted watcher point:** Claude Code `2.1.251`; every other version
  must reject watcher opt-in before bridge open, private materialization, or
  provider process effects. Omission preserves all existing qualified base
  route behavior
- **Model capability profile:** capable coding model with medium reasoning;
  lifecycle, secret-boundary, and deterministic continuation judgment required
- **Tool/runtime restrictions:** no subagents, provider prompt, login,
  authentication, provider session, live probe, credentials, paid work,
  install, host update, card 011, watcher claim, capability advertisement,
  consumer projection, contract change, release work, merge, or provider-native
  task commands. Safe local identity checks are allowed only to confirm the
  already-frozen exact point; stop if official or installed identity moved
- **Inherited health baseline:** `effigy doctor` reports 384 god-file findings:
  337 warnings and 47 errors, plus one generated-in-source warning and a stale
  graph index. Do not widen this lane into structural cleanup
- **Required validation:**
  `cargo fmt -p swallowtail-adapter-claude-agent -p swallowtail-runtime -p swallowtail-host-local`;
  `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent`;
  `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent`;
  `effigy qa:northstar`; `effigy qa:docs:index:logs`;
  `effigy qa:docs:index:roadmaps`; `effigy qa:docs:index:roadmaps:g05`;
  `effigy qa:docs:index:roadmaps:batch-cards`;
  `effigy qa:docs:next-action:roadmaps`; `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and exact head
- **Merge authorization:** not authorized

## Boundaries

- **In scope:** one opt-in route preparation for exact `2.1.251` using the
  existing Contract 060 service and one `WatcherHostService`; operation-private
  MCP, settings, skill, Stop-hook, and hook-event material; exact version and
  omission gates; deterministic fake-provider Stop-continuation fixtures;
  cancellation, deadline, provider-failure, cleanup, and lease-release paths;
  card, milestone, log, index, and sole Next Task closeout; one PR.
- **Out of scope:** real Claude execution, provider access, credentials, paid
  work, support claims, capability matrices, consumer-facing projection,
  watcher version ranges, earlier or later Claude versions, base-route
  currentness, card 011, skill discovery cards, contracts, Docker or process
  containment, unrelated cleanup, release work, and merge.
- **Outcome shape:** a complete contract-valid exact candidate binding with
  deterministic provider-free proof, honest docs closeout, and one reviewable
  PR. This is not diagnostics-only. If the frozen seam cannot satisfy the
  contracts without architecture or policy invention, stop with the exact
  blocker rather than weakening the boundary.
- Endpoint and bearer material must not enter argv, ambient environment,
  shared settings, project files, public records, default formatting, logs, or
  fixtures. Use operation-private leased files and preserve Contract 060's
  authority and redaction boundary.
- Preserve the existing empty strict-MCP command exactly when watcher support
  is omitted. Watcher omission must not open the bridge, lease private files,
  change argv, or affect existing base-route admission.
- The fake-provider completion fixture must exercise active watcher state
  through the exact Stop continuation path before terminal admission. It must
  not replace the existing host-owned registry, create provider-owned watcher
  state, or treat provider task ids/PIDs as watcher ids.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's assigned
  scope; if shared mutable scope or a hidden dependency appears, stop and report
  it through the operator.
- Work only in the selected clean worker worktree. Preserve unrelated state.
  Do not merge the PR.

## Important Context

- **Planning lineage:** Research 257 admitted Claude's private MCP + Stop seam.
  Cards 009 and 014 landed the host registry and ordinary managed-process
  supervision. Research 260 froze exact `2.1.251` transport ingredients. Card
  016 landed the Contract 060 host bridge. PR 121 qualified both existing Claude
  Code route axes through `2.1.251`. The orchestrator then separated binding
  from the later live acceptance to remove the former circular gate.
- **Why this card is ready:** the exact route identity is qualified, the bridge
  and host lifecycle exist, and the credential-free acceptance boundary is
  explicit. Binding can now be tested without provider access. Live same-turn
  behavior cannot be proved until the binding exists and remains card 011's
  separate gate.
- **Operator preferences:** use the Northstar manual worker/PR loop; do not spawn
  internal agents. Docker and hostile-process containment are not part of this
  feature. Do not move directly into live provider work.
- **Open tensions:** Research 260 proves exact candidate flags and frozen
  composition, not live same-turn behavior. Do not interpret deterministic
  fixtures as a support claim. If the adapter cannot keep authority material
  private or cannot return control through the same Stop continuation path,
  stop and report the contract conflict.
- **Closeout boundary:** mark card 010 complete only when its deterministic
  acceptance passes. Keep card 011 planned. Set the sole Next Task to
  orchestrator reassessment of card 011 after card 010 lands; state that live
  provider access, credentials, and any paid work require explicit operator
  authorization. Do not dispatch or start it.
- **Report after:** opt-in/version/omission composition is complete; then after
  lifecycle, Stop-continuation, failure, and cleanup fixtures plus final PR
  closeout
- **Report to:** the operator, who will relay progress and the PR to the
  orchestrator

## Suggested Next Move

Start with the worker preflight. Read the contracts, milestone, card, Research
257/260/261, current Claude Code command/preparation shape, Contract 060 bridge,
watcher registry, working-resource lease surfaces, and existing provider-free
test patterns. Trace the smallest binding seam before editing.

Implement exact admission, unchanged omission, and private composition as the
first coherent chunk. Then add deterministic Stop-continuation and terminal
lifecycle fixtures, complete the docs closeout, run the named validation, and
open one PR. Return after card 010; do not continue into card 011.

## Completion Protocol

### Before you start

1. Read this handoff. Its worker metadata activates implementation mode. Before
   broad reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a clean, registered, dedicated non-`main` worktree,
   accept it as launcher-provided. Record the actual root and branch. Do not
   create another because its names differ from this handoff.
3. If the launcher supplied a dirty or `main` worktree, stop and report it.
   Only for another unusable context, inspect the named worktree and then
   `.agents.local.env`; require `AGENTS_WORKTREE_CONTAINER_DIR` before a unique
   fallback. Never clean or reset another checkout and never use `/tmp` for a
   worktree.
4. In the selected worktree, fetch origin. Confirm `HEAD == origin/main`,
   confirm
   `git merge-base --is-ancestor f0c9211addfcd6480e89e6bfd6212860b45ae747 HEAD`,
   and load the tracked handoff with
   `git show HEAD:docs/handoffs/20260830-171932-g05-003-card-010-claude-watcher-binding.md`.
   If the absolute file differs, stop. The tracked copy is canonical.
5. Required sibling worktree links are `none`.
6. Read `AGENTS.md`, `PAPERCUTS.md`, g05.003, card 010, Contracts 001, 009,
   010, 012, 013, 023, 041, 044, 059, 060, Research 257, 260, 261, the
   post-currentness reassessment log, current watcher/bridge/lease code, Claude
   Code command and prepared-route code, fixtures, and Rust-quality profile.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited baseline; do not execute the broad workspace plan.

### While you work

- Implement only exact `2.1.251` watcher opt-in. Reject every other version
  before bridge open, private materialization, or provider process work.
- Preserve omission byte-for-byte where practical and behaviorally exactly:
  existing empty strict MCP configuration, argv, preparation, and route support
  remain unchanged.
- Build operation-private MCP/settings/skill/Stop material with temporary
  working-resource leases. Do not place the endpoint or bearer secret in argv,
  ambient environment, public/debug formatting, shared settings, or project
  state.
- Reuse the existing Contract 060 bridge and the same host-owned watcher
  registry for model and operator controls. Do not build a second lifecycle or
  provider-owned registry.
- Use a deterministic fake provider to prove active-watcher Stop continuation
  before terminal admission, plus cancellation, deadline, provider failure,
  cleanup, join, and private-material release. Do not run Claude.
- Keep support advertisement absent. Do not edit capability matrices or infer a
  watcher-compatible version range.
- Report the two meaningful chunks named above through the operator. Do not
  spawn internal agents; the operator owns thread parallelism in their harness.
- Stop on identity drift, a missing contract seam, secret-boundary failure,
  same-turn continuation ambiguity, or scope expansion.

### When the assigned runway is complete

1. Run every command under Required validation. Do not substitute broad
   workspace tests, live probes, provider checks, consumer checks, MSRV, or
   release validation.
2. Mark card 010 complete only after deterministic acceptance passes. Update
   g05.003, g05 and batch-card indexes, add and index an implementation log,
   and reconcile the sole Next Task. Keep card 011 planned and explicitly
   pending post-merge orchestrator reassessment plus operator authorization for
   live provider access, credentials, and any paid work.
3. Do not edit or promote the independent consumer feature/option projection
   triage lane.
4. Push the selected worker branch and open one reviewable PR against current
   pushed `main`.
5. In the PR body, link g05.003, card 010, Contracts 059-060, Research 257/260,
   changed adapter/runtime/host surfaces, deterministic fixtures, docs
   closeout, validation, and unresolved items.
6. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator will review the exact PR head against Contracts 059-060,
Research 257/260, card 010, the diff, deterministic fixtures, private-material
boundary, docs closeout, and hosted checks. With the shared GitHub identity,
the canonical verdict may be a PR comment rather than formal self-approval.
Requested changes are `none` at dispatch. Merge remains separately authorized
by the operator.

- **Closeout refs:** g05.003; card 010; implementation log; g05 and batch-card
  indexes; sole `docs/roadmaps/README.md` Next Task

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is.
