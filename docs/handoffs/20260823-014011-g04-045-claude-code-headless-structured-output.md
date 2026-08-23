---
title: g04.045 Claude Code headless structured output worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260823-014011-g04-045-claude-code-headless-structured-output.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator closed g04.044 after PR 43, resumed the sole roadmap Next
Task, reassessed the promoted per-route feature inventory, corrected a Kimi
version-line mismatch in triage, and compiled g04.045. Claude Code headless
structured-output work has not started. The ready runway begins with exact
`2.1.238` package evidence and permits binding only for Research 192
deliver-now rows.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. The worker can start from this file without a copied
transcript or a second prompt.

## Why It Matters

`claude-code.headless` already provides a bounded read-only Plan-mode
structured run with exact model selection, optional reasoning, fixed provider
tools, a working resource, stream JSON, activity, usage, cancellation, and
joined cleanup. It rejects structured output today. Current official Claude
Code docs name `--json-schema`, but the exact currentness corpus did not freeze
that flag. Response-only Research 121 also proves that the flag may expose a
model-visible schema tool, retry, and exit zero with a null structured result.
The route cannot claim structured output until exact dialect, enforcement,
attempt, terminal-result, and composition truth is proved.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `2fa3e761c531f20af4415a5297b209b41732532a`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `2fa3e761c531f20af4415a5297b209b41732532a` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.045, cards 124-126,
  Research 192 reservation, compilation log, closeout reservation, corrected
  triage selection, and updated sole Next Task
- **Worker branch:** `agent/g04-045-claude-structured-output-20260823-014011`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-045-claude-structured-output-20260823-014011`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-045-claude-structured-output-20260823-014011 -b agent/g04-045-claude-structured-output-20260823-014011 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path and branch;
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches. Only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator if the file or key is absent. Never use `/tmp`, `TMPDIR`, or a
  guessed path.
- **Active spec lane:** per-route feature completion; no spec edit
- **Roadmap milestone:** `docs/roadmaps/g04/045-claude-code-headless-structured-output.md`
- **Ready cards, in order:**
  `124-claude-code-headless-structured-output-evidence.md`, then conditional
  `125-claude-code-headless-structured-output-binding.md`, then conditional
  `126-claude-code-headless-structured-output-acceptance.md`
- **Allowed runway:** exact `@anthropic-ai/claude-code@2.1.238` headless JSON
  Schema evidence, then only Research 192 deliver-now binding
- **Remaining card budget:** three cards; cards 125-126 execute only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane
- **Parallel safety check:** serial by design; every card shares the Claude
  Code headless prepared profile, command, validator, event/result parser,
  fixtures, guide, research record, and closeout. Do not use internal
  subagents; report through the operator.
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 029, 033, 037, 039, 040, 044, and 052
- **Route identity:** `claude-code.headless`, driver
  `swallowtail.claude-code.headless`, axis
  `claude-code-headless-stream-json`, exact first evidence point `2.1.238`,
  current behavior `claude-code.headless.stream-json.v1`
- **Model capability profile:** exact-version, exact-command, evidence-first
  implementation; fail closed on dialect, enforcement, attempt, result,
  version, or composition ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents
  or parallel worker lanes; no package install, host executable replacement,
  login, credential/account inspection, provider prompt, live Claude request,
  or paid work. Current official source inspection, exact package download and
  extraction, local source/CLI inspection, and secret-free deterministic
  fixtures are allowed by card 124.
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-claude-agent`,
  `effigy validate:focused swallowtail-adapter-claude-agent`,
  `effigy package:verify-affected swallowtail-adapter-claude-agent`,
  `effigy check:examples`, `effigy qa:routes`, `effigy qa:northstar`, research,
  logs, roadmaps, g04, batch-card and next-action index gates,
  `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 373 inherited structural findings: 328 warnings
  and 45 errors. Keep them separate from lane-created findings.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; worker must not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-claude-agent/**`;
  `docs/guides/claude-agent-prepared-integration.md`; Research 192; g04.045;
  cards 124-126; the reserved g04.045 route-local closeout; the Claude Agent
  adapter package-specific unreleased public-API baseline when applicable;
  exact public official and package evidence; deterministic secret-free
  command, stream, result, failure, usage, cancellation, and cleanup fixtures
- **Out of scope:** `claude-code.response-only`, `claude-agent.acp`, Anthropic
  APIs, other Claude Code flags, arbitrary tools, MCP, callbacks, permission
  widening, writes, session persistence, fallback, search, compatibility-range
  widening, live provider work, contracts, `CHANGELOG.md`, shared architecture,
  route/feature matrices, programme/front doors/indexes, matrix assertions,
  shared package lists, release, publication, or merge work
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product, API, persistence, security, or compatibility decision.
- Do not backfill `claude-code.headless.stream-json.v1` or earlier versions.
  Research 192 must decide whether an admitted exact subset needs a new opaque
  private behavior revision.
- This handoff represents one worker lane. Do not edit another lane's scope. If
  shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Selection reason:** the route and portable structured-output vocabulary
  already exist, so the lane can state the exact candidate without inventing a
  cross-provider control. Remaining candidates need wider vocabulary, model
  entitlement, permission, billing, or process-topology decisions.
