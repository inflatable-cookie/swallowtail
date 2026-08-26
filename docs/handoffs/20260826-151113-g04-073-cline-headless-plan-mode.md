---
title: g04.073 Cline headless Plan mode worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260826-151113-g04-073-cline-headless-plan-mode.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator resumed after g04.072 stopped, reassessed the remaining
per-route feature inventory, and selected exact Cline headless Plan mode as the
next serial evidence-first lane. It published g04.073, cards 201-203, Research
220 reservation, programme/front-door updates, and the compilation log to
`main`.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

`cline.headless` already owns one bounded JSON child, but consumers cannot
select Cline's exact Plan behavior. Qualified `3.0.55` source shows a credible
fixed-argument path through parser precedence, run config, prompt, mode-tagged
input, tool preset, and command guard. The lane must decide whether that whole
path is equivalent to portable `HarnessMode::Plan` without turning provider
behavior into a false containment claim.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `8c65d976e4832ed948830ec2c84a77fd635a7610`
- **Pushed main verification:** local HEAD and fetched `origin/main` both
  resolved to `8c65d976e4832ed948830ec2c84a77fd635a7610` before this handoff commit
- **Planning checkout:** clean dedicated orchestrator worktree
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** g04.073, cards 201-203, Research
  220 reservation, compilation log, advanced-feature triage disposition, and
  sole Next Task
- **Worker branch:** `worker/g04-073-cline-headless-plan-mode`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-073-cline-headless-plan-mode`
- **Worktree creation command:** `git worktree add -b worker/g04-073-cline-headless-plan-mode /Users/tom/Dev/worktrees/swallowtail-g04-073-cline-headless-plan-mode origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual path/branch
  and never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask the
  operator first if the file or key is absent; never use `/tmp`, `TMPDIR`, or a
  guessed path.
- **Active spec lane:** per-route feature completion programme
- **Roadmap milestone:** `docs/roadmaps/g04/073-cline-headless-plan-mode.md`
- **Ready cards, in order:** `201-cline-headless-plan-mode-evidence.md`, then
  conditional `202-cline-headless-plan-mode-binding.md`, then conditional
  `203-cline-headless-plan-mode-acceptance.md`
- **Allowed runway:** execute card 201 and promote Research 220; continue to
  cards 202-203 only for a non-empty exact `cline.headless` `3.0.55`
  `HarnessMode::Plan` deliver-now row
- **Remaining card budget:** three cards; stop after card 201 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one reviewable PR
- **Parallel safety check:** serial because evidence determines whether the
  binding and acceptance cards exist; do not spawn internal agents
- **Canonical refs:** `docs/architecture/system-architecture.md`;
  Contracts 011, 012, 023, 029, 033, 034, 037, and 052
- **Model capability profile:** exact-source research plus route-local Rust
  implementation and deterministic conformance
- **Tool/runtime restrictions:** official exact-tag/package evidence and
  secret-free local parser/source work only; no install, login, account
  inspection, provider prompt, arbitrary tool execution, paid work, ambient
  config mutation, live probe, or sibling-route work
