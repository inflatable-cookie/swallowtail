---
title: g04.044 OpenAI background reasoning correction worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260823-002256-g04-044-openai-background-reasoning.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reviewed and fast-forwarded PR 42, closed g04.043 as an
honest OpenAI search evidence stop, and promoted its exact-model reasoning
finding into g04.044. No correction implementation has started. The ready
runway contains one route-local correction card and one acceptance card.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. The worker can start from this file without a copied
transcript or a second prompt.

## Why It Matters

`openai.background` claims exact GPT-5.6 reasoning selection. Research 191
proves the model's official set is `none|low|medium|high|xhigh|max`, but the
current guide and preparation validator also admit `minimal`. That makes the
production claim wider than its evidence. Contract 040 forbids translating an
unsupported value to the nearest tier or provider default. The route must
reject it before effects and version the corrected opaque facade truth.

The removal shrinks guaranteed behavior and is therefore breaking under
Contract 036. The code correction may land on unreleased source, but release
selection, version changes, tagging, and publication remain outside this lane.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `1b3b5bb243a14eeacc9475cc9efa595f4e009321`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `1b3b5bb243a14eeacc9475cc9efa595f4e009321` before this handoff commit
- **Planning checkout:** clean `main` after the planning-base push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.044, cards 122-123,
  compilation log, reserved route-local closeout, PR42 shared closeout, and the
  sole roadmap Next Task
- **Worker branch:** `t3code/openai-background-reasoning-correction`
- **Worker worktree:** `/Users/tom/.t3/worktrees/swallowtail/g04-044-openai-background-reasoning`
- **Worktree creation command:** launcher-owned when available; manual fallback
  is `git worktree add <AGENTS_WORKTREE_CONTAINER_DIR>/g04-044-openai-background-reasoning -b t3code/openai-background-reasoning-correction origin/main` only after the startup policy resolves the configured container
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path and branch;
  do not create a second worktree for that reason. If the current context is
  unusable, inspect the named worktree; only then read `.agents.local.env`,
  require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique manual worktree
  and branch under that container from `origin/main`. Ask the operator if the
  file or key is absent. Never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** per-route feature completion; no spec edit
- **Roadmap milestone:** `docs/roadmaps/g04/044-openai-background-reasoning-vocabulary-correction.md`
- **Ready cards, in order:**
  `122-openai-background-reasoning-vocabulary-correction.md`, then
  `123-openai-background-reasoning-vocabulary-acceptance.md`
- **Allowed runway:** exact GPT-5.6 reasoning-vocabulary correction on
  `openai.background`, new exact opaque facade point, deterministic acceptance,
  and route-local closeout
- **Remaining card budget:** two cards
- **Dispatch topology:** one serial worker lane
- **Parallel safety check:** serial by design; both cards share the OpenAI
  validator, selection/facade claim, fixtures, guide, research follow-up,
  milestone, and closeout
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 029, 036, 037, 040, and 052; Research 191
- **Route identity:** `openai.background`, driver
  `swallowtail.openai.background`, exact route
  `openai.public.gpt-5.6.background`, model `gpt-5.6`, axis
  `openai.responses-background-facade`, current point
  `openai-responses-background-2026-07-21`
- **Model capability profile:** exact-model, exact-facade, fail-closed
  correction; six admitted values, no alias, default, clamp, retry, or fallback
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no install, login, credential/account inspection,
  project mutation, provider prompt, live request, paid work, release, tag, or
  publication
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
  `docs/guides/openai-background-prepared-integration.md`; Research 191's named
  follow-up disposition; g04.043's named follow-up line; g04.044; cards
  122-123; the reserved g04.044 route-local closeout; the OpenAI adapter's
  package-specific unreleased API baseline only if the semantic API checker
  requires it; deterministic secret-free route fixtures and tests
- **Out of scope:** web search, Responses tools, service tier, Fast mode,
  another model or OpenAI route, global `ReasoningMode` syntax, another
  adapter's reasoning set, live provider work, currentness expansion,
  contracts, `CHANGELOG.md`, release notes or workspace versions, shared
  architecture, route/feature matrices, programme/front doors/indexes, matrix
  assertions, shared package lists, release, publication, or merge work
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product, API, persistence, release, or security decision.
- Remove only `minimal` from this exact route. Do not translate it to `none`,
  use a provider default, clamp it, retry it, fall back, or select another
  route/model.
- Give the corrected mapping a new exact opaque facade point and private
  behavior revision. Keep all bindings, claim membership, fixtures, and tests
  consistent; do not silently rewrite the July point.
- Preserve absent reasoning and exact `none|low|medium|high|xhigh|max` behavior,
  plus every existing output-bound, structured-output, retained-lifecycle,
  stream, cancellation, deletion, detachment, reconciliation, and cleanup
  guarantee.
- This handoff represents one worker lane. Do not edit another lane's scope. If
  shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** PR 42 merged Research 191 and g04.043 at exact head
  `685dbf1abf16ebc6f261343f472b6620b33e99d2`. The search deliver-now set is
  empty; cards 120-121 are blocked. Its named reasoning follow-up is now the
  sole g04.044 runway.
