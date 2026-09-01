# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Effigy graph explore can rebuild silently without a useful timeout — 2026-08-31
- Friction: `effigy graph explore ... --json` produced no envelope or progress
  while rebuilding for more than 60 seconds and required an interrupt.
- Impact: a bounded code-navigation query can stall an agent turn without a
  result or a clear fallback point.
- Fix: emit rebuild progress and enforce a bounded query/index timeout with a
  structured fallback diagnostic.
- Surface: Effigy graph indexing and `graph explore --json` in Swallowtail.
- Progress 2026-09-01: Swallowtail ownership stop. Host
  `effigy v0.12.1+local.47458a1` already bounds graph *queries* with
  `EFFIGY_GRAPH_TIMEOUT_MS` (default 120000) and returns
  `effigy.graph.timeout.v1` health/`next` diagnostics. Rebuild remains silent
  for the full budget (0 stdout/stderr bytes until timeout). No Swallowtail
  `effigy.toml`, task, script, or wrapper can emit in-command rebuild progress
  or change that built-in behavior without patching Effigy; leave open for an
  Effigy product lane.

### [x] Watcher proof repair restored the 390 god-file baseline — 2026-08-30
- Friction: the first PR 126 revision raised doctor from 390 findings
  (341 warnings / 49 errors) on `main` to 395 (346 / 49). New warnings were
  `claude_code_activity.rs`, both `watcher_proof.rs` files, `watcher/feed.rs`,
  and `watcher_bridge/mod.rs`.
- Impact: the closeout and PAPERCUTS entry understated the increase and called
  several new warnings inherited.
- Fix: split pump lifecycle, activity system hooks, feed buffer tests, bridge
  close, fake process handle, and Claude watcher fixtures. Measured baseline
  on this head is 390 (341 / 49), matching `main`.
- Surface: g05.006 card 019 PR 126 review revision.

### [x] Route-matrix docs validation leaves Python bytecode — 2026-08-30
- Friction: `effigy qa:docs` imports the route inventory checker and leaves
  `scripts/provider_route_matrix/__pycache__/` untracked.
- Impact: credential-free docs validation dirties a clean planning checkout
  immediately before its commit and push gate.
- Fix: set `sys.dont_write_bytecode = True` in the consumer-docs and guides
  checkers before the route-inventory import, and export
  `PYTHONDONTWRITEBYTECODE=1` from the routes shell wrapper so every affected
  selector prevents source-tree bytecode without relying on host
  `pycache_prefix` redirection or ignore rules.
- Surface: `qa:consumer-docs`, `qa:guides`, `qa:routes`, and route-inventory
  imports.
- Closed: 2026-08-31 papercuts wave 23 route-matrix bytecode.

### [x] Live-probe assertions bypass temporary-workspace cleanup — 2026-08-30
- Friction: the card 011 Claude watcher probe removes its temporary workspace
  only after success assertions. Its expected live evidence failure panicked
  first and left one empty workspace behind.
- Impact: failed opt-in probes can retain temporary state and contradict their
  own cleanup claims even when bridge-private material was released.
- Fix: g05.006 card 019 (PR 126 at `c8691e84`) owns the live-probe workspace
  with `TempWorkspace` Drop before provider contact and before fallible
  assertions; credential-free
  `temporary_workspace_cleanup_is_established_before_assertions` proves a
  caught assertion panic leaves no directory.
- Surface: ignored provider live probes with temporary working resources;
  prototype head `49f2692f`.
- Closed: 2026-09-01 live-probe temporary-workspace cleanup reconciliation.

### [x] Local watcher host methods cannot run inside a scoped-task executor — 2026-08-30
- Friction: `LocalScopedTaskService` polls work with `futures_executor::block_on`.
  `LocalWatcherHostService` also calls `block_on` inside method invocation for
  process start/stop/join. A bridge listener running on a scoped task panics
  with `EnterError`.
