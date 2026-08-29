---
title: g05.003 operation-scoped watcher HTTP bridge core worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-30
updated: 2026-08-30
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260830-005129-g05-003-watcher-http-bridge-core.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, rust, watchers, http, mcp]
---

## What This Thread Was Doing

Research 260 found that Claude has the provider-side MCP and Stop-hook pieces,
but Swallowtail has no transport from that process into the turn-owned watcher
registry. HTTP was the smallest plausible carrier. The operator has now chosen
to promote that boundary, without adding Docker or starting Claude wiring.

The orchestrator promoted Contract 060, reconciled Research 260 and the
architecture, and compiled g05.003 card 016. This worker owns the
provider-neutral bridge core only.

## Why It Matters

Watchers are useful only when model calls, operator controls, completion
blocking, and cleanup reach one host-owned registry. A loose MCP server or
ambient token would turn that lifecycle guarantee into convention. Card 016
adds the narrow transport authority needed for a later route proof while
keeping secrets, raw process data, and detached work out of the public surface.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `c3ef1982dbbf52c2a21c025c987e0d4029ca17b8`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  the planning base before this handoff was created
- **Planning checkout:** clean before the handoff commit
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts at the base:** active Contract 060; amended Contracts
  010, 041, and 059; promoted Research 260 disposition; ready card 016; revised
  cards 010-011; g05.003 and the sole Next Task reconciled
- **Worker branch:** `worker/g05-003-watcher-http-bridge-core`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-003-watcher-http-bridge-core`
- **Worktree creation command:** `git worktree add -b worker/g05-003-watcher-http-bridge-core /Users/tom/Dev/worktrees/swallowtail-g05-003-watcher-http-bridge-core origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even when its generated path
  or branch differs from the placeholders above. Do not create a second
  worktree for a naming difference. If the launcher context is unusable, use
  the named worktree; only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique fallback there. Never
  use `/tmp` or a guessed path
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contract 060 is canonical
- **Roadmap milestone:** `docs/roadmaps/g05/003-operation-scoped-watcher-proof.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/016-operation-scoped-watcher-http-bridge-core.md`
- **Allowed runway:** card 016 implementation, deterministic fixtures, public
  API baselines, realized architecture, implementation log, and directly
  affected front doors
- **Remaining card budget:** one card; return one reviewable PR
- **Dispatch topology:** serial
- **Parallel safety check:** no sibling implementation lane is authorized.
  Cards 010-011 depend on this bridge and must not run in parallel
- **Canonical refs:** `docs/architecture/product-guardrails.md`,
  `docs/architecture/system-architecture.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/009-async-operation-lifecycle.md`,
  `docs/contracts/010-execution-host-services-and-inputs.md`,
  `docs/contracts/041-input-callback-and-provider-tool-admission.md`,
  `docs/contracts/044-observable-agent-activity-and-disclosure.md`,
  `docs/contracts/059-operation-scoped-process-watchers.md`, and
  `docs/contracts/060-operation-scoped-watcher-http-bridge.md`
- **Evidence refs:** `docs/research/257-claude-code-watcher-seam-evidence.md`
  and `docs/research/260-claude-code-watcher-bridge-transport.md`
- **Primary code surfaces:**
  `crates/swallowtail-runtime/src/host_registry.rs`,
  `crates/swallowtail-runtime/src/watcher/host_service.rs`,
  `crates/swallowtail-core/src/watcher/`,
  `crates/swallowtail-host-local/src/services.rs`,
  `crates/swallowtail-host-local/src/watcher.rs`, host-local watcher modules,
  and matching `swallowtail-testkit` fixtures and public API baselines
- **Current implementation fact:** `HostServices` registers the optional
  watcher port and `LocalHostServices` installs `LocalWatcherHostService`.
  No provider-facing listener, bridge lease, request decoder, or MCP server
  exists
- **Reusable host fact:** operation-scoped temporary working resources,
  bounded text I/O, redacted materialized references, scoped tasks, and joined
  local process supervision already exist. Card 016 may reuse lifecycle and
  redaction patterns, but Claude configuration materialization belongs to card
  010
- **Model capability profile:** capable Rust implementation model with strong
  concurrency and protocol-boundary judgment; no model override required
