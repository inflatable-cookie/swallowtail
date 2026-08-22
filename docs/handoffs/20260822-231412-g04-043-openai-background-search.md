---
title: g04.043 OpenAI background hosted search worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260822-231412-g04-043-openai-background-search.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator closed the Cline evidence lane, resumed the sole roadmap Next
Task, reassessed the promoted per-route feature inventory, and compiled
g04.043. OpenAI background search work has not started. The ready runway begins
with exact official evidence and permits implementation only for Research 191
deliver-now rows.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. The worker can start from this file without a copied
transcript or a second prompt.

## Why It Matters

`openai.background` already owns exact `gpt-5.6` Responses background,
streaming, temporary-retention, reattachment, retrieval, cancellation,
deletion, detachment, reconciliation, reasoning, and structured-output truth.
It rejects external search today. Current official OpenAI surfaces expose a
provider-owned `web_search` tool, a total tool-call bound, and optional complete
source evidence. That makes a narrow Contract 041 mapping plausible, but does
not prove that the tool composes with this route's exact request and retained
lifecycle. The lane must settle that before changing a capability claim.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `2d969cbb79d3f660b2e9f97657a01c487d4d3dc4`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `2d969cbb79d3f660b2e9f97657a01c487d4d3dc4` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.043, cards 119-121,
  Research 191 reservation, compilation log, closeout reservation, triage
  selection, and updated sole Next Task
- **Worker branch:** `agent/g04-043-openai-background-search-20260822-231412`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-043-openai-background-search-20260822-231412`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-043-openai-background-search-20260822-231412 -b agent/g04-043-openai-background-search-20260822-231412 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path and branch;
  do not create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches. Only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree and branch under that container from `origin/main`.
  Ask the operator if the file or key is absent. Never use `/tmp`, `TMPDIR`, or
  a guessed path.
- **Active spec lane:** per-route feature completion; no spec edit
- **Roadmap milestone:** `docs/roadmaps/g04/043-openai-background-hosted-search.md`
- **Ready cards, in order:** `119-openai-background-search-evidence.md`, then
  conditional `120-openai-background-search-binding.md`, then conditional
  `121-openai-background-search-acceptance.md`
- **Allowed runway:** exact `gpt-5.6` Responses `web_search` evidence on
  `openai.background`, then only Research 191 deliver-now binding
- **Remaining card budget:** three cards; cards 120-121 execute only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane
- **Parallel safety check:** serial by design; every card shares the OpenAI
  adapter, prepared facade, request encoder, lifecycle fixtures, guide,
  research record, and closeout
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  009, 010, 011, 014, 021, 029, 037, 041, 044, and 052
- **Route identity:** `openai.background`, driver
  `swallowtail.openai.background`, exact route
  `openai.public.gpt-5.6.background`, axis
  `openai.responses-background-facade`, current point
  `openai-responses-background-2026-07-21`
- **Model capability profile:** exact-model, exact-facade, evidence-first
  implementation; fail closed on tool, bound, source, event, or lifecycle
  ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no package install, login, credential/account
  inspection, project mutation, provider prompt, live request, or paid work;
  current public official source inspection and secret-free deterministic
  fixtures are allowed by card 119
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-openai`,
  `effigy validate:focused swallowtail-adapter-openai`,
  `effigy package:verify-affected swallowtail-adapter-openai`,
  `effigy check:examples`, `effigy qa:routes`, `effigy qa:northstar`, research,
  logs, roadmaps, g04, batch-card and next-action index gates,
  `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 371 inherited god-file findings: 326 warnings and
  45 errors, plus stale-graph and generated-in-src warnings
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; worker must not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-openai/**`;
  `docs/guides/openai-background-prepared-integration.md`; Research 191;
  g04.043; cards 119-121; the reserved g04.043 route-local closeout; the
  OpenAI adapter package-specific unreleased public-API baseline when
  applicable; exact public official evidence; deterministic secret-free
  request, stream, retrieve, source, usage, and failure fixtures
