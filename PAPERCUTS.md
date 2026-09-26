# Papercuts

Small, recurring friction worth fixing later. One entry each: date, what
happened, impact, a plausible fix. Remove an entry when it is fixed. Agents add
an entry when they hit a solvable hurdle; they don't stop the current task to
fix it.

### [ ] Candidate E projection modules exceed the god-file high threshold — 2026-09-04
- Friction: Card 075's two adapter-local Contract 061 projection modules are
  548 and 403 code lines, respectively, and `effigy scan god-files` marks both
  high.
- Impact: the exact ledger and projected-open seams are reviewable but need a
  focused split before adjacent projection work accumulates.
- Surface: `swallowtail-adapter-gemini` and `swallowtail-adapter-grok`
  consumer-route projection modules.

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

### [ ] Compound acknowledgement rows need per-half generic state association — 2026-09-04
- Friction: Contract 061's existing row-level state flags cannot associate the
  exact provider token and state for the reasoning and Plan halves of Kimi's
  compound acknowledgement row.
- Impact: a generic consumer would need an adapter downcast, while marking a
  Plan half pending after terminal reasoning rejection would claim an
  acknowledgement that was never dispatched.
- Fix: add the accepted runtime-owned compound acknowledgement value with
  `reasoning()` and `plan()` accessors and a distinct `RequestedNotDispatched`
  state; promote the Contract 061 amendment and shared runtime/testkit
  baseline before Card 034.
- Surface: Contract 061 active-session acknowledgement projection; Kimi ACP
  reasoning-first/Plan-second driver order.

### [ ] Architecture still names Command Code 1.54.0 after the 1.65.0 rebind — 2026-09-24
- Friction: `docs/knowledge/architecture/system-architecture.md` still describes
  `swallowtail-adapter-command-code` as exact npm `1.54.0` after g06.021
  rebound the `QualifiedOnly` point to `1.65.0`.
- Impact: the architecture surface disagrees with selection, the prepared
  guide, and the feature matrix on the current qualified point.
- Fix: retarget that paragraph to exact `1.65.0` on a card that owns the
  architecture sentence.
- Surface: `docs/knowledge/architecture/system-architecture.md` Command Code route
  family paragraph.

### [ ] Live-probe harnesses ran without a self-proof — 2026-09-05
- Friction: card 100's live harness consumed two operator authorizations
  without producing provider evidence: once on a wrong endpoint audience,
  once by capturing `diagnostic().code()` and dropping the message that
  carried the sidecar subcode. The route was correct both times.
- Impact: each lost authorization costs an operator round trip and a
  subscription-backed attempt; the record gains nothing.
- Fix: no live authorization is relayed until the harness has been driven
  end to end against the fake SDK through the same path it uses live and
  its record shape is asserted by a committed test.
- Surface: Claude Agent SDK live-probe harness; Chatterbox live-authorization
  practice.

### [ ] Independent reviewers cannot post formal GitHub reviews — 2026-09-05
- Friction: every worker and reviewer pushes and comments as the same
  GitHub identity, so `REQUEST_CHANGES` and approvals return 422 "Can not
  request changes on your own pull request"; verdicts land as ordinary
  comments and the merge gate is procedural.
- Impact: no enforceable review requirement on `main`, and verdict comments
  are indistinguishable from discussion to branch protection.
- Fix: a second GitHub identity (bot or app token) for review children, or
  a required status check that a reviewer publishes with the exact head.
- Surface: coordinator review children; GitHub branch protection on `main`.

### [ ] Contract 061 ledger fixtures assert emitting facade declaratively — 2026-09-05
- Friction: candidate K's ledgers (and likely earlier tranches) carry
  `emitted_by` as a declarative field and compare observed rows as a set of
  semantic ids, so the `operation_shape` half of the
  `(route_id, operation_shape, semantic_id)` tuple is asserted only for
  uniqueness inside the const table, not against which facade emitted it.
- Impact: a row misattributed to the wrong facade or shape would only be
  caught if it moved a per-facade count; exact-tuple attribution is not
  proved.
- Fix: one sweep card that makes every ledger assert `(shape, semantic_id)`
  per emitting facade against the actual contribution, across all merged
  tranches.
- Surface: `crates/swallowtail-adapter-*/tests/**` ledger fixtures; g05.009.

