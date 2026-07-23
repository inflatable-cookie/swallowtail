# 109 Ollama Portability Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../038-ollama-native-attached-runtime-proof.md`

## Objective

Prove the production Ollama route under both execution-host identities and
close the first maintained-range proof.

## Scope

- unchanged attached-self-hosted profile under local and remote-authoritative
  hosts
- additive attached-runtime version, catalogue, and residency assertions
- baseline, latest, intermediate, prerelease, stale, and out-of-range coverage
- independent-run behavior, provider failure, malformed NDJSON, disconnect,
  cancellation, deadline, timer, redaction, and cleanup-failure matrix
- installed-runtime probe remains ignored and separately gated
- full repository QA, doctor delta, roadmap currentness, and closeout log
- compile the next compatibility or provider checkpoint only after evidence

## Acceptance Criteria

- [x] both topologies prove exact instance, route, model, version, and residency
- [x] the maintained range is no wider than the frozen corpus
- [x] the common profile gains no Ollama branch
- [x] no runtime, model, artifact, cloud, credential, or container authority
      appears
- [x] the proof adds no cross-run serialization absent an explicit contract
- [x] full validation passes apart from the documented doctor baseline
- [x] one sole next task remains

## Validation

- focused driver and conformance tests
- workspace all-target check
- workspace warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

No. Return to a coverage and installed-harness compatibility checkpoint.

## Evidence

- The additive attached-runtime assertion pack passes without an Ollama branch.
- Production fixtures cover both host identities, independent runs, provider
  failure, malformed NDJSON, disconnect, cancellation, deadline, timer,
  redaction, and visible join-cleanup failure.
- The maintained claim remains exactly `0.14.0` through `0.32.1`, with
  `0.18.0` and `0.30.0` intermediate qualification points and prereleases
  excluded.
- Full repository QA passes with 522 tests: 518 runnable tests pass and four
  installed/live probes remain ignored.
- Doctor returns to the inherited 19 findings: seven errors and twelve
  warnings. The Ollama files add no structural finding.
- `git diff --check` passes.
