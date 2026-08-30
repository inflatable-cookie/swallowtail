---
title: g05.007 Claude watcher live acceptance worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-30
updated: 2026-08-30
handoff_path: /home/box/Dev/projects/swallowtail/docs/handoffs/20260830-235644-g05-007-claude-watcher-live-acceptance.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, claude-code, watcher, live-proof]
---

## What This Thread Was Doing

Card 011 consumed one exact Haiku turn without creating a host watcher. Card
019 then repaired the acceptance surface without provider contact: complete
watcher lifecycle delivery is independent of provider stdout, the proof
recorder distinguishes native active Stop from proactive wait and direct-gate
counterexamples, and live workspace cleanup is panic-safe.

The operator has now authorized exactly one fresh Claude Code `2.1.251` turn
with exact `claude-haiku-4-5`, no fallback, and no rerun. This dispatches only
g05.007 card 020. No transcript or second prompt is part of the authority chain.

## Why It Matters

Credential-free fixtures can prove the oracle's shape, but not that the exact
harness actually calls the reserved watcher tool, blocks early completion
through native Stop, and returns control to the same model session. Swallowtail
must keep the watcher claim withheld unless one bounded live turn proves that
complete sequence and joined cleanup.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `c055b697b9db374e7996c3a7636fd58d713b5930`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `c055b697b9db374e7996c3a7636fd58d713b5930` before these planning and handoff
  edits
- **Planning checkout:** clean before the planning and handoff edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** Contracts 044, 059, and 060;
  stopped card 011; merged card 019 and its repaired selector. This handoff
  commit adds g05.007, ready card 020, the authorization log, reconciled
  indexes, and this file
- **Worker branch:** `worker/g05-007-claude-watcher-live-acceptance`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-007-claude-watcher-live-acceptance`
- **Worktree creation command:** `git worktree add -b worker/g05-007-claude-watcher-live-acceptance /Users/tom/Dev/worktrees/swallowtail-g05-007-claude-watcher-live-acceptance origin/main`
- **Worker worktree policy:** use a clean launcher-provided registered
  non-`main` worktree first. Names may differ from the fallback above
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contracts 044, 059, and 060 are canonical
- **Roadmap milestone:** `docs/roadmaps/g05/007-claude-watcher-live-acceptance.md`
- **Ready cards, in order:**
  `docs/roadmaps/g05/batch-cards/020-claude-code-watcher-live-acceptance.md`
  only
- **Allowed runway:** unchanged pre-contact validation; one exact live watcher
  acceptance turn; sanitized outcome, claim-or-stop closeout, and one PR
- **Remaining card budget:** one card; one provider turn; one evidence PR
- **Dispatch topology:** serial single-card lane
- **Parallel safety check:** the provider budget and claim surfaces are
  single-owner. No parallel worker may run Claude or edit card 020, g05.007,
  or the watcher claim/closeout surfaces
- **Canonical refs:** `AGENTS.md`, `docs/contracts/001-working-rules.md`,
  `docs/contracts/009-async-operation-lifecycle.md`,
  `docs/contracts/010-execution-host-services-and-inputs.md`,
  `docs/contracts/023-harness-operation-isolation-and-native-boundary.md`,
  `docs/contracts/041-input-callback-and-provider-tool-admission.md`,
  `docs/contracts/044-observable-agent-activity-and-disclosure.md`,
  `docs/contracts/059-operation-scoped-process-watchers.md`,
  `docs/contracts/060-operation-scoped-watcher-http-bridge.md`, and
  `docs/architecture/system-architecture.md`
- **Evidence refs:** `docs/research/257-claude-code-watcher-seam-evidence.md`,
  `docs/research/260-claude-code-watcher-bridge-transport.md`,
  `docs/research/261-claude-code-2-1-251-identity.md`,
  `docs/logs/2026-08-30-g05-003-card-011-live-stop-review.md`, and
  `docs/logs/2026-08-30-g05-006-card-019-watcher-proof-repair.md`
- **Review oracle:** card 020 `## Review Oracle`; active native Stop and
  same-session continuation are mandatory, and every named counterexample must
  stay negative
- **Model capability profile:** frontier worker with high reasoning for exact
  lifecycle, privacy, provider evidence, and claim review