### [ ] `effigy release prepare --check-gates` discards the failing gate's output — 2026-09-05
- Friction: the one-shot v0.4.1 prepare reported only `floor` failed after
  200409ms and rolled back; the Clippy or test output that named the failure
  was not retained anywhere. Reproducing the floor gate in the same worktree
  minutes later passed (clippy clean, 271 test binaries ok).
- Impact: a consumed release authorization with no diagnosable cause; the
  v0.4.0 lane lost two authorizations to races that were only named because
  the worker happened to have the terminal open.
- Fix: tee each gate's stdout and stderr to `.effigy/reports/release/<gate>.log`
  and print the last lines of the failing gate in the rollback summary.
- Surface: Effigy release prepare gate runner; Swallowtail `config/release.toml`.
- Progress 2026-09-05: Effigy Chatterbox confirms the gate output is captured
  in memory and rendered by `effigy --json release prepare ...`; only the text
  renderer drops it. Intake filed in Effigy triage
  `docs/triage/20260905-092527-release-gate-failure-diagnosability.md`
  (commit `47f66f93c`). Swallowtail release lanes should run prepare with
  `--json` until the text renderer is fixed.
- Progress 2026-09-05 (later): fixed on Effigy main by PR 90 (card 1112,
  merge `7cafd3b5`): per-gate logs under `.effigy/reports/release/gates/`,
  a redacted `environment.json`, failing-gate tail in text output, and an
  immediate `release gates` inventory. Adopt through the local-install route
  after the `v0.4.1` lane closes; do not change the release tool mid-lane.
  Keep-on-failure remains in Effigy triage.

### [ ] Review or validation command wrappers rely on unavailable host `timeout` — 2026-09-04
- Friction: review or verification pipelines wrapping commands with `timeout <duration> <cmd>`
  fail silently on macOS hosts because `timeout` is a GNU Coreutils binary absent from
  default Darwin environments.
- Impact: review or verification pipelines can mask command execution, exit with false positive
  status, or fail unexpectedly.
- Fix: avoid wrapping verification commands in `timeout` without checking binary availability,
  or use portable shell-level timeouts.
- Surface: verification / review shell snippets and automation on Darwin hosts.

### [ ] Public API baseline gate flags expected adapter consumer route projection methods — 2026-09-04
- Friction: `effigy package:api` (`scripts/check-public-api.sh`) enforces an
  exact zero-diff comparison against `release-baselines/public-api-0.4.0/`,
  failing when new `pub fn consumer_route_projection_contribution` methods
  are added to adapter prepared operation structs, while worker manifests
  strictly forbid worker mutations to `release-baselines/`.
- Impact: worker PRs implementing planned projection contributions report
  failed `effigy package:api` semantic diff checks that can only be reconciled
  at the coordinator/release gate level.
- Fix: permit unreleased additions to adapter packages via an unreleased delta
  directory or allow worker cards that introduce planned inherent methods to
  update package-scoped public-api baseline manifests.
- Surface: `scripts/check-public-api.sh`; `release-baselines/public-api-0.4.0/`.

### [ ] Rust everyday closeout collides after same-path content revision — 2026-09-03
- Friction: `northstar-rust-quality closeout` reused snapshot
  `6a3ace2581a207e4f7541ab74ae7dbe0a68e36eceb97a419c975d83c5cc8614b`
  and refused a second record after the implementation changed but the changed
  path set stayed the same.
- Impact: an exact-head repair can leave compact evidence bound to superseded
  file contents even after all repository-owned gates are rerun.
- Fix: include changed-file content or diff digests in the everyday closeout
  identity, or permit a new immutable record when those digests differ.
- Surface: Northstar Rust quality everyday closeout snapshot identity.

### [ ] `effigy release gates` does not return the documented gate inventory — 2026-09-02
- Friction: the read-only command stayed silent for more than 90 seconds and
  required interruption instead of listing the configured release gates.
- Impact: release planning cannot rely on the documented inspection route to
  inventory gates without risking an unbounded wait.
- Fix: make `release gates` print the configured names immediately, or emit
  progress and a bounded diagnostic if it performs gate work.
- Surface: Effigy release inspection against Swallowtail's
  `config/release.toml`; deferred outside the v0.4.0 milestone.

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

