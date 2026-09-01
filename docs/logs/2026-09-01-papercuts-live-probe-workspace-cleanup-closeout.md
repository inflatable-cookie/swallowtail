# Papercuts live-probe temporary-workspace cleanup closeout

Date: 2026-09-01
Handoff: `docs/handoffs/20260901-100306-papercuts-live-probe-cleanup.md`
PR: pending

## Outcome

- Verified current `main` already owns the ignored Claude watcher live-probe
  workspace with `TempWorkspace` Drop before `prepare_claude_code_headless` /
  `start_run` and before fallible assertions that hold the directory.
- Credential-free
  `temporary_workspace_cleanup_is_established_before_assertions` passes; omitting
  Drop leaves the directory after the caught panic and fails the proof.
- Closed the matching `PAPERCUTS.md` entry as stale bookkeeping after g05.006
  card 019 (PR 126 at `c8691e84`). No runtime, probe, or contract change.

## Validation

- `cargo test -p swallowtail-adapter-claude-agent --test claude_code_structured_run temporary_workspace_cleanup_is_established_before_assertions -- --exact`
- Drop-omission counterexample failed with `workspace survived an assertion panic`;
  Drop restored and the proof re-passed.
- `cargo test -p swallowtail-adapter-claude-agent --features live-probes --test live_watcher_probe --no-run`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `git diff --check`

## Scope and next

- Next open Swallowtail papercut after this one:
  `Local watcher host methods cannot run inside a scoped-task executor`.
- Card 032 closeout remains queued on shared log/front-door surfaces.
- No provider contact; live selector not run.