- Impact: operation-scoped HTTP listeners cannot reuse the scoped-task executor
  without changing the watcher host to true async.
- Fix: watcher host start/stop/join helpers drive process and task futures on a
  joined scoped thread via `drive_future`, so nested executor entry cannot
  panic; `watcher_host_methods_succeed_inside_a_scoped_task_executor` proves
  accept/stop/join from work polled by `LocalScopedTaskService`.
- Surface: `swallowtail-host-local` watcher host and watcher HTTP bridge.
- Closed: 2026-09-01 papercuts scoped-task watcher EnterError repair.

### [x] Host-local watcher registry widens the god-file warning baseline — 2026-08-29
- Friction: PR 117 added four warning-level files above the configured size
  threshold: `watcher/accept.rs`, `process.rs`, `watcher_service/policy.rs`, and
  `watcher.rs`. Effigy doctor rose from 381 findings (334 warnings / 47 errors)
  to 385 (338 warnings / 47 errors).
- Impact: later g05 lanes inherit noisier structural-health evidence even
  though the error-level baseline is unchanged.
- Fix: split watcher acceptance/lookup and local process construction/validation
  into focused private modules without reducing lifecycle coverage; reconcile
  the stale four-file claim before changing already-compliant files.
- Surface: `swallowtail-host-local` process and watcher registry implementation
  and policy tests; g05.003 card 009 closeout.
- Closed: 2026-09-01 papercuts host-local watcher god-file split. The live
  four-path re-measure found only `watcher/accept.rs` at 288 code lines and
  `process.rs` at 284 above threshold; `watcher.rs` and
  `watcher_service/policy.rs` were already below it. The split reduced the
  total from 387 findings (7 critical / 42 high / 338 warning) to 385
  (7 critical / 42 high / 336 warning).

### [x] xAI docs HTML is a Next.js SPA; `.md` is the digestable corpus — 2026-08-27
- Friction: `docs.x.ai/developers/...` HTML bodies are 0.4–1.1 MiB Next.js
  shells with `x-nextjs-cache`. Markdown exports exist by appending `.md` and
  are 0.8–47 KiB. `https://docs.x.ai/openapi.json` is a separate 219 KiB schema
  document.
- Impact: hashing the HTML shell does not identify the converted text used as
  evidence. Research 187 hashed HTML; Research 227 hashes `.md` and OpenAPI as
  the corpus and records HTML only as corroboration.
- Fix: retrieve xAI docs `.md` exports and OpenAPI JSON; hash those bodies;
  treat HTML as SPA corroboration only.
- Surface: g04.080 / Research 227 official-source retrieval.
- Closed: 2026-09-01 papercuts xAI docs corpus. Research 227 already records
  binding `.md`/OpenAPI digests with HTML as corroboration only. Research 187
  keeps its historical HTML table and now notes that Research 227 supersedes
  the retrieval method.

### [x] Cline Plan acceptance widens the god-file warning baseline — 2026-08-26
- Friction: PR 72 expands `tests/prepared_headless_facade.rs` to 395 code
  lines, raising doctor findings from 378 to 379 while the error count remains
  46.
- Impact: later lanes inherit noisier structural-health evidence, and the
  closeout does not name the committed warning increase separately from local
  evidence-directory noise.
- Fix: split default-mode and Plan-mode prepared-facade proofs into focused
  test modules without reducing coverage; record the 379 finding baseline.
- Surface: `swallowtail-adapter-cline` prepared headless acceptance tests;
  g04.073 closeout evidence.
- Closed: 2026-09-01 papercuts Cline prepared-headless god-file split. Moved
  default-run, Plan, and rejection/binding proofs plus shared fixture builders
  into `tests/prepared_headless_facade/{default_run,plan,rejections,support}.rs`
  with bodies intact under the existing `prepared_headless_facade` target.
  `effigy --json scan god-files` dropped from 385 findings (7 critical / 42
  high / 336 warning) to 384 (7 critical / 42 high / 335 warning); the 395-line
  root finding is gone and no new module entered the scan.

