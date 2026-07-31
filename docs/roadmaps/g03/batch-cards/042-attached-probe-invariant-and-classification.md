# 042 Attached Probe Invariant And Classification

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../016-attached-harness-probe-compatibility-truth.md`
Depends on: card 041

## Goal

Remove OpenCode's obsolete exact live-probe pin and freeze the production
compatibility posture in deterministic health-response tests.

## Scope

1. Confirm no other attached-runtime probe repeats the stale hard-pin pattern.
2. Parse OpenCode health version evidence through the bounded public binding.
3. Assess the observation through `opencode_http_claim`.
4. Accept qualified and visible unverified-newer stable observations only.
5. Cover current, newer, incompatible, malformed, and unhealthy cases.
6. Retain the existing OpenAPI and selected-path checks.

## Acceptance Criteria

- [x] exact `1.18.10` is qualified
- [x] exact `1.18.11` is accepted as unverified newer
- [x] `1.14.47`, unpublished gaps, prereleases, and malformed values fail
- [x] `healthy: false` fails before schema acceptance
- [x] the optional live selector no longer requires exact `1.14.48`
- [x] no provider or consumer effect runs
- [x] card 043 becomes the sole ready and next task

## Validation

- `cargo test -p swallowtail-adapter-opencode --features live-probes --test installed_probe`
- `git diff --check`
- no ignored live test or broad workspace suite

## Auto-Continuation

Yes. Continue to card 043 after deterministic classification passes.

## Evidence

- Ollama already classified attached runtime evidence through its public claim;
  no second hard-pin repair was needed
- OpenCode health classification now uses its bounded binding and production
  claim
- gated-target validation passed four deterministic tests; the network test
  remained ignored
