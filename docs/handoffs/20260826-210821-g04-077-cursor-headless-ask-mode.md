---
title: g04.077 Cursor headless Ask mode worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/worktrees/swallowtail-review-pr69/docs/handoffs/20260826-210821-g04-077-cursor-headless-ask-mode.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator fast-forward merged PR 75 at exact head
`347ba0a95bbc8187c4210db944c83ec636d25132`, resumed the sole roadmap
continuation, and reassessed the remaining per-route feature inventory. It
selected Cursor headless Ask as the next closed route-local behavior candidate.

The orchestrator compiled g04.077, cards 213-215, Research 224, programme and
front-door updates, triage disposition, and the compilation log. The planning
base was validated and fast-forwarded to `main` at
`c12eeaf3ac041d66b31bd4cd26dd569efc1e6efd`.

This is one bounded manual implementation thread. Start from this file without
a copied transcript or a second prompt. Do not spawn internal agents; the
operator owns parallelism in their harness.

## Why It Matters

`cursor-agent.headless` maps `ResourceAccess::Read` to `--mode plan` and leaves
`ReadWrite` in default Agent behavior with no mode argument. All four exact
qualified Cursor builds also expose `--mode ask`; current official docs
describe Ask as read-only exploration and Q&A. Swallowtail cannot select it.

The route already owns the exact child, working-resource access, explicit
model and model parameters, configuration posture, deadline, cancellation,
activity, terminal result, retention, and cleanup. That creates a credible
adapter-local binding seam. It does not prove exact read-only behavior,
configuration precedence, applied mode, or effective mode. Research 224 must
settle those facts before production code changes.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning base:** `main`
- **Planning commit before this handoff:** `c12eeaf3ac041d66b31bd4cd26dd569efc1e6efd`
- **Planning branch:** `orchestrator/g04-077-cursor-headless-ask-mode`
- **Planning publication:** planning commit is exact `origin/main` before this
  handoff commit
- **Planning checkout:** clean dedicated orchestrator worktree before this
  handoff file
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates worker-only worktree preflight
- **Planning artifacts:** g04.077, cards 213-215, Research 224 reservation,
  compilation log, programme/triage/index updates, and sole Next Task
- **Worker branch:** `worker/g04-077-cursor-headless-ask-mode`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-077-cursor-headless-ask-mode`
- **Worktree creation command:** `git worktree add -b worker/g04-077-cursor-headless-ask-mode /Users/tom/Dev/worktrees/swallowtail-g04-077-cursor-headless-ask-mode origin/main`
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
- **Roadmap milestone:** `docs/roadmaps/g04/077-cursor-headless-ask-mode.md`
- **Ready cards, in order:**
  `213-cursor-headless-ask-mode-evidence.md`, then conditional
  `214-cursor-headless-ask-mode-binding.md`, then conditional
  `215-cursor-headless-ask-mode-acceptance.md`
- **Allowed runway:** execute card 213 and promote Research 224; continue to
  cards 214-215 only for a non-empty exact Ask row with immutable selection and
  a proved behavioral boundary
- **Remaining card budget:** three cards; stop after card 213 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one reviewable PR; no internal
  agents or subagents
- **Parallel safety check:** serial because evidence decides whether binding
  and acceptance exist and every card touches the same Cursor route
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  010, 011, 023, 029, 033, 034, 037, 040, and 052
- **Model capability profile:** exact artifact/source research plus route-local
  Rust implementation and deterministic conformance
- **Tool/runtime restrictions:** exact official source/artifact evidence and
  secret-free local parser/source work only; no install, update, login, account
  inspection, authenticated catalogue, provider prompt, paid work, model run,
  tool execution, ambient config mutation, or sibling-route work
- **Required validation:** card 213 checks first; if delivery proceeds,
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

- **In scope:** exact qualified Cursor `--mode ask` parser, precedence,
  configuration, behavior, read-only, output, and version evidence;
  conditional closed adapter-local preparation and low-level driver binding;
  canonical argv; deterministic acceptance; route-local docs/matrices/API
  truth; Research 224; closeout; and sole Next Task
- **Out of scope:** portable `HarnessMode::Ask`, raw mode strings, default Agent
  selection, write authority, `--force`, `--yolo`, `--auto-review`, sandboxing,
  approvals, permission or tool-policy selection, Cursor ACP/catalogue, model-
  parameter changes, another route feature, currentness, release, generation
  rollover, g04 closure, or merge
- Existing construction must remain exact: `Read` selects `--mode plan` and
  `ReadWrite` omits `--mode`. Ask plus `ReadWrite` must reject before process
  work.
- Ask remains provider behavior. It does not imply process isolation,
  filesystem containment, callback mediation, provider-configuration
  suppression, working-resource authority, tools, permissions, approval, or
  network authority.
- Requested, prepared, dispatched, parser-accepted, applied, effective, and
  observed mode truth remain separate. Claim only the strongest exact evidence
  level Research 224 admits.
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
  inherit a guaranteed result.
- **Prompt-free lead:** exact disposable-artifact probes already found
  `--mode <mode>` with choices `plan|ask` on the July 1, July 23, and August 11
  builds. `ask|plan` parse; `agent`, `ASK`, invalid, and empty values reject.
  Reproduce and freeze this evidence under card 213 rather than relying on the
  orchestration transcript.
- **Current host evidence:** Research 223 froze installed exact
  `2026.08.04-aaa8809` help with the same hash as the August 11 specimen. A
  prompt-free version/help observation is authorized; a model run is not.
- **Current argv:** `--print --output-format stream-json --model <exact>
  --trust`; `Read` adds `--mode plan`; `ReadWrite` adds no mode. Prompt content
  goes through stdin.
- **Current route truth:** every run is one owned child; Research 183 governs
  exact model parameters; configuration is `Ambient`; isolation is
  `AmbientHost`; durable provider state may remain without management
  authority.
- **Current official lead:** `https://cursor.com/docs/cli/overview` names
  Agent, Plan, and Ask; it describes Ask as read-only exploration and Q&A and
  shows `--mode=ask`. This mutable page corroborates current semantics only.