### [ ] Launcher cleanup leaves stale Git worktree registrations — 2026-08-26
- Friction: the PR 67 launcher worktree directory was removed after merge, but
  `git worktree list` still reported its branch and path as registered.
- Impact: follow-on review commands select a nonexistent working directory and
  fail before repository inspection begins.
- Fix: have launcher cleanup run safe worktree metadata pruning after removing
  its owned directory, or retain the directory until Git deregistration finishes.
- Surface: T3 launcher-owned Swallowtail review worktrees; Git worktree metadata.
- Progress 2026-09-01: Swallowtail ownership stop. Host Paseo `0.6.1` and
  published `deletePaseoWorktree` (`git worktree remove --force`, then
  directory delete, then `git worktree prune`) own launcher cleanup; T3 used
  `~/.t3/worktrees/…`. Swallowtail has no `git worktree add|remove|prune`
  surface. `paseo.json` teardown only runs Northstar `paseo:worktree unlink`
  (Effigy deps) and runs while the directory still exists, so it cannot
  deregister a later-deleted tree without a repo-wide prune. Leave open for
  the T3/Paseo launcher lane; do not close from Swallowtail.

### [x] GitHub Copilot CLI docs HTML is a Next.js SPA; `.md` is the digestable corpus — 2026-08-26
- Friction: `docs.github.com/en/copilot/...` HTML bodies are 0.6–1.6 MiB
  Next.js shells. Markdown exports exist by appending `.md` and are 11–348 KiB.
- Impact: hashing the HTML shell does not identify the converted text used as
  evidence. Research 188 hashed HTML; Research 218 hashes `.md` as the corpus
  and records HTML only as corroboration.
- Fix: retrieve GitHub docs `.md` exports for Copilot CLI reference pages;
  hash those bodies; treat HTML as SPA corroboration only.
- Surface: g04.071 / Research 218 official-source retrieval.
- Closed: 2026-09-01 papercuts Copilot docs corpus. Research 218 already
  records binding `.md` digests with HTML as corroboration only. Research 188
  keeps its historical HTML table and now notes that Research 218 supersedes
  the retrieval method.

### [x] Codex config docs HTML is a Learn SPA; `.md` is the digestable corpus — 2026-08-25
- Friction: `developers.openai.com/codex/config-*` 200-redirects to
  `learn.chatgpt.com` HTML shells (~0.4–1.2 MiB). Markdown exports exist by
  appending `.md` and are 11–91 KiB.
- Impact: hashing the HTML shell does not identify the converted text used as
  evidence. Current `main` `codex-rs/core/models.json` 404s; the tag stores
  models at `codex-rs/models-manager/models.json`.
- Fix: retrieve Learn `.md` exports for Codex config docs; hash those bodies;
  treat GitHub tag paths as binding and current-main URLs as leads only.
- Surface: g04.066 / Research 213 official-source retrieval.
- Closed: 2026-09-01 papercuts Codex docs corpus. Research 213 already hashed
  Learn `.md` as the digestable corpus. The 2026-09-01 reconciliation re-fetched
  all four config pages' Learn `.md`/HTML bodies, the `developers.openai.com`
  308 hops (now exact; not 200), and the tag/main models paths; preserved the
  2026-08-25 digests; recorded current URL/status/body-kind/byte/digest
  evidence; confirmed basics/advanced `.md` and tag `models.json` still match
  while reference/sample `.md` moved; left deliver-now claims unchanged.

### [x] Anthropic platform docs return cache-less SPA HTML — 2026-08-25
- Friction: `platform.claude.com/docs` HTTP bodies are 0.9–2.0 MiB Next.js
  shells with no `Last-Modified` or `ETag`. Several thinking URLs 307-redirect.
  The digestable corpus is converted page text, not the hashed HTML shell.
