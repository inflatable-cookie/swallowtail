---
title: g04.078 llama.cpp owned reasoning controls worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260826-224209-g04-078-llama-cpp-owned-reasoning-controls.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator fast-forward merged PR 76 at exact head
`f87be6924c71ee411336b3ca4b1d272c820dfdd9`, resumed the sole roadmap
continuation, and reassessed the remaining per-route feature inventory. It
selected llama.cpp owned-runtime reasoning selection and budget as the next
evidence candidate.

The orchestrator compiled g04.078, cards 216-218, Research 225, programme and
front-door updates, triage disposition, and the compilation log. The planning
base was validated and pushed to `main` at
`8e53a3e6b0b7e68fb5651ae1f50ef9680c4acb4f`.

This is one bounded manual implementation thread. Start from this file without
a copied transcript or a second prompt. Do not spawn internal agents; the
operator owns parallelism in their harness.

## Why It Matters

`llama-cpp.owned` controls exact server point `b10069-178a6c449` and already
owns the child process, immutable launch plan, operator-supplied model path,
context size, readiness, cancellation, terminal state, and joined cleanup.
Exact llama.cpp documents `--reasoning on|off|auto` and
`--reasoning-budget -1|0|N`, but Swallowtail exposes neither.

That is a credible serving-owned seam, not yet a model reasoning capability.
The effective behavior depends on the selected GGUF/chat template, reasoning
tags, format, and server application. Research 225 must prove a useful
model/template row that can be bound or rejected before process work. A flag
that parses but can silently do nothing is an evidence stop, not a feature.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning base:** `main`
- **Planning commit before this handoff:**
  `8e53a3e6b0b7e68fb5651ae1f50ef9680c4acb4f`
- **Planning publication:** planning commit is exact `origin/main` before this
  handoff commit
- **Planning checkout:** shared main checkout; do not use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates worker-only worktree preflight
- **Planning artifacts:** g04.078, cards 216-218, Research 225 reservation,
  compilation log, programme/triage/index updates, and sole Next Task
- **Worker branch:** `worker/g04-078-llama-cpp-owned-reasoning-controls`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-078-llama-cpp-owned-reasoning-controls`
- **Worktree creation command:** `git worktree add -b
  worker/g04-078-llama-cpp-owned-reasoning-controls
  /Users/tom/Dev/worktrees/swallowtail-g04-078-llama-cpp-owned-reasoning-controls
  origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent; never use `/tmp`, `TMPDIR`,
  or a guessed path for a worktree.
- **Active spec lane:** per-route feature completion programme
- **Roadmap milestone:**
  `docs/roadmaps/g04/078-llama-cpp-owned-reasoning-controls.md`
- **Ready cards, in order:**
  `216-llama-cpp-owned-reasoning-controls-evidence.md`, then conditional
  `217-llama-cpp-owned-reasoning-controls-binding.md`, then conditional
  `218-llama-cpp-owned-reasoning-controls-acceptance.md`
- **Allowed runway:** execute card 216 and promote Research 225; continue to
  cards 217-218 only for a non-empty exact row with preflight-bindable
  model/template applicability and deterministic selected behavior
- **Remaining card budget:** three cards; stop after card 216 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one reviewable PR; no internal
  agents or subagents
- **Parallel safety check:** serial because evidence decides whether binding
  and acceptance exist and every card touches the same llama.cpp owned route
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  010, 011, 023, 024, 029, 037, 040, 041, and 052
- **Model capability profile:** exact source/artifact research plus route-local
  Rust implementation and deterministic conformance
- **Tool/runtime restrictions:** exact official source/artifact evidence and
  secret-free local parser/source work only; no install, update, model
  download/load, prompt, inference, tool execution, provider work, paid work,
  ambient config mutation, or sibling-route work
- **Required validation:** card 216 checks first; if delivery proceeds,
  `cargo fmt -p swallowtail-adapter-llama-cpp`,
  `effigy validate:focused swallowtail-adapter-llama-cpp`,
  `effigy package:verify-affected swallowtail-adapter-llama-cpp`,
  `effigy check:examples`, `effigy package:api`, `effigy qa:northstar`, named
  research/log/roadmap/card/next-action checks, `effigy doctor`, and
  `git diff --check`
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings (334
  warnings, 46 errors); `scan.generated-in-src` reports one warning; graph
  index is stale. Existing papercut records cover the structural baseline;
  record drift and do not add duplicates or repair unrelated findings.
- **PR base:** `main`
- **PR head:** worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised; operator must explicitly request it

## Boundaries

Keep this run inside the named runway:

- **In scope:** exact `b10069-178a6c449` `--reasoning` and
  `--reasoning-budget` parser, precedence, defaults, model/template,
  application, no-op/failure, and observation evidence; conditional closed
  adapter-local preparation and owned-driver binding; canonical argv;
  deterministic acceptance; context-size composition; route-local
  docs/matrices/API truth; Research 225; closeout; and sole Next Task
- **Out of scope:** portable reasoning effort/budget, raw values, model
  selection/download, chat-template or reasoning-format changes, sampling,
  output-parser changes, `llama-cpp.attached`, owned-route inference, live
  model work, another route feature, currentness, release, generation rollover,
  g04 closure, or merge
- Existing construction must remain exact: omission sends no reasoning
  arguments and every current context-size row keeps its exact argv and bounds.
- The route remains an owned serving route. Do not infer model reasoning
  capability, inference effectiveness, or attached-route output support from a
  server launch flag.
- Requested, prepared, dispatched, parser-accepted, applied, effective, and
  observed truth remain separate. Claim only the strongest exact evidence
  level Research 225 admits.