- **Why these cards are ready:** Research 191 already freezes the official
  exact-model page, retrieval date, and digest. No new external evidence or
  live probe is needed to identify the six admitted values and omitted
  `minimal`.
- **Correction site:** the current route guide names seven values and
  `crates/swallowtail-adapter-openai/src/prepared_profile/background.rs`
  admits the same seven in `validate_reasoning`. Existing prepared-facade tests
  prove `high` and reject `ultra`, but do not cover all six supported values or
  explicit `minimal` rejection.
- **Facade burden:**
  `crates/swallowtail-adapter-openai/src/selection.rs` binds the current opaque
  facade point and private behavior revision. Contract 029 requires a distinct
  point when adapter-private mapping changes. Update direct fixture and plan
  assertions consistently and prove stale binding drift fails closed.
- **Compatibility burden:** removal of a guaranteed route value is a Contract
  036 breaking change. Record the required next-minor delta in the route-local
  closeout and PR body. Do not edit versions, release notes, tags, or publish.
- **API baseline:** the expected correction changes behavior, not public Rust
  signatures. Run `effigy package:api`; update a package-specific unreleased
  baseline only if the semantic tool reports a legitimate route-local delta.
- **Claim precision:** prove qualified dispatch only. Do not claim provider
  acceptance, effective reasoning depth, or observed reasoning from request
  encoding.
- **Known baseline:** do not repair inherited doctor findings unless this lane
  creates distinct friction. Record new Northstar friction in `PAPERCUTS.md`.
- **Report after:** card 122's complete validator/facade correction and focused
  validation, then after card 123's full acceptance and PR-ready closeout
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top. Before
broad repository reads, run the quick startup worktree-safety preflight in
`## Completion Protocol`. If the current context is a clean, dedicated,
non-`main` registered worktree, use it immediately, record its actual path and
branch, and do not compare its generated identity with the placeholders above.
If it is unusable, inspect the named worktree; only then read
`.agents.local.env` and follow its required container setting. Never fall back
to `/tmp` or `TMPDIR`.

Read `AGENTS.md`, g04.044, cards 122-123, Research 191, g04.043's named
follow-up, the OpenAI background guide, exact selection/preparation/driver
code and fixtures, and Contracts 029, 036, and 040 from the selected worker
worktree.

Take card 122 as one coherent correction chunk. Keep global vocabulary and
other routes untouched. Once the new exact facade point, six-value admission,
explicit `minimal` rejection, absent path, and zero-effect failures pass, take
card 123 as the acceptance/closeout chunk. Push one branch and open one PR.
At each named report boundary, tell the operator what changed, validation
actually run, what remains, and whether a planning decision is needed.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch; do not compare them with the placeholder path/branch
   or create another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the
   operator if it is absent. Create a unique worktree and branch under that
   container from pushed `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed
   path; never clean, reset, stash over, or discard the original checkout's
   state. If the launcher supplied a dirty or `main` worktree, stop and report
   it instead of silently creating a second worktree.
4. From the selected worktree, run `git fetch origin`; confirm
   `git rev-parse HEAD` equals `git rev-parse origin/main`; confirm
   `git merge-base --is-ancestor 1b3b5bb243a14eeacc9475cc9efa595f4e009321 HEAD`
   succeeds; and confirm this handoff exists in the selected `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, Research 191, route guide, relevant
   implementation, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline separately from lane-created failures.

### While you work

- Execute cards 122-123 in order and keep commits aligned with the two
  meaningful chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into new architecture or release work.

### When the assigned runway is complete

1. Run the required final validation named in Current State and card 123.
2. Update cards 122-123, g04.044, Research 191's follow-up disposition,
   g04.043's named follow-up line, the OpenAI guide, the route-local closeout,
   applicable fixtures/tests, and the package API baseline only if required.
   Keep every shared surface listed above unchanged.
3. Record the actual worktree/branch, base, validation, and exact shared
   next-minor closeout delta. Do not claim review, merge, version selection, or
   release.
4. Push the selected worker branch.
5. Open one reviewable PR against the current pushed `main` tip. The handoff's
   `1b3b5bb243a14eeacc9475cc9efa595f4e009321` is the planning base before this
   handoff commit, not a self-referential hash for the commit containing it.
6. In the PR body, link g04.044, cards 122-123, Research 191, changed surfaces,
   exact correction/facade evidence, validation, Contract 036 next-minor delta,
   and unresolved items.
7. Report the PR URL and evidence to the operator. Do not merge.

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

- **Closeout refs:** cards 122-123; g04.044; Research 191; g04.043 named
  follow-up; reserved g04.044 route-local closeout; `docs/roadmaps/README.md`
  sole Next Task after orchestrator merge closeout

### Handoff closeout

Before calling the runway complete, leave the cards, milestone, research,
guide, log, and follow-up state honest. If the work is blocked, record the
blocker and stop rather than making the handoff look more complete than it is.
