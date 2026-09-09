# g05.031 CI Latency

Status: ready; `v0.4.1` is tagged; card 095 work complete (PR 230 merged as `ba8275eb`); operator workflow authority granted 2026-09-05
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Depends on: Contract 036 (workflow edits need explicit operator request; granted); the `v0.4.1` tag
Vision tags: delivery, validation, CI

## Purpose

Cut the pull-request gate from ten to fifteen minutes to four or five
without weakening the release floor. Today both slow jobs run on
`macos-latest`: the stable job runs format, two full clippy passes, the whole
nextest suite, examples, metadata, and route guides serially, and the pinned
MSRV job repeats a full clippy and a full `cargo test`. Everything else
finishes in three minutes.

## Operator Authority

Contract 036 keeps workflow edits out of ordinary lanes. On 2026-09-05 the
operator explicitly requested this optimisation ("All sounds good, go for
it"). That grant covers `.github/workflows/ci.yml` and any nextest
configuration it needs, and nothing else.

## Runway

1. Card 095 restructured CI: split the stable job into parallel jobs sharing
   the rust-cache; made the pinned MSRV job clippy-only on pull requests and
   full-test on `main` pushes and workflow-dispatch (release candidates);
   sharded nextest across runners; isolated the process-spawning sidecar suites
   into their own shard; moved every check that is not Apple Silicon
   verified-target evidence to Linux runners.

Folded evidence (g05.038): card 095 work complete through PR 230 merged as `ba8275eb`.

## Boundary

The release floor keeps its full macOS pinned clippy-plus-test run on
`main` pushes and release-candidate dispatches. Required-check names used by
the merge gate change only with the coordinator's agreement. No test is
removed or weakened; no gate command in `config/release.toml` changes.

## Folded card evidence (g05.038)

Card 095 completed against `v0.4.1` tagged at `c3cce750` under the operator
workflow authority of 2026-09-05, through PR 230 merged as `ba8275eb`.
Delivered: parallel stable jobs sharing the rust-cache; nextest sharded with
the process-spawning suites isolated; MSRV clippy-only on pull requests with
the full pinned run on `main` pushes and workflow-dispatch; non-target checks
on Linux; every prior step still running on every PR; before-and-after timings
recorded on one representative PR; required-check names documented and agreed
with the coordinator before any branch-protection change. Card 094's remaining
sweep (PR 227 and follow-ups) ran as the approved concurrent sibling. No step
removed; release floor intact on `main` pushes and dispatches.

## Acceptance

- [ ] a typical pull request reaches all-green in about five minutes
- [ ] `main` pushes and release-candidate dispatches still run the full
      macOS pinned floor
- [ ] every test that ran before still runs somewhere on every PR
- [ ] the required-check set for merge is documented and unchanged in
      strength

### Card 104 held gate (task-owned)

Card 104 stopped as a task-owned held gate; it was ready concurrent with the
g05.029 runway. Recorded basis: `v0.4.2` tagged with cards 094 and 103 merged.
Scope: four surfaces classified and fixed at the class; 20+ loaded runs per
binary clean; card 094's list annotated; loaded-run logs and classification
table with anchors as evidence. `swallowtail-adapter-claude-agent` stays with
the g05.029 runway; every other crate, baselines, and contracts stay out. A
real leak needing a shared runtime change stops the gate.

### Card 139 held gate (task-owned)

Card 139 stopped as a task-owned held gate. Recorded basis: the 2026-09-08
papercut on current `main`. Scope: reproduction under load or an anchored
explanation; bounded process output and exit evidence on fixture setup failure;
deterministic startup and listener readiness signals; 20+ loaded runs clean;
papercut retired; the loaded-run log naming the failure's own cause as
evidence. Every other crate, baselines, contracts, and the live gate stay out.
A cause in shared runtime stops the gate.

### Card 141 completed (folded)

Card 141 completed through PR 292 merged at
`b68a1ccc757eb6adf768fb7ae3d330c5daef1547`, on card 139 merged and current
`main`. Delivered: close no longer pays a fixed `IO_TIMEOUT`;
before/after measurements recorded; watcher behaviour unchanged; every teardown
guarantee preserved; papercut retired. Evidence: before/after close
measurements on registered and watcher cases.
