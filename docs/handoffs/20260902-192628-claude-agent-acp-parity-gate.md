---
title: Claude Agent ACP parity gate worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260902-192628-claude-agent-acp-parity-gate.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, claude, acp]
---

## What This Thread Was Doing

Swallowtail is splitting Claude parity into a native Agent SDK route and an
independent ACP expansion. This worker owns card 054 only: a complete qualified
bridge capability census and bounded delivery gate.

This dispatches one bounded evidence lane. No transcript or second prompt is
part of the authority chain.

## Why It Matters

ACP remains the portable interoperable route. The qualified bridge already
advertises substantially more than Swallowtail exposes, so useful parity can
land independently while the native SDK route is designed.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `0e4768055dd4854417d44c9a2cf84d809bbedfa6`
- **Pushed main verification:** local `HEAD` and `origin/main` matched exactly before handoff creation
- **Planning checkout:** clean
- **Worker mode:** implementation worker dispatched by the orchestrator; this handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Research 277; g05.022; card 054
- **Worker branch:** `worker/g05-card054-claude-agent-acp-parity-gate`
- **Worker worktree:** `/Users/tom/Dev/worktrees/g05-card054-claude-agent-acp-parity-gate`
- **Worktree creation command:** launcher-owned; manual fallback uses `.agents.local.env` only
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** Research 277
- **Roadmap milestone:** `docs/roadmaps/g05/022-claude-agent-dual-route-parity.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/054-claude-agent-acp-parity-census-and-delivery-gate.md`
- **Allowed runway:** card 054 only; Research 279 plus one ACP-specific delivery gate and honest closeout
- **Remaining card budget:** one
- **Dispatch topology:** parallel with card 053 native SDK route gate
- **Parallel safety check:** ACP lane owns qualified bridge/current adapter evidence; SDK lane owns native artifact/auth/sidecar evidence; shared promotion is reserved to the orchestrator
- **Surfaces this lane owns:** `docs/research/279-*`, one `docs/triage/2026-09-02-claude-agent-acp-*` gate if required, card 054, one ACP lane log, and required index entries
- **Integration ownership:** orchestrator owns g05.022, front doors, architecture/contracts promotion, shared vocabulary, implementation-card compilation, and release-lane state
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator refreshes this head against current `main` and re-reviews it if a sibling lane merges first
- **Canonical refs:** Research 277; Contracts 015, 017, 023, 029, 038, 041, 047; g05.022
- **Review oracle:** card 054 Review Oracle
- **Model capability profile:** economical long mechanical evidence worker
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no provider turn, login, live probe, host mutation, production code, package pin, claim, fixture, matrix, shared contract, release command, or workflow edit
- **Required validation:** card 054 validation plus `git diff --check`
- **PR base/head:** current pushed `main` / worker branch head
- **PR URL:** pending
- **Review state:** awaiting exact-head orchestrator review
- **Merge path:** orchestrator after accepted review of the current head and passing required checks

## Boundaries

- **In scope:** execute card 054 exactly: complete route/bridge capability
  partition, source/wire/prepared/active/public mapping, dependency-safe tranche
  selection, Research 279, and one exact ACP delivery gate.
- **Out of scope:** native SDK evidence, production changes, shared contract or
  public API choices, version qualification, provider contact, consumer edits,
  or release work.
- **Outcome shape:** evidence-and-delivery gate. An honest empty tranche is
  acceptable only when the blocker is exact and load-bearing.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved shared API/persistence/security decision.
- Write only inside **Surfaces this lane owns**. Leave shared mutable surfaces
  to **Integration ownership**.
- Work only in the clean worker worktree selected by `Completion Protocol`.
- Do not merge the PR.

## Important Context

- **Planning lineage:** consumer parity triage promoted into Research 277 and g05.022; g05.021/cards 050-052 are paused.
- **Why this card is ready:** qualified ACP through official `0.73.0` already has exact corpora and the operator requested an independent maximal expansion.
- **Decisions and preferences:** prioritize read-write interactive, session permission/mode, and model/effort controls; keep terminal, MCP, auth, packaging, management, and richer metadata dependency-distinct.
- **Open tensions:** advertised versus effective capability; discarded acknowledgements; process authority for Bash; source identity across resume and MCP; shared public facade gaps.
- **Report after:** one complete no-filter census and exact first-tranche gate.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read `AGENTS.md`,
Research 277, g05.022, card 054, the qualified `0.73.0` corpora, and current
Claude adapter source. Derive the census from source rather than the old triage
list.

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

Execute the card in meaningful chunks. Keep every census row exact and separate
protocol presence, admission, effectiveness, observation, lifecycle, authority,
facade, and projection. Do not contact a provider. Stop on an unqualified
artifact, shared-public-API choice, or unbounded process/MCP authority.

At completion, run the card validation and falsify every exact, universal, and
negative claim. Reconcile owned research/card/log/index state without editing
shared front doors. Push the branch, open a PR against current `main`, and report
the exact head, evidence, validation, and unresolved questions. Do not merge.

The orchestrator reviews the exact head. Same-identity review may use a canonical
PR comment. Requested changes stay on this branch. Shared integration and any
implementation card are separate later work.

