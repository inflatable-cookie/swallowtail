# g05.031 CI Latency

Status: ready; `v0.4.1` is tagged; card 095 is ready; operator workflow authority granted 2026-09-05
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

1. Card 095 restructures CI: split the stable job into parallel jobs sharing
   the rust-cache; make the pinned MSRV job clippy-only on pull requests and
   full-test on `main` pushes and workflow-dispatch (release candidates);
   shard nextest across runners; isolate the process-spawning sidecar suites
   into their own shard; move every check that is not Apple Silicon
   verified-target evidence to Linux runners.

## Boundary

The release floor keeps its full macOS pinned clippy-plus-test run on
`main` pushes and release-candidate dispatches. Required-check names used by
the merge gate change only with the coordinator's agreement in the same
card. No test is removed or weakened; no gate command in
`config/release.toml` changes.

## Batch Cards

- [095 CI Gate Restructure](batch-cards/095-ci-gate-restructure.md) — ready

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this table.

| Field | Card 095 |
| --- | --- |
| Readiness | ready |
| Prerequisites | `v0.4.1` tagged at `c3cce750`; operator workflow authority of 2026-09-05 |
| Completion conditions | parallel stable jobs; nextest sharded with the process-spawning suites isolated; MSRV clippy-only on pull requests with the full pinned run on `main` pushes and workflow-dispatch; non-target checks on Linux; every prior step still runs on every PR; before-and-after timings recorded; required-check names documented and agreed with the coordinator before any branch-protection change |
| Owned mutable paths | `.github/workflows/ci.yml`; `.config/nextest.toml`; this card's `## Result`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md` |
| Forbidden paths | every `crates/**` path; `config/release.toml` and `scripts/`; contracts; test content |
| Approved concurrent siblings | card 094's remaining sweep (PR 227 and follow-ups) |
| Serial edges | none |
| Worker capability class | CI and workflow engineer with GitHub Actions and nextest experience; no credentials beyond the repo workflow |
| Acceptance evidence | the card's own PR runs green under the new layout; timing comparison on one representative PR |
| Review oracle | no step removed; release floor intact on `main` pushes and dispatches |
| Stop conditions | a required check cannot be preserved in strength; branch protection needs a change the coordinator does not agree to |
| Escalation owner | operator via Chatterbox; coordinator for mechanical blockers |

## Acceptance

- [ ] a typical pull request reaches all-green in about five minutes
- [ ] `main` pushes and release-candidate dispatches still run the full
      macOS pinned floor
- [ ] every test that ran before still runs somewhere on every PR
- [ ] the required-check set for merge is documented and unchanged in
      strength
