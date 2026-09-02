---
title: Claude Agent SDK route gate worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260902-192627-claude-agent-sdk-route-gate.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, claude, sdk]
---

## What This Thread Was Doing

Swallowtail is splitting Claude parity into a native Agent SDK route and an
independent ACP expansion. This worker owns card 053 only: the official SDK,
subscription, credential, sidecar, lifecycle, and route-contract evidence gate.

This dispatches one bounded evidence and contract lane. No transcript or second
prompt is part of the authority chain.

## Why It Matters

The operator wants a featureful desktop-app connection using each client's own
Claude subscription without account-policy or credential-custody risk. This is
the priority lane before the paused `v0.4.0` release audit can restart.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `0e4768055dd4854417d44c9a2cf84d809bbedfa6`
- **Pushed main verification:** local `HEAD` and `origin/main` matched exactly before handoff creation
- **Planning checkout:** clean
- **Worker mode:** implementation worker dispatched by the orchestrator; this handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Research 277; g05.022; card 053
- **Worker branch:** `worker/g05-card053-claude-agent-sdk-route-gate`
- **Worker worktree:** `/Users/tom/Dev/worktrees/g05-card053-claude-agent-sdk-route-gate`
- **Worktree creation command:** launcher-owned; manual fallback uses `.agents.local.env` only
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** Research 277
- **Roadmap milestone:** `docs/roadmaps/g05/022-claude-agent-dual-route-parity.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/053-claude-agent-sdk-route-evidence-and-contract-gate.md`
- **Allowed runway:** card 053 only; Research 278 plus one SDK-specific triage/contract-gate packet and honest closeout
- **Remaining card budget:** one
- **Dispatch topology:** parallel with card 054 ACP parity census
- **Parallel safety check:** SDK lane owns native artifact/auth/sidecar evidence; ACP lane owns qualified bridge census; shared promotion is reserved to the orchestrator
- **Surfaces this lane owns:** `docs/research/278-*`, one `docs/triage/2026-09-02-claude-agent-sdk-*` gate if required, card 053, one SDK lane log, and required index entries
- **Integration ownership:** orchestrator owns g05.022, front doors, architecture/contracts promotion, shared vocabulary, implementation-card compilation, and release-lane state
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator refreshes this head against current `main` and re-reviews it if a sibling lane merges first
- **Canonical refs:** Research 277; Contracts 010, 017, 023, 029, 038, 041, 047; g05.022
- **Review oracle:** card 053 Review Oracle
- **Model capability profile:** frontier implementation/research worker
- **Frontier-worker justification:** exceptional post-planning reasoning is required across subscription policy, credential security, cross-language lifecycle, public route identity, and SDK declaration-versus-runtime evidence; this is the highest-priority material release blocker because the operator broke the v0.4.0 freeze for it
- **Tool/runtime restrictions:** no provider turn, login, OAuth flow, token read, installed-host mutation, downloaded-code execution, release command, production code, package pin, claim, fixture, matrix, or workflow edit
- **Required validation:** card 053 validation plus `git diff --check`
- **PR base/head:** current pushed `main` / worker branch head
- **PR URL:** pending
- **Review state:** awaiting exact-head orchestrator review
- **Merge path:** orchestrator after accepted review of the current head and passing required checks

## Boundaries

- **In scope:** execute card 053 exactly, including official primary-source and artifact evidence, complete selected TypeScript public API inventory, credential non-custody proof, bounded sidecar lifecycle, route comparison, Research 278, and a reviewable contract gate.
- **Out of scope:** production Rust or TypeScript, shared contract promotion, manifest/package changes, route or compatibility claims, provider contact, subscription login, consumer changes, release work, or ACP lane edits.
- **Outcome shape:** evidence-and-contract gate. An honest stop is valid only when the exact blocker and smallest operator/shared-contract decision are proved.
- Do not invent architecture, change contracts, widen the roadmap, or choose an unresolved product/API/persistence/security decision.
- Write only inside **Surfaces this lane owns**. Leave shared mutable surfaces to **Integration ownership**.
- Work only in the clean worker worktree selected by `Completion Protocol`.
- Do not merge the PR.

## Important Context

- **Planning lineage:** consumer parity triage promoted into Research 277 and g05.022; g05.021/cards 050-052 are paused.
- **Why this card is ready:** the operator selected a distinct native SDK route, subscription-backed per-user auth, credential non-custody, bounded Node sidecar, and parallel ACP work.
- **Decisions and preferences:** use official TypeScript SDK; never flatten into ACP/CLI identities; provider-specific features remain route-local unless portability is proved; policy authority is current but provisional.
- **Open tensions:** official overview wording versus the newer subscription-specific Help Center update; public declarations versus runtime guarantees; auth readiness without credential access; sidecar crash/resume semantics.
- **Report after:** one complete evidence and contract packet with exact stable identity and falsification.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read `AGENTS.md`,
Research 277, g05.022, card 053, and the canonical contracts. Recheck official
subscription guidance before freezing an SDK artifact.

## Completion Protocol

Before broad reads, run `git rev-parse --show-toplevel`, `git branch
--show-current`, `git status --porcelain`, and `git worktree list --porcelain`.
Accept a clean launcher-provided non-`main` worktree. Otherwise use only the
named worktree or the absolute container from `.agents.local.env`; never use
`/tmp` as a worktree and never clean, reset, or stash unrelated state.

Fetch origin with a bounded non-interactive command. Confirm `HEAD` equals
`origin/main`, the planning base is an ancestor, and this handoff's tracked blob
matches the absolute dispatch file. Read the card and canonical refs. Run cheap
orientation only.

Execute the card in meaningful chunks. Keep evidence reproducible, freeze exact
sets, and distinguish declarations, source, shipped artifact, and runtime
claims. Do not run downloaded code or authenticated probes. Stop on a policy,
license, token-custody, lifecycle, or shared-public-API decision.

At completion, run the card validation and falsify every exact, universal, and
negative claim. Reconcile owned research/card/log/index state without editing
shared front doors. Push the branch, open a PR against current `main`, and report
the exact head, evidence, validation, and unresolved questions. Do not merge.

The orchestrator reviews the exact head. Same-identity review may use a canonical
PR comment. Requested changes stay on this branch. Shared integration and any
implementation card are separate later work.

