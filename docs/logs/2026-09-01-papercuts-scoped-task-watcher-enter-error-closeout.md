# Papercuts scoped-task watcher EnterError closeout

Date: 2026-09-01
Handoff: `docs/handoffs/20260901-124337-papercuts-scoped-task-watcher-enter-error.md`
PR: pending

## Outcome

- Closed `Local watcher host methods cannot run inside a scoped-task executor`
  in `PAPERCUTS.md`.
- `LocalWatcherHostService` start/stop/join helpers no longer call
  `futures_executor::block_on` on the caller thread. They drive process and
  joined-task futures through `drive_future` on a joined scoped thread, so a
  watcher host method invoked from work polled by `LocalScopedTaskService`
  returns a normal `RuntimeFailure` or snapshot instead of panicking with
  `EnterError`.
- Public watcher lifecycle, cleanup-once join, and outside-scoped-task call
  paths stay the same. No public runtime/core API or contract change.

## Validation

- `cargo test -p swallowtail-host-local --test watcher_service scoped_task::`
- Pre-repair counterexample: restoring caller-thread `block_on` inside
  `drive_future` failed with `EnterError` /
  `swallowtail.local_task.panicked`; repair restored and the proof re-passed.
- `effigy validate:focused swallowtail-host-local`
- `effigy package:verify-affected swallowtail-host-local`
- `effigy --json scan god-files` held at the pre-change 387-finding baseline
- `git diff --check`

## Scope and next

- Next open Swallowtail papercut after this one:
  `Host-local watcher registry widens the god-file warning baseline`.
- Roadmap `Next Task` unchanged; no provider contact.
