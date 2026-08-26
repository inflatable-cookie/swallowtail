---
title: g04.074 Cline headless model selection worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260826-162222-g04-074-cline-headless-model-selection.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator resumed after g04.073 merged, reconciled the stale feature-gap
inventory with the current per-route programme, and selected exact Cline
headless model routing as the next serial evidence-first lane. It published
g04.074, cards 204-206, Research 221 reservation, programme/front-door updates,
and the compilation log to `main`.

This is the handoff from the planning/orchestrator thread to one bounded manual
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not spawn internal agents; the operator owns parallelism in
their harness.

## Why It Matters

`cline.headless` owns one exact JSON child but binds neither provider nor model.
Exact `3.0.55` parses `--model` and gives explicit model identity precedence,
making this the named dependency behind the earlier headless-thinking stop.
The same source also resolves provider identity from ambient last-used state,
may derive model membership dynamically, and persists the resolved selection.
The lane must decide whether any exact model route survives those gates before
it changes production code.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `279a53c0f8ddf5896e457dd57eb3b639ae07d272`
- **Pushed main verification:** local HEAD and fetched `origin/main` both
  resolved to `279a53c0f8ddf5896e457dd57eb3b639ae07d272` before this handoff commit
- **Planning checkout:** clean dedicated orchestrator worktree
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.074, cards 204-206, Research
  221 reservation, compilation log, advanced-feature triage disposition, and
  sole Next Task
- **Worker branch:** `worker/g04-074-cline-headless-model-selection`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-074-cline-headless-model-selection`
- **Worktree creation command:** `git worktree add -b worker/g04-074-cline-headless-model-selection /Users/tom/Dev/worktrees/swallowtail-g04-074-cline-headless-model-selection origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent; never use `/tmp`, `TMPDIR`,
  or a guessed path.
- **Active spec lane:** per-route feature completion programme
- **Roadmap milestone:** `docs/roadmaps/g04/074-cline-headless-model-selection.md`
- **Ready cards, in order:** `204-cline-headless-model-selection-evidence.md`,
  then conditional `205-cline-headless-model-selection-binding.md`, then
  conditional `206-cline-headless-model-selection-acceptance.md`
- **Allowed runway:** execute card 204 and promote Research 221; continue to
  cards 205-206 only for a non-empty exact `cline.headless` `3.0.55`
  provider/model row with closed membership, preflight agreement, and legal
  configuration behavior
- **Remaining card budget:** three cards; stop after card 204 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one reviewable PR; no internal
  agents or subagents
- **Parallel safety check:** serial because evidence determines whether the
  binding and acceptance cards exist and all cards touch the same Cline route
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  005, 008, 020, 023, 029, 033, 037, 040, and 052
- **Model capability profile:** exact-source research plus route-local Rust
  implementation and deterministic conformance
- **Tool/runtime restrictions:** official exact-tag/package evidence and
  secret-free local parser/source work only; no install, login, account
  inspection, live catalogue, provider prompt, paid work, ambient config
  mutation, live probe, or sibling-route work
- **Required validation:** card 204 checks first; if delivery proceeds,
  `cargo fmt -p swallowtail-adapter-cline`,
  `effigy validate:focused swallowtail-adapter-cline`,
  `effigy package:verify-affected swallowtail-adapter-cline`,
  `effigy check:examples`, `effigy package:api`, `effigy qa:northstar`, named
  research/log/roadmap/card/next-action index checks, `effigy doctor`, and
  `git diff --check`
- **Inherited doctor baseline:** 379 findings: 333 warnings and 46 errors,
  plus one generated-in-src finding after g04.073; record drift, do not repair
  unrelated findings
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised; operator must explicitly request it

## Boundaries

Keep this run inside the named runway:

- **In scope:** exact `cline@3.0.55` `--model` and required provider-agreement
  evidence; conditional immutable model-route/preflight/argv binding;
  deterministic acceptance; optional Plan composition; route-local docs,
  matrices, Research 221, closeout, and Next Task
- **Out of scope:** caller provider selection, API keys, OAuth, credential or
  catalogue work, arbitrary model strings, Cline ACP, thinking delivery,
  aliases/fallback, settings mutation, temporary config/home construction,
  sibling routes, currentness, release, generation rollover, g04 closure, or
  merge
