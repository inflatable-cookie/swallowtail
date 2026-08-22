---
title: g04.042 Cline thinking controls worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260822-220034-g04-042-cline-thinking-controls.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator closed Qwen headless reasoning delivery, resumed the sole
roadmap Next Task, and compiled g04.042 as the next route-local feature lane.
Cline thinking work has not started. The ready runway begins with exact
package evidence and permits implementation only for independently qualified
ACP or headless route/value rows.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

The per-route feature programme is working through current matrix failures one
route and one coherent control family at a time. `cline.acp` and
`cline.headless` both omit the official `--thinking` control, but sharing
`cline@3.0.55` does not make their transport, lifetime, model dependence, or
capability truth interchangeable. Current official sources also conflict on
whether omission is off or defaults to medium. Exact package evidence must
settle that before Swallowtail exposes portable reasoning selection.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `c07a184e3ec212d9e5ab78142ed4eac7ee45e97d`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `c07a184e3ec212d9e5ab78142ed4eac7ee45e97d` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** commit `c07a184e`, g04.042,
  cards 116-118, Research 190 reservation, compilation log, closeout
  reservation, triage promotion, and updated sole Next Task
- **Worker branch:** `agent/g04-042-cline-thinking-20260822-220034`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-042-cline-thinking-20260822-220034`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-042-cline-thinking-20260822-220034 -b agent/g04-042-cline-thinking-20260822-220034 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual path/branch
  and never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask the
  operator first if the file or key is absent; never use `/tmp`, `TMPDIR`, or a
  guessed path.
- **Active spec lane:** per-route feature completion; no spec edit
- **Roadmap milestone:** `docs/roadmaps/g04/042-cline-thinking-controls.md`
- **Ready cards, in order:** `116-cline-thinking-control-evidence.md`, then
  conditional `117-cline-thinking-control-binding.md`, then conditional
  `118-cline-thinking-control-acceptance.md`
- **Allowed runway:** exact Cline `3.0.55` thinking evidence for
  `cline.acp` and `cline.headless`, then only Research 190 deliver-now binding
- **Remaining card budget:** three cards; cards 117-118 execute only after
  their named gates
- **Dispatch topology:** one serial worker lane
- **Parallel safety check:** serial by design; the cards share one Cline crate,
  package fixtures, preparation/driver surfaces, guides, research, and closeout
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 029, 033, 037, 040, 050, and 052
- **Model capability profile:** exact-package, two-transport, evidence-first
  implementation; fail closed on model/default/normalization ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no package install, login, credential/account
  inspection, user/project config mutation, live catalogue, provider prompt,
  or live Cline session; disposable registry package extraction and official
  source inspection are allowed by card 116
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-cline`,
  `effigy validate:focused swallowtail-adapter-cline`,
  `effigy package:verify-affected swallowtail-adapter-cline`,
  `effigy check:examples`, `effigy qa:routes`, `effigy qa:northstar`, research,
  logs, roadmaps, g04, batch-card and next-action index gates,
  `effigy package:api`, and `git diff --check`
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; worker must not merge

## Boundaries