- **Tool/runtime restrictions:** no subagents, provider prompts, live provider
  probes, login, credentials, installs, containers, Docker/OCI, remote
  topology, Claude adapter edits, route claims, consumer guide publication,
  workflow edits, release work, or merge
- **Inherited health baseline:** the prior doctor run reported 384 god-file
  findings: 337 warnings and 47 errors, plus one generated-in-source warning.
  The graph index was refreshed during planning. Do not add a finding or widen
  this card into baseline cleanup
- **Required validation:**
  `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`;
  `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`;
  `effigy package:api`; `effigy qa:docs`; `effigy qa:northstar`;
  `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and exact head
- **Merge authorization:** not authorized

## Boundaries

- **In scope:** the complete card 016 tranche: stable optional host-service
  registration, object-safe bridge lease, local ephemeral-loopback listener,
  fresh operation-private bearer capability, exact scope correlation, bounded
  closed HTTP/MCP decoding, reserved watcher dispatch, completion barrier,
  deterministic races, joined cleanup, fixtures, API baselines, docs closeout,
  and one PR.
- **Out of scope:** Claude flags, MCP config files, settings, Stop hooks,
  watcher-skill injection, current Claude qualification, authenticated or paid
  provider work, generic MCP or HTTP infrastructure, consumer tool exchange,
  public serving, sign-in-port reuse, remote exposure, TLS, firewall policy,
  arbitrary commands, ambient configuration, containers, sandboxing, route
  capability claims, and merge.
- **Outcome shape:** smallest complete Contract 060 implementation. Diagnose,
  implement, test, clean temporary diagnostics, update exact evidence, and
  open one PR. Do not stop at an API sketch.
- The bearer authenticates bounded calls within one open lease. It is not a
  one-use token, but it cannot cross leases or survive close. Duplicate request
  correlation still fails closed.
- Loopback placement is not authentication. Validate the bearer and exact
  execution host, operation, turn, lease generation, and open state before
  watcher work.
- Preserve one registry. Model bridge calls retain requester identity and
  reach the same `WatcherHostService` used by operator controls.
- Freeze listener admission before successful completion. Join accepted
  connection, dispatch, wait, watcher, and listener work on every terminal
  path. Defensive drop is not cleanup success evidence.
- Keep public formatting inert. Endpoint, bearer, headers, bodies, paths,
  commands, PIDs, environment, and raw watcher output never enter stable
  records, events, diagnostics, or default formatting.
- Work only in the selected worker worktree. Preserve every unrelated checkout
  and change. Do not merge the PR.

## Important Context

- **Planning lineage:** cards 008-009 realized the portable watcher lifecycle
  and host registry. Card 014 connected ordinary host-approved process
  supervision. Card 015 and Research 260 proved the remaining transport gap.
  The operator then selected HTTP and Contract 060 promoted the boundary.
- **Why this card is ready:** host watcher ownership and process join already
  exist; the product choice, authority split, protocol limits, privacy rules,
  completion barrier, and cleanup order are now canonical.
- **Operator preference:** this feature lets an agent run ordinary background
  work while a consumer app sees truthful progress. It is not a container or
  hostile-process containment project.
- **Future sequencing:** card 010 will bind Claude through operation-private
  temporary MCP/settings/skill material only after this PR and current provider
  gates. Card 011 owns live same-turn acceptance and consumer claims. Neither
  card is part of this worker's continuation envelope.
- **Open tension:** select the smallest HTTP implementation that can express
  strict bounds, cancellation, and joined tasks without creating a generic
  server framework. Reuse existing operation scopes, redacted lease patterns,
  and scoped task ownership. Bring back any need for broader protocol or host
  authority rather than inventing it.
- **Repo posture:** follow `AGENTS.md`, Effigy selectors, the strict everyday
  Rust-quality profile and named deviations, meaningful-batch rules, and
  `PAPERCUTS.md`. Record only a genuine incidental papercut; do not divert the
  implementation.
- **Report after:** first coherent type/service/listener slice plus its
  authentication and close fixtures; then final validation and PR closeout.
- **Report to:** the operator, who will relay progress and the PR to the
  orchestrator.

## Suggested Next Move

Start with the worker preflight. Then trace `HostServiceKind`, `HostServices`,
`WatcherHostService`, `LocalHostServices::compose`, `LocalWatcherHostService`,
scoped tasks, and redacted resource leases before designing new types.

Write down the ownership sequence from open through ready, request dispatch,
completion freeze, watcher join, listener join, and private-material release.
Implement that sequence as one coherent tranche with deterministic fixtures.
If strict MCP decoding or cleanup requires generic server authority, pause and
return the exact gap instead of widening Contract 060.

## Completion Protocol

### Before you start

1. Read this handoff. Its worker metadata activates implementation mode. Before
   broad reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a clean, registered, dedicated non-`main` worktree,
   accept it as launcher-provided. Record the actual root and branch. Do not
   create another because names differ.
3. If the launcher supplied a dirty or `main` worktree, stop and report it.
   Only for another unusable context, inspect the named worktree and then
   `.agents.local.env`; require `AGENTS_WORKTREE_CONTAINER_DIR` before a unique
   fallback. Never clean or reset another checkout and never use `/tmp`.
4. In the selected worktree, fetch origin. Confirm `HEAD == origin/main`,
   confirm `git merge-base --is-ancestor c3ef1982dbbf52c2a21c025c987e0d4029ca17b8 HEAD`,
   and load the tracked handoff with
   `git show HEAD:docs/handoffs/20260830-005129-g05-003-watcher-http-bridge-core.md`.
   If the absolute file differs, stop. The tracked copy is canonical.
5. Required sibling worktree links are `none`.
6. Read `AGENTS.md`, `PAPERCUTS.md`, card 016, g05.003, Contract 060, the
   canonical refs, Research 260, the named code surfaces, and the repository
   Rust-quality profile and deviations before Rust edits.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited baseline but do not run the broad workspace plan.

### While you work

- Execute card 016 only. Keep API, local-host implementation, lifecycle, and
  deterministic fixtures in meaningful batches.
- Do not spawn internal agents. The operator owns thread parallelism in their
  harness.
- Use the four exact package names in the card's focused selectors. Do not
  infer scope from changed files or substitute broad workspace tests.
- Preserve omission, redaction, one-registry control, fail-closed admission,
  and joined cleanup. Add no provider, container, public-network, arbitrary
  tool, or ambient configuration authority.
- Report the first coherent implementation slice through the operator before
  final closeout. Stop on a contract gap, new product choice, or scope change.

### When the assigned runway is complete

1. Run every required validation command named in `## Current State`.
2. Mark card 016 complete with exact changed surfaces and validation. Create
   one dated g05.003 bridge-core implementation log and reconcile the milestone,
   batch-card index, generation index, logs index, and sole roadmap Next Task.
   Set the next move to an orchestrator planning checkpoint. Do not mark cards
   010-011 ready or authorize live provider work.