- Impact: Research 209 hashes are complete retrieved bodies, but they are
  noisy compared with DeepSeek's cache-validated docs and do not uniquely
  identify the converted text an agent actually read.
- Fix: prefer Anthropic's converted/markdown export when one exists; record
  both HTTP body and converted-text hashes; treat 307 targets as the corpus
  page rather than hashing the redirect hop twice.
- Surface: g04.062 / Research 209 official-source retrieval.
- Closed: 2026-09-01 papercuts Anthropic docs corpus. Research 209 records all
  9 source rows' followed `.md` bodies, retrieved from public official URLs at
  `2026-09-01T13:00:30Z`, with requested/effective URLs, statuses, body kinds,
  byte counts, and SHA-256 digests. The historical HTML hashes remain intact;
  `adaptive-thinking` is explicitly equivalent to
  `thinking-steering-and-cost`. The Markdown reconciliation preserves the
  existing `claude-opus-4-7` adaptive/omitted-display conclusion without any
  capability, fixture, or claim change.

### [ ] Antigravity invalid-`--agent` probes crossed card 161's no-prompt boundary — 2026-08-24
- Friction: card 161 requires promptless help/listing and forbids provider
  prompts. Two unauthorized `--print` / `--output-format json` probes still
  ran: nonexistent `--agent` and a whitespace-only follow-up. Both returned
  `status: SUCCESS` with usage. The session failed to stop after the first
  breach.
- Impact: Research 205 must record both runs as authority-boundary /
  `UnverifiedNewer` incidents only. They are out of scope for qualification
  and are not decisive fail-open proof for `1.1.9..=1.1.17`.
- Fix: treat the card's no-provider-prompt rule as required, not optional;
  refuse any `--print` probe unless a version-pinned pre-init failure is first
  proved from help/docs/fixtures or extracted binaries; stop immediately after
  one boundary breach.
- Surface: g04.058 / Research 205 invalid-`--agent` incidents; Antigravity
  headless worker method.
- Progress 2026-09-01: Swallowtail ownership stop. Card 161 already forbade
  provider prompts; Research 205 already records the two `--print` runs as
  authority-boundary / `UnverifiedNewer` incidents only (nonexistent
  `--agent swallowtail-nonexistent-agent-zzzz`, then whitespace-only
  `--agent`; both JSON `status: SUCCESS` with usage). Production
  `headless_command.rs` still omits `--agent`. No Swallowtail script, wrapper,
  or task owns host `agy` argv: `scripts/run-with-isolated-home.sh` forwards
  the incident shapes to a fake `agy` unchanged, and agent-direct PATH
  invocation has the same empty intercept. A repo wrapper would be fail-open
  against the method that actually ran. Leave open; do not close from
  Swallowtail and do not treat those runs as qualified fail-open proof.

### [ ] Host `agy` auto-updated from 1.1.9 to 1.1.19 mid-research — 2026-08-24
- Friction: PATH `agy` reported `1.1.9` with stdout help matching the frozen
  fixture, then later reported `1.1.19` with help on stderr during the same
  card 161 session.
- Impact: live observations can silently leave the qualified window;
  `UnverifiedNewer` noise mixes with baseline evidence.
- Fix: pin or record `agy --version` immediately before every live probe and
  refuse host probes once the binary drifts from the named qualified range;
  prefer extracted release artifacts for version-scoped help.
- Surface: Antigravity host PATH binary; Research 205 method notes.
- Progress 2026-09-01: Swallowtail ownership stop. No live-probe, script, or
  card mechanism owns host PATH `agy` argv. Production discovery records
  `--version` at prepare and then permits `UnverifiedNewer` under
  `AllowUnverified`, so `1.1.19` is not a qualified-range refusal.
  Catalogue/headless spawn `models` / `--print` without re-probing.
  `scripts/run-with-isolated-home.sh` forwards a fake that drifted
  `1.1.9` → `1.1.19` between `--version` and `--help`. Agent-direct PATH
  `--help` has the same empty intercept. Frozen `antigravity-cli-1.1.9`
  help remains the extracted specimen; nothing forces research sessions to
  use it. Leave open; do not close from Swallowtail. Research 205 stays
  historical.

