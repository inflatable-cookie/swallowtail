# 139 Claude Registered-Tool Fixture Startup Determinism

Status: ready
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