- **Tool/runtime restrictions:** no subagents; no production or probe edit
  before contact; no provider command except identity-only preflight and the
  one exact selector; no fallback, retry, direct `claude -p`, response-only
  probe, API key, credential inspection, login, install/update, ambient settings
  mutation, raw/private evidence retention, containers, generic process/MCP
  authority, consumer feature facade, release, merge, or continuation
- **Inherited health baseline:** card 019 closed with 390 god-file findings
  (341 warnings / 49 errors) and one generated-in-source warning. Record the
  worker environment's actual doctor output; do not widen into cleanup
- **Required validation:** card 020 `## Validation`, in order. The live selector
  runs once only after every pre-contact check passes
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting one live result and exact-head review
- **Merge authorization:** not authorized

## Boundaries

- **In scope:** validate the unchanged repaired implementation and compile the
  ignored probe; verify clean source, exact installed `2.1.251`, frozen native
  digest `625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5`,
  absent `ANTHROPIC_API_KEY`, and exact `claude-haiku-4-5`; run only
  `effigy probe:claude-code-watcher-live` once; retain sanitized ordered facts;
  update card, milestone, log, indexes, and Next Task; publish exact existing
  claim surfaces only after the full oracle passes; open one evidence PR.
- **Out of scope:** changing the probe or production code before contact;
  another provider turn, model alias, Sonnet/Opus/fallback, API-key billing,
  `--max-budget-usd`, credential setup or inspection, direct prompt, wider
  watcher version/model range, raw provider/process material, contract or
  architecture changes, containers or hostile-process containment, arbitrary
  process authority, generic MCP/event infrastructure, skill discovery,
  consumer route-feature projection, unrelated cleanup, release, or merge.
- **Outcome shape:** one exact live-proved claim and reviewable PR, or one
  sanitized evidence stop and reviewable PR. A missing or reordered oracle fact
  is failure, not diagnostics authority or permission to retry.
- Identity-only version/digest checks do not consume the turn. Any request
  reaching Claude consumes it regardless of model, hook, deadline, or test
  outcome. Never rerun the selector, including for review.
- Run the live selector from the unchanged clean handoff head before outcome
  edits. If code or probe changes appear necessary, stop before contact and
  return to planning.
- Retain only bounded event kinds, counts, safe identities/revisions, and
  ordering. Never persist prompt text, raw bodies or headers, endpoint, bearer,
  credentials, paths, commands, arguments, environment, PID, watcher output,
  or source artifacts.
- On success, claim only the exact live-proved `2.1.251` and
  `claude-haiku-4-5` point. On failure or ambiguity, keep all watcher claims
  withheld and record the exact sanitized stop.
- Card 011 and g05.003 remain immutable evidence stops. Do not rewrite the
  first failed attempt as part of this proof.
- Do not invent architecture, change contracts, widen the roadmap, or choose
  an unresolved product/API/security decision. Do not merge the PR.

## Important Context

- **Planning lineage:** cards 008-010 and 014-016 delivered the portable
  watcher lifecycle, host supervision, private HTTP bridge, and exact Claude
  binding. Card 011's provider turn missed watcher start. Card 019 then landed
  complete host-owned lifecycle activity, a direct native Stop recorder,
  deterministic negative traces, and panic-safe cleanup through PR 126.
- **Why this card is ready:** the exact version, digest, model, selector,
  deadline, privacy policy, failure stop, and no-rerun rule are fixed. The
  repaired oracle and its adversarial counterexamples are already implemented
  and credential-free green. The operator has granted the sole remaining
  external authority once.
- **Decisions and preferences:** manual Northstar worker/PR loop; no internal
  subagents. Exact `claude-haiku-4-5`, not `haiku`. Existing local subscription
  state only. Watchers supervise ordinary host-managed processes, not hostile
  descendants. Consumer activity must show running state, and successful turn
  completion waits for joined watcher work.
- **Open tensions:** the model may again decline to start a watcher, or Claude
  may expose a Stop limitation. Either consumes the attempt and produces an
  honest stop. It does not permit prompt tweaking, a local auto-wait, terminal
  rejection as success, or another turn.
- **Report after:** pre-contact validation and identity checks are complete,
  before the sole provider request; then after the live result and outcome PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Confirm the tracked
handoff and clean worker context, then read card 020 and the named authority.
Run every pre-contact validation row without modifying the probe or production
code. Report that checkpoint to the operator. If and only if it is green and
the exact identity envelope holds, run `effigy probe:claude-code-watcher-live`
once. Reconcile the outcome and open the evidence PR; never rerun.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare generated names or create another
   worktree merely because they differ from this handoff.
