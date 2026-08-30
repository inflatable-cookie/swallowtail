---
title: g05.007 Linux watcher live acceptance worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-31
updated: 2026-08-31
handoff_path: /home/box/Dev/projects/swallowtail/docs/handoffs/20260830-235248-g05-007-linux-watcher-live-acceptance.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, claude-code, watcher, linux, live-proof]
---

## What This Thread Was Doing

Card 020's first worker stopped before provider contact because the worker host
was `linux-x86_64` while the authorization envelope and live probe froze only
Research 261's `darwin-arm64` native digest. PR 127 records that pre-contact
finding, but the operator withheld its merge as a terminal stop. No request
reached Claude, so the single authorized provider turn remains unconsumed.

The operator selected Linux and authorized the smallest per-platform probe
repair before that same one-shot turn. This handoff dispatches only g05.007 card
020: repair and prove native-digest selection, commit a clean repair head, then
run at most one exact Claude Code `2.1.251` / `claude-haiku-4-5` watcher turn.

## Why It Matters

Artifact identity is platform-specific. A Linux live run must fail closed on
anything except Research 261's official `linux-x64` digest, without weakening
the existing Darwin identity or accepting either digest on either platform.
Only after that credential-free boundary and every pre-contact gate are green
may one live turn test the already-repaired watcher lifecycle oracle.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `348090bf6ddc82155ba14e376d5257fe8d42dcbc`
- **Pushed-main verification before planning edits:** local `HEAD` and
  `origin/main` both resolved to the planning base above
- **Planning checkout:** clean before the authorized lock, planning, and
  handoff edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  tracked handoff activates the worker-only worktree preflight
- **Worker branch:** `worker/g05-007-linux-watcher-live-acceptance`
- **Worker worktree:** use the clean launcher-provided registered non-`main`
  worktree regardless of its generated path or branch name
- **Manual fallback command:** launcher-owned first; only after resolving
  `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`, use
  `git worktree add -b worker/g05-007-linux-watcher-live-acceptance <resolved-container>/swallowtail-g05-007-linux-watcher-live-acceptance origin/main`
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contracts 044, 059, and 060 remain canonical
- **Roadmap milestone:**
  `docs/roadmaps/g05/007-claude-watcher-live-acceptance.md`
- **Ready cards, in order:**
  `docs/roadmaps/g05/batch-cards/020-claude-code-watcher-live-acceptance.md`
  only
- **Allowed runway:** bounded per-platform live-probe repair; credential-free
  proof and pre-contact validation; one exact Linux live turn; sanitized
  claim-or-stop closeout; one new evidence PR
- **Remaining budget:** one card, one provider turn, one worker PR
- **Dispatch topology:** serial single-owner lane. Do not overlap another
  write-heavy worker on the watcher probe, card, milestone, outcome log, or
  claim surfaces
- **Prerequisites already cleared on the selected host:** `Cargo.lock` selects
  non-yanked `chacha20 0.10.2`; exact `cargo-public-api 0.52.0`,
  `nightly-2026-08-05`, and `pkg-config 1.8.1` are present;
  `package:verify-affected` and `package:api` are green
- **Exact Linux envelope:** `linux-x86_64`; Claude Code `2.1.251`; native
  SHA-256
  `fd5f10ff0eb58daec04900466b143ea98aab50abf208a422bc008eaec13f61f7`;
  model `claude-haiku-4-5`; local subscription state; no
  `ANTHROPIC_API_KEY`; `90`-second operation deadline
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
  `docs/logs/2026-08-30-g05-003-card-011-live-stop-review.md`,
  `docs/logs/2026-08-30-g05-006-card-019-watcher-proof-repair.md`, and
  `docs/logs/2026-08-31-g05-007-card-020-linux-envelope.md`
- **Review oracle:** card 020 `## Review Oracle`, including platform identity,
  native active Stop, same-session continuation, complete lifecycle activity,
  and joined cleanup
- **Inherited health baseline:** 390 god-file findings (341 warnings / 49
  errors) and one generated-in-source warning; record actual output without
  widening into cleanup
- **PR 127:** open pre-contact stop evidence; excluded from this implementation
  lane and not merge-authorized