### [x] llama.cpp context-size proofs widen the god-file warning baseline — 2026-08-24
- Friction: PR 55 expanded `prepared_facades.rs` and `owned_driver.rs` past the
  warning threshold, raising doctor findings from 376 to 378 while its worker
  closeout reported the inherited baseline unchanged.
- Impact: later lanes inherit noisier structural-health evidence and a stale
  validation record.
- Fix: split context-size prepared-facade and owned-driver proofs into focused
  test modules without reducing lifecycle coverage; correct the closeout
  baseline to 378 findings (332 warnings / 46 errors).
- Surface: `swallowtail-adapter-llama-cpp` context-size acceptance tests;
  g04.056 closeout evidence.
- Progress 2026-08-26 (g04.078): adding reasoning proofs pushed
  `owned_driver.rs` from warning to error, so its selection proofs moved to
  `tests/owned_driver/selections.rs`. That file is a warning again at 260 code
  lines and doctor returned to the inherited 380/334/46 baseline.
  `prepared_facades.rs` remains a warning at 381 code lines and still wants the
  same treatment; the stale g04.056 closeout baseline is still uncorrected.
- Closed: 2026-09-01 papercuts llama.cpp context-size god-file split. Current
  scan named two still-live targets: `prepared_facades.rs` at 381 code lines
  and `owned_driver.rs` at 260. Context-size/reasoning prepared-facade proofs
  plus shared `owned_start` helpers moved into
  `tests/prepared_facades/{selections,support}.rs` under the existing
  `prepared_facades` target. Remaining owned-driver startup-failure proofs
  moved into the existing `tests/owned_driver/failures.rs` module. Test bodies
  and counts held (7 prepared-facade, 12 owned-driver). `effigy --json scan
  god-files` dropped from 383 findings (7 critical / 42 high / 334 warning) to
  381 (7 critical / 42 high / 332 warning); both named findings are gone and
  no new module entered the scan. The g04.056 closeout already records the
  historical 378 (332 warnings / 46 errors) correction from g04.057
  compilation (`a40cefd5`); current checker taxonomy is
  critical/high/warning, so that historical paragraph was left unchanged.

### [x] Gemini Live feature proofs widen the god-file warning baseline — 2026-08-23
- Friction: the context-compression batch left `live_protocol/tests.rs`,
  `live_context_compression.rs`, and the earlier `live_output_maximum.rs` above
  the warning threshold, raising doctor findings from 371 to 374.
- Impact: later feature lanes inherit noisier structural-health evidence even
  though the error-level baseline is unchanged.
- Fix: split protocol encoding, context-compression, and output-maximum proofs
  into focused test modules without reducing route-local coverage.
- Surface: `swallowtail-adapter-gemini` Live protocol and acceptance tests.
- Closed: 2026-09-01 papercuts Gemini Live god-file split. Current scan named
  two live targets: `tests/live_context_compression.rs` (267 code lines) and
  `tests/live_output_maximum.rs` (257 code lines). The historical
  `live_protocol/tests.rs` path cited in the initial entry was stale papercut
  evidence and not present in `tests/`. Split `live_context_compression.rs` into
  `tests/live_context_compression/{support,rollover,restoration,composition}.rs`
  and `live_output_maximum.rs` into
  `tests/live_output_maximum/{support,preparation,rejections,facades}.rs` under
  the existing `live_prepared_facade` target. Test bodies and counts held (4
  context-compression tests, 7 output-maximum tests). `effigy --json scan
  god-files` improved from 381 findings (7 critical / 42 high / 332 warning) to
  379 (7 critical / 42 high / 330 warning); both warning findings are resolved
  and no new file entered the scan.

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