3. If current context is `main`, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique worktree/branch there from pushed `origin/main`. Never use `/tmp`,
   clean, reset, stash over, or discard dirty state. Report a launcher-supplied
   dirty or `main` worktree instead of creating another.
4. From the selected worktree, run
   `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor c055b697b9db374e7996c3a7636fd58d713b5930 HEAD`,
   and load
   `git show HEAD:docs/handoffs/20260830-235644-g05-007-claude-watcher-live-acceptance.md`.
   If the absolute dispatch file differs from that tracked blob, stop. The
   committed `HEAD` copy is canonical.
5. Required sibling worktree links are `none`.
6. Read `AGENTS.md`, `PAPERCUTS.md`, g05.007, card 020, Contracts 001, 009,
   010, 023, 041, 044, 059, 060, Research 257, 260, 261, both named watcher
   logs, the repaired live probe and proof recorder, current claim surfaces,
   and the Rust-quality profile/deviations.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record what
   actually ran; do not execute the broad workspace plan.

### While you work

- Do not edit the probe or production code before the live run. Execute every
  pre-contact validation row in card 020 and keep the tree clean.
- Re-probe only installed path, `claude --version`, native SHA-256, API-key
  absence, and source cleanliness. These checks may not send a prompt, inspect
  credentials, login, update, install, or mutate configuration.
- Report the green pre-contact checkpoint through the operator. Stop without
  provider contact on any mismatch, validation failure, dirty state, missing
  exact model, or setup need.
- Run `effigy probe:claude-code-watcher-live` exactly once. Do not run any other
  prompt or selector. The first request reaching Claude consumes the entire
  authorization. Never rerun after success, failure, timeout, cancellation,
  assertion failure, worker restart, or review request.
- Record only the card's sanitized ordered facts. Treat proactive wait, direct
  gate use, wrong session, terminal-only rejection, missing watcher start,
  incomplete activity, or unjoined cleanup as failure.
- After the run, update planning and evidence surfaces. On complete success,
  update only existing route/feature/activity/guide claim surfaces for the
  exact proved point. On any failure or ambiguity, leave all claims absent and
  record the sanitized stop.
- If the outcome exposes an implementation or planning change, do not repair or
  rerun in this lane. Return the finding to the orchestrator.

### When the assigned runway is complete

1. Run the applicable post-outcome validation from card 020. Never rerun the
   live selector.
2. Falsify the outcome against every card 020 review-oracle row and negative
   trace. Map each accepted claim to deterministic proof plus the one sanitized
   live trace; terminal text or a final zero snapshot is not enough.
3. Reconcile card 020, g05.007, the outcome log, batch-card/generation/log
   indexes, g05 front door, and the sole roadmaps Next Task. Keep card 011 and
   g05.003 unchanged. Keep the consumer route-feature triage note open.
4. Push the selected worker branch and open one reviewable evidence PR against
   current pushed `main`, whether the result is success or an honest stop.
5. In the PR body, link g05.007, card 020, Contracts 044/059/060, the card 011
   stop, card 019 repair, exact version/model, sanitized proof or missing fact,
   validation, claim disposition, and unresolved items. Include no raw/private
   material.
6. Report the PR URL and sanitized evidence to the operator. Do not merge,
   continue into another card, or request another provider turn.

### Review and merge path

The orchestrator reviews the exact worker head against card 020, Contracts
044, 059, and 060, the diff, the deterministic oracle, and the single sanitized
live ordering. With a shared GitHub identity, the verdict is a PR comment.
Blocking findings use `execution-miss`, `oracle-gap`, `planning-change`,
`validation-gap`, or `integration-drift`. A `planning-change` returns to
planning. Requested changes are `none` until review. No revision may contact
Claude or rerun the live selector. Merge remains separately authorized.

- **Closeout refs:** card 020; g05.007; g05 and roadmaps front doors; batch-card,
  generation, and log indexes; outcome log; exact route/feature/activity/guide
  claim surfaces only after proof

### Handoff closeout

Before calling the runway complete, leave the consumed-attempt count, exact
claim or stop, validation, PR state, and sole Next Task honest. A failed or
inconclusive provider turn is a completed evidence attempt and an immediate
stop, never authority for another run.