- **Exact-version gap:** Research 175's `2.1.238` protocol record says host help
  was not reprobed and reused the `2.1.235` selected subset. Current web docs do
  not amend that frozen package claim. Inspect the exact package.
- **Response-only warning:** Research 121 is negative evidence for a sibling
  route, not a headless disposition. It found a model-visible
  `StructuredOutput` tool, multiple attempts, and zero exit with
  `structured_output: null`. Repeat the relevant proof on the selected command.
- **Schema burden:** name the exact accepted dialect and keyword subset. Do not
  claim JSON Schema 2020-12, another draft, or generic JSON Schema from a flag
  label alone.
- **Enforcement burden:** classify the exact path as `ProviderNative` or
  `HarnessValidated`. Prompt instructions and JSON-shaped text do not qualify.
- **Attempt burden:** any non-zero retry requires an exact preflight-bound
  maximum under Contract 040. `--max-turns` is only a candidate to inspect; do
  not assume it bounds schema attempts or add it incidentally.
- **Terminal burden:** missing, null, malformed, duplicate, foreign, or
  schema-invalid structured output is failure even when the child exits zero.
  Keep ordinary text result, structured result, usage, turns, exit, and model
  evidence distinct.
- **Composition burden:** classify fixed Plan mode, `Read,Glob,Grep`, strict
  empty MCP, no persistence, working resource, model, every qualified effort,
  activity, usage, deadline, cancellation, process termination, and cleanup.
- **Absent path:** preserve the exact current command and ordinary terminal
  text behavior when schema is not selected.
- **Honest stop:** an empty Research 192 deliver-now set is a successful
  evidence result. Close cards 125-126 as blocked and open the evidence PR.
- **Known baseline:** do not claim or repair inherited doctor findings unless
  this lane creates distinct friction. Record new recurring Northstar friction
  in `PAPERCUTS.md`.
- **Report after:** card 124's exact route/schema decision. Continue only for a
  non-empty deliver-now set, then report after the complete cards 125-126
  implementation and validation chunk.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the quick worktree-safety preflight in
`## Completion Protocol`. If the current context is a clean, dedicated,
non-`main` registered worktree, use it immediately, record its actual path and
branch, and do not compare its generated identity with the fallback above. If
it is unusable, use the named worktree if it matches; only then read
`.agents.local.env` and follow its required container setting. Never fall back
to `/tmp` or `TMPDIR`.

Read `AGENTS.md`, g04.045, cards 124-126, Research 121, Research 175, Research
192, the Claude Agent prepared guide, exact route preparation/command/
validation/parser code and fixtures, and the canonical contracts from the
selected worker worktree.

Take card 124 as one coherent evidence chunk. Use current official sources,
the exact extracted `2.1.238` package, and deterministic secret-free specimens;
do not install or send a live prompt. If Research 192 has no deliver-now row,
close cards 125-126 as blocked, finish the route-local stop record, validate,
and open the evidence PR. If an exact bounded row survives, execute cards
125-126 in order and open one implementation PR. At each natural pause, tell
the operator what changed, what validation ran, what remains, and whether a
planning decision is needed.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch; do not create another worktree merely because they
   differ from the placeholders.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the
   operator if it is absent. Create a unique worktree and branch under that
   container from `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path.
   If the launcher supplied a dirty or `main` worktree, stop and report it
   instead of silently creating a second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 2fa3e761c531f20af4415a5297b209b41732532a HEAD`
   succeeds, and confirm this handoff exists in the selected `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, guide, relevant research and
   implementation, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline separately from lane-created failures.

### While you work

- Execute the ready cards in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the required final validation named in Current State and card 126. If
   card 124 stops the lane, run its acceptance gates plus every applicable
   route-local/index gate and explain why binding-only gates did not run.
2. Update Research 192, cards, milestone, route-local closeout, applicable
   guide/API baseline, and actual worktree/branch evidence. Keep the shared
   surfaces listed above unchanged.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The handoff's
   `2fa3e761c531f20af4415a5297b209b41732532a` is the planning base before this
   handoff commit, not a self-referential hash for the commit containing it.
5. In the PR body, link g04.045, cards 124-126, Research 192, changed surfaces,
   exact source/specimen evidence, validation, stop/delivery truth, and
   unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

The orchestrator records an evidence-backed verdict in the provider review
surface. When orchestrator and worker share a GitHub identity, formal
self-approval is unavailable, so the orchestrator posts the verdict as a PR
comment. That comment is the canonical review record. If changes are
requested, make only those changes on this branch, push again, and report back
through the operator. Requested changes are: none. The operator must explicitly
authorise any merge.

- **Closeout refs:** Research 192; cards 124-126; g04.045; reserved Claude Code
  headless structured-output closeout; `docs/roadmaps/README.md` sole Next Task
  after orchestrator merge closeout

### Handoff closeout

Before calling the runway complete, leave the card, milestone, research, log,
and next-task state honest. If the work is blocked, record the blocker and stop
rather than making the handoff look more complete than it is.
