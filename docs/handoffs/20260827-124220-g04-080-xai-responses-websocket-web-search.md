---
title: g04.080 xAI Responses WebSocket web search worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-124220-g04-080-xai-responses-websocket-web-search.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator resumed the sole roadmap continuation after g04.079 and
reassessed the remaining advanced per-route feature inventory. It selected
provider-owned web search on `xai.responses-websocket` as the next bounded
evidence candidate.

The orchestrator compiled g04.080, cards 222-224, Research 227, programme and
front-door updates, triage disposition, and the compilation log. It also
repaired inherited generation-index drift that still described g04.079 as
ready. The planning base was validated and pushed to `main` at
`27dcc8fc5e520576b5dbceda90f32a84dddba254`.

This is one bounded manual implementation thread. Start from this file without
a copied transcript or a second prompt. Do not spawn internal agents; the
operator owns parallelism in their harness.

## Why It Matters

`xai.responses-websocket` already owns one exact Responses WebSocket,
structured runs, serial text sessions, selected Grok 4.5/4.6 models,
model-qualified reasoning, positive output bounds, `store=false`, private
continuation, usage, billed cost, restoration, terminal mapping, and joined
cleanup. Every request still emits `tools: []`.

Current official xAI material says WebSocket `response.create` uses the
Responses create body and separately documents provider-owned `web_search`,
server-side search-call items, citations, and tool-turn bounds. Those mutable
claims do not widen Swallowtail's exact dated xAI facade. Exact WebSocket
composition, model/profile membership, request bounds, response grammar,
citation, usage, billing, and lifecycle truth are the gate.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning base:** `main`
- **Planning commit before this handoff:**
  `27dcc8fc5e520576b5dbceda90f32a84dddba254`
- **Planning publication:** planning commit is exact `origin/main` before this
  handoff commit
- **Planning checkout:** shared main checkout; do not use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates worker-only worktree preflight
- **Planning artifacts:** g04.080, cards 222-224, Research 227 reservation,
  compilation log, programme/triage/index updates, and sole Next Task
- **Worker branch:** `worker/g04-080-xai-responses-websocket-web-search`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-080-xai-responses-websocket-web-search`
- **Worktree creation command:** `git worktree add -b
  worker/g04-080-xai-responses-websocket-web-search
  /Users/tom/Dev/worktrees/swallowtail-g04-080-xai-responses-websocket-web-search
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
  `docs/roadmaps/g04/080-xai-responses-websocket-web-search.md`
- **Ready cards, in order:**
  `222-xai-responses-websocket-web-search-evidence.md`, then conditional
  `223-xai-responses-websocket-web-search-binding.md`, then conditional
  `224-xai-responses-websocket-web-search-acceptance.md`
- **Allowed runway:** execute card 222 and promote Research 227; continue to
  cards 223-224 only for a non-empty exact row with proved WebSocket request
  support, closed model/profile membership, a positive provider-side use
  bound, and bounded response/citation truth
- **Remaining card budget:** three cards; stop after card 222 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one reviewable PR; no internal
  agents or subagents
- **Parallel safety check:** serial because evidence decides whether binding
  and acceptance exist and every card touches the same xAI route
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 029, 037, 039, 040, 041, 044, and 052
- **Model capability profile:** exact official WebSocket/API/schema research
  plus route-local Rust implementation and deterministic conformance
- **Tool/runtime restrictions:** secret-free official-doc/schema and local
  fixture/source work only; no provider request, credential use, paid search,
  account or organization inspection, ambient configuration mutation, or
  sibling-route work
- **Required validation:** card 222 checks first; if delivery proceeds,
  `cargo fmt -p swallowtail-adapter-xai`,
  `effigy validate:focused swallowtail-adapter-xai`,
  `effigy package:verify-affected swallowtail-adapter-xai`,
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

- **In scope:** exact `xai.responses-websocket` WebSocket and Responses-body
  `web_search` composition, selected-model and operation-profile support,
  canonical tool syntax, positive provider-side use bound, optional-filter and
  source-inclusion disposition, search-call events, assistant content,
  citations, usage, billed cost, provider failures, cancellation, deadline,
  disconnect, terminal ordering, current route mapping, conditional binding,
  deterministic acceptance, route-local docs/matrices/API truth, Research 227,
  closeout, and sole Next Task
- **Out of scope:** X search, image search, code execution, file or collection
  search, MCP, functions, consumer tool exchange, raw tool arrays, caller
  filters, browser control, host fetch, arbitrary host networking, live
  provider work, sibling routes, currentness, release, generation rollover,
  g04 closure, or merge
- Existing omission must remain exact: `ExternalSearchPolicy::Disabled`, denied
  host networking, and byte-equivalent `tools: []` request bodies.
- The only eligible public intent is existing
  `ExternalSearchPolicy::Enabled`. Exact provider tool shape, any required
  include fields, and the fixed positive bound remain adapter-owned.
- Structured runs, first session turns, continuation turns, and fresh
  restoration must be classified independently. Do not infer one from another.
- Dispatch, provider acceptance, invocation, result delivery, citation
  delivery, usage, billing, model choice, and terminal truth remain separate.
  A model declining to search is not adapter failure.
- Provider-owned search does not authorize host networking or create a
  consumer callback/tool-result port.
