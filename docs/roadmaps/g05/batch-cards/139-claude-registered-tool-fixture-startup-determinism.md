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

- [x] the failure is reproduced under load, or its impossibility explained with anchors
- [x] setup failure surfaces bounded process output and exit evidence, not a bare code
- [x] startup and listener readiness are deterministic signals, not timing
- [x] 20+ loaded runs clean; the papercut entry is retired

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

The loaded reproduction ran the isolated process-spawning selector
(`--profile ci-process -E 'binary(claude_agent_sdk_driver)'`) with sixteen CPU
burners. Run 2 of 22 failed
`registered_tool_route::ready_follows_kernel_authenticated_connect`, and the
new evidence named the cause outright: the courier
`exited signal: 9 (SIGKILL)` before claiming its rendezvous. Each of the 117
test processes runs the nested courier build, and a concurrent nested build
that re-uplifts the shared `target/card125-courier/debug/` path kills a courier
already executing from it. That is the load-dependent, opaque setup failure
PR 285 hit; nothing in the adapter or the route is involved.

The fix is to stop spawning the volatile path. `courier_binary()` now
republishes the built binary under a content-addressed, write-once path that no
builder ever touches, staged and installed by atomic rename.

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

Closing a registered-tool route costs a flat 5.00s. It is
`OperationBridgeListener::close` joining an accepted connection thread that
waits out `IO_TIMEOUT`; narrowing that constant to two seconds moves the close
to a flat 2.00s. Killing the provider-side child first does not shorten it, so
it is not a fixture effect. It is in `swallowtail-host-local`, which this
card's manifest forbids, so it is recorded in `PAPERCUTS.md` and left for
Chatterbox. It bounds close latency; it does not make the case flaky.

### Proof

24 runs of the isolated process-spawning selector under sixteen CPU burners,
117 tests each, zero failures. `cargo fmt`, `validate:focused`,
`package:verify-affected`, and `qa:northstar` are clean.
