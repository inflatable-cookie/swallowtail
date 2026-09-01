---
title: Gemini Live god-file papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-223500-papercuts-gemini-live-god-files.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, rust]
---

## Objective

Close the remaining Gemini Live feature-proof god-file papercut by splitting
the oversized context-compression and output-maximum test roots into focused
private modules without changing behavior, assertions, target identity, public
API, or test coverage.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `de7fc5c42a0f4b122e6afe202714000c8574123c`
- **Worker branch:** `worker/papercuts-gemini-live-god-files`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, carrying the
  capitalized `Papercuts` workspace label.
- **Worker class:** mechanical. This is a source-preserving test-module split,
  not frontier implementation work.
- **Authority:** `AGENTS.md`, the exact `PAPERCUTS.md` entry, the repository
  Rust-quality profile, current Gemini test targets, the current Effigy scan,
  and this handoff.
- **Current reproduction:** the scan is 381 findings: 7 critical, 42 high, 332
  warning. `tests/live_context_compression.rs` is a 267-code-line warning and
  `tests/live_output_maximum.rs` is a 257-code-line warning. The entry's older
  `live_protocol/tests.rs` path no longer exists and is not a current finding.
- **In scope:** those two test roots and focused private modules beneath them;
  the exact `PAPERCUTS.md` closeout; one concise closeout log and logs index if
  useful.
- **Out of scope:** production source, manifests, test-target names, public API,
  fixtures, semantics, other Gemini findings, version claims, currentness,
  roadmap/front-door surfaces, and other papercuts.
- **Parallel partition:** PR 172 owns only the version-currentness skill files.
  This lane owns the Gemini tests plus PAPERCUTS/log closeout surfaces.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Review Oracle

Move existing proof bodies into the smallest focused private modules. Preserve
every current test name, discovery path where practical, assertion, fixture,
feature gate, ignored/live posture, and negative case. Do not run a live probe
or contact a provider.

Both named current warnings must disappear. No new file may enter the configured
god-file findings. If no unrelated mainline movement changes the baseline, the
total must improve from 381 to 379 with 7 critical, 42 high, 330 warning. If the
baseline moves before closeout, reconcile the exact current-main baseline and
prove a two-finding improvement. Treat the missing historical
`live_protocol/tests.rs` path as stale papercut evidence and record that
correction; do not recreate it.

Falsify the split by listing and running both test targets, comparing moved
proof bodies against the base, and showing that removal of a moved module or
assertion changes discovery or fails the relevant target. Stop if a behavior,
API, manifest, production, or provider-contact change is needed.

## Validation And Completion

Confirm a clean non-`main` worktree, exact branch, `HEAD == origin/main`, and
this handoff from `HEAD` before editing. Read the repository Effigy skill and
strict everyday Rust-authoring instructions selected by `AGENTS.md`.

Run one coherent validation round after the split:

- `cargo fmt -p swallowtail-adapter-gemini --check`
- the exact Cargo test targets containing both roots
- `effigy validate:focused swallowtail-adapter-gemini`
- `effigy package:verify-affected swallowtail-adapter-gemini`
- `effigy --json scan god-files`
- `effigy qa:docs`
- `git diff --check`

Close the exact papercut with measured before/after evidence and the stale-path
correction. Commit, push, and open one PR. Report exact head/base, moved proof
grouping, test counts, god-file result, validation, changed files, and PR URL.
Do not merge.

