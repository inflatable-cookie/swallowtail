---
title: route feature and option projection census worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-30
updated: 2026-08-30
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260830-163912-route-feature-option-projection-census.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, research, triage, capabilities]
---

## What This Thread Was Doing

The operator wants consuming applications to query one cohesive Swallowtail
surface for route/model features and controls, including whether each control
is fixed at session start, supplied per turn, negotiable between turns,
separately proved mutable mid-turn, or observable only after open.

The idea is still triage. The operator authorized its evidence census to run in
parallel with the Claude Code `2.1.251` currentness worker. This handoff owns
that census only. It does not promote a contract, compile implementation cards,
or change runtime code.

## Why It Matters

Swallowtail already has capability profiles, configured-instance catalogues,
route readiness, model observations, prepared inputs, and negotiated session
state. Consumers can assemble those pieces themselves, but that duplicates
Swallowtail semantics and risks false UI: route-wide support shown for an
incompatible model, post-open observation shown as mutable configuration, or
temporary unavailability flattened into unsupported.

The census establishes what truth exists today and which lifecycle distinctions
a future cohesive projection must preserve. Running it as a disjoint triage
lane keeps the broader design moving without colliding with currentness or
watcher implementation.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `6f5f13cc126eb8aedeb6b69e7b22739ac43d01de`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `6f5f13cc126eb8aedeb6b69e7b22739ac43d01de` before this handoff was created
- **Planning checkout:** clean before the handoff commit
- **Worker mode:** evidence worker dispatched by the orchestrator; the required
  metadata uses Northstar's implementation-worker preflight, but no production
  implementation is authorized
- **Planning artifacts included at the base:** the open projection triage note
  now contains the exact parallel evidence scope, row requirements, and
  no-promotion boundary
- **Worker branch:** `worker/route-feature-option-projection-census`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-route-feature-option-projection-census`
- **Worktree creation command:** `git worktree add -b worker/route-feature-option-projection-census /Users/tom/Dev/worktrees/swallowtail-route-feature-option-projection-census origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even when its generated path
  or branch differs from the placeholders above. Do not create a second
  worktree for a naming difference. If that context is unusable, use the named
  worktree; only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique fallback there. Never
  use `/tmp` or a guessed worktree path
- **Required sibling worktree links:** none
- **Active spec lane:** none; this is pre-promotion evidence under
  `docs/triage/2026-08-30-consumer-route-feature-and-option-projection.md`
- **Roadmap milestone:** none; no execution milestone is authorized before the
  census returns to the orchestrator
- **Assigned evidence packet:** the triage note's `Parallel Evidence Dispatch`
  and `Promotion Gate`
- **Allowed runway:** update the existing triage note with findings and add
  `docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv`
