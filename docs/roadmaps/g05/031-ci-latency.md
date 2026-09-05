# g05.031 CI Latency

Status: planned; card 095 is planned behind the `v0.4.1` tag; operator workflow authority granted 2026-09-05
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

- [095 CI Gate Restructure](batch-cards/095-ci-gate-restructure.md) — planned; after the `v0.4.1` tag

## Dispatch Manifest

Published when card 095 becomes ready after the tag.

## Acceptance

- [ ] a typical pull request reaches all-green in about five minutes
- [ ] `main` pushes and release-candidate dispatches still run the full
      macOS pinned floor
- [ ] every test that ran before still runs somewhere on every PR
- [ ] the required-check set for merge is documented and unchanged in
      strength
