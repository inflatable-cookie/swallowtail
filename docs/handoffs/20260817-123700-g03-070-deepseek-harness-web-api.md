---
title: g03.070 DeepSeek Harness Web /api worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-17
updated: 2026-08-17
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260817-123700-g03-070-deepseek-harness-web-api.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Swallowtail already has a live-proven DeepSeek Harness JSON-RPC route. The
planning thread then qualified the Web GUI host's `/api` RPC and WebSocket mux
as a second route: catalogue, paged history, native cancel, fork, and archive.
Research 125, Spec 009, and g03.070 are compiled. JSON-RPC stays. This is not
ACP, not the browser UI, and not Open Platform continuation.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

JSON-RPC is a one-shot stdio structured run. The useful product surface is the
local Web `/api`: session list, history without a live Agent, prompt with
native cancel, fork, and archive. Swallowtail should expose that as a Kimi
local-server analogue on the existing Harness package, with a hard method
allowlist, so credentials and settings never ride the same driver.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `8861792ab0cf8e6bf567c0f372ddabfd63e0bcc6`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed. At handoff writing, `origin/main` was
  still behind local `main`; the operator must push `main` before this
  worker starts.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Research 125, Spec 009,
  milestone g03.070, ready card 222, planned cards 223-225, and the 2026-08-17
  Web `/api` planning log
