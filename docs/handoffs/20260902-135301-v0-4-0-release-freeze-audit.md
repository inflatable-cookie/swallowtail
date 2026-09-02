---
title: v0.4.0 release freeze audit worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260902-135301-v0-4-0-release-freeze-audit.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, release]
---

## What This Thread Was Doing

Swallowtail has frozen feature and currentness implementation to prepare the
next source release. PR 184 compiled g05.021 and made card 050 the sole ready
card: audit the full `v0.3.3` to current-source compatibility boundary and
freeze one exact reviewed census before any candidate mutation.

This dispatches that bounded audit lane. No transcript or second prompt is part
of the authority chain.

## Why It Matters

Contract 036 forces the next coordinated pre-1.0 release to `v0.4.0` because
current source removed the previously guaranteed OpenAI Background `minimal`
reasoning value. The repository has moved by more than 770 commits since
`v0.3.3`; the release cannot rely on the changelog or that known break alone.
Card 050 must classify the complete package, route, semantic API, and
guaranteed-behavior delta before candidate preparation can become ready.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `212ebfcf9263a1a75c3127679bc792cbff8704b4`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `212ebfcf9263a1a75c3127679bc792cbff8704b4` before this handoff commit.
- **Planning checkout:** clean.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** merged PR 184, g05.021, cards
  050–052, and the release/currentness freeze.
- **Worker branch:** `worker/g05-card050-v0-4-0-freeze-audit`
- **Worker worktree:** intended Paseo worktree
  `/Users/tom/.paseo/worktrees/2ee7rnl8/g05-card050-v0-4-0-freeze-audit`;
  the launcher-supplied clean non-`main` worktree is authoritative when it
  differs.
- **Worktree creation command:** Paseo `branch-off` worktree from
  `origin/main`; manual fallback only through `AGENTS_WORKTREE_CONTAINER_DIR`
  under the Completion Protocol.
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none.
- **Active spec lane:** Contract 036 release and compatibility boundary.
- **Roadmap milestone:**
  `docs/roadmaps/g05/021-v0-4-0-release-readiness.md`.
- **Ready cards, in order:**
  `docs/roadmaps/g05/batch-cards/050-v0-3-3-to-candidate-compatibility-and-freeze-audit.md` only.
- **Allowed runway:** execute card 050 completely, record its audit evidence,
  reconcile its closeout, and open one reviewable PR.
- **Remaining card budget:** one card. Do not execute card 051 or 052.
- **Dispatch topology:** one serial release-readiness lane. No sibling worker
  launches beside it because card 051 consumes this audit and all release
  cards share candidate/front-door authority.
- **Parallel safety check:** feature/currentness implementation and non-gating
  papercuts are frozen; card 051 has an exact dependency on accepted card 050.
- **Surfaces this lane owns:** a new card-050 audit record under
  `docs/research/`, its research index entry, a card-050 closeout under
  `docs/logs/` and its index entry, card 050, g05.021, g05 and batch-card
  indexes, `docs/roadmaps/README.md`, `docs/roadmaps/generation-index.md`, and
  `docs/roadmaps/standing-lanes.md`. Temporary generated audit output must stay
  outside tracked release baselines.
- **Integration ownership:** this worker reconciles card 050 and promotes card
  051 to ready only if the complete audit passes its review oracle with no
  unresolved compatibility or operator choice. Otherwise record the stop and
  leave card 051 planned.
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if another lane
  merges first.
- **Canonical refs:** `docs/contracts/036-crate-release-and-compatibility-boundary.md`;
  `docs/research/276-all-route-version-currentness-checkpoint.md`;
  immutable tag `v0.3.3`; `config/release.toml`.
- **Review oracle:** card 050 `## Review Oracle` and g05.021 `## Review Oracle`.
- **Model capability profile:** long, mechanically oriented audit/documentation
  worker. Exact compatibility judgments remain bounded by card 050 and receive
  frontier exact-head review.
- **Frontier-worker justification:** none. The release is high priority, but
  card 050's partitions, evidence requirements, stop conditions, and review
  oracle bound the implementation reasoning; priority alone does not justify a
  frontier worker.
- **Tool/runtime restrictions:** read-only canonical fetches and temporary
  audit generation are allowed. No Cargo version/requirement change,
  changelog promotion, release baseline mutation, code/claim/fixture/workflow
  edit, release prepare/execute, tag, provider call, consumer mutation, or
  application smoke.
- **Required validation:** every command in card 050 `## Validation`, including
  package metadata/API, route QA, docs indexes/status/next-action/links,
  canonical roadmap numbering, Northstar, and `git diff --check`.
- **PR base/head:** base current pushed `main`; worker reports the exact pushed
  head.
- **PR URL:** pending.
- **Review state:** awaiting worker delivery, then frontier exact-head review.
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks.

## Boundaries

Please keep this run inside the named runway:

- **In scope:** card 050's exact base/head identity, full Git delta, 40-package
  and 48-route partitions, semantic API comparison, guaranteed-behavior audit,
  compatibility ledgers, immutable-baseline proof, candidate-input freeze, and
  honest planning/log closeout.