## Closed

### [x] OpenAI adapter test target name does not match its suite file — 2026-08-31
- Friction: `crates/swallowtail-adapter-openai/Cargo.toml` binds the test target
  `prepared_facade` to `tests/direct_suite.rs`, so
  `cargo test -p swallowtail-adapter-openai --test direct_suite` fails with
  "no test target named `direct_suite`".
- Impact: adding a module to the direct suite costs one failed command before
  the real target name is discovered; the same mismatch exists for
  `catalogue_activity` and `realtime_prepared_facade`.
- Fix: renamed the three explicit `[[test]]` targets to `catalogue_suite`,
  `direct_suite`, and `realtime_suite` so each matches its suite-root filename.
  Suite counts retained: catalogue 7, direct 35, realtime 33.
- Surface: `swallowtail-adapter-openai` integration test targets.
- Closed: 2026-08-31 papercuts wave 23 OpenAI test target names.

### [x] Effigy validation materializes an untracked repo skill — 2026-08-30
- Friction: running the card 011 docs/Northstar validation copied the Effigy
  skill and references into untracked `.agents/skills/effigy/`.
- Impact: a read-only validation round dirties the planning checkout and risks
  accidental inclusion in unrelated commits.
- Fix: Swallowtail PR 125 committed the 11-file project-local skill tree.
  Effigy `f3057b9bb554f1a54b4c2d4cab2df27d5f6da202` (PR 58) syncs that managed
  tree instead of leaving an untracked copy. On
  `effigy v0.12.1+local.f3057b9`, `effigy qa:docs` left
  `git status --porcelain` empty for `.agents/skills/effigy/`.
- Surface: Effigy startup or validation skill installation; `.agents/skills/`.
- Closed: 2026-08-31 papercuts wave 21 skill closeout.

### [x] Roadmap status census requires undocumented exact prose — 2026-08-30
- Friction: `qa:docs:roadmaps:status` rejected truthful g05 census wording
  until it used the exact phrases `N completed milestones`, `honest evidence
  stops`, and `ready milestones` with numeric counts.
- Impact: ordinary planning reconciliation fails through trial and error even
  when milestone frontmatter and the stated census agree.
- Fix: documented the live census regexes and Status buckets in
  `docs/roadmaps/status-grammar.md`, linked from `docs/roadmaps/README.md` and
  `scripts/README.md`.
- Surface: `scripts/check-roadmap-status-drift.py`; generation indexes.
- Closed: 2026-08-30 papercuts wave 19 census grammar.

### [x] Batch cards use `gated` as a status outside the accepted buckets — 2026-08-29
- Friction: card 010 used `Status: gated`, while roadmap status QA accepts only
  planned, ready, blocked, stopped, and complete variants. The dependency was
  truthful, but `effigy qa:docs` could not classify the card.
- Impact: a valid planning gate fails late, and later agents may repeat the
  unsupported status because the batch-card template does not name the allowed
  buckets.
- Fix: card 010 is `Status: complete`. Swallowtail-local
  `docs/roadmaps/status-grammar.md` names the accepted buckets and that a gate
  is `Status: planned; gated behind …` or `ready; …`, not `Status: gated`.
- Surface: Swallowtail roadmap status QA; card 010.
- Closed: 2026-08-30 papercuts wave 19 gated status.

### [x] Docs link QA omits research and lane-log bodies — 2026-08-28
- Friction: PR 112 passed `effigy qa:docs`, but Research 255 contained six
  links to nonexistent contract filenames. The link selector checks a bounded
  front-door set and did not inspect the changed research file.
- Impact: promoted evidence can claim canonical authority while its durable
  links are broken, and green CI does not catch the defect.
