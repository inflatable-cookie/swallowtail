# 094 Deadline Fixture Determinism Sweep

Status: complete; PR 227 merged at `3506d9a5`
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../030-v0-4-1-release-readiness.md`
Depends on: card 091 attempt-4 floor excerpt; card 093 as the pattern precedent

## Goal

Remove every test in the workspace whose pass or fail depends on real
process timing against a host deadline, starting with the Pi sidecar
lifecycle failure that stopped card 091's fourth prepare. Release-lane repair
under the `v0.4.1` feature freeze; production code unchanged.

## Defect

`crates/swallowtail-adapter-pi/tests/sidecar_driver/lifecycle.rs`
`host_deadline_uses_native_abort_and_resolves_timed_out` opens a session
against `SidecarScenario::Hold` with `with_immediate_time()` (fixture clock
at 1000 with every timer firing immediately), times out a turn, then asserts
that `close_session` reports
`swallowtail.session_cleanup.deadline_expired`. Runtime
`bound_session_cleanup` is correct: it returns the inner cleanup outcome if
that future is ready before the deadline observation, otherwise
`deadline_expired`. Whether the inner cleanup is already ready on its first
poll depends on whether the held fake sidecar process has exited after the
turn's native abort. That is real-process timing, so the assertion is
scheduler-dependent: `None` (clean) on a loaded host, `deadline_expired` on
a quiet one. The v0.4.0 OpenCode deadline fixture and card 093's sidecar
asset fixture were the same shape.

## Scope

1. Fix the Pi lifecycle test by controlling the process lifecycle
   explicitly: the `Hold` scenario must keep the fake sidecar alive and
   unresponsive to close until the test releases it, so cleanup can never be
   ready before the immediate deadline fires; release it for teardown. The
   test then proves one outcome deterministically. If the intent is to prove
   the clean path too, add a separate test that releases the child before
   close and asserts `Clean`.
2. Sweep the workspace for the same shape: every test that asserts a
   deadline, timeout, cancellation, or cleanup outcome while a real child
   process, thread, or independently scheduled timer can change which branch
   wins. Start from `grep -rln "deadline_expired\|TimedOut\|with_immediate_time\|from_millis" crates/*/tests`
   and the Claude Agent structured-run, SDK driver cancellation, and SDK
   driver lifecycle tests already known to share the pattern. Classify each
   as deterministic, fixed here, or out of scope with a one-line reason.
3. For each fixed test, apply the card 093 rule: ordering first, one large
   named hang guard only for a dead process, never a bound a passing test
   relies on.
4. Prove determinism per touched test binary: 20+ runs under deliberate CPU
   load with zero failures, recorded in the card result, plus one full
   `cargo test --workspace --all-features --locked` under pinned `1.95.0`.
5. Keep every `crates/**/src` path unchanged. If a fixture cannot be made
   deterministic without a production change, record it and stop.

## Out Of Scope

Production behaviour; the release candidate; raising bounds as the fix;
tests whose timing sensitivity is already proven absent.

## Acceptance Criteria

- [ ] the Pi lifecycle test proves one outcome under any host load
- [ ] the sweep ledger classifies every candidate test with a reason
- [ ] every fixed binary passed the loop-under-load proof
- [ ] a full pinned-toolchain workspace test run passed
- [ ] production source is byte-identical

## Validation

- `cargo fmt --all -- --check`
- `effigy validate:focused <each touched adapter package, at most four per run>`
- `rustup run 1.95.0 cargo test --workspace --all-features --locked`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: no passing test in the workspace depends on which of two
independently scheduled events wins. Smallest counterexample: a retained
race with a widened bound, or a sweep entry marked deterministic without a
reason.

## Auto-Continuation

No. Stop for exact-head review; card 091 re-prepares on the merged base.

## Result

Release-blocking batch prepared on current main `2b919503`, which contains
the operator compression decision at `820bc4d1`:

- **Pi sidecar driver — fixed here.** `SidecarScenario::Hold` now keeps its
  fake process alive and ignores close until the test explicitly releases it.
  The deadline test observes turn startup, fires the fixture deadline, asserts
  only `swallowtail.session_cleanup.deadline_expired`, then releases and joins
  the fake process for teardown. Existing clean-path tests release before
  session close.
- **Claude Agent SDK driver — fixed here.** `OpenHold` process wait remains
  blocked until explicit fixture release, so the open-deadline test asserts
  only `swallowtail.claude-agent.sdk.open_cleanup_unconfirmed` before teardown.
- **OpenCode prepared facade — fixed here.** Cancellation and deadline fixtures
  use response gates and observed dispatch instead of finite sleeps for delete
  and session-import operations.
- **Kimi local-server lifecycle — fixed here.** Cancellation, deadline, and
  binding-import fixtures use an observed two-way response gate instead of a
  finite response delay.
- **Local-host watcher — fixed here.** The deadline fixture uses an indefinite
  child mode and explicit cleanup instead of relying on a 30-second child.

All five changes are test-only. No `crates/**/src`, production sidecar, Cargo,
changelog, release-baseline, contract, or Card 091 path changed.

### Under-load proof

Each row ran the complete touched test binary 24 times under Rust `1.95.0`
with `--test-threads=1` while 18 CPU burners were active. Final result: zero
failures.

| Package | Test binary | Runs | Failures |
| --- | --- | ---: | ---: |
| `swallowtail-adapter-pi` | `sidecar_driver` | 24 | 0 |
| `swallowtail-adapter-claude-agent` | `claude_agent_sdk_driver` | 24 | 0 |
| `swallowtail-adapter-opencode` | `prepared_facade` | 24 | 0 |
| `swallowtail-adapter-kimi` | `local_server_interactive` | 24 | 0 |
| `swallowtail-host-local` | `watcher_service` | 24 | 0 |

### Release-lane validation

- `cargo fmt --all -- --check` — passed
- `effigy validate:focused swallowtail-adapter-pi swallowtail-adapter-claude-agent swallowtail-adapter-opencode swallowtail-adapter-kimi` — passed, 767 tests
- `effigy validate:focused swallowtail-host-local` — passed, 131 tests
- `effigy qa:northstar` — passed
- `git diff --check` — passed
- `rustup run 1.95.0 cargo test --workspace --all-features --locked` — passed

### Deferred post-tag continuation

The remaining workspace-wide candidate classification is explicitly deferred
until after the tag under Card 094. The initial grep produced 102 candidate
test files, including the named Claude structured-run and SDK areas. This
release-blocking result records only the five repaired and proved binaries
above; it does not claim the 102-candidate ledger or the full Card 094 sweep is
complete, and that continuation does not gate `v0.4.1`. The post-tag
continuation below records that ledger.

Fixture-uniqueness release gate prepared from current main `dc04df04`:

- **Claude Agent SDK sidecar asset — fixed here.** Its temporary-directory
  name now includes a process-wide `AtomicUsize` sequence after the process ID
  and timestamp. `create_dir` rejects any collision instead of sharing an
  existing root through `create_dir_all`.
- **Anthropic prepared fixture — deterministic.** Its attachment path already
  includes the process-wide `NEXT_ATTACHMENT: AtomicUsize` sequence.
- **Claude Code support — deterministic.** Its watcher temporary root already
  includes the process-wide `TEMPORARY_ROOT_SEQUENCE` counter.
- **Claude structured-run watcher proof — fixed here.** Its workspace now has
  an atomic sequence suffix, uses `create_dir`, and its unwind test checks the
  exact allocated path after owner teardown.
- **Claude live watcher probe — fixed here.** Its workspace now has an atomic
  sequence suffix and uses `create_dir`.
- **Oh My Pi fixture host — fixed here.** Attachment paths now have an atomic
  sequence suffix and use `File::create_new`, the fail-loud file equivalent of
  `create_dir`.
- **OpenCode prepared fixture — fixed here.** Attachment paths now have an
  atomic sequence suffix and use `File::create_new`.

The exact pinned reproduction command completed 20 runs with 20 passes and
zero failures:

```text
rustup run 1.95.0 cargo test -p swallowtail-adapter-claude-agent --all-features --locked --test claude_agent_sdk_sidecar_asset
```

Focused validation passed 492 tests across the three touched packages. Pinned
Rust `1.95.0` package tests passed for Oh My Pi and OpenCode; pinned Claude
structured-run and live-watcher tests passed. Formatting and the 40-package
semantic API gate passed.

This result covers only the fixture-uniqueness release gate recorded after the
fifth prepare. The broader Card 094 deadline sweep was deferred post-tag under
the operator compression decision at this gate; the post-tag continuation below
records its ledger. All changes are test-only plus this Result. Production source,
sidecars, Cargo files, changelog, release baselines, contracts, and Card 091 are
unchanged.

### Resume repair after origin refresh

After rebasing onto current `origin/main` `4c703616`, every PR-introduced
`Condvar::wait` in the five repaired test-support files was replaced by the
named 120-second `HANG_GUARD`; no untimed condvar wait remains in those files.
The five under-load rows above, the focused 772-test validation, the pinned
workspace test, formatting, `git diff --check`, and `effigy qa:northstar` were
reproduced after this repair and passed. The deferred 102-candidate sweep
ledger above remains explicitly deferred and is not claimed complete.

### Post-tag continuation: PR #227 correction

Prepared on current main `1b1b023c` by the authorized replacement
implementation lane for the deferred continuation; PR #227 stayed open and
deferred through the tag. The correction rebases the PR on current main,
bounds every untimed condvar wait the PR introduced, and records the
deferred sweep ledger.

- **Untimed condvar waits — fixed here.** The nine wait loops PR #227
  introduced (claude `sdk_support/host/process.rs` open-hold wait; kimi
  `local_server_lifecycle_support/server.rs` hold, release,
  seen-count, gated-drain, and serve waits; opencode
  `http_support/delete_gate.rs` dispatch wait; pi `support/sidecar_host.rs`
  process-exit wait; pi `support/sidecar_host/process.rs` hold wait) are now
  `wait_timeout_while` under a file-local
  `HANG_GUARD: Duration = Duration::from_secs(120)` with a named loud
  assertion on expiry. The guard fails a broken ordering contract instead
  of hanging the run; no passing test relies on the bound and no existing
  assertion bound moved. Untimed waits outside PR #227's additions are
  untouched.

### Continuation validation

- `cargo fmt --all -- --check` — passed
- `effigy validate:focused swallowtail-adapter-pi swallowtail-adapter-claude-agent swallowtail-adapter-opencode swallowtail-adapter-kimi` — passed, 767 tests
- `rustup run 1.95.0 cargo test --workspace --all-features --locked` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed

### Continuation under-load proof

Each row ran the complete touched test binary 24 times under Rust `1.95.0`
while 18 CPU burners were active. The eight rows cover every test binary
that compiles a corrected fixture. Final result: zero failures.

| Package | Test target | Runs | Failures |
| --- | --- | ---: | ---: |
| `swallowtail-adapter-pi` | `driver` | 24 | 0 |
| `swallowtail-adapter-pi` | `prepared_facade` | 24 | 0 |
| `swallowtail-adapter-pi` | `sidecar_addable` | 24 | 0 |
| `swallowtail-adapter-pi` | `sidecar_driver` | 24 | 0 |
| `swallowtail-adapter-pi` | `structured_run` | 24 | 0 |
| `swallowtail-adapter-claude-agent` | `claude_agent_sdk_driver` | 24 | 0 |
| `swallowtail-adapter-opencode` | `prepared_facade` (`runtime_suite.rs`) | 24 | 0 |
| `swallowtail-adapter-kimi` | `local_server_interactive` (`local_suite.rs`) | 24 | 0 |

### Deferred sweep ledger (post-tag continuation)

The candidate population is the card's grep
(`deadline_expired|TimedOut|with_immediate_time|from_millis`) over
`crates/*/tests`: 102 files when PR #227 was prepared, 100 files in the
current tree after the fixture-uniqueness gate and the rebase onto
`1b1b023c`. Six independent read-only passes classified every candidate
against the review oracle, and every entry below carries its mechanism:
60 candidate files are deterministic, 21 support fixtures are
ordering-safe, 9 tests remain timing-sensitive, and 10 support fixtures
can still decide their users' outcomes. Repairing the nine
timing-sensitive tests and ten support hazards is outside this lane's one
bounded correction, so that residue returns to roadmap sequencing through
the coordinator.

#### Timing-sensitive tests remaining (9)

- `swallowtail-adapter-kimi-platform/tests/direct_driver.rs` —
  in_flight_deadline test arms a real-clock 20ms deadline (ThreadServices)
  that must be observed before the fixture's ~2s keepalive window ends; the
  pump polls the subscription before the deadline, so window expiry yields
  sse_disconnected instead of the asserted TimedOut — no gate orders the two
  events

- `swallowtail-adapter-llama-cpp/tests/attached_driver.rs` —
  deadline_stays_distinct test arms a real-clock 100ms deadline
  (deadline_after on ThreadServices) against the fixture's ~2s keepalive
  window; next_run_signal polls the subscription before the deadline, so a
  delayed observation past window expiry reports sse_disconnected instead of
  TimedOut

- `swallowtail-adapter-opencode/tests/http_driver/lifecycle.rs` —
  Timeout/cancel/detach orderings are explicit (the 30ms real deadline is sole
  decider against WaitForAbort, which never completes until abort), but every
  test asserts Clean from close_session under a real-clock 1s deadline — pass
  requires cleanup beating wall-clock, which a loaded host can flip to
  Degraded (driver races work vs wait_until and errors on expiry)

- `swallowtail-adapter-opencode/tests/prepared_facade/session_import.rs` —
  verified repair: delete/import gating is explicit — wait_for_dispatch under
  the 120s named HANG_GUARD (asserts on expiry, hang guard only), cancellation
  requested or manual deadline fired-and-observed before join, gate released
  after.

- `swallowtail-adapter-qwen/tests/live_installed_probe.rs` — Asserts a real
  child qwen CLI process's live exit success while a 45s real-clock try_wait
  poll kills-and-panics on overrun — a real process/network exit racing a
  deadline observation that a loaded host can lose

- `swallowtail-adapter-xai/tests/direct_driver.rs` — Clean close_session
  assertions (serial_session, active_turn, disconnect, provider-failure,
  deadline tests) yield inside bound_session_cleanup on the blocking-work
  oneshot, racing the support timer that fires unconditionally 10ms after
  first poll — a loaded host flips Clean to deadline_expired; deadline/cancel
  branches themselves are gated

- `swallowtail-host-local/tests/local_services.rs` — monotonic_deadlines test
  asserts started.elapsed() < 1s after dropping a 60s-deadline wait_until — a
  pass-relied wall-clock upper bound a loaded host can violate (correct impl
  takes µs, buggy impl 60s, but the bound itself decides pass); all other
  tests are synchronous fs/state ops

- `swallowtail-host-local/tests/watcher_bridge/proof.rs` —
  retired_proof_retains_an_in_flight_wait spawns the WAIT request then
  sleep(100ms) before close_lease — registration-vs-retirement is ungated, so
  a slow thread flips the status==200 / proof-contains-Wait asserts; other two
  tests are synchronous request/response

- `swallowtail-host-local/tests/watcher_service/lifecycle.rs` —
  watcher_wait_is_pending asserts started.elapsed() < 100ms for a single poll
  (pass-relied bound a loaded host can violate), and the cooperative-child
  test probes !process_is_alive via ps after killpg+join with no gate on the
  orphan's reap — a transient zombie flips the assert

#### Support fixtures that can decide outcomes (10)

- `swallowtail-adapter-kimi-platform/tests/support/mod.rs` — WaitForCancel
  keepalive writer holds the SSE stream open for only ~2s real time; window
  expiry drops the connection and flips users' asserted TimedOut/Cancelled
  outcomes into sse_disconnected (exercised by direct_driver's deadline test)

- `swallowtail-adapter-kimi-platform/tests/support/services.rs` — ThreadServices
  TimeService is the real clock (millis-since-origin, real thread sleep in
  wait_until), so users' TimedOut outcomes depend on that real-time
  observation winning a race against ungated competing fixture events rather
  than a fixture clock

- `swallowtail-adapter-llama-cpp/tests/support/server.rs` —
  respond_wait_for_cancel bounds the SSE stream to ~2s of real-time
  keepalives; expiry closes the stream and converts users' expected
  Cancelled/TimedOut terminals into provider disconnect failures

- `swallowtail-adapter-llama-cpp/tests/support/services.rs` — Real-clock
  TimeService (millis thread sleep) plus deadline_after arms genuine
  wall-clock deadlines, so users' TimedOut assertions rest on real-time
  arrival order against ungated fixture events, not explicit ordering

- `swallowtail-adapter-ollama/tests/support/services.rs` — Wall-clock wait_until
  spawns a thread sleeping the real remaining time and then fires the deadline
  — expected-success users (1s probes in
  connection_lifecycle.rs/prepared_facade, 10s close bounds in fixture.rs)
  race completion against that real sleep, ungated (parked-clock mode is fine)

- `swallowtail-adapter-openai/tests/realtime_support/services.rs` — Delayed
  timer = real thread sleep(20ms) then oneshot must elapse before users
  observe the deadline — firing guaranteed but arrival vs independent provider
  events ungated (current users hold the provider so the timer is sole
  decider, but the bound is wall-clock)

- `swallowtail-adapter-openai/tests/support/services.rs` — TimeMode::Delayed =
  thread::sleep(20ms) on a real thread before the oneshot deadline observation
  — firing guaranteed but arrival vs independent events ungated; Pending never
  fires, so any Delayed+completing pairing would flip Completed/TimedOut by
  host load

- `swallowtail-adapter-opencode/tests/http_support/services.rs` — Real-clock
  TimeService: wait_until sleeps the actual remaining wall time and
  deadline_after uses real now, so deadline arrival vs independent provider
  events is ungated and can decide users' outcomes (e.g. Clean vs Degraded
  cleanup under load); no fixture-clock override

- `swallowtail-adapter-opencode/tests/prepared_facade/fixture.rs` — Manual-armed
  deadlines are explicit (fire_and_wait_for_observation with 2s observed
  assert), but the TestClock fallback real-sleeps to the deadline on a spawned
  thread and the fixture's own probe/close_session impose pass-relied real 1s
  bounds that can decide users' outcomes on a loaded host

- `swallowtail-adapter-xai/tests/support/services.rs` —
  ThreadServices::wait_until ignores the deadline instant and fires after a
  fixed 10ms thread sleep, so driver_fixture's 1s cleanup bound effectively
  expires at 10ms and races real blocking cleanup; CleanupTime likewise sleeps
  real remaining time

#### Deterministic candidates (60)

-
  `swallowtail-adapter-alibaba-model-studio/tests/direct_driver/failure_cases.rs`
  — 1ms bounded poll only awaits the server counter for the turn's
  already-dispatched request (hang guard).

- `swallowtail-adapter-alibaba-model-studio/tests/protocol_fixtures.rs` — pure
  synchronous fixture/parse asserts — no threads, sleeps, deadlines, or
  spawned work anywhere

- `swallowtail-adapter-anthropic/tests/direct_driver.rs` — cancellation test
  polls inference_attempts (100×1ms) for the run's already-dispatched POST
  (hang guard) then r.

- `swallowtail-adapter-anthropic/tests/managed_driver.rs` — stream_attachments
  poll is a hang guard for the run's already-dispatched attach.

- `swallowtail-adapter-anthropic/tests/managed_driver/failure_tests.rs` —
  deadline_after(100) fixture-armed as the sole terminal path of the
  never-completing WaitForInterrupt stream.

- `swallowtail-adapter-anthropic/tests/managed_driver/interrupt_tests.rs` —
  200×1ms poll awaits the guaranteed stream attach (hang guard), cancellation
  requested explicitly, then termina.

- `swallowtail-adapter-antigravity/tests/continuation_suite.rs` — FixtureHost is
  an in-process fake process (scripted/held_open) with PendingTime (deadline
  never fires) and Imm.

- `swallowtail-adapter-antigravity/tests/headless_suite.rs` — same in-process
  fake host with fixture clocks.

- `swallowtail-adapter-claude-agent/tests/acp_driver/deadline.rs` —
  FixtureHost::with_immediate_deadline makes wait_until return Ready at once.

-
  `swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/cancellation.rs`
  — hold_pump/release_pump gates, fire_deadlines fixture clock, drop_within
  hang-guarded drop.

- `swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/framing.rs` —
  Scripted scenarios plus fire_deadlines fixture clock.

- `swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/lifecycle.rs`
  — Close outcomes scripted via scenario/exit-evidence knobs; assert_ordered
  reads recorded CleanupEvents.

- `swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/readiness.rs`
  — with_immediate_time fixture clock ends the open hold.

- `swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/stalls.rs` —
  Stall gates with with_immediate_time/fire_deadlines decide every outcome.

- `swallowtail-adapter-claude-agent/tests/claude_code_response_only.rs` —
  FakeProcessService plus Pending/ImmediateTimeService fixture clocks decide
  every deadline and cancellation.

-
  `swallowtail-adapter-claude-agent/tests/claude_code_structured_run/control_cases.rs`
  — FakeProcessService held_open/completed with Pending/ImmediateTimeService
  clocks.

-
  `swallowtail-adapter-claude-agent/tests/claude_code_structured_run/watcher_cases.rs`
  — 2s yield-spin waits only for the run's own guaranteed process start (hang
  guard).

-
  `swallowtail-adapter-claude-agent/tests/claude_code_structured_run/watcher_deadline.rs`
  — ControllableTimeService armed manually (time.fire) after a synchronous
  watcher start over loopback.

-
  `swallowtail-adapter-claude-agent/tests/claude_code_structured_run/watcher_stop_reentry.rs`
  — wait_for_fact's 10ms/2s poll is a hang guard for proof facts each caused
  by an action just taken (completer.pu.

- `swallowtail-adapter-claude-agent/tests/structured_run.rs` — Wire fixture
  host: deadline case uses with_immediate_deadline fixture clock, cancellation
  gated by wait_for_wr.

- `swallowtail-adapter-cline/tests/headless_suite.rs` — scripted fake process
  host with PendingTime default and ImmediateTime for the timeout test.

- `swallowtail-adapter-codex/tests/app_server/failure_boundaries.rs` —
  ControllableTime.advance_to(50) manually arms the deadline and
  wait_for_message condvar-waits (2s) a message t.

- `swallowtail-adapter-codex/tests/exec_deadline.rs` — fixture clocks only:
  RecordingService fires the armed deadline immediately (TimedOut),
  PendingTimeService neve.

- `swallowtail-adapter-codex/tests/installed_discovery.rs` — FakeProcessService
  probes (no real child process) with PendingTime or the immediately-firing
  RecordingService .

-
  `swallowtail-adapter-codex/tests/prepared_profile_cases/provider_session_import/acceptance.rs`
  — yield-spin gate waits for the scripted thread/list dispatch (guaranteed
  event) before requesting cancellation .

- `swallowtail-adapter-codex/tests/structured_run_parity.rs` — scripted fake
  processes.

- `swallowtail-adapter-cursor/tests/headless_suite.rs` — fake process host with
  PendingTime/ImmediateTime fixture clocks.

- `swallowtail-adapter-deepseek/tests/driver/failures.rs` — attempts()==2 poll
  awaits the already-dispatched second request (hang guard).

- `swallowtail-adapter-gemini/tests/headless_structured_run/production.rs` —
  every process exit supplied explicitly via FakeProcessService::with_exit.

- `swallowtail-adapter-gemini/tests/live_failures.rs` — fake WS server sends
  scripted frames.

- `swallowtail-adapter-grok/tests/acp/cases/structured.rs` — FixtureHost
  wait_until returns DeadlineObservation immediately for Scenario::Deadline
  (fixture clock).

- `swallowtail-adapter-grok/tests/installed_probe.rs` — FakeProcessService
  completed/held_open with ImmediateTime fixture clock for the TimedOut case
  and explicit Dis.

- `swallowtail-adapter-kimi/tests/headless_structured_run.rs` —
  FakeProcessService fixture process control scripts every run.

- `swallowtail-adapter-kimi/tests/local_server_interactive/failures.rs` — Turn
  deadline armed by host.set_now fixture clock before awaiting TimedOut.

- `swallowtail-adapter-kimi/tests/local_server_structured_run.rs` — Fixture
  servers order everything through gates.

- `swallowtail-adapter-kimi/tests/provider_session_acceptance.rs` — Cancellation
  is gated by a wait-until-seen spin for session/list.

- `swallowtail-adapter-mistral-vibe/tests/headless_suite.rs` — Scripted fixture
  host with ImmediateTime fixture clock for timeout and held_open + explicit
  cancellation.

- `swallowtail-adapter-oh-my-pi/tests/driver/lifecycle.rs` — cancellation is
  explicit on a default (never-firing) parked clock.

- `swallowtail-adapter-oh-my-pi/tests/prepared_facade/catalogue.rs` — failure
  scenarios scripted by the fake process.

- `swallowtail-adapter-oh-my-pi/tests/structured_run.rs` — condvar-gated fake
  process scripts all frames.

- `swallowtail-adapter-ollama/tests/attached_driver/failures.rs` — 100×1ms polls
  only wait for inference attempts guaranteed by the task the test itself
  launched (hang guard).

- `swallowtail-adapter-openai/tests/direct_driver.rs` — block_on executor.


- `swallowtail-adapter-openai/tests/prepared_facade.rs` — Same fixture
  machinery: Pending timers never fire, the Delayed deadline leg is sole
  decider over HoldForCancel.

- `swallowtail-adapter-openai/tests/realtime_driver.rs` — wait_for_frames(4) is
  a 2s hang-guard poll for frames the driver itself already sent (guaranteed
  event).

- `swallowtail-adapter-opencode/tests/http_driver/fixture_join.rs` — The
  20ms/50ms sleeps are settle guards before drop(server), whose join
  synchronizes: the PanicOnEvent panic is.

- `swallowtail-adapter-openhands/tests/driver_suite.rs` — Scripted event corpus
  and fixture host.

- `swallowtail-adapter-pi/tests/driver/lifecycle.rs` — explicit cancellation on
  a default never-firing parked clock; timeout case uses with_immediate_time.

- `swallowtail-adapter-pi/tests/prepared_facade/catalogue.rs` — scripted failure
  scenarios; timeout case is Hold + with_immediate_time fixture clock.

- `swallowtail-adapter-pi/tests/sidecar_driver/lifecycle.rs` — verified repair:
  Hold pins the fake process until explicit release_hold(), deadline fired by
  fire_all_deadline.

- `swallowtail-adapter-pi/tests/structured_run.rs` — fixture-clock deadline
  (with_immediate_time) on Scenario::Hold.

- `swallowtail-adapter-qoder/tests/headless_suite.rs` — Scripted FixtureHost
  controls the process (held_open/scripted/with_exit), the timeout test
  injects the Immedia.

- `swallowtail-adapter-qwen/tests/conformance.rs` — All evidence flows through
  helpers using FakeProcessService with Pending/ImmediateTimeService fixture
  clocks.

- `swallowtail-adapter-qwen/tests/driver.rs` — Fake process control throughout:
  PendingTimeService keeps deadline paths inert, the timeout test uses
  Immediat.

- `swallowtail-host-local/tests/hosted_services.rs` — In-process
  authorize/acquire/release sequenced by block_on.

- `swallowtail-host-local/tests/local_composition.rs` — Every assert follows
  explicit ordering: block_on(task.join()) before completion asserts,
  cancellation.request(.

- `swallowtail-host-local/tests/local_process/attestation.rs` — Bounded polls
  (2-3s, generous) sample kernel-guaranteed transitions.

- `swallowtail-host-local/tests/local_process/descendant_tree.rs` — Markers
  awaited via 8s polls for fixture-guaranteed writes, stops/joins explicit
  (force_stop + wait).

- `swallowtail-host-local/tests/owned_serving_services.rs` — Descriptor/digest
  checks and lease release are synchronous in-process operations.

- `swallowtail-host-local/tests/watcher_service/feed.rs` —
  drain_until_terminal/count bounds (2s/200ms) are hang guards over events
  already guaranteed.

- `swallowtail-host-local/tests/watcher_service/wait.rs` — Verified post-PR-227
  state: cancellation test resolves via the ImmediateCancellation gate
  (explicit request be.

#### Support fixtures, ordering-safe (21)

- `swallowtail-adapter-alibaba-model-studio/tests/support/driver/server.rs` —
  single server thread answers each request synchronously from frozen fixture
  bodies and wait_for_cancel only ke.

- `swallowtail-adapter-alibaba-model-studio/tests/support/driver/services.rs` —
  wait_until sleeps exactly the test-armed remaining ms then reports the
  observation (never fires early).

- `swallowtail-adapter-anthropic/tests/support/server/managed/support.rs` —
  respond_managed_wait's keepalive loop only holds the managed stream open
  until client disconnect.

- `swallowtail-adapter-anthropic/tests/support/server/responses.rs` — responds
  synchronously per request from frozen SSE bodies.

- `swallowtail-adapter-anthropic/tests/support/services.rs` — wait_until sleeps
  exactly the test-armed remaining ms and never fires early, and spawn/run
  results cross threa.

- `swallowtail-adapter-claude-agent/tests/sdk_support/host.rs` — No own tests.


- `swallowtail-adapter-claude-agent/tests/sdk_support/host/task_time.rs` — No
  own tests.

- `swallowtail-adapter-deepseek/tests/support/server.rs` — server thread replies
  synchronously per request with attempt-indexed frozen bodies and
  wait_for_stop only keep.

- `swallowtail-adapter-deepseek/tests/support/services.rs` — wait_until sleeps
  exactly the test-armed remaining ms before reporting (never early) and
  blocking results cros.

- `swallowtail-adapter-gemini/tests/live_support/services.rs` — Delayed fixture
  timer thread always fires after a fixed 20ms sleep and its only arming user
  is the never-compl.

- `swallowtail-adapter-kimi/tests/local_server_interactive_support/server.rs` —
  1ms accept poll and bounded request peek only serve requests the driver
  itself initiates, and each request is .

-
  `swallowtail-adapter-kimi/tests/local_server_interactive_support/server/websocket.rs`
  — Scenario flows are scripted with protocol-ordered blocking reads.

- `swallowtail-adapter-kimi/tests/local_server_lifecycle_support/server.rs` —
  verified repair: every condvar wait is wait_timeout_while bounded by the
  120s HANG_GUARD with a loud assert on.

- `swallowtail-adapter-oh-my-pi/tests/support/host.rs` — all process I/O gated
  on Mutex+Condvar with notify after each scripted response and time is a
  parked fixture c.

- `swallowtail-adapter-ollama/tests/support/server.rs` — responses are
  synchronous per request and WaitForCancel merely holds the stream (1ms space
  trickle, ~2s self-e.

- `swallowtail-adapter-openai/tests/support/server.rs` — Responses are pure
  functions of (mode, request).

- `swallowtail-adapter-opencode/tests/http_support/mod.rs` — Hold fixtures
  (WaitForAbort/InputCallbacks 1ms poll loops) only withhold completion until
  explicit abort/reply.

- `swallowtail-adapter-opencode/tests/http_support/responses.rs` — Pure canned
  writes.

- `swallowtail-adapter-pi/tests/support/host.rs` — condvar-gated scripted
  process, parked fixture clock (fire_through/waiters), canned wait() exit.

- `swallowtail-adapter-pi/tests/support/sidecar_host.rs` — verified:
  fire_all_deadlines explicitly wakes parked-clock waiters.

- `swallowtail-host-local/tests/local_process/support.rs` — No assertions of its
  own (process_fixture returns unless CHILD_MODE is set).
