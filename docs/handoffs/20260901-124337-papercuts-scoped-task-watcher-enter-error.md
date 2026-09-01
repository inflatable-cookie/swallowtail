---
title: Scoped-task watcher host EnterError papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-124337-papercuts-scoped-task-watcher-enter-error.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, watcher]
---

## Objective

Resolve the open papercut where a watcher host method invoked from a
`LocalScopedTaskService` task can nest `futures_executor::block_on` and panic
with `EnterError`. Reproduce the failure without provider contact, implement
the smallest executor-neutral repair, and make the counterexample load-bearing.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base:** `f56b54d6bc829975141088c4afa5ab2f5ab0454b`
- **Pushed-main check:** local `main` and `origin/main` matched at that commit
  before this handoff was compiled.
- **Worker branch:** `worker/papercuts-scoped-task-watcher-enter-error`
- **Worker worktree:** Paseo-managed worktree branched from pushed
  `origin/main`, titled with the capitalized `Papercuts` workspace label.
- **Authority:** the matching open `PAPERCUTS.md` entry, existing watcher host
  contracts and architecture, current code, and this handoff.
- **Worker class:** day-to-day. The repair is bounded to one host-local
  executor interaction; it does not warrant a frontier implementation worker.
- **Ready-frontier shape:** independent implementation lane. It may run beside
  version-currentness because their mutable surfaces do not overlap.
- **Serial edge:** no other papercut may edit `PAPERCUTS.md` or
  `docs/logs/README.md` until this lane reaches merge or stop.
- **Review posture:** material concurrency/lifecycle risk; orchestrator exact-
  head review remains required before merge.

## Scope

In scope:

- reproduce the nested-executor panic through the public/local host composition
  without provider contact;
- make watcher host start/stop/join invocation safe when called from a scoped
  task, or prove that the correct bounded listener ownership is a joined
  standard thread and encode that invariant;
- preserve watcher cleanup, join, wakeup, feed, capacity, and turn-retirement
  semantics;
- add the smallest focused regression proof;
- close only the matching papercut when the defect is fixed and falsified;
- add one bounded closeout log and log-index entry when warranted by existing
  papercut practice.

Out of scope:

- Claude CLI isolation or the proposed Agent SDK sidecar;
- watcher route live-readiness, provider contact, prompts, authentication, or
  live probes;
- Contracts 059-060 changes, public runtime/core API changes, or a new executor;
- Kimi projection work, skill discovery, currentness research, or other
  papercuts;
- roadmap front-door edits.

## Review Oracle

The accepting proof must run the exact watcher host operation from work polled
by `LocalScopedTaskService` and observe a normal result, never an executor-entry
panic. It must fail on current `main` or on a locally restored pre-repair shape.

Also prove:

- the scoped task remains joined and its completion stays observable;
- watcher process start/stop/join cleanup still occurs exactly once;
- no detached thread or future survives turn finalization;
- errors remain `RuntimeFailure` values rather than panics;
- ordinary calls outside a scoped task retain their existing behavior.

Do not accept a test that merely catches or suppresses `EnterError`, replaces a
real watcher operation with a no-op, or routes only the test around the shared
host composition.

## Boundaries And Stop Conditions

Prefer an internal executor-neutral implementation when it preserves the
existing public lifecycle. A joined standard thread is acceptable only when
ownership, wakeup, error propagation, and bounded cleanup remain explicit.

Stop and report evidence if the complete repair requires a public runtime/core
API change, a new contract decision, detached work, changed watcher lifecycle
semantics, or a provider/live probe. Do not choose between such architecture
options inside this papercut.

## Validation

After one coherent implementation batch, run:

- the exact new `swallowtail-host-local` regression test;
- `effigy validate:focused swallowtail-host-local`
- `effigy package:verify-affected swallowtail-host-local`
- warranted docs checks for the papercut/log edit;
- `git diff --check`

Record the existing god-file baseline before the change and do not widen it.
No provider command or live probe is authorized.

## Completion Protocol

Before broad reads, confirm a clean registered non-`main` worktree, exact
branch, `HEAD == origin/main`, and that this handoff is loaded from `HEAD`.
Read `AGENTS.md`, the open papercut, watcher host/runtime contracts and
architecture, `LocalScopedTaskService`, `LocalWatcherHostService`, bridge
ownership, and existing watcher race/wakeup tests.

Implement one bounded fix, falsify the regression against the pre-repair
shape, restore the repair, run the named validation, commit, push, and open one
PR against current pushed `main`. Report exact head/base, diagnosis, changed
files, falsification, lifecycle residuals, god-file result, validation, and PR
URL. Do not merge.