- **Out of scope:** cards 051–052; any version, dependency requirement,
  changelog-release-state, candidate baseline, production code, claim, fixture,
  CI workflow, release command, tag, provider, application, consumer-repo,
  currentness, feature, or papercut mutation.
- **Outcome shape:** evidence-backed audit and freeze. If any semantic break,
  route/package mismatch, baseline drift, overlapping mergeable feature or
  currentness PR, or unresolved compatibility policy prevents a complete
  partition, record the exact stop; do not force card 051 ready.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Write only inside **Surfaces this
  lane owns**. If shared mutable scope or hidden dependency appears, stop and
  report it instead of resolving it.
- Work only in the clean worker worktree selected by `Completion Protocol`.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge belongs to the orchestrator after accepted review
  and passing checks.

## Important Context

- **Planning lineage:** Research 276 selected a release pivot and parked Kimi
  local server `0.40.1` plus closed PR 182 post-release. PR 184 merged the
  bounded g05.021 runway at `212ebfcf`.
- **Why this card is ready:** Contract 036 fixes the version and source-only
  boundary; the package and route inventories are named; the audit acceptance,
  validation, evidence, stop conditions, and adversarial counterexamples are
  explicit.
- **Decisions and preferences:** keep historical release baselines immutable;
  treat changelog text as input, not proof; classify every changed and unchanged
  package/route; keep source-only Apple Silicon macOS and Rust `1.95.0` truth;
  do not infer the later authenticated application choice.
- **Open tensions:** the known `minimal` removal may not be the only break. The
  worker must audit the full delta and return any new policy choice rather than
  minimizing it.
- **Report after:** identity/inventory reconciliation, semantic API partition,
  guaranteed-behavior ledger, and final closeout/validation.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, Contract 036, g05.021, card 050, Research 276, and the immutable
`v0.3.3` baselines. Start with exact identity and census reconciliation before
generating semantic API evidence.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare its generated path/branch with the
   intended paths above or create another worktree merely because they differ.
3. If current context is `main`, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique worktree/branch there from pushed `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard dirty
   state. Report a launcher-supplied dirty or `main` worktree instead of
   creating another.
4. From the selected worktree, record this handoff's repository-relative path.
   Run
   `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm `git rev-parse HEAD` equals `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 212ebfcf9263a1a75c3127679bc792cbff8704b4 HEAD`
   succeeds, and confirm the relative handoff exists in selected `HEAD`. Load
   the tracked handoff with `git show HEAD:<relative-path>`. If the absolute
   dispatch file differs from that tracked blob, stop. The committed `HEAD`
   copy is canonical.
5. Required sibling links are `none`; skip sibling-link creation.
6. Read the active milestone, card 050, `AGENTS.md`, and canonical refs.
7. Use the card's first bounded commands for orientation. Do not run release
   prepare/execute or broad provider/live probes.

### While you work

- Execute only card 050 and keep commits aligned with meaningful evidence
  chunks, not arbitrary model turns.
- Preserve temporary audit output outside tracked release baselines. Do not
  overwrite historical files to make a diff easier.
- After each meaningful chunk, report changed evidence paths, validation run,
  remaining audit partitions, risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands, authority
  is missing, or validation changes the plan.
- Do not quietly turn an open question into a new compatibility policy.

### When the assigned runway is complete

1. Run every validation command in card 050.
2. Falsify the diff against card 050: enumerate every exact, universal, and
   negative claim; exercise the missing-public-item, omitted route behavior,
   and overwritten-historical-baseline counterexamples; map each to proof; and
   reconcile card, milestone, log, handoff, and front-door state.
3. Update card/log evidence with the exact base/head and actual worktree/branch.
4. Push the selected worker branch. If another lane changed `main`, refresh,
   revalidate, and report the changed head.
5. Open one reviewable PR against current pushed `main`.
6. Link the milestone, card, research/audit record, ledgers, changed surfaces,
   validation, and unresolved items in the PR body.
7. Report the PR URL and exact head. Do not merge.

### Review and merge path

The orchestrator reviews the PR against Contract 036, g05.021, card 050, the
full diff, and checks. Current review state: awaiting worker delivery.

The orchestrator records its verdict on the PR. Formal self-approval may be
unavailable, so the canonical record may be a top-level PR comment. Requested
changes are: none yet. When the exact reviewed head is current, required checks
pass, the PR is mergeable, and no stricter rule or operator pause applies, the
orchestrator merges without another approval prompt.

- **Closeout refs:** card 050; g05.021; new audit research and closeout log;
  research/log/batch-card/g05/generation/standing-lane indexes; the sole
  `docs/roadmaps/README.md` Next Task.

### Handoff closeout

Leave card 050, g05.021, log, indexes, and Next Task honest. Promote card 051
only if the complete audit is accepted and no unresolved release policy remains.
If blocked, record the blocker and stop rather than making the release lane look
ready.