3. Update realized architecture and public API baselines for code that actually
   landed. Confirm the diff makes no Claude, route-support, container, public
   listener, generic MCP, or secret-disclosure claim.
4. Push the selected worker branch and open one reviewable PR against current
   pushed `main`.
5. Link card 016, g05.003, Contract 060, Research 260, the implementation log,
   changed host/runtime surfaces, fixtures, API baseline, and validation in the
   PR body.
6. Report the PR URL and exact head to the operator. Do not merge or start
   cards 010-011.

### Review and merge path

The orchestrator will review the exact PR head against Contract 060, card 016,
the code diff, public API baseline, focused validation, and hosted checks. With
the shared GitHub identity, the canonical verdict may be a PR comment rather
than formal self-approval. Requested changes are `none` at dispatch. The
operator must explicitly authorize merge.

- **Closeout refs:** card 016; g05.003 milestone; bridge-core implementation
  log; g05 batch-card, generation, log, architecture, and roadmap front doors
- **Currentness:** Contract 029 remains a separate standing lane and is not
  part of this PR

### Handoff closeout

Return one PR and stop. Leave Claude cards 010-011 behind the orchestrator
planning checkpoint. If the closed bridge cannot be implemented without a
generic server, leaked authority, or unjoined work, record the smallest exact
blocker and stop rather than weakening the contract.
