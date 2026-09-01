# Papercuts host-local watcher god-file closeout

Date: 2026-09-01
Handoff: `docs/handoffs/20260901-125509-papercuts-host-local-watcher-god-files.md`
Base: `19a7c786034dfb7c4cc8ca08eef74b51d34d048e`
Worker: `worker/papercuts-host-local-watcher-god-files`

## Outcome

- Closed `Host-local watcher registry widens the god-file warning baseline` in
  `PAPERCUTS.md`.
- Split `watcher/accept.rs` into acceptance/rollback and private lookup/wait
  modules. Split `process.rs` into the process-service adapter, launch
  construction, and validation modules.
- Moved function bodies and branches without changing diagnostic codes or
  messages, ordering, limits, cfg gates, ownership cleanup, process-group
  handling, watcher registry behavior, or public API.
- Re-measured the four paths named by the papercut: `watcher/accept.rs` (288
  code lines) and `process.rs` (284) were the only findings; `watcher.rs` and
  `tests/watcher_service/policy.rs` were already below threshold and remained
  behaviorally untouched.
- God-file findings fell from 387 (7 critical / 42 high / 338 warning) to 385
  (7 critical / 42 high / 336 warning). The remaining host-local findings are
  pre-existing files outside this bounded lane.

## Structural proof

- The 15 moved function bodies compare byte-for-byte with their `HEAD` bodies;
  only module placement and required private sibling visibility changed.
- A clean `HEAD` snapshot reproduced both original warnings and all 387
  findings. Its focused host-local suite passed 111/111 tests; the split tree
  passed the same 111/111 tests.
- New `process/launch.rs`, `process/validation.rs`, and `watcher/lookup.rs`
  stay below the 250-code-line warning threshold.

## Validation

- `cargo fmt --check`
- `effigy validate:focused swallowtail-host-local`
- `effigy package:verify-affected swallowtail-host-local`
- `effigy package:api swallowtail-host-local`
- `effigy --json scan god-files`
- `git diff --check`

All accepting checks passed. No provider command, live probe, install,
authentication, or broad workspace QA was run.

## Scope and next

- No public or semantic API change; no tests were weakened or changed.
- No roadmap, contract, architecture, research, feature-matrix, or route
  changes.
- Next open Swallowtail papercut remains the next unchecked entry in
  `PAPERCUTS.md`; this lane does not select another papercut.
