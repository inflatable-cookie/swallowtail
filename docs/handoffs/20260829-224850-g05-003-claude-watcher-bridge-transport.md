---
title: g05.003 Claude watcher bridge transport evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-29
updated: 2026-08-29
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260829-224850-g05-003-claude-watcher-bridge-transport.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, research, claude, watchers]
---

## What This Thread Was Doing

The orchestrator merged PR 118, then reassessed the planned Claude Code watcher
bridge. The host registry and process supervision are now real, but card 010
still assumes a private MCP server can reach the in-process watcher port. No
such production transport exists yet, and the installed Claude version has
moved beyond the evidence window.

This handoff gives one research worker the bounded job of closing that gap
before anyone writes route code. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents; the operator owns
parallelism in their harness.

## Why It Matters

The watcher feature only works if model calls, operator controls, completion
blocking, and host cleanup all reach one turn-owned registry. Guessing the MCP
bridge would create ambient listener authority or work that Swallowtail cannot
authenticate, cancel, and join. Card 015 protects the feature from becoming a
prompt-only or best-effort claim.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `eb014a9b72fb06b55d967a952ba618952991b5fa`
- **Pushed main verification:** this handoff and card 015 are committed and
  pushed after the planning base; verify the tracked handoff in `origin/main`.
- **Planning checkout:** clean when dispatched.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** g05.003 readiness assessment,
  ready card 015, and this handoff.
- **Worker branch:** `worker/g05-003-claude-watcher-bridge-transport`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-003-claude-watcher-bridge-transport`
- **Worktree creation command:** `git worktree add -b worker/g05-003-claude-watcher-bridge-transport /Users/tom/Dev/worktrees/swallowtail-g05-003-claude-watcher-bridge-transport origin/main`
- **Worker worktree policy:** use the clean, dedicated, non-`main` registered
  worktree supplied by the launcher, even when its generated path or branch
  differs from the placeholders above. Record the actual values and do not
  create a second worktree for that reason. If the launcher context is
  unusable, follow the fallback in `## Completion Protocol`.
- **Required sibling worktree links:** none.
- **Active spec lane:** none; strict contract-first g05 roadmap.
- **Roadmap milestone:** `docs/roadmaps/g05/003-operation-scoped-watcher-proof.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/015-claude-code-watcher-bridge-transport-evidence.md`
- **Allowed runway:** card 015 evidence and card 010 readiness disposition only.
- **Remaining card budget:** one card.
- **Dispatch topology:** serial; cards 010-011 remain gated.
- **Parallel safety check:** no parallel lane is authorized; this worker owns
  unique Research 260 and evidence surfaces.
- **Canonical refs:** `docs/architecture/system-architecture.md`;
  `docs/contracts/010-execution-host-services-and-inputs.md`,
  `docs/contracts/012-interactive-session-options-and-callback-exchange.md`,
  `docs/contracts/041-input-callback-and-provider-tool-admission.md`,
  `docs/contracts/044-observable-agent-activity-and-disclosure.md`, and
  `docs/contracts/059-operation-scoped-process-watchers.md`.
- **Model capability profile:** research and architecture judgment with exact
  source attribution; no model override required.
- **Tool/runtime restrictions:** use official Claude documentation and exact
  package/binary evidence. Prompt-free local probes are allowed. No install,
  update, login, credential mutation, paid provider turn, or model prompt
  without explicit operator authorization.
- **Required validation:** `effigy validate:focused swallowtail-adapter-claude-agent`;
  `effigy qa:docs`; `effigy qa:northstar`; `git diff --check`.
- **PR base/head:** current pushed `main` /
  `worker/g05-003-claude-watcher-bridge-transport`.
- **PR URL:** pending.
- **Review state:** awaiting worker evidence and PR.
- **Merge authorisation:** not authorized; the operator must explicitly
  authorize merge after orchestrator review.

## Boundaries

Please keep this run inside the named runway:

- **In scope:** Research 260; exact current Claude MCP/hook/skill evidence;
  provider-to-host transport options; listener/helper ownership,
  authentication, turn correlation, failure, cleanup, and version gates; card
  015 and its evidence log; card 010 readiness disposition.
- **Out of scope:** production MCP server, listener, helper, skill, hook, or
  Claude command wiring; card 010/011 implementation; generic consumer tools;
  contract or architecture promotion; route/feature claims; release; merge; or
  another route.
- **Outcome shape:** diagnostics-only. Return one exact viable bridge and the
  smallest required planning delta, or an honest stop. Do not implement the
  bridge in this lane.
- Do not infer that the sign-in loopback callback port can carry MCP traffic.
  It is purpose-limited and exposes no generic request channel.
- Do not infer that `ServingEndpointService` binds a listener. It publishes
  an endpoint already observed from an owned child.
