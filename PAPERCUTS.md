# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Timing-sensitive deadline fixtures make unrelated PR heads red — 2026-08-21
- Friction: restacked PR 23 failed Stable in Ollama
  `deadline_remains_distinct_from_cancellation`, while PR 21 failed MSRV in
  Codex `callback_wait_ends_when_the_host_deadline_is_observed`; neither PR
  changes the failing adapter surface.
- Impact: otherwise mergeable stacked heads need unrelated CI reruns before
  the orchestrator can satisfy the exact-head green gate.
- Fix: make the fake-time/fixture-message handoff deterministic in both tests,
  or give the asserted transition an explicit synchronization boundary.
- Surface: Ollama attached-driver deadline test; Codex app-server callback
  deadline test; Stable and MSRV CI jobs.

### [ ] rustfmt --edition 2021 cannot parse this 2024 workspace — 2026-08-20
- Friction: `rustfmt --edition 2021 <file>` fails on let-chains in sibling
  modules (`preflight/validation.rs`, `provider_session_history/page.rs`)
  even when those files are not the format target.
- Impact: file-scoped rustfmt with the wrong edition aborts instead of
  formatting the requested sources.
- Fix: use `cargo fmt -p <crate>` or `rustfmt --edition 2024`.
- Surface: local rustfmt invocation vs workspace edition 2024.

### [ ] DeepSeek stream-cancellation test flakes as ProviderFailed — 2026-08-19
- Friction: `swallowtail-adapter-deepseek::driver::active_stream_cancellation_joins_before_session_credential_release`
  expected `Cancelled` and observed `ProviderFailed` with
  `swallowtail.deepseek.stream_incomplete` / `TransportInterrupted` on tag CI
  run `32309276223` attempt 1. The same SHA had already passed PR and
  dispatched CI; the in-place rerun passed.
- Impact: tag-triggered CI can fail a green SHA without a product change,
  tempting a retag.
- Fix: make the cancellation join deterministic, or accept
  `TransportInterrupted` as a documented cancellation race at the stream
  boundary.
- Surface: DeepSeek driver cancellation test; tag CI Stable job.

## Closed

### [x] Stable clippy `result_large_err` on ACP start_session Err pairs — 2026-08-20
- Friction: CI Stable clippy 1.98.0 failed `result_large_err` on
  `Result<Handle, (RuntimeFailure, ResourceLease)>` in Cline, Goose,
  Copilot CLI, Gemini, Kiro, and Deep Agents, then
  `chunks_exact_to_as_chunks` in ACP lifecycle fixtures.
- Impact: any PR could go red on Stable without a product change.
- Fix: boxed the Err pairs; replaced `chunks_exact(2)` with
  `as_chunks::<2>().0`. PR 14, merge SHA `47b94efc`.
- Surface: ACP `start_session` helpers; ACP lifecycle fixtures; Stable
  Clippy (all features).

### [x] Roadmap docs policy applies parent checks to excluded child indexes — 2026-08-11
- Friction: `effigy qa:docs` reports existing `g01/README.md`, `g02/README.md`,
  `g03/README.md`, and `backlog/README.md` links as missing even though they
  exist under `docs/roadmaps/`.
- Impact: the broad docs selector cannot certify an otherwise indexable docs
  change; its next-action gate also requires `## Next Task` in every generation
  and backlog index, contrary to this repo's single front-door pointer rule.
- Fix: Effigy commit `53a4971da31344c0f1f3bb24308e78ee2e85ec3c`
  applies index exclusions to collected links for both index and next-action
  checks. Swallowtail's unchanged roadmap policy and all release gates pass.
- Surface: Effigy roadmap index and next-action docs policies.

### [x] Release prepare omits coordinated workspace dependency versions — 2026-08-08
- Friction: Effigy updated `workspace.package.version` before gates but left
  versioned path entries under `workspace.dependencies` at the previous release.
- Impact: the first Cargo-backed gate could not resolve the newly versioned local
  packages, so an otherwise valid coordinated release preparation failed.
- Fix: Effigy v0.11 release planning now updates exact-version path dependencies
  for workspace members inheriting `workspace.package.version`. The focused
  fixture covers plan previews, guarded lock sync, exclusions, and rollback.
- Surface: Effigy Cargo-workspace release preparation.

### [x] Bootstrap papercuts before an exact-SHA release lane — 2026-08-06
- Friction: Northstar first required this file after the release candidate had already passed exact-commit CI.
- Impact: Adding the repository hygiene file during tag closeout would invalidate the clean-tree release check or move the tag beyond the green SHA.
- Fix: Northstar adopt/upgrade and release-posture guidance now seed
  `PAPERCUTS.md` before exact-SHA / clean-tree release prep (skill repo-contract,
  normalize-docs, bundle-docs/papercuts.md, template-bundle).
- Surface: Northstar adoption and tagged-release preparation.