- An adapter-fixed provider argument is eligible only if Research 221 proves it
  from the current configured-instance and access facts. If a public provider
  choice is required, stop.
- Contract 033 grants no adapter configuration discovery, parsing, mutation,
  migration, creation, or deletion authority. Treat unavoidable
  `saveProviderSettings` effects as a decision gate, not incidental behavior.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's scope or
  spawn subagents. If shared mutable scope or a hidden dependency appears, stop
  and report it through the operator.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** g04.042 proved Cline ACP discards thinking and headless
  thinking is model-entitled while the route selects neither provider nor
  model. g04.073 then delivered exact headless Plan. g04.074 addresses only the
  model-route dependency.
- **Exact source lead:** `apps/cli/src/commands/program.ts` parses `-m,
  --model <model-id>`. `apps/cli/src/main.ts` resolves provider from explicit
  argv, ambient last-used settings, or `cline`; resolves model from explicit
  argv, provider settings, `knownModels[0]`, or a hardcoded fallback; then calls
  `saveProviderSettings` before the run.
- **Current route truth:** `ClineHeadlessRunProfileInput` has no model route;
  low-level validation rejects non-empty plan model identity; the guide names
  no catalogue or caller-supplied model; access is local-account,
  configuration is `Ambient`, and isolation is `AmbientHost`.
- **Decisions and preferences:** exact closed rows only; omission preserves
  current argv and ambient behavior; no caller provider choice; optional Plan
  remains independent; no effective-value observation may be invented from
  unselected verbose output; g04 stays open.
- **Open tensions:** exact model membership may be dynamic or account-scoped;
  invalid identifiers may fail late or fall back; model/provider agreement may
  require explicit provider argv; settings persistence may make the lane an
  evidence stop.
- **Thinking boundary:** if exact model routing ships, note only that it removes
  one g04.042 dependency. Do not deliver thinking or reopen cards 117-118 in
  this lane.
- **Report after:** Research 221 and card 204 are complete, or earlier when a
  stop condition fires; if evidence is non-empty, continue through cards
  205-206 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Execute card 204 as one coherent evidence chunk. Start with the exact tagged
parser and headless dispatch, then trace provider/model agreement and
`saveProviderSettings` through the production Cline seam before deciding
whether cards 205-206 unlock. Report the exact empty or non-empty Research 221
set and the evidence behind it.

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
   container from pushed `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed
   path; never clean, reset, stash-over, or discard the original checkout's
   dirty state. If the launcher supplied a dirty or `main` worktree, stop and
   report it instead of silently creating a second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 279a53c0f8ddf5896e457dd57eb3b639ae07d272 HEAD`
   succeeds, and confirm this handoff file exists in selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline rather than repairing unrelated findings.

### While you work

- Execute cards 204-206 in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- Do not spawn agents or subagents. The operator coordinates parallel work in
  their harness.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, provider/model membership remains open,
  configuration authority is absent, intent is ambiguous, or scope expands.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the required final validation named above and in the active cards.
2. Update Research 221, cards, roadmap, log, programme, triage, indexes, guide,
   matrices, API baseline, and sole Next Task required by the actual
   disposition.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The handoff's
   planning-base SHA is the planning base before the handoff commit, not a
   self-referential hash for the commit containing this file.
5. Link the milestone, cards, Research 221, changed surfaces, evidence,
   validation, and unresolved items in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

When orchestrator and worker share a GitHub identity, formal self-approval is
unavailable. The orchestrator posts the evidence-backed verdict as a PR
comment; that comment is the canonical review record. If changes are
requested, make only those changes on this branch, push, and report through the
operator. Requested changes: none. The operator must explicitly authorise any
merge.

- **Closeout refs:** Research 221; cards 204-206; g04.074; compilation/closeout
  log; Cline headless guide; route/feature matrices where truth changes;
  programme, triage, indexes, and sole Next Task

### Handoff closeout

Before calling the runway complete, leave card, roadmap, log, and next-task
state honest. If blocked, record the blocker and stop rather than making the
handoff look more complete than it is.