- **Worker branch:** `g03-070-deepseek-harness-web-api`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g03-070-deepseek-harness-web-api`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g03-070-deepseek-harness-web-api "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g03-070-deepseek-harness-web-api" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** `docs/specs/009-deepseek-harness-web-api-route.md`
- **Roadmap milestone:** `docs/roadmaps/g03/070-deepseek-harness-web-api-foundation.md`
- **Ready cards, in order:** `docs/roadmaps/g03/batch-cards/222-deepseek-harness-web-api-corpus.md`
- **Allowed runway:** g03.070 cards 222 → 223 → 224 → 225. Only 222 is ready
  at dispatch. Continue to 223, then 224, then 225 only through each card's
  Auto-Continuation after the predecessor's acceptance is actually met.
- **Remaining card budget:** four cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** cards 222-225 share
  `swallowtail-adapter-deepseek-harness` and public route truth with the
  existing JSON-RPC route, so they stay serial. Do not start a second Harness
  lane.
- **Canonical refs:** `docs/architecture/system-architecture.md`;
  `docs/architecture/release-and-package-topology.md`; Contracts 005-006,
  009-010, 017, 023, 029, 032-033, 036-039, 044-045, 051-052, 054;
  `docs/research/125-deepseek-harness-web-api-route-qualification.md`;
  `docs/contracts/001-working-rules.md`
- **Model capability profile:** capable coding model, medium reasoning
- **Tool/runtime restrictions:** spawn host-approved `dsh web` / `dsh --profile web`
  on loopback; do not wrap a browser; do not spawn
  `dsh-jsonrpc-agent-pkg-macos-arm64` for this route; do not run ACP or
  headless CLI drivers
- **Required validation:** card 222: package-independent fixture/parser tests
  plus `effigy qa:northstar`. Cards 223-224:
  `effigy validate:focused swallowtail-adapter-deepseek-harness`. Card 225:
  that focused selector, `effigy package:verify-affected swallowtail-adapter-deepseek-harness`,
  `effigy qa:guides`, `effigy qa:routes`, `effigy qa:docs`, plus separately
  gated installed and live probes
- **PR base/head:** `main` / selected worker branch (`g03-070-deepseek-harness-web-api`
  unless the launcher supplied a different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** g03.070 cards 222-225 for package
  `swallowtail-adapter-deepseek-harness`, family `deepseek-harness`, route
  `deepseek-harness.local-server`, driver
  `swallowtail.deepseek-harness.local-server`, version axis
  `deepseek-harness.web`, exact pin `@deepseek-ai/dsh@0.1.0-rc.6`
- **Out of scope:** changes to `deepseek-harness.jsonrpc` behavior or its
  runtime-bin pin; `deepseek.continuation` / `swallowtail-adapter-deepseek`;
  ACP, headless CLI, and the browser UI as a driver; settings, credentials,
  `llm.*`, directory picker, filesystem open, preset authoring, ZIP export;
  attachments, queue, subagents, skills, goals, commands; restore or
  hard-delete; non-loopback bind; invented bearer auth; default
  `danger-full-access`; unverified-newer on this RC; Contract 054 public
  support until history proof; version bump, tag, GitHub Release, or
  registry mutation; rewriting immutable `v0.3.2` inventories
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's assigned
  scope; if shared mutable scope or a hidden dependency appears, stop and report
  it through the operator.
- Work only in the selected clean worker worktree: prefer the current
  launcher-provided worktree and record its actual path/branch; otherwise use
  the named fallback created by the startup preflight. Never edit the
  orchestrator's planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** Research 125 → Spec 009 → g03.070. Pattern refs are
  Kimi local-server g02.020 / cards 061-065
  (`crates/swallowtail-adapter-kimi/src/local_server/`) and the existing
  JSON-RPC surface in this same package (`g03.069` / cards 218-221,
  `tests/fixtures/deepseek-harness-runtime-bin-0.1.0rc6/`). Add a distinct
  web fixture tree. Keep the HTTP+WebSocket codec in this adapter. Do not
  flatten onto JSON-RPC types or `deepseek.continuation`.
- **Why these cards are ready:** the pin, spawn, loopback fence, allowlist,
  deny list, and first subset are settled. Card 222 can freeze redacted
  unary and mux fixtures without a production driver. 223-225 are planned on
  purpose; they become executable only after each predecessor lands.
- **Decisions and preferences:**
  - keep JSON-RPC unchanged as the one-shot stdio structured run
  - spawn `dsh web`, not the JSON-RPC binary and not a browser
  - `dsh -V` and `host.describe` are not the compatibility axis
  - do not reuse the JSON-RPC payload digest; freeze npm/CLI file identity
  - POST `/api/<method>` with `application/json`; HTTP 404/415/400/500 are
    carrier-only; business errors are HTTP 200 plus an error branch
  - downlink is `/api/events.mux` and `/api/events.host`, not JSON-RPC
    `session.event`
  - bind `127.0.0.1` only; Host/Origin fence; no bearer lease
  - allowlist: `session.list`, `session.search`, `session.create`,
    `session.history`, `session.models`, `session.prompt`, `session.cancel`,
    `session.fork`, `workspace.list`, `workspace.archiveSession`,
    `host.describe` as bind/liveness only
  - hard deny: `settings.*`, `credentials.*`, `llm.*`, host filesystem and
    directory picker, agent-preset authoring, `GET /api/session.export`
  - `session.history` is a Contract 054 candidate; do not mark 054 supported
    unless corpus and driver prove inspect-without-resume
  - `session.cancel` is native turn abort; do not advertise JSON-RPC
    process-kill as this route's cancel
  - `workspace.archiveSession` hides grouping and is not restore or delete
  - live proof may use host-local Ollama; that does not qualify
    `deepseek-official`
  - catalog id `ollama` is still a dead end (`PI_AI_ERROR`); use a
    hand-declared OpenAI-compat Cordis route if live work needs a local model
  - host supplies CLI, Cordis `--patch`, cwd, provider, and model; Swallowtail
    does not ship a `danger-full-access` default
  - never ingest prompts, reasoning text, tool bodies, credentials, or raw
    export bytes into diagnostics
  - Contract 036 package-count and architecture updates wait for card 225
  - current source is already 31 packages / 37 JSON-RPC routes; this lane
    adds a route, not a package; immutable `v0.3.2` stays 30 / 36
- **Open tensions:** no `dsh web` process was booted in the planning record.
  Reconstruct redacted fixtures from Research 125 and published source. A
  local loopback handshake for corpus capture is in bounds on card 222 if
  fixtures stay redacted. Private probe checkouts and transcripts stay out of
  git. If history cannot be shown control-free, stop rather than claiming
  Contract 054. Card 225's live smoke needs an operator-authorized local
  model path; stop before claiming DeepSeek-official SSE behavior. Additive
  route handling must not mutate immutable tagged baselines.
- **Report after:** each card closeout, and sooner if a stop condition,
  missing capture, or live-probe gate appears
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the top.
Before broad repository reads, run the quick startup worktree-safety preflight in
`## Completion Protocol`. If the
current context is a clean, dedicated, non-`main` registered worktree, it is the
launcher-provided worktree: use it immediately, record its actual path/branch,
and do not compare its generated path/branch with this handoff or create another
worktree. If it is `main`, dirty, unregistered, or otherwise unusable, use the
named worktree if it matches; only then read `.agents.local.env`, require a valid
`AGENTS_WORKTREE_CONTAINER_DIR`, ask the operator if it is absent, and create a
unique manual worktree and branch under that container from pushed `origin/main`.
Never fall back to `/tmp` or `TMPDIR`. Do not run broad repo orientation before
this decision. Read `AGENTS.md`, the active milestone, each assigned card, and
the canonical architecture/contracts from the selected worker worktree.

