# Nucleus Native Pilot Handoff

Date: 2026-07-25

## Outcome

The Nucleus companion lane completed its deterministic readiness and exact
live-pilot handoff without an authenticated catalogue request or model turn.
Swallowtail card 041 is ready except for explicit live-effect approval.

The fixed runtime tuple is:

- Nucleus `2a6d72a8d3326cc70c6852f8fa86ff7f8ca995f2`
- Swallowtail `ea22603d5fc50545b0ef477187b3ab83a8ab785c`
- Codex `0.145.0`, qualified latest, app-server v2 workspace-roots behavior
- local macOS 26.5.2 arm64 host over stdio
- ChatGPT interactive OAuth and subscription allowance
- read-only session access
- exact `gpt-5.4-mini`, low reasoning, no fallback
- isolated empty state and filesystem-read-only disposable Git fixture

The host-approved Codex target hash is
`1da3f4e0e96028b8a771814293c3033dafd1971f943f6c7e79b0897fe705f590`.
The fixture commit is `04f7eb371e4e3ac0010a69d3f96052a7becbe43a`.

## Workload

The exact envelope remains 3 catalogue attempts, 12 planned turns, 3 reruns
maximum, 15 turn attempts maximum, 6 provider-thread lifecycles, 3 live
children maximum, serial turns, 2 restarts, and 60 minutes.

The attempts cover 6 ordinary successes, 3 read-only callback successes, 1
operator cancellation, 1 controlled deadline, and 1 post-restart recovery.
No workspace, task, SCM, forge, proposal, fixture, provider-account, push,
publication, tag, or release mutation is authorized.

## Validation

The Nucleus desktop Rust library passes 56 tests, desktop checking reports zero
errors, all 20 client tests pass, and both repositories pass documentation,
Northstar, and diff checks.

## Remaining Gate

The current handoff does not claim that this turn re-observed
`gpt-5.4-mini`. The first approved catalogue attempt must confirm the exact
route and unchanged ChatGPT audience. Absence or drift stops with zero turns.

Card 041 requires explicit approval of the ChatGPT-backed 15-turn and
60-minute envelope.