- **Exact source authorities:** Research 077, 087, and 135 plus their frozen
  installed/official bundles and fixture identities. Research 183 governs
  model-parameter composition. Research 223 contains the latest exact Cursor
  artifact/source techniques and current host evidence.
- **Implementation seam:** current command construction lives in
  `crates/swallowtail-adapter-cursor/src/headless_command.rs`; current prepared
  input/result live in `src/prepared/headless.rs`. The low-level driver derives
  Plan from access today. A candidate Ask binding must retain an immutable
  adapter-local selection without pretending the common `PreflightPlan`
  contains a portable Ask mode.
- **Open tensions:** exact source may expose only a label and parser; persisted
  or project state may override mode; stream JSON may not confirm application;
  provider transcript/config writes may coexist with read-only resource
  behavior; low-level callers need a fail-closed exact binding; Ask may not
  remain independent of tool and approval policy.
- **Decisions and preferences:** only a closed Cursor-local selection; no raw
  string and no portable Ask; preserve Plan/no-mode defaults; no live proof;
  an empty Research 224 set is valid if any decisive behavior remains open.
- **Report after:** Research 224 and card 213 are complete, or earlier when a
  stop condition fires. If evidence is non-empty, continue through cards
  214-215 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Execute card 213 as one coherent evidence chunk. Begin with exact artifact
identity and parser/source tracing for `--mode`, then freeze repetition and
configuration precedence, the Ask/Plan/Agent behavioral split, read-only and
tool/write seams, and stream observation. Promote an exact empty or non-empty
Research 224 set before touching production binding.

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
   `c12eeaf3ac041d66b31bd4cd26dd569efc1e6efd`, confirm this handoff file exists
   in selected `HEAD`, and confirm the worker branch targets current `main`.
5. Read `AGENTS.md`, the active milestone, all assigned cards, and the
   canonical architecture/contracts from the selected worker worktree.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline; do not run the broad test plan unless a card
   names it.

### While you work

- Execute cards 213-215 in order. Card 213 decides whether 214-215 exist.
- Keep commits aligned with meaningful evidence, binding, and acceptance
  chunks rather than arbitrary turns.
- After each meaningful chunk, report changed files, validation actually run,
  remaining cards, new risks, and blockers through the operator.
- Stop if exact behavior, contract fit, authority, or intent remains ambiguous.
  Do not turn a help label or parser result into an applied/effective claim.
- Use a disposable directory created by `mktemp -d` for exact artifacts. Do not
  install or replace the selected host executable.
- Keep prompt, credentials, account state, live catalogue, and provider tool
  execution out of deterministic evidence.

### When the assigned runway is complete

1. Run the card's full required validation once for the completed batch.
2. Update Research 224, milestone/cards, programme, triage, logs, indexes,
   relevant route/feature docs, and the sole Next Task honestly.
3. If card 213 produces an empty set, mark 214-215 blocked and close out the
   evidence stop. Do not implement a placeholder or weaker mode.
4. If delivery proceeds, preserve exact Plan/no-mode behavior and run the API
   baseline check for any public adapter-local type.
5. Push the selected worker branch and open one reviewable PR against current
   pushed `main`.
6. In the PR body, link the milestone, cards, Research 224, changed surfaces,
   exact evidence, validation, and unresolved items.
7. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

When orchestrator and worker share a GitHub identity, formal self-approval is
unavailable. The orchestrator will post the evidence-backed verdict as the
canonical PR review comment. If changes are requested, make only those changes
on this worker branch, push again, and report through the operator. Merge needs
separate explicit operator authorization.

- **Requested changes:** none yet
- **Closeout refs:** Research 224, cards 213-215, g04.077, per-route feature
  programme, triage reassessment, compilation/closeout logs, and sole roadmap
  Next Task

### Handoff closeout

Before calling the runway complete, leave the research, cards, milestone,
programme, log, and Next Task honest. If the evidence is empty or work is
blocked, record the reason and stop rather than making the lane look delivered.