- **Review state:** awaiting the bounded repair and the one live result
- **Merge authority:** the worker never merges. For the later worker PR, Helm
  may merge under Tom's standing authority only after required checks pass and
  the orchestrator posts a merge-authorized exact-head verdict. A withheld
  verdict means no merge

## Boundaries

- **In scope before provider contact:** modify only
  `crates/swallowtail-adapter-claude-agent/tests/live_watcher_probe.rs` as needed
  to select the frozen native digest by actual target platform; preserve the
  existing `darwin-arm64` value, add the exact `linux-x64` value, reject
  unsupported targets, and add credential-free adversarial proof. Commit and
  push that bounded repair before any provider request.
- **In scope after the repair:** rerun every card 020 pre-contact gate from the
  clean committed repair head; verify the exact Linux envelope; run only
  `effigy probe:claude-code-watcher-live` once; retain sanitized ordered facts;
  reconcile the card, milestone, one outcome log, indexes, and sole Next Task;
  update existing claim surfaces only if every live oracle row passes; open one
  new evidence PR.
- **Out of scope:** production changes; prompt, model, deadline, lifecycle
  oracle, or contract changes; Darwin dispatch; card 011 reuse; another Claude
  command or selector; fallback, retry, rerun, direct `claude -p`, response-only
  probe, API-key billing, credential inspection, login, Claude install/update,
  ambient settings mutation, raw/private evidence, containers, generic
  process/MCP authority, consumer feature expansion, unrelated cleanup,
  release, PR 127 revision/merge, or merge of the new PR.
- Identity-only path/version/digest checks and credential-free validation do
  not consume the turn. The first request reaching Claude consumes the entire
  authorization regardless of result. Never rerun after success, failure,
  timeout, cancellation, assertion failure, worker restart, or review request.
- The live selector must start from the exact clean repair commit that passed
  validation. Any later source drift stops before contact.
- Retain only bounded event kinds, counts, safe turn/session correlation,
  revisions, and ordering. Never persist prompt text, raw provider or HTTP
  bodies, endpoint, bearer, credentials, paths, commands, arguments,
  environment, PID, watcher output, or source artifacts.
- Card 011 and g05.003 remain immutable evidence stops. Do not rewrite the
  consumed first attempt or use its provider session.
- Do not use subagents, invent architecture, widen the roadmap, or choose a new
  product/API/security rule. Return any such need to the orchestrator before
  provider contact.

## Important Context

- Research 261 freezes both official `2.1.251` native identities. The current
  probe hard-codes only the Darwin ARM64 digest. The operator chose the Linux
  value; accepting a set of digests without binding one to the current target
  is not sufficient.
- Card 019 already delivered the lossless watcher lifecycle feed, native
  Stop-reentry recorder, deterministic negative traces, and panic-safe live
  workspace cleanup. This card does not reopen those production decisions.
- `Cargo.lock` and the selected host now clear the two pre-contact validation
  gaps reported by the first card 020 worker. The worker reruns both selectors;
  it does not repair or install around a new failure.
- The model may still decline to start a watcher or expose a Stop limitation.
  Either result consumes the turn and produces an honest evidence stop. It does
  not permit prompt changes, proactive auto-wait, terminal rejection as
  success, or another turn.
- Report through Helm after the repair commit and green pre-contact checkpoint,
  then after the live result and PR. The operator authorization is already
  complete; a green checkpoint does not require a second authorization.

## Suggested Next Move

Run the worker preflight before broad reads. Confirm the tracked handoff and
clean dedicated worktree, then read card 020 and its refs. Implement only the
per-platform digest selection and adversarial credential-free proof. Commit and
push the repair, rerun every pre-contact gate, verify the exact Linux identity,
and report the green checkpoint. Only then run the live selector once. Reconcile
the outcome and open one new evidence PR; never rerun or merge.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record the
   actual root/branch; do not compare generated names or create another
   worktree merely because they differ from this handoff.