- **Remaining budget:** one evidence tranche; one reviewable PR
- **Dispatch topology:** parallel with g05.005 currentness
- **Parallel safety check:** this worker may change only the projection triage
  note and its new CSV. The currentness worker does not own either path. This
  worker must not touch the shared research, log, roadmap, matrix, changelog,
  code, or API-baseline surfaces used by currentness
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/contracts/037-prepared-consumer-integration.md`,
  `docs/contracts/047-configured-provider-instance-catalogue.md`, and
  `docs/contracts/057-route-readiness-and-connection-admission.md`
- **Evidence refs:** `docs/guides/provider-route-matrix.md`,
  `docs/guides/provider-solution-feature-matrix.csv`,
  `docs/roadmaps/g04/per-route-feature-inventory.md`,
  `docs/triage/2026-08-19-route-readiness-facade.md`, and
  `docs/triage/2026-08-21-advanced-route-features.md`
- **Primary code evidence:** public types in `swallowtail-core`; preparation,
  configured-instance, catalogue, and negotiation surfaces in
  `swallowtail-runtime`; every production adapter's public prepared input and
  selection types
- **Coverage baseline:** current release documentation names 47 production
  routes. The worker must derive the live source route set from current `main`,
  not copy this number blindly
- **Model capability profile:** fast or balanced evidence-reduction model with
  strong code navigation and tabular normalization; no model override required
- **Tool/runtime restrictions:** no subagents, provider contact, web research,
  prompts, login, credentials, live probes, installs, code edits, contract
  edits, numbered research, roadmap/card/index edits, matrix edits, API changes,
  release work, or merge
- **Inherited health baseline:** doctor reports 384 god-file findings: 337
  warnings and 47 errors, plus one generated-in-source warning and a stale
  graph index. This docs-only census must not become structural cleanup
- **Required validation:** `effigy qa:docs`; `effigy qa:northstar`;
  a deterministic CSV coverage check proving every current production route
  appears and every row has the required columns; `git diff --check`; exact
  changed-path proof limited to the triage note and census CSV
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting evidence and exact head
- **Merge authorization:** not authorized

## Boundaries

- **In scope:** repository-local census of current production routes, public
  semantic features, public consumer-selectable controls, current source
  ownership, applicability, value domains, omission, lifecycle/mutability,
  evidence strength, projection coverage, and exact gaps.
- **Out of scope:** public API design, contract promotion, implementation,
  adapter changes, provider research, new route or feature claims, matrix
  correction, currentness, watcher work, skill discovery, consumer UI schema,
  product defaults, localization, routing policy, numbered research, shared
  indexes, and merge.
- **Outcome shape:** one evidence CSV plus a concise synthesis appended to the
  existing open triage note. Unknown or unavailable remains an honest census
  result. Do not turn ambiguity into invented semantics.
- Use one row per exact route/feature or route/control applicability. Add a
  `route-audit` row when a production route exposes no public selectable
  control, so every route has explicit coverage without constructing a false
  route-by-feature Cartesian product.
- Distinguish row kinds `feature`, `control`, and `route-audit`. A feature is
  descriptive truth; a control is caller-selectable or provider-negotiable;
  route-audit records coverage only.
- Required CSV columns:
  `row_kind`, `route_id`, `operation_shape`, `semantic_id`, `public_source`,
  `owning_package`, `value_kind`, `value_domain`, `omission_semantics`,
  `applicability`, `lifecycle`, `state_support`, `evidence_source`,
  `evidence_strength`, `current_projection`, `gap_or_non_claim`.
- Lifecycle values are exactly: `selection-summary`, `session-start-only`,
  `per-turn`, `between-turn-negotiable`, `mid-turn-negotiable`,
  `post-open-observation-only`, or `not-applicable`. Use `unknown` only when
  current authority cannot classify the row, and explain why in
  `gap_or_non_claim`.
- Do not equate a route capability, model catalogue value, accepted input,
  dispatched flag, provider-effective value, or negotiated acknowledgement.
  Record the strongest evidenced layer and the missing layer separately.
- Do not infer mutation from an option list or successful local setter. Do not
  infer model applicability from a route-wide capability. Documentation
  matrices are cross-checks, never runtime authority.
- The triage note remains `Status: open`. Do not reserve a research number or
  claim a contract direction. The orchestrator owns promotion after review.
- Work only in the selected clean worker worktree. Preserve unrelated state.
  Do not merge the PR.

## Important Context

- **Planning lineage:** Contract 047 permits low-level consumers to assemble
  configured-instance, route, capability, and model observations. Contract 057
  adds addable-route/readiness/configuration descriptors. Neither currently
  promises one cohesive feature/control projection. The triage note proposes
  three views—selection summary, session-start controls, and active-session
  controls—but that remains a hypothesis until this census proves the source
  truth.
- **Why this evidence lane is ready:** the operator has settled the desired
  consumer outcome and lifecycle distinction. Existing repository types and
  matrices provide bounded evidence. No external provider or product decision
  is required to inventory current truth.
- **Why it is not numbered research yet:** the lane is deliberately isolated
  from currentness's shared indexes. Its output returns to the orchestrator for
  promotion, contract selection, and roadmap compilation after parallel work
  lands.
- **Operator preference:** applications such as Nucleus should receive one
  cohesive interface rather than adapter downcasts or unstructured queries.
  Swallowtail describes truth; applications still own UI, persistence, and
  routing policy.
- **Open tensions:** some options may be accepted through generic types without
  enumerable route-local domains; some catalogues describe values without
  mutation; some controls may be fixed at open while others apply per turn.
  Record these as evidence gaps rather than normalizing them away.
- **Report after:** route/control source inventory and CSV coverage pass, then
  final synthesis and PR validation
- **Report to:** the operator, who will relay progress and the PR to the
  orchestrator

## Suggested Next Move

Start with the worker preflight. Use `effigy graph explore` for ownership and
flow questions, then `rg` for exact public symbols and final coverage proof.
Derive the current production route set from the route matrix and adapter
descriptors. Inventory public control/input types by source rather than by
provider marketing name.

Build the CSV in coherent families, then validate route coverage and required
columns. Append a short synthesis to the triage note: counts, authoritative
source classes, lifecycle distribution, routes with exact active-session
acknowledgement, unenumerated domains, unsafe inferences, and which promotion
questions evidence can now answer. Do not propose Rust types or promote a
contract in this worker.

## Completion Protocol

### Before you start

1. Read this handoff. Its worker metadata activates worker mode. Before broad
   reads, run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
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
   `git merge-base --is-ancestor 6f5f13cc126eb8aedeb6b69e7b22739ac43d01de HEAD`,
   and load the tracked handoff with
   `git show HEAD:docs/handoffs/20260830-163912-route-feature-option-projection-census.md`.
   If the absolute file differs, stop. The tracked copy is canonical.
5. Required sibling worktree links are `none`.
6. Read `AGENTS.md`, `PAPERCUTS.md`, the projection triage note, Contracts 037,
   047, and 057, the route and feature matrices, g04 feature inventory, the two
   related triage notes, and the relevant public core/runtime/adapter types.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited baseline; do not execute the broad workspace plan.

### While you work

- Change only
  `docs/triage/2026-08-30-consumer-route-feature-and-option-projection.md` and
  `docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv`.
- Use graph navigation for ownership questions and exact-token search for final
  symbol and route coverage. Read code and docs; do not modify them.
- Cover every current production route. Use explicit route-audit rows for
  routes with no selectable control. Do not manufacture irrelevant negative
  combinations.
- Cover every public consumer-selectable control type found in current source.
  Preserve route/model/access/resource applicability and lifecycle.
- Record unknown, conditional, unavailable, and unsupported separately. Do not
  infer provider-effective state or mutation acknowledgement.
- Validate the CSV mechanically for required headers, allowed lifecycle values,
  duplicate exact keys, and full route coverage. Keep the validation command in
  the PR evidence; do not add a repository script.
- Append synthesis only after the census is complete. Keep the note open and
  return promotion questions to the orchestrator.
- Do not spawn internal agents. The operator owns thread parallelism in their
  harness.
- Stop if accurate coverage requires provider contact, a product decision, a
  code change, or editing any path outside the two-file boundary.

### When the assigned runway is complete

1. Run `effigy qa:docs`, `effigy qa:northstar`, the deterministic CSV coverage
   check, and `git diff --check`.
2. Prove the changed-path set contains exactly the existing triage note and new
   census CSV. If any other file changed, restore only this worker's own
   unintended edit; never discard unrelated work.
3. Keep the note `Status: open`. Do not add Research 262, contracts, roadmap
   cards, logs, indexes, matrix corrections, code, or API proposals.
4. Push the selected worker branch and open one reviewable PR against current
   pushed `main`.
5. In the PR body, link the triage note, census CSV, Contracts 037/047/057,
   route coverage evidence, lifecycle classification counts, unresolved gaps,
   validation, and this no-promotion boundary.
6. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator will review the exact PR head for source fidelity, complete
route/control coverage, lifecycle distinctions, non-claims, two-file scope,
and hosted checks. With the shared GitHub identity, the canonical verdict may
be a PR comment rather than formal self-approval. Requested changes are `none`
at dispatch. Merge remains separately authorized by the operator.

- **Closeout refs:** the open projection triage note and its census CSV only
- **Promotion:** after merge, the orchestrator decides whether to create
  numbered research, amend Contracts 037/047/057 or add a composing contract,
  and compile a later implementation runway

### Handoff closeout

Return one evidence PR and stop. Do not continue into API design, contract
promotion, roadmap compilation, or implementation. Unknowns and empty sets are
valid evidence when current repository authority cannot support a stronger
claim.