- The operator-supplied GGUF may lack a reasoning-capable chat template. Every
  deliver-now row needs a preflight-bindable applicability fact or an exact
  fail-closed gate before process work.
- Do not invent architecture, change contracts, widen the roadmap, or choose
  an unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's scope or
  spawn subagents. If shared mutable scope or a hidden dependency appears,
  stop and report it through the operator.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Exact route point:** route `llama-cpp.owned`, driver
  `swallowtail.llama-cpp.owned-b10069-openai-chat`, axis
  `llama.cpp.owned-runtime`, point `b10069-178a6c449`.
- **Exact official lead:**
  `https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/README.md`
  documents `--reasoning on|off|auto`, default `auto`; and
  `--reasoning-budget N`, where `-1` is unrestricted, `0` ends immediately,
  positive `N` is a token budget, and the default is `-1`. Freeze decisive
  source and parser behavior rather than relying on the README alone.
- **Related exact controls:** the same exact README documents
  `--reasoning-format`, budget messages, and reasoning preservation. Research
  225 must trace interactions but may not widen the lane to expose them.
- **Existing precedent:** Research 203 and g04.056 delivered positive
  `--ctx-size` as a dispatch-only owned-runtime control. Reuse its exact
  identity, behavior revision, immutable evidence, omission, and argv patterns.
- **Current implementation seam:** owned driver construction lives in
  `crates/swallowtail-adapter-llama-cpp/src/driver/owned.rs`; context-size
  typed input/evidence and prepared owned state live under
  `src/context_size.rs` and `src/prepared/owned/`.
- **Protocol truth:** `ChatTemplateCapabilities` includes
  `supports_preserve_reasoning`. The current attached inference profile rejects
  reasoning-related capabilities/content; that separate boundary must not
  change.
- **Readiness tension:** `/props` may report chat-template capabilities, but it
  may not report the selected reasoning mode/budget or effective model
  behavior. Record exactly which evidence tier any prompt-free channel proves.
- **Open tensions:** `auto` may depend entirely on template detection; budget
  may require start/end tags; explicit selection may parse yet remain inert;
  repeated CLI/environment values may have precedence; model metadata may be
  unavailable before launch; `--reasoning-format` may alter response shape.
- **Decisions and preferences:** closed llama.cpp-local types only; selection
  and budget may qualify independently; no raw string/number escape hatch; no
  live proof; an empty Research 225 set is valid when applicability cannot be
  bound before process work.
- **Report after:** Research 225 and card 216 are complete, or earlier when a
  stop condition fires. If evidence is non-empty, continue through cards
  217-218 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Execute card 216 as one coherent evidence chunk. Begin with exact source and
artifact identity, then trace both flags through parsing, stored server params,
template/model applicability, request rendering, response formatting, and
prompt-free observation. Promote an exact empty or non-empty Research 225 set
before touching production binding.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare it with the placeholders or create
   another worktree merely because they differ.
3. Otherwise stop before edits. Use the named registered worktree if it is
   clean and correctly based. If no usable worker worktree exists, follow the
   `.agents.local.env` fallback policy in `## Current State`. Never edit
   `main`, a dirty checkout, or another worker's branch.
4. Fetch `origin/main`. Require the worker base to contain planning commit
   `8e53a3e6b0b7e68fb5651ae1f50ef9680c4acb4f`. Fast-forward or recreate the
   clean worker branch if needed; do not merge main into it.
5. Read `AGENTS.md`, the `northstar` and `effigy` skills, the g04.078 roadmap,
   cards 216-218, Research 225, Research 203, the llama.cpp prepared guide,
   relevant contracts, and the advanced-feature triage tail before edits.

### Execute and stop correctly

6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Treat the
   inherited doctor baseline above as known; report only drift.
7. Execute card 216. Use exact `b10069` source/artifacts and deterministic,
   prompt-free local evidence. Do not install, update, load a model, prompt a
   provider, or mutate ambient configuration.
8. Promote Research 225 and update card/milestone state. If the deliver-now set
   is empty, mark cards 217-218 blocked, complete the honest evidence-stop
   closeout, update indexes and the sole Next Task, validate, and stop.
9. If and only if Research 225 admits a non-empty exact set, execute cards 217
   and 218 serially. Bind only admitted rows. Do not use a test to invent a
   model/template fact the route cannot preflight in production.
10. Work in meaningful batches. Run focused validation after the evidence
    chunk and the complete named acceptance round once after implementation.
    Do not repair inherited doctor findings or unrelated papercuts.

### Prepare the review handoff

11. Update Research 225, the roadmap/cards, guide, matrices, programme, triage,
    logs, indexes, changelog, API baseline when changed, and sole Next Task so
    they agree on complete delivery or honest stop.
12. Run every applicable card command. At minimum run the exact package-focused
    selectors and all named docs/index checks. Run `git diff --check` and
    `effigy doctor`; record exact failures or baseline drift.
13. Review `git diff --stat`, `git diff --check`, `git status --short`, and the
    full changed-file list. Ensure the branch contains no credentials,
    downloaded models, runtime caches, ambient config, generated probe debris,
    or unrelated changes.
14. Commit coherent worker changes, push the worker branch, and open one PR to
    `main`. Do not merge it. Confirm the PR head SHA equals the pushed branch
    head and report required CI state.
15. Return a compact operator report containing: outcome and evidence tier;
    exact Research 225 deliver-now or empty set; cards executed/blocked;
    files and public API changed; validation and doctor drift; PR URL, number,
    base, head SHA, mergeability, and CI; unresolved risks; and the precise
    next move. Keep g04 open.
