---
title: g05.003 Docker containment evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-29
updated: 2026-08-29
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260829-175456-g05-003-docker-containment-evidence.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, research, watchers, docker, containment]
---

## What This Thread Was Doing

PR 117 landed the portable and host-local watcher registry without making a
false default process-containment claim. The next gate is now narrower: prove
one real host composition before any Claude watcher wiring can begin.

The operator selected macOS through an opt-in OCI-backed supervisor. The
orchestrator narrowed the first candidate to the exact Docker Engine API
already available on this host and compiled g05.003 card 014. This worker owns
that research-and-live-proof card only. Produce Research 260 and one reviewable
PR; do not implement the production backend.

## Why It Matters

The watcher feature promises that owned background work cannot outlive its
turn. A container product name is not proof. The engine lifecycle must contain
closed-pipe `setsid` descendants and concurrent forks, retain exact cleanup
identity through failures, and prove the owned scope destroyed without numeric
PID authority. A positive result unlocks an implementation card. An honest
empty result prevents card 010 from building on a cosmetic sandbox.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `0d1ef473b54f3160ad68c0c70bf1c40917a4e396`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  the planning base before this handoff commit
- **Planning checkout:** clean after the card 014 planning commit
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** operator target disposition,
  ready card 014, g05.003 runway, repaired card 010 status, front doors, and
  process-containment decision log