### [ ] evidence-download cwd steals later repo commands — 2026-08-22
- Friction: a disposable evidence directory became the persistent shell cwd, so later `effigy` and `git diff --check` ran outside the worktree.
- Impact: card-gate commands fail with missing-catalog or "not a git repository" errors after an otherwise successful evidence fetch.
- Fix: `cd` back to the worktree after temp-dir evidence work, or run later repo commands with an explicit working directory.
- Surface: agent shell sessions that download provider evidence outside the worktree.
- Progress 2026-09-01 (evidence-download cwd lane): inventory finds no
  repo-owned script or helper that cds into temp evidence dirs — all
  `scripts/*.sh` temp use stays in-process and `run-with-isolated-home.sh`
  restores `HOME` only — so there is no execution seam to repair. The skill
  prose rule "Always `cd` to the Swallowtail repo root before `cargo` /
  `effigy`" was already in force (2026-08-18) when this was observed
  (2026-08-22), so prose does not fail-close the leak. The only leak-shaped
  repo text is the reference.md manifest snippet run "inside"
  `/tmp/<pkg>-<ver>/package`; repairing it to a subshell form requires
  editing `.cursor/skills/version-currentness/**`, out of scope while the
  currentness worker is active. Ownership stop: the remaining seam is agent
  shell behavior; reproduced `git diff --check` → "not a git repository"
  (exit 129) and `effigy validate:focused` → "not defined in effective
  catalogs" (exit 1) from a leaked cwd, while a subshell evidence run keeps
  the caller cwd stable.

### [ ] Preflight keys the consumer-tool exclusion on access, not the boundary — 2026-09-04
- Friction: `swallowtail-core` preflight refuses any interactive session that
  pairs `ResourceAccess::ReadWrite` with `Capability::ToolCalls`, while
  Contract 013 keys that exclusion on a bounded profile's claimed filesystem
  boundary.
- Impact: an ambient route whose writes are admitted per call through a
  namespaced provider extension claims no boundary and is admissible under the
  contract, but cannot bind a read-write lease; `claude-agent.sdk` card 080
  stopped at exactly this line and refuses write profiles with
  `swallowtail.claude-agent.sdk.preparation.write_admission_unavailable`.
- Fix: card 089 narrows the shared guard to key on the boundary claim; card
  080's second PR then drops the typed refusal.
- Surface: shared preflight in `swallowtail-core`.

### [ ] Retained worker workspace can disappear before deferred continuation — 2026-09-05
- Friction: the retained Card 094 worker agent existed in prior coordination history, but its registered worktree had been removed before the post-tag continuation arrived.
- Impact: the original agent could not be resumed; recreating the implementation lane required a replacement worker and fresh same-workspace reviewer placement.
- Fix: preserve deferred worker workspaces until the lane is merged and closed out, or make continuation preflight surface the archival/removal event before dispatch.
- Surface: Paseo workspace lifecycle and Northstar deferred-lane continuation.

### [ ] Public API baseline directory naming invites post-tag edits to look like tag mutation — 2026-09-06

- Friction: `release-baselines/public-api-<current>` is the working baseline
  that absorbs additive API between tags; `public-api-<previous>` is the
  immutable one. Nothing says so, and `public-api-unreleased` exists but is
  stale and unread by `scripts/check-public-api.sh`. Card 082 was steered
  into the unreleased directory and lost a review round.
- Fix shape: either delete `public-api-unreleased` and document the
  working/immutable roles in `scripts/README.md` and Contract 036, or make
  the gate overlay an `unreleased` directory and have prepare fold it in.
  Decide once; the release lane should not re-derive this.
- Surface: `scripts/check-public-api.sh`, `release-baselines/`, Contract 036.

### [ ] Bounded-recovery timing flake follows the tests onto Linux runners — 2026-09-06

- Friction: `swallowtail-adapter-anthropic::managed_prepared_facade
  prepared_interrupt_deletes_owned_resources_before_credential_release`
  returned `remote_state_unconfirmed` instead of `Cancelled` after 4.73s
  on `ubuntu-latest` (run 34057905627), and passed on the same SHA in run
  34057905516. The macOS shard flakes recorded in card 094 were read as
  macOS scheduling; the bound is simply wall clock under load.
- Impact: a red PR gate that a rerun would clear, which the g05.034 target
  explicitly rules out.
- Fix shape: replace the wall-clock recovery bound with a deterministic
  fixture signal, as cards 093 and 094 did elsewhere. Card 104 owns the
  sweep.
- Surface: `crates/swallowtail-adapter-anthropic/tests/managed_driver/`.