Please keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-cline/**`;
  `docs/guides/cline-acp-prepared-integration.md`;
  `docs/guides/cline-headless-prepared-integration.md`; Research 190; g04.042;
  cards 116-118; the reserved g04.042 route-local closeout; the Cline
  package-specific unreleased public-API baseline; deterministic exact-package
  evidence and fixtures
- **Out of scope:** Cline model, plan, config, timeout, retry, permission,
  tool, search, team, hub, load/resume, or other route work; generic argv or
  configuration APIs; contracts; `CHANGELOG.md`; shared architecture,
  route/feature matrices, programme/front doors/indexes, matrix assertions,
  either shared `packages.txt`; currentness changes; consumer, release,
  publication, live-provider, login, install, or merge work
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's assigned
  scope; if shared mutable scope or a hidden dependency appears, stop and report
  it through the operator.
- Work only in the selected clean worker worktree: prefer the current
  launcher-provided worktree and record its actual path/branch; otherwise use
  the named fallback path/branch above, or the recorded local-path fallback
  created by the startup preflight. Never edit the orchestrator's planning
  checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** g04.035-039 delivered the initial feature sequence;
  g04.040 stopped after exact Copilot evidence found model-default
  substitution; g04.041 delivered Qwen's exact model-qualified control; the
  sole Next Task now points to g04.042 cards 116-118.
- **Why these cards are ready:** Cline `3.0.55` is already qualified on the
  `cline.package` axis, both production routes have prepared facades, and
  current official docs expose a bounded named value surface. The uncertainty
  is isolated into card 116 with separate ACP/headless stop conditions.
- **Decisions and preferences:** evidence first; an empty deliver-now set or a
  one-transport subset is valid. Sharing one package does not share a claim.
  Omission is not a selected portable value. Upstream `none` maps to portable
  `off` only with exact semantic evidence. Do not accept aliases, clamps,
  defaults, persisted settings, or inferred provider/model capability.
- **Open tensions:** current official CLI reference says `--thinking` defaults
  to medium while the official CLI README says omission is off; exact `3.0.55`
  may differ from both. Neither route selects a model today. ACP selection is
  fixed at child spawn and must repeat on fresh replacement; headless selection
  must remain one-run private. A private behavior revision may be needed at the
  exact qualified point.
- **Known baseline:** `effigy doctor` reports the inherited 371 god-file
  findings (326 warnings, 45 errors), stale graph warning, and generated-in-src
  warning. Do not claim or repair them unless this lane creates distinct
  friction. Record any distinct Northstar friction in `PAPERCUTS.md`.
- **Report after:** card 116's two-transport evidence decision; if it admits a
  subset, report again after the complete cards 117-118 implementation and
  validation chunk
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the top.
Before broad repository reads, run the quick startup worktree-safety preflight in
`## Completion Protocol`. If the current context is a clean, dedicated,
non-`main` registered worktree, it is the launcher-provided worktree: use it
immediately, record its actual path/branch, and do not compare its generated
path/branch with this handoff or create another worktree. If it is `main`, dirty,
unregistered, or otherwise unusable, use the named worktree if it matches; only
then read `.agents.local.env`, require a valid
`AGENTS_WORKTREE_CONTAINER_DIR`, ask the operator if it is absent, and create a
unique manual worktree and branch under that container from pushed
`origin/main`. Never fall back to `/tmp` or `TMPDIR`. Do not run broad repo
orientation before this decision. Read `AGENTS.md`, the milestone, cards
116-118, both existing Cline identity records and guides, and the canonical
contracts from the selected worker worktree.

Take card 116 as one coherent evidence chunk. Use exact official sources and a
disposable `cline@3.0.55` package extraction without installing or launching a
provider session. If Research 190 has no deliver-now row, close cards 117-118 as
blocked, finish the route-local stop record, validate, and open the evidence PR.
If only one transport survives, implement only that transport. If both survive,
execute cards 117-118 in order for their independently admitted rows and open
one implementation PR. At each natural pause, tell the operator what changed,
what validation actually ran, what remains, and whether a planning decision is
needed.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Then run
   one quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare them with the fallback path/branch or
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
   `git merge-base --is-ancestor c07a184e3ec212d9e5ab78142ed4eac7ee45e97d HEAD`
   succeeds, and confirm this handoff file exists in the selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline separately from lane-created failures.

### While you work

- Execute the ready cards in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop and say so if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into a new architecture.

### When the assigned runway is complete

1. Run the required final validation named in Current State and card 118. If
   card 116 stops one or both routes, run its acceptance gates plus every
   applicable route-local/index gate and explain why binding-only gates did not
   run.
2. Update Research 190, cards, milestone, route-local closeout, applicable
   guides/API baseline, and actual worktree/branch evidence. Keep the shared
   surfaces listed above unchanged.
3. Push the selected worker branch.
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   `c07a184e3ec212d9e5ab78142ed4eac7ee45e97d` is the planning base before this
   handoff commit, not a self-referential hash for the commit containing it.
5. In the PR body, link the milestone, cards, Research 190, changed surfaces,
   exact source/package evidence, validation, stop/delivery truth, and
   unresolved items.
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
Research 190, changed surfaces, evidence, validation, and unresolved items. The
operator must explicitly authorise any merge.

- **Closeout refs:** Research 190; cards 116-118; g04.042; reserved Cline
  thinking-controls closeout; `docs/roadmaps/README.md` sole Next Task after
  orchestrator merge closeout

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is.
