# 093 Claude SDK Sidecar Fixture Determinism

Status: ready
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../030-v0-4-1-release-readiness.md`
Depends on: card 091's frozen-tree floor failure record; card 080's sidecar fixture on `main`

## Goal

Make the `claude-agent.sdk` sidecar-asset test fixture deterministic under
any host load, so the release floor gate cannot fail on timing. Release-lane
repair under the `v0.4.1` feature freeze, on the `v0.4.0` fixture-race
precedent (watcher path reuse, OpenCode deadline race).

## Defect

`crates/swallowtail-adapter-claude-agent/tests/sidecar_asset_support/mod.rs`
proves fake-SDK behaviour by polling `observations.json` every 20 ms until a
predicate holds or a 20-second `WIRE_BOUND` expires, and reads wire records
with `recv_timeout(WIRE_BOUND)`. Twelve tests each spawn a Node sidecar plus
a fake native child. Under a host load average of about 15, two tests
(`a_write_profile_restricts_availability_without_auto_allowing_anything`,
`a_tool_outside_the_read_only_set_is_denied_without_asking_the_host`) hit
the bound; the same tests pass on the same tree when the host is quiet. A
test whose outcome depends on machine load is a defect in the test.

## Scope

1. Replace wall-clock polling with ordered evidence. The fake SDK
   (`tests/sidecar_asset_support/fake-sdk.mjs`) must publish each
   observation before the event the test observes next: write and flush
   `observations.json` synchronously before returning the `init` system
   message (so `options` is present the moment `open` echoes), and before
   yielding the message that follows each `canUseTool` decision (so an
   admission is present the moment the corresponding callback or turn-end
   record reaches the host). The Rust fixture then reads observations after
   the wire record that guarantees them, with no retry loop.
2. Keep one generous hang guard only where a process could genuinely never
   answer (sidecar death). Make it explicit, large (minutes, not seconds),
   and named for what it is; it must never be the mechanism a passing test
   relies on.
3. Review every other `WIRE_BOUND` use in the fixture with the same rule:
   ordering first, hang guard second.
4. Prove determinism: run the sidecar-asset test binary at least 20 times in
   a loop while a deliberate CPU load runs on the host (for example a
   parallel `cargo build` of another crate or a `yes > /dev/null` per core),
   and record zero failures in the card result.
5. Keep the production sidecar (`sidecar/claude-agent-sdk-sidecar.mjs`) and
   every non-test Rust path unchanged. If the ordering guarantee genuinely
   requires a production change, stop and return to Chatterbox.

## Out Of Scope

Any production behaviour; any other adapter; the release candidate itself;
raising the bound as the fix.

## Acceptance Criteria

- [ ] no test in the sidecar-asset binary depends on a timing bound to pass
- [ ] the loop-under-load proof recorded zero failures
- [ ] production sidecar and adapter source are byte-identical
- [ ] focused validation green under both stable and pinned `1.95.0`

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `rustup run 1.95.0 cargo test -p swallowtail-adapter-claude-agent --all-features --locked`
- `effigy qa:northstar`
- `git diff --check`

## Result

Implemented the smallest test-only ordering repair. The fake SDK now writes
observations through a synchronous write plus fsyncSync before the initial
init and each turn's following wire event. The Rust fixture consumes the
turn_ended event before reading turn observations; the observation poll loop
is gone. next_record retains one named five-minute SIDECAR_DEATH_GUARD for a
genuinely dead or wedged sidecar.

Evidence:

- Stable focused sidecar-asset binary:
  cargo test -p swallowtail-adapter-claude-agent --test
  claude_agent_sdk_sidecar_asset --all-features --locked — 12 passed, 0
  failed.
- Under-load proof: the same binary ran 20 times with --test-threads=1 while
  18 yes >/dev/null CPU burners ran, one per sysctl -n hw.ncpu reported core.
  Result: 20 passed, 0 failed.
- effigy validate:focused swallowtail-adapter-claude-agent — 329 passed,
  0 skipped; focused package validation passed.
- effigy package:verify-affected swallowtail-adapter-claude-agent — affected
  package proof passed.
- rustup run 1.95.0 cargo test -p swallowtail-adapter-claude-agent
  --all-features --locked — passed; the sidecar-asset binary reported 12
  passed, 0 failed.
- cargo fmt -p swallowtail-adapter-claude-agent -- --check — passed.

The diff is confined to tests/sidecar_asset_support/fake-sdk.mjs,
tests/sidecar_asset_support/mod.rs, and this card result. Production sidecar
and adapter source remain unchanged.

## Review Oracle

Invariant: a passing sidecar-asset test never waits on the clock. Smallest
counterexample: a retained poll loop or a raised bound that a passing test
still depends on.

## Auto-Continuation

No. Stop for exact-head review; card 091 re-prepares on the merged base.