- Fix: `scripts/check-docs-links.py` keeps the front-door set and also scans
  `docs/research` and `docs/logs` Markdown bodies behind `qa:docs:links`,
  without restoring broad child-index churn. Corrected one existing
  `../../research/` misspell in the g04.083c lane log.
- Surface: `qa:docs:links`; research and lane-log review.
- Closed: 2026-08-28 papercuts wave 8 docs links.

### [x] Docs index QA misses roadmap-status drift — 2026-08-26
- Friction: PR 73 passed every named docs-index selector while
  `generation-index.md` still called g04.074 ready and the batch-card index
  still listed completed card 204 as Ready and blocked cards 205-206 as
  Planned.
- Impact: a review-ready closeout can leave the canonical planning indexes in
  mutually contradictory states despite green validation.
- Fix: `scripts/check-roadmap-status-drift.py` reconciles batch-card section
  membership, milestone annotations, and generation-index ready/completed/stop
  census against Status frontmatter; wired as `qa:docs:roadmaps:status`.
- Surface: Effigy Northstar roadmap and batch-card index QA.
- Closed: 2026-08-28 papercuts wave 4 QA flakes.

### [x] Timing-sensitive deadline fixtures make unrelated PR heads red — 2026-08-21
- Friction: restacked PR 23 failed Stable in Ollama
  `deadline_remains_distinct_from_cancellation`, while PR 21 failed MSRV in
  Codex `callback_wait_ends_when_the_host_deadline_is_observed`; neither PR
  changes the failing adapter surface.
- Impact: otherwise mergeable stacked heads need unrelated CI reruns before
  the orchestrator can satisfy the exact-head green gate.
- Fix: Ollama and Codex deadline proofs now use parked controllable clocks and
  sync after the fixture hold is observed; Alibaba retained-load deadline uses
  a 500ms bound instead of 5ms so setup cannot trip `deadline_elapsed`.
- Surface: Ollama attached-driver deadline test; Codex app-server callback
  deadline test; Alibaba retained-load deadline test; Stable and MSRV CI jobs.
- Recurrence 2026-08-27 (PR 82): Alibaba Model Studio
  `retained_load_deadline_joins_transport_before_releasing_access` observed
  `deadline_elapsed` instead of `timed_out` on a Gemini evidence-only head. A
  retry moved past it without a code change.
- Closed: 2026-08-28 papercuts wave 4 QA flakes.

### [x] DeepSeek stream-cancellation test flakes as ProviderFailed — 2026-08-19
- Friction: `swallowtail-adapter-deepseek::driver::active_stream_cancellation_joins_before_session_credential_release`
  expected `Cancelled` and observed `ProviderFailed` with
  `swallowtail.deepseek.stream_incomplete` / `TransportInterrupted` on tag CI
  run `32309276223` attempt 1. The same SHA had already passed PR and
  dispatched CI; the in-place rerun passed.
- Impact: tag-triggered CI can fail a green SHA without a product change,
  tempting a retag.
- Fix: treat stream `Closed` with an already-requested cancellation as
  `Cancelled`, matching the existing `Item(Err)` cancellation path.
- Surface: DeepSeek driver cancellation test; tag CI Stable job.
- Recurrence 2026-08-27 (PR 82): Stable again observed
  `swallowtail.deepseek.stream_incomplete` instead of `Cancelled` on a Gemini
  evidence-only head. The next unchanged-head retry moved past it.
- Closed: 2026-08-28 papercuts wave 4 QA flakes.

### [x] Pi replay-during-resume fixture can hang MSRV CI — 2026-08-24
- Friction: PR 54's first pinned-MSRV run stalled for six hours in
  `resume_fails_closed_on_replay_evidence`; one local exact-head run also
  stalled, while immediate reruns passed on Rust 1.95.0 and Stable.
- Impact: an unrelated Qoder documentation head can remain non-green until the
  hosted timeout or a successful rerun.
- Fix: emit the unexpected `replay_item` before the `session_switch` success
  response so resume fails closed on the pending command instead of racing a
  completed switch against force-stop wait.