Once that checks out, start card 222: freeze exact `@deepseek-ai/dsh@0.1.0-rc.6`
CLI/npm identity, loopback trust fence, method allowlist, redacted unary and
mux fixtures, and control-free history paging. When you reach a natural pause,
tell the operator what changed, what validation you actually ran, what
remains, and whether anything needs a planning decision. Keep the conversation
natural and useful; you do not need to repeat this whole file back.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Then run one
   quick read-only safety probe before
   broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare them with the named placeholders
   above or create another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; if it is
   absent, ask the operator before creating the file or worktree. Then create a
   unique worktree and branch under that container from pushed `origin/main`,
   record the actual path and branch, and run all subsequent commands there.
   Never use `/tmp`, `TMPDIR`, or a guessed path; never clean, reset, stash-over,
   or discard the original checkout's dirty state. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 8861792ab0cf8e6bf567c0f372ddabfd63e0bcc6 HEAD`
   succeeds, and confirm this handoff file exists in the selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run the repo's cheap orientation checks and record what you actually ran.
   Prefer `effigy <task>` for supported work. Use explicit package scope for
   normal batch feedback; do not infer scope from changed files.

### While you work

- Execute the ready cards in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop and say so if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into a new architecture.
- Update card status, milestone checkboxes, logs, and the single Next Task
  pointer as each card actually lands. Leave 223-225 `planned` until their
  predecessor's Auto-Continuation is honest.
- Prefer `effigy validate:focused swallowtail-adapter-deepseek-harness` and
  `effigy package:verify-affected swallowtail-adapter-deepseek-harness`. Do not
  run workspace `qa`, MSRV, or live probes unless the accepting card names
  that tier.
- On card 225, keep current-source package and route counts distinct from
  immutable `v0.3.2`. If live acceptance needs a host-local model, ask before
  running it and keep `deepseek-official` unqualified. Do not claim Contract
  054 unless history proof actually passed.

### When the assigned runway is complete

1. Run the required final validation named by card 225, plus any earlier
   card-named checks that have not been re-run on the final tree.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used. Check Spec 009
   boxes this milestone owns. Return the Contract 054 / ACP / JSON-RPC
   continuity checkpoint to the operator; do not start those routes.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   `8861792ab0cf8e6bf567c0f372ddabfd63e0bcc6` is the planning base before the
   handoff commit, not a self-referential hash for the commit that contains
   this file.
5. In the PR body, link Spec 009, milestone g03.070, cards 222-225, changed
   surfaces, evidence, validation, and unresolved items.
6. Report the PR URL and the evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and checks.
Current review state: `awaiting worker PR`.

The orchestrator records an evidence-backed verdict in the provider's review
surface. When the orchestrator and worker share a GitHub identity, formal
self-approval is unavailable, so the orchestrator posts the verdict as a PR
comment; that comment is the canonical review record. If changes are requested,
make only those changes on this branch, push again, and report back through the
operator. Requested changes are: none yet. The PR should
link the card, milestone, spec, changed surfaces, evidence, validation, and
unresolved items. The operator must explicitly authorise any merge.

- **Closeout refs:** `docs/roadmaps/g03/batch-cards/222-deepseek-harness-web-api-corpus.md`,
  `docs/roadmaps/g03/070-deepseek-harness-web-api-foundation.md`,
  `docs/specs/009-deepseek-harness-web-api-route.md`,
  `docs/logs/2026-08-17-deepseek-harness-web-api-planning.md`,
  `docs/roadmaps/README.md`, `docs/roadmaps/g03/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is.