- **Worker branch:** `worker/g05-003-docker-containment-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-003-docker-containment-evidence`
- **Worktree creation command:** `git worktree add -b worker/g05-003-docker-containment-evidence /Users/tom/Dev/worktrees/swallowtail-g05-003-docker-containment-evidence origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, regardless of generated path
  or branch. If unusable, use the named worktree; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique fallback there. Never use `/tmp` or a guessed path.
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contracts 010 and 059 are canonical
- **Roadmap milestone:** `docs/roadmaps/g05/003-operation-scoped-watcher-proof.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/014-docker-engine-containment-composition-evidence.md`
- **Allowed runway:** card 014, Research 260, its frozen evidence, lane log,
  and honest closeout surfaces only
- **Remaining card budget:** one research/proof card; stop at reviewable PR
- **Dispatch topology:** serial
- **Parallel safety check:** cards 010-011 depend on this decision; no other
  containment or Claude watcher worker may run concurrently
- **Canonical refs:** `docs/architecture/product-guardrails.md`;
  `docs/contracts/009-async-operation-lifecycle.md`,
  `docs/contracts/010-execution-host-services-and-inputs.md`,
  `docs/contracts/023-harness-runtime-isolation-and-access-policy.md`,
  `docs/contracts/059-operation-scoped-process-watchers.md`;
  `docs/research/259-process-containment-backend-evidence.md`
- **Research output:** `docs/research/260-docker-engine-containment-composition-evidence.md`
- **Lane log:** `docs/logs/2026-08-29-g05-003-docker-engine-containment-evidence.md`
- **Observed host candidate:** Docker Desktop `4.87.0`, Engine `29.7.2`, API
  `1.55` (minimum `1.40`), containerd `2.2.5`, runc `1.3.6`, LinuxKit kernel
  `7.0.12`, client and server `darwin/arm64` to `linux/arm64`
- **Pre-existing probe image:** local image id
  `sha256:2edbbc5dc405e9612ba3584ce95480277e3eb374407b5505fe26f17df77c7dbc`
  (`ubuntu:22.04` at planning time); verify it still exists before use and stop
  rather than substituting or pulling when absent
- **Model capability profile:** frontier research/review model, high reasoning;
  containment, process authority, and security semantics require it
- **Tool/runtime restrictions:** no subagents, provider prompts, image pull or
  build, registry login, install/update, network fetch from a probe container,
  Docker configuration mutation, production code, workflow edits, card 010,
  release work, or merge
- **Inherited doctor baseline:** 385 god-file findings: 338 warnings and 47
  errors; stale graph; one generated-in-src warning. Do not add a finding.
- **Required validation:** `effigy validate:focused swallowtail-host-local`
  when a fixture/test changes; `effigy qa:docs`; `effigy qa:northstar`;
  `git diff --check`; exact live-probe commands and sanitized results in
  Research 260
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting Research 260 and exact evidence
- **Merge authorisation:** not authorized

## Boundaries

- **In scope:** official primary-source freeze, prompt-free host identity,
  bounded Docker Engine lifecycle probes, existing seam mapping, Research 260,
  a small sanitized identity fixture/test when useful, card/log closeout, and
  the affected g05/research/log/front-door indexes.
- **Out of scope:** production Docker dependencies or code, a new host package,
  public API, broad version support, card 010 or 011, Claude MCP/skill/hook
  wiring, generic shell authority, arbitrary images or paths, CI workflow
  changes, release work, or merge.
- **Outcome shape:** diagnostics-only research, explicitly requested by card
  014. Return one exact implementable composition or an honest empty set. Do
  not turn a positive result into implementation inside this PR.
- Use official Docker/OCI/kernel sources. Freeze exact retrieved bodies, dates,
  redirects, and SHA-256 digests. Separate API documentation from live engine
  behavior and local behavior from any compatibility claim.
- Probe only the pre-existing image id named above. Do not pull, build, push,
  tag, delete, or mutate images or volumes.
- Give every temporary container a unique per-run name and label under
  `com.swallowtail.research=260`. Record each returned container id before
  starting it. Never infer ownership from the shared label alone.
- Before mutation, list the exact containers the worker already owns; initially
  that set must be empty. Create, inspect, start, wait, stop, kill, and remove
  only ids returned by this run. Never run prune, broad label deletion, or
  cleanup against an unresolved name, glob, or ambient container.
- Cleanup owned temporary containers on success and failure. Do not remove an
  unrelated pre-existing container. If exact ownership cannot be established,
  stop and report instead of deleting.
- Workloads receive no Docker/engine socket, privileged mode, host PID or
  cgroup namespace, host device, restart policy, or engine credential. Probe
  networking stays disabled. Any bind mount is a fresh task-owned directory
  used only for bounded heartbeat evidence.
- Host PIDs may be observed privately to test descendant liveness but are never
  signalled, stored as watcher identity, or used for cleanup. Container id and
  engine control are the candidate lease authority.
- A project-specific immutable image and host-approved mount/command map may be
  necessary for usefulness. Record that cost honestly. Do not pretend the OCI
  composition preserves the arbitrary host toolchain.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  checkout or discard another checkout's state. Do not merge the PR.

## Important Context

- Research 259 already rejects default macOS process groups, `launchd`,
  process-table observation, and output pipes. Do not repeat that decision or
  weaken Contract 059.
- OCI specifies create/start/kill/delete lifecycle, but Docker Engine behavior
  is the exact control plane under test. “OCI compliant” alone is not an
  accepted result.
- The host candidate is available now, so the evidence lane needs no install,
  account, provider prompt, or image download.
- The decisive adversarial case is a descendant that calls `setsid`, closes
  stdout/stderr, and remains active while the root exits or stop races new
  forks. The proof must observe liveness independently of inherited pipes.
- Container `wait` alone may be insufficient. Research must determine whether
  stopped-state inspection plus successful non-force removal gives the durable
  empty-scope and supervisor-join truth Contract 059 needs.
- The existing `ProcessContainmentBackend` seam receives bounded
  `ProcessRequest` references. Research must show how a host-private image,
  mount, user, network, environment, and command approval map can resolve those
  references without adding Docker concepts to portable records.
- A positive Research 260 only allows the orchestrator to compile a separate
  implementation/conformance card. Cards 010-011 stay gated until that code
  lands and is reviewed.
- **Report after:** official source and exact host identity freeze; then the
  normal/escape/fork/partial-failure probe tranche; then PR-ready closeout.
- **Report to:** the operator, who relays progress and the PR to the
  orchestrator.

## Suggested Next Move

Complete the worker preflight first. Then read card 014, Contract 059, Research
259, and the current containment seam. Confirm the exact cached image id and
record a read-only inventory of running containers without changing them.

Design the probe and cleanup ledger before creating anything: one unique run
token, explicit returned container ids, a fresh heartbeat directory, no
network, and no ambient deletion. Freeze the official lifecycle sources, then
run one coherent normal/closed-pipe/fork/partial-start evidence tranche. If
the engine cannot prove destroyed scope without numeric PID control, stop and
return an honest empty Research 260.

## Completion Protocol

### Before you start

1. Read this handoff. Its worker metadata activates implementation mode. Before
   broad reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a clean, registered, dedicated non-`main` worktree,
   accept it as launcher-provided. Record its actual root/branch and do not
   create another because names differ from the placeholders.
3. If the launcher supplied a dirty or `main` worktree, stop and report it.
   Only for another unusable context, inspect the named worktree and then
   `.agents.local.env`; require `AGENTS_WORKTREE_CONTAINER_DIR` before a unique
   fallback. Never clean/reset another checkout or use `/tmp`.
4. In the selected worktree, fetch origin. Confirm `HEAD == origin/main`,
   confirm `git merge-base --is-ancestor 0d1ef473b54f3160ad68c0c70bf1c40917a4e396 HEAD`,
   and load the tracked handoff with
   `git show HEAD:docs/handoffs/20260829-175456-g05-003-docker-containment-evidence.md`.
   If the absolute file differs, stop. The tracked copy is canonical.
5. Required sibling links are `none`.
6. Read `AGENTS.md`, card 014, g05.003, Contracts 009/010/023/059, Research
   259, product guardrails, and the process-containment decision log.
7. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record but do
   not execute the broad test plan.
8. Reconfirm prompt-free Docker identity and the exact cached image id. If the
   daemon, version segment, architecture, or image differs from this handoff,
   stop and report rather than silently widening or substituting the proof.

### While you work

- Execute card 014 only. Keep a private in-run ledger of every created
  container id and fresh temporary heartbeat directory.
- Retrieve official sources into a disposable task directory, hash the exact
  bodies, and record only safe source metadata and sanitized probe results.
- Run normal exit, closed-pipe `setsid`, concurrent-fork stop, force-stop,
  partial-create/start, wait, inspect, and remove probes. Use engine/container
  operations only; never signal a host PID.
- After each probe, prove the task-owned container is stopped and removed, the
  heartbeat cannot advance, and the ownership ledger is empty. Preserve
  enough failure evidence to explain an honest stop.
- Do not place raw host paths, container ids, PIDs, environment, or engine
  endpoint data in public records. Stable image and version digests are
  evidence, not process authority.
- Report meaningful chunks through the operator. Stop on a contract gap,
  leaked control authority, ambiguous cleanup identity, unexpected runtime
  mutation, or evidence that changes the plan.

### When the assigned runway is complete

1. Run every required validation command named above.
2. Write Research 260 and the lane log. Mark card 014 complete only for a
   complete positive or honest-empty result; record every unmet criterion and
   stop condition.
3. Reconcile the g05.003 milestone, batch-card index, generation index,
   research/log indexes, triage disposition, and the sole roadmap Next Task.
   A positive result returns to the orchestrator for a separate implementation
   card. An empty result keeps cards 010-011 gated.
4. Confirm every task-owned temporary container is removed and the temporary
   heartbeat directory contains no live writer before closeout. Do not delete
   images, volumes, or unrelated containers.
5. Push the selected worker branch and open one reviewable PR against current
   pushed `main`. Link card 014, Research 260, exact host/image identity,
   frozen source digests, live-probe matrix, validation, and unresolved costs.
6. Report the PR URL and exact head to the operator. Do not merge or start an
   implementation card.

### Review and merge path

The orchestrator will review the exact PR head against Contract 059, Research
259, card 014, official source identity, the live-probe cleanup ledger,
sanitized evidence, changed files, and hosted checks. Same-identity review may
use a canonical PR comment. Requested changes are `none` at dispatch. The
operator must explicitly authorize any merge.

- **Closeout refs:** card 014; g05.003 milestone; Research 260; Docker
  containment evidence log; g05 and batch-card indexes; generation index;
  roadmap front door; containment target triage note

### Handoff closeout

Leave cards 010-011 gated and return to the orchestrator after the Research
260 PR is reviewable. If the exact engine composition cannot prove empty scope
or requires wider authority than card 014 allows, record the honest empty
result and stop rather than weakening Contract 059.
