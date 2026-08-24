# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Pi replay-during-resume fixture can hang MSRV CI — 2026-08-24
- Friction: PR 54's first pinned-MSRV run stalled for six hours in
  `resume_fails_closed_on_replay_evidence`; one local exact-head run also
  stalled, while immediate reruns passed on Rust 1.95.0 and Stable.
- Impact: an unrelated Qoder documentation head can remain non-green until the
  hosted timeout or a successful rerun.
- Fix: make the replay-event/`session_switch` response ordering deterministic
  in the Pi sidecar fixture and bound the failure proof so it cannot wait
  indefinitely.
- Surface: Pi SDK sidecar `ReplayDuringResume` fixture; pinned-MSRV CI.

### [ ] Gemini Live feature proofs widen the god-file warning baseline — 2026-08-23
- Friction: the context-compression batch left `live_protocol/tests.rs`,
  `live_context_compression.rs`, and the earlier `live_output_maximum.rs` above
  the warning threshold, raising doctor findings from 371 to 374.
- Impact: later feature lanes inherit noisier structural-health evidence even
  though the error-level baseline is unchanged.
- Fix: split protocol encoding, context-compression, and output-maximum proofs
  into focused test modules without reducing route-local coverage.
- Surface: `swallowtail-adapter-gemini` Live protocol and acceptance tests.

### [ ] evidence-download cwd steals later repo commands — 2026-08-22
- Friction: a disposable evidence directory became the persistent shell cwd, so later `effigy` and `git diff --check` ran outside the worktree.
- Impact: card-gate commands fail with missing-catalog or "not a git repository" errors after an otherwise successful evidence fetch.
- Fix: `cd` back to the worktree after temp-dir evidence work, or run later repo commands with an explicit working directory.
- Surface: agent shell sessions that download provider evidence outside the worktree.

### [ ] zsh special variables break ordinary shell snippets — 2026-08-22
- Friction: authority-read snippets used `path` and `status` as ordinary
  variables; zsh hid the executable search path for the former and rejected
  assignment to the read-only latter.
- Impact: otherwise read-only agent scripts can fail partway through a batch
  with a misleading `command not found` error.
- Fix: document common reserved names or lint generated zsh snippets for
  assignments to zsh special parameters.
- Surface: agent-authored zsh orchestration commands.

### [ ] OpenCode cancellation fixture panics on expected broken pipe — 2026-08-22
- Friction: PR 35 MSRV failed in
  `post_dispatch_cancellation_is_joined_and_unconfirmed` because the fixture
  response writer treated the cancelled client's `BrokenPipe` as fatal and the
  fixture server then panicked while joining.
- Impact: unrelated exact heads can fail MSRV after the product cancellation
  path has already produced the expected disconnect.
- Fix: let the cancellation fixture accept `BrokenPipe`/connection reset while
  writing the abandoned response, while preserving failures for other write
  errors.
- Surface: OpenCode prepared-facade HTTP fixture response writer; MSRV CI.

### [ ] Cursor model-parameter proof exceeds the god-file threshold — 2026-08-22
- Friction: PR 34 expanded Cursor `tests/prepared_suite.rs` to 454 lines,
  raising the doctor god-file baseline from 41 to 42 errors.
- Impact: the completed feature lane leaves structural health worse and makes
  later doctor comparisons noisier.
- Fix: split model-parameter preparation and rejection proofs into a focused
  test module without reducing coverage.
- Surface: `crates/swallowtail-adapter-cursor/tests/prepared_suite.rs`.

### [ ] Kimi lifecycle proof exceeds the god-file threshold — 2026-08-21
- Friction: PR 31 added a 566-line Kimi Platform lifecycle integration test,
  raising the doctor god-file baseline from 40 to 41 errors.
- Impact: the completed route lane leaves repository structure health worse and
  makes later doctor comparisons noisier.
- Fix: split admission/preparation and refresh/catalogue/047 proofs into focused
  test modules without changing coverage.
- Surface: `crates/swallowtail-adapter-kimi-platform/tests/connection_lifecycle.rs`.

### [ ] Parallel currentness branches allocate duplicate roadmap cards — 2026-08-21
- Friction: PRs 24-30 allocated cards 076-085 from older planning bases while
  pushed `main` already assigns 076-078 to g04.024.
- Impact: independently correct family branches conflict at integration and
  cannot preserve both the active generation runway and unique card identity.
- Fix: allocate currentness roadmap and card numbers from current pushed
  `main`, or defer final numbering to the orchestrator restack.
- Surface: version-currentness worker handoffs; g04 roadmap and batch-card
  indexes.

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

### [x] Qwen budget proof raises the god-file error baseline — 2026-08-23
- Friction: PR 50 added a 441-line `prepared_facade/budgets.rs`, raising doctor
  findings from 376 to 377 and error-level findings from 46 to 47 while the
  worker closeout reported the inherited baseline unchanged.
- Impact: the feature proof left structural health worse and recorded false
  validation evidence for later review and closeout.
- Fix: split run/version/terminal and session/replacement budget proofs into
  focused modules; the reviewed head restored 376 findings (330 warning / 46
  error) and fast-forwarded through PR 50 at `9807e322`.
- Surface: `swallowtail-adapter-qwen` prepared-facade budget acceptance tests.

### [x] Worker-local environment file dirties the planning checkout — 2026-08-21
- Friction: `.agents.local.env` is an intended machine-local worktree-path
  surface but appeared as an untracked file on orchestrator `main`.
- Impact: strict Northstar planning could not publish from a clean verified
  base without touching or hiding operator-local configuration.
- Fix: ignored the root-local file explicitly while keeping its contents local.
- Surface: `.gitignore`; Northstar orchestrator planning checkout.

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