- Preserve endpoint/access authority, `store=false`, model/reasoning/output
  controls, continuation, restoration, cancellation, deadline, socket
  invalidation, billed cost, and joined cleanup.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's scope or
  spawn subagents. If shared mutable scope or a hidden dependency appears,
  stop and report it through the operator.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Exact route point:** route `xai.responses-websocket`, driver
  `swallowtail.xai.websocket`, facade axis
  `xai.responses-websocket-facade`, selected Grok 4.5/4.6 route revisions,
  structured-run and serial-session profiles, and current private behavior
  revisions.
- **Official leads:**
  `https://docs.x.ai/developers/advanced-api-usage/websocket-mode`,
  `https://docs.x.ai/developers/tools/web-search`,
  `https://docs.x.ai/developers/tools/tool-usage-details`, and
  `https://docs.x.ai/developers/tools/citations`. Freeze retrieved dates,
  final URLs, complete-body digests, decisive excerpts or schemas, and exact
  applicability. Mutable pages are not qualified-version proof by themselves.
- **Current request:** both run and session request paths encode exact selected
  model, reasoning, output bound, `store=false`, and `tools: []`. Serial
  continuation adds private `previous_response_id`. Omission must preserve the
  current canonical body exactly.
- **Existing public policy:** Contract 041 already distinguishes
  `ExternalSearchPolicy` from `ExternalNetworkPolicy`. Reuse enabled search
  intent only when exact evidence admits it; keep host networking denied.
- **Response tension:** freeze the complete WebSocket event/item grammar for
  `web_search_call`, action/query metadata, provider-owned output visibility,
  assistant content, annotations or inline citations, usage, cost, errors,
  completion, and close. Do not expose raw provider payloads.
- **Bound tension:** settle the smallest positive provider-side query/use/tool
  bound, its field and numeric domain, default and omission, overflow, and
  interaction with reasoning/output bounds. Do not replace it with host retry
  or client truncation.
- **Model tension:** current material explicitly names `grok-4.6`; classify
  exact selected Grok 4.5 and 4.6 route revisions rather than using catalogue
  presence or a family prefix.
- **Related evidence:** Research 187 freezes current exact xAI reasoning and
  output-control truth. Research 169 and the realtime prepared integration
  guide freeze WebSocket lifecycle and continuation behavior.
- **Decisions and preferences:** one closed provider-owned web-search intent;
  no raw provider options; no live proof; an empty Research 227 set is valid
  when exact WebSocket composition, model membership, bound, or event truth
  cannot be frozen without provider work.
- **Report after:** Research 227 and card 222 are complete, or earlier when a
  stop condition fires. If evidence is non-empty, continue through cards
  223-224 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Execute card 222 as one coherent evidence chunk. Begin with exact WebSocket
body composition and model/profile membership, then trace canonical tool
syntax, provider-side bounds, response events/items, citations, usage, billing,
failures, continuation, restoration, and terminal ordering. Promote an exact
empty or non-empty Research 227 set before touching production binding.

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
   `27dcc8fc5e520576b5dbceda90f32a84dddba254`. Fast-forward or recreate the
   clean worker branch if needed; do not merge main into it.
5. Read `AGENTS.md`, the `northstar` and `effigy` skills, the g04.080 roadmap,
   cards 222-224, Research 227, Research 187, Research 169, the realtime
   prepared guide, relevant contracts, and the advanced-feature triage tail
   before edits.

### Execute and stop correctly

6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Treat the
   inherited doctor baseline above as known; report only drift.
7. Execute card 222. Use exact official documentation/schema and deterministic,
   prompt-free local evidence. Do not send a provider request, use credentials,
   incur paid search, inspect account state, or mutate ambient configuration.
8. Promote Research 227 and update card/milestone state. If the deliver-now set
   is empty, mark cards 223-224 blocked, complete the honest evidence-stop
   closeout, update indexes and the sole Next Task, validate, and stop.
9. If and only if Research 227 admits a non-empty exact set, execute cards 223
   and 224 serially. Bind only admitted model/profile rows. Do not use a fixture
   to invent WebSocket support, bounds, citations, or provider behavior the
   production route cannot guarantee.
10. Work in meaningful batches. Run focused validation after the evidence chunk
    and the complete named acceptance round once after implementation. Do not
    repair inherited doctor findings or unrelated papercuts.

### Prepare the review handoff

11. Update Research 227, the roadmap/cards, guide, matrices, programme, triage,
    logs, indexes, changelog, API baseline when changed, and sole Next Task so
    they agree on complete delivery or honest stop.
12. Run every applicable card command. At minimum run the exact package-focused
    selectors and all named docs/index checks. Run `git diff --check` and
    `effigy doctor`; record exact failures or baseline drift.
13. Review `git diff --stat`, `git diff --check`, `git status --short`, and the
    full changed-file list. Ensure the branch contains no credentials, fetched
    page caches outside authorized evidence, runtime caches, ambient config,
    generated probe debris, or unrelated changes.
14. Commit coherent worker changes, push the worker branch, and open one PR to
    `main`. Do not merge it. Confirm the PR head SHA equals the pushed branch
    head and report required CI state.
15. Return a compact operator report containing: outcome and evidence tier;
    exact Research 227 deliver-now or empty set; cards executed/blocked; files
    and public API changed; validation and doctor drift; PR URL, number, base,
    head SHA, mergeability, and CI; unresolved risks; and the precise next move.
    Keep g04 open.
