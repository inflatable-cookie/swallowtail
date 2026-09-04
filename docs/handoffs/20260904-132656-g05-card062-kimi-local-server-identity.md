---
title: g05.026 Card 062 Kimi Code local server 0.40.1 identity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260904-132656-g05-card062-kimi-local-server-identity.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g05.026]
---

## What This Thread Was Doing

The coordinator is dispatching the single approved g05.026 Card 062 identity
lane after Chatterbox promoted and the operator confirmed the post-v0.4.0
currentness direction. This handoff activates implementation-worker mode.

## Why It Matters

Card 062 is the evidence gate for the first post-release Contract 029 family:
Kimi Code local server 0.40.1. It must establish identity and authority facts
before any claim range can change. Card 063 is gated behind an admitted segment.

## Current State

- **Repository:** `git@github.com:inflatable-cookie/swallowtail.git`
- **Planning branch:** `main`
- **Planning base commit:** `8cfe4f24b72e192db5df4be434c2bb7255ddbb67`
- **Pushed main verification:** `HEAD == origin/main == 8cfe4f24b72e192db5df4be434c2bb7255ddbb67`
- **Planning checkout:** clean before handoff creation
- **Worker mode:** implementation worker dispatched by the orchestrator; this handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** g05.026 ready manifest, Card 062, release closeout, lifted freeze, and tag-push CI evidence
- **Worker branch:** `worker/g05-card062-kimi-local-server-0401-identity`
- **Worker worktree:** Paseo-managed worktree with slug `g05-card062-kimi-local-server-0401-identity`; use the launcher-provided actual root
- **Worktree creation command:** Paseo `create_workspace` with `isolation: worktree`, `mode: branch-off`, `baseBranch: origin/main`, and this branch
- **Worker worktree policy:** follow Completion Protocol; launcher worktree first, named/manual fallback only when required.
- **Required sibling worktree links:** `none`
- **Active spec lane:** Contract 029, with Contracts 017 and 023; Research 270, 276, and the card's new Research 282
- **Roadmap milestone:** `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g05/026-kimi-code-local-server-0-40-1-useful-newer.md`
- **Ready cards, in order:** `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g05/batch-cards/062-kimi-code-local-server-0-40-1-identity.md`
- **Allowed runway:** Card 062 only: freeze 0.40.1 identity and adjacency, recompute the 0.38.0 corpus, compare selected web/server v2 surfaces, trace the 0.40.0 Bash `cwd` authority change, and record exactly one outcome without changing claims.
- **Remaining card budget:** one card; stop is an acceptable outcome
- **Dispatch topology:** exactly one lane; no concurrent siblings
- **Parallel safety check:** no sibling lanes; all reserved shared closeout surfaces remain with the orchestrator
- **Surfaces this lane owns:** `docs/research/282-*.md`; one `docs/research/README.md` index line; local-server-only Kimi fixtures/corpus modules under `crates/swallowtail-adapter-kimi/**`; Card 062 result/status; append-only `PAPERCUTS.md`
- **Integration ownership:** orchestrator owns `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, g05.026, batch-card index, generation index, standing lanes, and `docs/logs/README.md` at closeout
- **Merge ordering:** same-repository PRs merge one at a time; no sibling is active
- **Canonical refs:** Contract 029; Contracts 017 and 023; Research 270 and 276; tagged `v0.4.0` at `56f3913ac99af44b6ff45384cfc53a0adea587ba`
- **Review oracle:** Card 062's evidence-only invariant: production claim bytes remain unchanged; exact identity evidence is recomputed; the authority conclusion traces a containing control or its absence. Counterexamples are a changed selection constant, widened/narrowed range, copied digest, or compatible verdict without a named Bash `cwd` boundary.
- **Model capability profile:** Grok Worker; ordinary bounded evidence-first implementation
- **Worker provider/model identity:** `grok/grok-4.6`
- **Frontier-worker justification:** `none`
- **Tool/runtime restrictions:** no provider credentials, prompt, login, install, host update, live server/session/catalogue request, downloaded-binary execution, release/publication/artifact work, or changes to ACP/headless/other Kimi routes
- **Required validation:** `effigy validate:focused swallowtail-adapter-kimi`; `effigy package:verify-affected swallowtail-adapter-kimi`; `effigy qa:northstar`; `git diff --check`
- **PR base/head:** current pushed `main` / worker branch head
- **PR URL:** pending
- **Review state:** awaiting worker PR, then independent exact-head review
- **Merge path:** orchestrator after accepted review of the current head and passing required checks

## Boundaries

- **In scope:** Card 062 only, as defined by the canonical g05.026 manifest and card.
- **Out of scope:** production claims and `local_server/selection.rs`; ACP/headless/Kimi Platform; feature matrices, guides, contracts, architecture, changelog, release work, provider contact, install/update/run, live server/probe/session/catalogue, and g05.009 card 034.
- **Outcome shape:** one of compatible extension, private milestone, new revision, or stop; identity evidence must land before any claim edit, and claim edits are forbidden in this lane.
- Do not invent architecture, change contracts, widen the roadmap, or choose unresolved product/API/persistence/security policy. Product-policy or authority questions, especially Bash `cwd` containment, go to Tom via Chatterbox.
- Work only in the clean worker worktree selected by Completion Protocol. Never edit the planning checkout.
- Do not merge the PR. Merge belongs to the orchestrator after its accepted review/check gate.

## Important Context

- **Planning lineage:** v0.4.0 is tagged at exact candidate `56f3913a`; tag-push CI run `33870017023` passed all six jobs. The feature/currentness freeze is lifted. Research 276 selected Kimi Code local server 0.40.1 as the sole first post-release candidate; Research 270 recorded the relevant 0.39.x web/server deltas.
- **Why these cards are ready:** operator-confirmed direction from Chatterbox on 2026-09-04 promoted commit `8cfe4f24`; g05.026 is ready and Card 063 is explicitly gated behind this card's admitted segment.
- **Decisions and preferences:** downloaded official binaries may be hashed but never executed; keep installed host 0.34.0 observation-only; no claim change in this lane.
- **Open tensions:** whether the 0.40.0 Bash `cwd` removal is contained for this transport, and whether selected protocol deltas map deterministically; escalate authority conclusions that require product policy.
- **Report after:** each coherent evidence/fixture/validation chunk, including changed files, commands, remaining scope, and blockers
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the Completion Protocol preflight before broad reads. Then read `AGENTS.md`,
the active milestone, Card 062, and canonical refs from the selected worker
worktree. Start the identity and corpus evidence chunk. Do not execute any
downloaded binary.

## Completion Protocol

Follow the generic worker Completion Protocol in the Northstar orchestrator
handoff template, including clean-worktree verification, exact pushed-main
lineage, evidence-only review-oracle falsification, required validation, PR
creation, and no merge. The committed copy of this handoff is canonical.

### Handoff closeout

The orchestrator owns the reserved roadmap, front-door, standing-lane, and log
closeout surfaces. If Card 062 stops, record the unchanged `0.38.0` ceiling and
reopen trigger honestly; do not make Card 063 ready.