3. If the current context is `main`, dirty, unregistered, or unusable, inspect
   the named launcher state. Only then read `.agents.local.env`, require an
   absolute `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique fallback there
   from `origin/main`. Never use `/tmp`, guess a path, clean, reset, stash over,
   or discard dirty state. Report a launcher-supplied dirty or `main` worktree
   instead of creating another behind it.
4. From the selected worktree, run
   `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor 348090bf6ddc82155ba14e376d5257fe8d42dcbc HEAD`,
   and load
   `git show HEAD:docs/handoffs/20260830-235248-g05-007-linux-watcher-live-acceptance.md`.
   If the absolute dispatch file differs from that tracked blob, stop. The
   committed `HEAD` copy is canonical.
5. Required sibling worktree links are `none`.
6. Read `AGENTS.md`, `PAPERCUTS.md`, g05.007, card 020, Contracts 001, 009,
   010, 023, 041, 044, 059, 060, Research 257, 260, 261, the named watcher
   logs, the current live probe and proof recorder, claim surfaces, and the
   Rust-quality profile/deviations.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record what
   actually ran; do not execute the broad workspace plan.

### While you work

- Repair and prove only the per-platform native-digest selection. Do not run a
  Claude prompt during diagnosis, implementation, test, review, or repair.
- Commit and push the bounded repair. Record its exact SHA, require a clean
  tree, and run the card 020 validation rows in order, including the non-live
  `live_watcher_probe` test and compile-only row.
- Re-probe only installed path, host target, `claude --version`, native
  SHA-256, API-key absence, and source cleanliness. These checks may not inspect
  credentials, login, update Claude, or mutate configuration.
- Report the repair SHA and green pre-contact checkpoint through Helm. If and
  only if every row and the exact Linux envelope pass, run
  `effigy probe:claude-code-watcher-live` exactly once. Do not run another
  prompt or selector.
- Record only the card's sanitized ordered facts. Treat cross-platform digest
  acceptance, unsupported-target fallback, proactive wait, direct gate use,
  wrong session, terminal-only rejection, missing watcher start, incomplete
  activity, or unjoined cleanup as failure.
- After the run, update evidence and planning surfaces. On complete success,
  update only existing route/feature/activity/guide claim surfaces for exact
  `2.1.251` / `claude-haiku-4-5`. On failure or ambiguity, leave all claims
  absent and record the sanitized stop.
- If the outcome exposes a change outside the bounded probe repair, do not
  repair or rerun. Return the finding to the orchestrator.

### When the assigned runway is complete

1. Run the applicable post-outcome validation from card 020. Never rerun the
   live selector.
2. Falsify the diff against both card 020 oracle groups. Map platform
   counterexamples to credential-free proof and every accepted live claim to
   deterministic proof plus the one sanitized live trace.
3. Reconcile card 020, g05.007, the outcome log, batch-card/generation/log
   indexes, g05 front door, and the sole roadmaps Next Task. Keep card 011,
   g05.003, and PR 127 unchanged.
4. Push the selected worker branch and open one new reviewable evidence PR
   against current pushed `main`, whether the result is success or an honest
   stop.
5. In the PR body, link g05.007, card 020, Contracts 044/059/060, the Linux
   envelope log, card 011 stop, card 019 repair, exact repair SHA,
   version/model/digest, sanitized proof or missing fact, validation, claim
   disposition, and unresolved items. Include no raw/private material.
6. Report the PR URL and sanitized evidence through Helm. Do not merge,
   continue into another card, or request another provider turn.

### Review and merge path

The orchestrator reviews the exact worker head against card 020, Contracts 044,
059, and 060, both review-oracle groups, the full diff, checks, deterministic
platform proof, and the single sanitized live ordering. With a shared GitHub
identity, the verdict is a PR comment. Blocking findings use
`execution-miss`, `oracle-gap`, `planning-change`, `validation-gap`, or
`integration-drift`; a `planning-change` returns to planning before revision.
Requested changes are `none` until review.

After required checks pass and review is complete, the orchestrator either
posts a merge-authorized verdict or withholds merge. Under Tom's standing
instruction, Helm may merge the later worker PR after a merge-authorized
verdict without another Tom confirmation. A withheld verdict means Helm does
not merge. The worker never merges. PR 127 is never the merge target.

- **Closeout refs:** card 020, g05.007, one outcome log, logs and batch-card
  indexes, g05 and generation front doors, and the sole roadmaps Next Task

### Handoff closeout

Leave the card, milestone, log, claim surfaces, and Next Task honest. One live
turn is the entire budget. If blocked before contact, record the sanitized
blocker and stop. If the turn is consumed, record its exact bounded result and
stop. No continuation is automatic.
