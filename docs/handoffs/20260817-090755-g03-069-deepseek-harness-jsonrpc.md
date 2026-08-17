---
title: g03.069 DeepSeek Harness JSON-RPC worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: merged
owner: Tom
created: 2026-08-17
updated: 2026-08-17 11:36:00 +0100
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260817-090755-g03-069-deepseek-harness-jsonrpc.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Swallowtail needs a route for DeepSeek AI's new DeepSeek Harness. The planning
thread probed the JSON-RPC stdio runtime, promoted Research 124, wrote Spec
008, and compiled g03.069. The first surface is owned-process NDJSON JSON-RPC,
not Open Platform continuation, not ACP, and not the Web UI.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

DeepSeek Harness is a developer-preview installed runtime with its own wire,
pin, and composition model. Swallowtail already has `deepseek.continuation`
for hosted Open Platform V4 Pro. Flattening the harness onto that package
would hide a different process, protocol, and capability set. This tranche
adds one exact structured-run route the same way Muse and Command Code landed:
corpus, driver, prepared facade, then package truth.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `8b45ea505a42e53528735a25d983c18149bfd643`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Research 124, Spec 008,
  milestone g03.069, ready card 218, planned cards 219-221, and the 2026-08-17
  planning log
- **Worker branch:** `g03-069-deepseek-harness-jsonrpc`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g03-069-deepseek-harness-jsonrpc`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g03-069-deepseek-harness-jsonrpc "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g03-069-deepseek-harness-jsonrpc" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** `docs/specs/008-deepseek-harness-jsonrpc-route.md`
- **Roadmap milestone:** `docs/roadmaps/g03/069-deepseek-harness-jsonrpc-foundation.md`
- **Ready cards, in order:** `docs/roadmaps/g03/batch-cards/218-deepseek-harness-artifact-and-event-corpus.md`
- **Allowed runway:** g03.069 cards 218 → 219 → 220 → 221. Only 218 is ready
  at dispatch. Continue to 219, then 220, then 221 only through each card's
  Auto-Continuation after the predecessor's acceptance is actually met.
- **Remaining card budget:** four cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** card 217 is Figmatic work in another repository.
  Cards 218-221 share the new adapter crate and public route truth, so they
  stay serial.
- **Canonical refs:** `docs/architecture/system-architecture.md`;
  `docs/architecture/release-and-package-topology.md`; Contracts 005-006,
  009-010, 023, 029, 032-033, 036-037, 039-041, 044-045, 051-052;
  `docs/research/124-deepseek-harness-jsonrpc-route-qualification.md`;
  `docs/contracts/001-working-rules.md`
- **Model capability profile:** capable coding model, medium reasoning
- **Tool/runtime restrictions:** spawn the host-approved bundled
  `dsh-jsonrpc-agent` payload; do not wrap the Python SDK; do not boot the
  Web UI; do not run ACP or headless CLI drivers
- **Required validation:** card 218: package-independent fixture/parser tests
  plus `effigy qa:northstar`. Cards 219-220:
  `effigy validate:focused swallowtail-adapter-deepseek-harness`. Card 221:
  that focused selector, `effigy package:verify-affected swallowtail-adapter-deepseek-harness`,
  `effigy qa:guides`, `effigy qa:routes`, `effigy qa:docs`, plus separately
  gated installed and live probes
- **PR base/head:** `main` / selected worker branch (`g03-069-deepseek-harness-jsonrpc`
  unless the launcher supplied a different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** g03.069 cards 218-221 for package
  `swallowtail-adapter-deepseek-harness`, family `deepseek-harness`, route
  `deepseek-harness.jsonrpc`, driver `swallowtail.deepseek-harness.jsonrpc`,
  version axis `deepseek-harness.runtime-bin`, exact pin `0.1.0rc6` plus
  payload digest `ac1c91462518427467bd0a0ca3bf1049df62be0dbe8b0ee8014c6761cb8f80bf`
- **Out of scope:** `deepseek.continuation` / `swallowtail-adapter-deepseek`;
  ACP, headless CLI, and Web `/api` routes; session-id continuity; subagent
  control; model catalogue; native JSON-RPC cancel; default
  `danger-full-access`; wrapping Python; unverified-newer on this RC;
  version bump, tag, GitHub Release, or registry mutation; rewriting
  immutable `v0.3.2` inventories; Figmatic card 217
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

- **Planning lineage:** Research 124 → Spec 008 → g03.069. Pattern refs are
  Muse g03.045 / cards 135-138 and Command Code g03.059 / cards 181-184.
  Follow `crates/swallowtail-adapter-command-code` and
  `crates/swallowtail-adapter-muse`: fixture tree under
  `tests/fixtures/<pin>/`, `[[test]] name = "corpus"`, then driver, prepared
  facade, live probe. Keep the JSON-RPC codec in the new adapter, as Command
  Code kept NDJSON in-package.
- **Why these cards are ready:** the pin, wire, first subset, and exclusions
  are settled. Card 218 can freeze redacted fixtures without production run
  behavior. 219-221 are planned on purpose; they become executable only after
  each predecessor lands.
- **Decisions and preferences:**
  - `serverInfo.version` (`0.0.1`) is not the compatibility axis
  - `session/prompt` returns `{ messageId }` only; Swallowtail owns the idle
    interval through `session.status` idle
  - cancel is force-stop of the owned process; do not advertise a wire cancel
  - live proof may use host-local Ollama through `dsh-llm-pi-ai` with a
    non-empty dummy bearer; that does not qualify `deepseek-official`
  - catalog id `ollama` is a known dead end (`PI_AI_ERROR`); the probe used a
    hand-declared OpenAI-compat route `local-ollama`
  - observe harness-owned `bash` / `str_replace_editor` when the approved
    Cordis composition mounts them; never ingest argument or result bodies,
    reasoning text, prompts, or raw JSONL into diagnostics
  - live JSON-RPC stream cardinality is much larger than durable JSONL;
    bound the live stream
  - host supplies executable, Cordis path, cwd, provider, and model; no
    invented defaults
  - Contract 036 package-count and architecture updates wait for card 221
- **Open tensions:** private probe captures live outside this repo and must
  not be committed. Reconstruct redacted fixtures from Research 124. If a
  private capture is required to keep lifecycle meaning, ask rather than
  importing that checkout as a hidden build input. Card 221's live smoke
  needs an operator-authorized local model path; stop before claiming
  DeepSeek-official SSE behavior. Additive package handling must not mutate
  immutable tagged baselines.
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

Once that checks out, start card 218: freeze the exact `0.1.0rc6` artifact,
JSON-RPC handshake, redacted text/tool/failure fixtures, and stream rules.
When you reach a natural pause, tell the operator what changed, what
validation you actually ran, what remains, and whether anything needs a
planning decision. Keep the conversation natural and useful; you do not need
to repeat this whole file back.

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
   `git merge-base --is-ancestor 8b45ea505a42e53528735a25d983c18149bfd643 HEAD`
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
  pointer as each card actually lands. Leave 219-221 `planned` until their
  predecessor's Auto-Continuation is honest.
- Prefer `effigy validate:focused swallowtail-adapter-deepseek-harness` and
  `effigy package:verify-affected swallowtail-adapter-deepseek-harness` once
  that package exists. Do not run workspace `qa`, MSRV, or live probes unless
  the accepting card names that tier.
- On card 221, keep current-source package counts distinct from immutable
  `v0.3.2`. If live acceptance needs a host-local model, ask before running
  it and keep `deepseek-official` unqualified.

### When the assigned runway is complete

1. Run the required final validation named by card 221, plus any earlier
   card-named checks that have not been re-run on the final tree.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used. Check Spec 008
   boxes this milestone owns. Return the ACP / Web `/api` / session-id
   checkpoint to the operator; do not start those routes.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   `8b45ea505a42e53528735a25d983c18149bfd643` is the planning base before the
   handoff commit, not a self-referential hash for the commit that contains
   this file.
5. In the PR body, link Spec 008, milestone g03.069, cards 218-221, changed
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

- **Closeout refs:** `docs/roadmaps/g03/batch-cards/218-deepseek-harness-artifact-and-event-corpus.md`,
  `docs/roadmaps/g03/069-deepseek-harness-jsonrpc-foundation.md`,
  `docs/specs/008-deepseek-harness-jsonrpc-route.md`,
  `docs/logs/2026-08-17-deepseek-harness-jsonrpc-planning.md`,
  `docs/roadmaps/README.md`, `docs/roadmaps/g03/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is.
