# 139 Claude Registered-Tool Fixture Startup Determinism

Status: complete; PR 287; courier startup is an observed event with named causes; 24 loaded runs clean; the operation-bridge close bound is disclosed, not fixed
Owner: Tom
Created: 2026-09-08
Updated: 2026-09-08
Milestone: `../031-ci-latency.md`
Depends on: the 2026-09-08 papercut "Claude registered-tool close/join fixture flakes in the process-spawning shard"; cards 093/094/104 method

## Why now

PR 285 run `34169972936` failed
`claude_agent_sdk_driver::registered_tool_route::close_joins_the_registered_listener`
at session open with `fixture.claude_agent_sdk.failed`, and the same adapter
sources passed on the branch's earlier and later heads. The coordinator
correctly recorded that a green rerun is weak evidence and did not claim a
root cause.

That matters more than an ordinary flake: this is the close-and-join path the
pending Claude registered-tool live gate (card 132) exercises for real. If it
is nondeterministic here, a Desktop live failure will be ambiguous between a
route defect and our own fixture, which is exactly the ambiguity that cost
days on the earlier Bovine probes.

## Scope

1. Reproduce under load with the card 093/094 method: the isolated
   process-spawning selector, CPU burners, concurrent binaries, enough runs to
   see it.
2. Root-cause the setup failure. `fixture.claude_agent_sdk.failed` at session
   open is currently opaque: retain and surface the fixture's bounded process
   output and exit evidence on setup failure so the next occurrence names its
   own cause instead of a generic code.
3. Make sidecar startup and listener readiness observable through deterministic
   fixture signals rather than timing: the fixture should wait on an explicit
   readiness event, and the close/join case should be provable repeatedly
   without a rerun.
4. Prove it: 20+ loaded runs of the affected binary, zero failures.

## Out Of Scope

Production adapter source unless a real defect is found and disclosed; the
live gate itself; other crates.

## Acceptance Criteria

- [ ] the failure is reproduced under load, or its impossibility explained with anchors
- [ ] setup failure surfaces bounded process output and exit evidence, not a bare code
- [ ] startup and listener readiness are deterministic signals, not timing
- [ ] 20+ loaded runs clean; the papercut entry is retired

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a fixture failure names its own cause, and the close/join case
passes deterministically. Smallest counterexample: a setup failure reported as
`fixture.claude_agent_sdk.failed` with nothing else.

## Auto-Continuation

No. Stop for exact-head review.

## Result

### The failure, reproduced and named

At session open the fixture has exactly one path to
`fixture.claude_agent_sdk.failed`: the provider-side courier spawn in
`sdk_support/host/mcp_child.rs`. The `open` responder in
`sdk_support/host/script.rs` cannot fail, and the other error arms in
`sdk_support/host/process.rs` require a malformed wire line, which would fail
every run rather than one. That spawn mapped every `io::Error` to `()`.

The mechanism is Cargo's uplift, and it is measured rather than inferred.
Cargo's build lock does not cover the window after a build returns, and the
uplift is not atomic: a sibling test process rebuilding into the same nested
target removes and recreates
`target/card125-courier/debug/swallowtail-registered-tool-courier`. A probe
holding that path open across eight rebuilds recorded 42 `ENOENT`
observations. Every one of the 117 test processes in this binary runs that
nested build, so a spawn of the same path lands in that window, returns
`ENOENT`, and — under the old helper — became a bare
`fixture.claude_agent_sdk.failed` at session open. That is PR 285's exact
symptom, and nothing in the adapter or the route is involved.

A second startup death appeared during the loaded reproduction: run 2 of 22
under the isolated process-spawning selector with sixteen CPU burners failed
`ready_follows_kernel_authenticated_connect` with the courier
`exited signal: 9 (SIGKILL)` before claiming its rendezvous. What is proved
there is post-spawn death, not who sent the signal; a concurrent build
replacing a mapped executable is the plausible reading, and it stays labelled
as a reading. It is a distinct failure mode from the one above — the old code
would have reported it as the proxy's own `proxy_not_ready` after a ten-second
wait, not as the fixture code PR 285 saw. Both modes end at the same fix.

The fix is to stop spawning the volatile path at all. `courier_binary()`
acquires the completed artifact with a bounded build-and-read retry, because
the artifact is transiently absent rather than permanently wrong, and
republishes it under a content-addressed path installed by hard link, which
fails rather than replaces when the name exists. No builder ever touches the
file that is spawned.

### Evidence, not codes

- The spawn failure carries the command, its arguments, the operating-system
  error with its raw code and kind.
- Child stderr is retained under a 4 KiB bound instead of being discarded, and
  the observed exit — code or signal — is reported with it.
- `SdkFixtureHost::courier_evidence()` exposes that record for live and reaped
  children, and `open_route` prints it when open fails.
- The nested courier build reports its own exit status and bounded output
  instead of a bare assertion, and the built binary is asserted executable.
- `a_courier_that_cannot_exec_names_its_own_cause` holds the invariant: a
  courier that exists but cannot be executed fails as
  `fixture.claude_agent_sdk.registered_courier_startup_failed` naming the
  command and `os error 13`.

### Startup as an event

The fixture now answers `open` only after the courier has claimed and unlinked
its one-shot rendezvous, which is the courier's own first act before it
connects. The wait ends early on child exit and reports that exit with the
child's output; its 30-second bound is a broken-contract guard, not a timing
assumption. `ready_follows_kernel_authenticated_connect` asserts the child is
live at that point. The fixture also tears its stdio children down when the
`close` command reaches the wire, as the real SDK does.

### Disclosed, not fixed

Closing a registered-tool route measured 5.00s on every registered case here.
It is `OperationBridgeListener::close` joining an accepted connection thread
that waits out `IO_TIMEOUT` without waking its read; narrowing that constant
to two seconds moved the close to 2.00s. Guardian cleanup closes the
registered lease before the provider close reaches the wire, so this card's
close-command child teardown lands too late to shorten it — it is not a
fixture effect. The bound is the timeout, not a guaranteed constant. It is in
`swallowtail-host-local`, which this card's manifest forbids, so it is
recorded in `PAPERCUTS.md` and left for Chatterbox. It bounds close latency;
it does not make the case flaky.

### Review

Cross-model review at the first exact head returned changes required, and it
was right on every count. Three defects are fixed here: the artifact was still
read from the volatile path without retry, so publication inherited the race
it was meant to remove; the new regression test used one machine-wide
temporary filename, which is the shared mutable state this card exists to
remove rather than add; and the stderr reader was detached, so evidence could
render `stderr empty` before a dying child's output had drained. The causal
claim above was rewritten because the original overstated SIGKILL as proof of
the PR 285 chain. Evidence retention is now bounded per note, and a malformed
declaration is named by its shape rather than echoing declared `env` values.

### Proof

- 24 runs of the isolated process-spawning selector under sixteen CPU burners,
  117 tests each, zero failures, at the first head.
- 32 concurrent instances of the two scratch-executable tests: all pass. The
  same probe failed 24 of 32 before the per-invocation directory fix.
- 16 concurrent test processes against continuous courier rebuild churn: all
  pass, exercising the acquisition retry and the hard-linked publish.
- `cargo fmt`, `validate:focused`, `package:verify-affected`, and
  `qa:northstar` are clean.
