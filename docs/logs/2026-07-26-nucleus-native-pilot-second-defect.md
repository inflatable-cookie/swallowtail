# Nucleus Native Pilot Second Defect

Date: 2026-07-26

## Outcome

Physical launch two used the rebuilt bundled Nucleus app and a fresh isolated
state root. Catalogue attempt two revalidated:

- Codex `0.145.0`;
- exact `gpt-5.4-mini`, low reasoning;
- unchanged ChatGPT subscription audience;
- local stdio topology; and
- the read-only disposable fixture.

The first ordinary turn failed before Codex `turn/start`:

`swallowtail.codex.app_server.preflight_mismatch`

Safe Nucleus evidence reports one failed turn, zero active turns, and zero
unexpected terminal classes. The provider session opened and joined when the
app stopped. No model turn, callback, fixture write, workspace write, or
retained Nucleus child occurred.

## Ownership

Swallowtail's prepared interactive-session facade planned task and process
services but omitted time. Nucleus's normal Agent Chat path correctly attached
its bounded turn deadline. The low-level Codex driver correctly rejected a
deadline service absent from immutable preflight.

This is a Swallowtail facade-composition defect. It requires no Nucleus prompt,
tool, persistence, authorization, or UI change.

## Repair

Commit `a26b54f0c264abf1712c94db442e9cb0b4078208`:

- binds task, time, and process services in prepared Codex interactive-session
  plans;
- retains unsupported session-open deadlines as a separate rejection;
- adds a prepared deadline-bound turn through the gated app-server fixture;
  and
- proves joined turn and session close.

All 90 Codex-adapter tests pass. Nucleus compiles against the repair and all 19
deterministic adapter tests pass; two authenticated probes remain gated.

## Envelope State

Consumed:

- 2 physical launches and catalogue attempts;
- 1 failed turn attempt;
- 1 of 3 failed-scenario reruns;
- 1 joined provider-thread lifecycle;
- zero provider model turns.

Recommended reset:

- permit 5 physical launches and 5 catalogue attempts total;
- retain both failed launches as defect evidence;
- execute 12 planned outcomes across 3 clean launches;
- keep 15 turn attempts maximum;
- keep 6 provider threads maximum, counting the failed joined session and
  using at most 5 further threads;
- keep 3 live children, serial turns, read-only effects, and 60 minutes of
  cumulative active execution; and
- exclude paused operator time without resetting accumulated execution time.

## Next

Approve or reject the second one-launch and one-catalogue reset. Card 041
remains active and paused.