- Surface: Pi SDK sidecar `ReplayDuringResume` fixture; pinned-MSRV CI.
- Closed: 2026-08-28 papercuts wave 4 QA flakes.

### [x] A `/var` review worktree breaks affected-package path patches — 2026-08-26
- Friction: macOS canonicalizes a `mktemp` worktree from `/var/...` to
  `/private/var/...`, while the affected-package verifier writes Cargo patch
  paths with the non-canonical spelling.
- Impact: `package:verify-affected` reports unused patches and a locked
  `Cargo.lock` update even when the reviewed package and lockfile are sound.
- Fix: `scripts/verify-affected-packages.sh` now resolves the repo root with
  `pwd -P` via `scripts/validation/path.sh` before writing patch paths.
- Surface: `scripts/verify-affected-packages.sh`; disposable review worktrees
  on macOS.
- Closed: 2026-08-27 papercuts wave 2 CI/path.

### [x] Isolated HOME for provider probes steals rustup — 2026-08-26
- Friction: Grok parser probes set `HOME`/`GROK_HOME` to an empty isolated tree
  and left those exports in the agent shell. Later `effigy validate:focused`
  ran cargo through rustup with `rustup home` under the isolated tree and no
  toolchain.
- Impact: docs-only closeout validation fails with `rustup could not choose a
  version of cargo` even though host rustup is healthy.
- Fix: `scripts/run-with-isolated-home.sh` wraps the probe and restores host
  `HOME` / unsets provider-home vars on exit; `AGENTS.md` points agents there.
- Surface: g04.072 / Research 219 isolated extracted-binary help and initialize.
- Closed: 2026-08-27 papercuts wave 2 CI/path.

### [x] OpenCode cancellation fixture panics on expected broken pipe — 2026-08-22
- Friction: PR 35 MSRV failed in
  `post_dispatch_cancellation_is_joined_and_unconfirmed` because the fixture
  response writer treated the cancelled client's `BrokenPipe` as fatal and the
  fixture server then panicked while joining.
- Impact: unrelated exact heads can fail MSRV after the product cancellation
  path has already produced the expected disconnect.
- Fix: let the cancellation fixture accept `BrokenPipe`/connection reset while
  writing the abandoned response, while preserving failures for other write
  errors. Drop no longer panics on join after an expected disconnect abort.
- Surface: OpenCode prepared-facade HTTP fixture response writer; MSRV CI.
- Recurrence 2026-08-26 (PR 77): the same writer aborted the Stable job, not
  just MSRV, this time as `ConnectionReset` in
  `cancellation_deadline_and_cleanup_release_leases_without_owning_the_server`.
  The drop-time panic is non-unwinding, so the run dies with SIGABRT and
  cancels 887 unrelated tests. A plain rerun went green, and the branch touched
  no OpenCode file. Still open and now blocking unrelated lanes.
- Recurrence 2026-08-27 (PR 82): the same cancellation fixture aborted Stable
  with `BrokenPipe` and a destructor-time double panic. The branch changed only
  Gemini evidence/docs; a later unchanged-head retry passed.
- Closed: 2026-08-27 papercuts wave 2 CI/path.

### [x] rustfmt --edition 2021 cannot parse this 2024 workspace — 2026-08-20
- Friction: `rustfmt --edition 2021 <file>` fails on let-chains in sibling
  modules (`preflight/validation.rs`, `provider_session_history/page.rs`)
  even when those files are not the format target.
- Impact: file-scoped rustfmt with the wrong edition aborts instead of
  formatting the requested sources.
- Fix: `AGENTS.md` now requires `cargo fmt -p <crate>` / edition 2024 and
  forbids `rustfmt --edition 2021` on this workspace.
- Surface: local rustfmt invocation vs workspace edition 2024.
- Closed: 2026-08-27 papercuts wave 2 CI/path.

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