### [ ] Nextest reports a rare leak for a test that spawns nothing — 2026-09-08
- Friction: `claude_agent_sdk_driver` occasionally reports `118 passed
  (1 leaky)` in the process shard. Card 139 captured the name by running with
  `--status-level leak`: it is always
  `registered_tool_route::open_without_a_host_composition_fails_typed`, at
  roughly one occurrence in 24 full-binary runs. That test spawns no child
  process at all — no courier, no nested build, no `LocalProcessHost` — so no
  descendant can be holding the inherited stdout/stderr that nextest's leak
  detection keys on.
- Impact: a leak annotation nobody can act on, in the shard whose determinism
  card 139 was opened to establish. It does not fail the run, but it is
  exactly the kind of unexplained signal that makes the next real one easy to
  wave through.
- Fix shape: most likely the default 100ms `leak-timeout` expiring during
  process teardown under load rather than a held pipe; `.config/nextest.toml`
  sets no `leak-timeout`. Confirm by raising it and re-running loaded, then
  either set it or explain the held descriptor. Keeping `--status-level leak`
  on the shard would stop the name being hidden again.
- Surface: `.config/nextest.toml`; the `ci-process` profile. Card 139's owned
  paths exclude that file, which card 095 owns.

### [ ] Effigy cannot skip release gates from hosted exact-SHA evidence — 2026-09-06
- Friction: Card 109 needs `lint`, `lint:no-features`, `test`, and `floor`
  satisfied by a green hosted run at the SHA to tag, or at a commit with an
  identical tree. Effigy v0.12.1 (`4a17a76`) preserves `[release.gates]`
  order, has one table, and has no skip or `--hosted` flag.
- Impact: Swallowtail omits those four gates from the default table and
  keeps them as a comment-only local-heavy profile. A later native skip would
  let one complete table stay loaded.
- Fix: Effigy Chatterbox — skip named gates when a green hosted run at HEAD
  is recorded, or add a second gates profile / `--hosted` evidence flag.
- Surface: Effigy `release prepare`; Swallowtail `config/release.toml`;
  Contract 036 hosted-delegation clause.

### [ ] Root README still hardcodes the release version after card 110 — 2026-09-07
- Friction: the `v0.4.4` prepare (card 122) failed its `qa` gate because root
  `README.md` carried `v0.4.3` in the release posture, coordinated package
  version, four source-install tags, runtime prerequisite, and release-note
  link. Card 110 derived the gate scripts from the tree but left the README
  literals in place, so every candidate still needs a manual README edit.
- Fix shape: either generate those README lines from `Cargo.toml` during
  prepare (an Effigy sync-file or a small script the cheap gate runs), or drop
  the version literals from the README and point at the release index.
- Surface: root `README.md`; `scripts/check-consumer-front-door.py`; card 110.

### [ ] Cold nested courier target starves the capture wrapper test budget — 2026-09-08
- Friction: `readiness::wrapper_death_preserves_partial_capture_journal`
  allows five seconds for its child wrapper to persist the journal, but when
  `target/card125-courier` is cold the parallel `courier_binary()` build runs
  a nested `cargo build` while all driver tests execute, and the starved child
  missed the budget twice on this machine. Reproduced once on the clean base
  head with a cold nested target; warm runs pass repeatedly.
- Fix shape: either raise the child budget to cover a cold sibling build, or
  build the courier outside the parallel test set (an Effigy before-task step
  or a lazy dedicated test that runs alone).
- Surface: `crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/readiness.rs`;
  `registered_tool_route.rs` courier acquisition.

### [ ] Orphaned Cursor release-identity test never compiles — 2026-09-10
- Friction: `crates/swallowtail-adapter-cursor/tests/cursor_agent_release_identity.rs`
  has no `[[test]]` target and no suite root references it, so with
  `autotests = false` it never compiles or runs; its assertions still pin the
  old `2026.08.11` ceiling. Found during g05.062.
- Fix: wire it to a target or fold it into `compatibility_corpus.rs`.

### [ ] Package secret pattern false-positives on hyphenated English — 2026-09-22
- Friction: `sk-[A-Za-z0-9_-]{20,}` in `scripts/validation/archive.sh` matches
  ordinary labels such as `headless-background-task-waiting-notice` through
  `task-waiting`, so `package:verify-affected` rejects a clean package. Found
  in g06.027; the labels were rephrased to keep the proof green.
- Fix: anchor the pattern so `sk-` must start a token.