- **Out of scope:** arbitrary Responses tools, functions, MCP, file/image
  search, consumer callbacks, Codex search, sibling OpenAI routes, search
  filters or raw provider options, model expansion, service tier, live
  provider work, currentness changes, contracts, `CHANGELOG.md`, shared
  architecture, route/feature matrices, programme/front doors/indexes, matrix
  assertions, shared package lists, release, publication, or merge work
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product, API, persistence, or security decision.
- Do not backfill the existing July facade point. Research 191 must decide
  whether exact additive evidence supports it or requires a new opaque private
  behavior revision.
- This handoff represents one worker lane. Do not edit another lane's scope. If
  shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Selection reason:** the route, exact model, lifecycle, portable external-
  search policy, provider-network capability, activity vocabulary, and tool-
  admission contract already exist. Remaining candidates need wider model,
  permission, process-topology, or vocabulary decisions.
- **Evidence gate:** card 119 must freeze current official web-search,
  Responses create/retrieve/stream/background, and model evidence with dates,
  stable specimens, and SHA-256 digests. It must distinguish `web_search` from
  legacy `web_search_preview`, other tools, and other OpenAI search surfaces.
- **Decisions and preferences:** provider-owned search is optional, bounded,
  and separate from consumer tools. Enablement is not invocation. Invocation
  is not source or citation delivery. None of those proves usage, billing, or
  assistant output. Do not leak query text, sources, or provider bodies into
  diagnostics.
- **Compatibility burden:** classify `background=true`, `stream=true`,
  `store=false`, the positive output bound, reasoning, structured output, one
  stream reattachment, retrieve, cancel, delete, detachment, and restart
  reconciliation. Do not assume independently documented fields compose.
- **Bound burden:** a positive total tool-call maximum must remain immutable
  across input, policy, plan, evidence, driver, and wire. No unbounded or
  provider-default path is deliver-now.
- **Absent path:** when search is not selected, preserve the existing tool-free
  request bytes and all current route behavior.
- **Honest stop:** an empty Research 191 deliver-now set is a successful
  evidence result. Close cards 120-121 as blocked and open the evidence PR.
- **Known baseline:** do not claim or repair the inherited doctor findings
  unless this lane creates distinct friction. Record new Northstar friction in
  `PAPERCUTS.md`.
- **Report after:** card 119's exact route/request decision. Continue only for
  a non-empty deliver-now set, then report after the complete cards 120-121
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

Read `AGENTS.md`, g04.043, cards 119-121, Research 191, the existing OpenAI
background guide, the exact route preparation/request/lifecycle code and
fixtures, and the canonical contracts from the selected worker worktree.

Take card 119 as one coherent evidence chunk. Use current official OpenAI
sources and deterministic secret-free specimens; do not send a live request.
If Research 191 has no deliver-now row, close cards 120-121 as blocked, finish
the route-local stop record, validate, and open the evidence PR. If a bounded
exact row survives, execute cards 120-121 in order and open one implementation
PR. At each natural pause, tell the operator what changed, what validation ran,
what remains, and whether a planning decision is needed.

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
   container from pushed `origin/main`. Never use `/tmp`, `TMPDIR`, or a
   guessed path. If the launcher supplied a dirty or `main` worktree, stop and
   report it instead of silently creating a second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 2d969cbb79d3f660b2e9f97657a01c487d4d3dc4 HEAD`
   succeeds, and confirm this handoff exists in the selected `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, guide, relevant implementation, and
   canonical refs.
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

1. Run the required final validation named in Current State and card 121. If
   card 119 stops the lane, run its acceptance gates plus every applicable
   route-local/index gate and explain why binding-only gates did not run.
2. Update Research 191, cards, milestone, route-local closeout, applicable
   guide/API baseline, and actual worktree/branch evidence. Keep the shared
   surfaces listed above unchanged.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The handoff's
   `2d969cbb79d3f660b2e9f97657a01c487d4d3dc4` is the planning base before this
   handoff commit, not a self-referential hash for the commit containing it.
5. In the PR body, link g04.043, cards 119-121, Research 191, changed surfaces,
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

- **Closeout refs:** Research 191; cards 119-121; g04.043; reserved OpenAI
  background search closeout; `docs/roadmaps/README.md` sole Next Task after
  orchestrator merge closeout

### Handoff closeout

Before calling the runway complete, leave the card, milestone, research, log,
and next-task state honest. If the work is blocked, record the blocker and stop
rather than making the handoff look more complete than it is.