- **Required validation:** card 201 checks first; if delivery proceeds,
  `cargo fmt -p swallowtail-adapter-cline`,
  `effigy validate:focused swallowtail-adapter-cline`,
  `effigy package:verify-affected swallowtail-adapter-cline`,
  `effigy check:examples`, `effigy package:api`, `effigy qa:northstar`, named
  research/log/roadmap/card/next-action index checks, `effigy doctor`, and
  `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised; operator must explicitly request it

## Boundaries

Please keep this run inside the named runway:

- **In scope:** exact `cline@3.0.55` headless `--plan` evidence; conditional
  portable `HarnessMode::Plan` preparation/argv binding; deterministic
  acceptance; route-local docs, matrices, Research 220, closeout, and Next Task
- **Out of scope:** Cline ACP, model/provider selection, thinking, timeout,
  auto-approve true, yolo/act/zen public modes, runtime Plan-to-Act mutation,
  generic configuration, permissions, arbitrary tools, sibling routes,
  currentness, release, generation rollover, g04 closure, or merge
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's assigned
  scope; if shared mutable scope or a hidden dependency appears, stop and report
  it through the operator.
- Work only in the selected clean worker worktree: prefer the current
  launcher-provided worktree and record its actual path/branch; otherwise use
  `/Users/tom/Dev/worktrees/swallowtail-g04-073-cline-headless-plan-mode` /
  `worker/g04-073-cline-headless-plan-mode`, or the recorded local-path fallback
  created by the startup preflight. Never edit the orchestrator's planning
  checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** g04.042 already proved Cline ACP discards thinking and
  headless thinking is model-entitled. g04.072 then stopped on unfrozen Grok
  spawn effects. g04.073 deliberately chooses an exact request/argv seam with
  public source tied to the qualified package.
- **Why these cards are ready:** exact `3.0.55` source shows explicit Plan wins
  over persisted settings and reaches the selected headless config. Contracts
  023 and 034 already define fixed-argument Plan behavior and its separation
  from isolation.
- **Decisions and preferences:** headless only; only portable `Plan` is a
  candidate; omission preserves current argv; `--auto-approve false`, read-only
  working-resource policy, `Ambient`, and `AmbientHost` remain independent and
  exact; g04 stays open.
- **Open tensions:** the selected JSON wire reports mode only under unselected
  verbosity; the command guard is explicitly a blacklist rather than full
  containment; exact source contains Plan-to-Act machinery. Stop if the model
  can widen itself to Act during this one-prompt operation or if complete Plan
  equivalence cannot be proved without provider work.
- **Report after:** Research 220 and card 201 are complete, or earlier when a
  stop condition fires; if evidence is non-empty, continue through cards
  202-203 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the top.
Before broad repository reads, run the quick startup worktree-safety preflight in
`## Completion Protocol`. If the current context is a clean, dedicated,
non-`main` registered worktree, it is the launcher-provided worktree: use it
immediately, record its actual path/branch, and do not compare its generated
path/branch with this handoff or create another worktree. If it is `main`, dirty,
unregistered, or otherwise unusable, use the named worktree if it matches; only
then read `.agents.local.env`, require a valid `AGENTS_WORKTREE_CONTAINER_DIR`,
ask the operator if it is absent, and create a unique manual worktree and branch
under that container from pushed `origin/main`. Never fall back to `/tmp` or
`TMPDIR`. Do not run broad repo orientation before this decision. Read
`AGENTS.md`, the active milestone, each assigned card, and the canonical
architecture/contracts from the selected worker worktree.

Once that checks out, execute card 201 as one coherent evidence chunk. Use the
exact tagged sources named by the roadmap and audit the production Cline
headless seam before deciding whether cards 202-203 unlock. When you reach a
natural pause, tell the operator what changed, what validation you actually
ran, what remains, and whether anything needs a planning decision.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Then run
   one quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare them with the named worktree/branch or
   create another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; if it
   is absent, ask the operator before creating the file or worktree. Then create
   a unique worktree and branch under that container from pushed `origin/main`,
   record the actual path and branch, and run all subsequent commands there.
   Never use `/tmp`, `TMPDIR`, or a guessed path; never clean, reset, stash-over,
   or discard the original checkout's dirty state. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 8c65d976e4832ed948830ec2c84a77fd635a7610 HEAD`
   succeeds, and confirm this handoff file exists in the selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline rather than repairing unrelated findings.

### While you work

- Execute cards 201-203 in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop and say so if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into a new architecture.

### When the assigned runway is complete

1. Run the required final validation named above and in the active cards.
2. Update Research 220, card, roadmap, log, programme, triage, index, guide,
   matrix, API, and sole Next Task evidence required by the actual disposition.
3. Push the selected worker branch.
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   planning-base SHA is the planning base before the handoff commit, not a
   self-referential hash for the commit that contains this file.
5. In the PR body, link the milestone, cards, Research 220, changed surfaces,
   evidence, validation, and unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

The orchestrator records an evidence-backed verdict in the provider's review
surface. When the orchestrator and worker share a GitHub identity, formal
self-approval is unavailable, so the orchestrator posts the verdict as a PR
comment; that comment is the canonical review record. If changes are requested,
make only those changes on this branch, push again, and report back through the
operator. Requested changes are: none. The PR should link the milestone, cards,
Research 220, changed surfaces, evidence, validation, and unresolved items. The
operator must explicitly authorise any merge.

- **Closeout refs:** Research 220; cards 201-203; g04.073; compilation/closeout
  log; Cline guide; route/feature matrices where truth changes; programme,
  triage, indexes, and sole Next Task

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is.