- A loopback endpoint without operation-private authentication and exact turn
  correlation is not a qualifying watcher bridge.
- Do not invent architecture, change contracts, widen the roadmap, or choose
  an unresolved product/API/persistence/security decision.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or unrelated dirty work.
- Do not merge the PR.

## Important Context

- **Planning lineage:** Research 257 proved Claude's candidate
  `--bare` + strict MCP + operation-private skill + Stop-hook mechanism for
  `2.1.220..=2.1.241`, but did not run live same-turn re-entry. Cards 009 and
  014 then delivered the shared registry and ordinary host-process
  supervision. The readiness log explains why transport remains open.
- **Why this card is ready:** the missing questions are bounded and evidence
  producing. They do not require production code to answer.
- **Decisions and preferences:** watchers are ordinary host-managed work, not
  containers or hostile-descendant containment. Model and operator controls
  share one registry. Explicit wait pauses the turn. No successful turn may
  leave watcher work active or unjoined. Docker and hosted OAuth are outside
  this lane.
- **Open tensions:** HTTP requires an owned listener and operation-private
  authentication; stdio normally requires Claude to launch a helper that still
  needs an owned channel to the in-process registry. The current installed
  Claude is `2.1.251`, beyond Research 257's qualified ceiling. Live Stop
  re-entry still needs explicit operator authority if it incurs provider work.
- **Report after:** the current-version source corpus and transport comparison
  are complete, before proposing any contract delta.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Start by running the worktree-safety preflight below. Then read card 015,
Research 257, the readiness log, and Contracts 010/059. Map Claude's exact
current MCP transports onto the concrete runtime host ports before looking for
an implementation shortcut.

For each candidate, write down the provider process, listener or helper owner,
the channel into `WatcherHostService`, request authentication and turn
correlation, and terminal cleanup order. If neither HTTP nor stdio fits the
current contracts, stop at the smallest explicit contract decision packet.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered, clean, non-`main` worktree, use it
   immediately and record its actual root and branch. Do not compare generated
   names with the placeholders or create another worktree.
3. If the launcher supplied a dirty or `main` worktree, stop and report it.
   Otherwise inspect the named worktree only when the current context is
   unusable. If both fail, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator if it is absent.
   Create a unique fallback worktree only under that container from
   `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path.
4. In the selected worktree, confirm `HEAD == origin/main`, confirm
   `eb014a9b72fb06b55d967a952ba618952991b5fa` is an ancestor, and confirm
   `docs/handoffs/20260829-224850-g05-003-claude-watcher-bridge-transport.md`
   exists in `HEAD`. Load it with `git show HEAD:<path>`. If the absolute
   dispatch file differs from that tracked blob, stop and report.
5. Required sibling links are `none`.
6. Read `AGENTS.md`, g05.003, card 015, Research 257, the readiness log, and
   the canonical refs listed above.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Retain
   the inherited doctor baseline rather than widening this lane into cleanup.

### While you work

- Use official sources and exact package or local-binary evidence. Record
  retrieval dates, identities, and digests for decisive artifacts.
- Separate requested, parsed, configured, dispatched, applied, model-visible,
  blocking, terminal, and cleanup truth.
- Compare HTTP and stdio honestly. A configuration shape is not a host-owned
  transport proof.
- Do not run an authenticated or paid Claude turn unless the operator
  explicitly authorizes it. If live proof is necessary, report the smallest
  exact probe first.
- Stop and report if a contract decision is required. Do not quietly promote
  one inside Research 260.
- After the first meaningful chunk, report changed files, evidence gathered,
  validation run, remaining work, and blockers through the operator.

### When the assigned runway is complete

1. Produce `docs/research/260-claude-code-watcher-bridge-transport.md` and
   `docs/logs/2026-08-29-g05-003-claude-watcher-bridge-transport-evidence.md`.
   Update card 015 and card 010's readiness gate. Do not edit shared roadmap,
   generation, or batch-card indexes; the orchestrator owns those after merge.
2. Run `effigy validate:focused swallowtail-adapter-claude-agent`,
   `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check`.
3. Push the worker branch and open a reviewable PR against current `main`.
4. Link the milestone, cards, Research 257/260, readiness log, changed
   evidence, validation, and unresolved items in the PR body.
5. Report the PR URL and exact head to the operator. Do not merge or start card
   010.

### Review and merge path

The orchestrator will review the PR against the canonical refs, exact diff,
and checks. Formal self-approval may be unavailable; the canonical verdict
will then be a PR comment. Requested changes: none yet. The operator must
explicitly authorize merge.

- **Closeout refs:** card 015; card 010 readiness gate; Research 260; assigned
  evidence log; g05.003.

### Handoff closeout

Leave card 015 and card 010 honest. A negative transport result or an
unauthorized live gate is a valid stop. Do not make card 010 ready unless the
transport, current version, and required acceptance proof are closed.
