# 069 Selected Route Driver And Conformance

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../020-post-portability-coverage-expansion.md`

## Objective

Implement the Bedrock Runtime `ConverseStream` route against Contract 019 and
card 068's typed SDK corpus, then prove its hosted-direct profile and close
roadmap 020.

## Scope

- one distinct in-process Rust SDK structured-run driver
- exact host-approved endpoint, region, delegated AWS credential provider,
  model route, positive output-token bound, and one-attempt configuration
- explicit registration, access, capability, topology, and lifecycle mapping
- deterministic local and remote-authoritative hosted-direct conformance
- typed EventStream order, cancellation, deadline, disconnect, SDK drift,
  redaction, credential release, private Tokio lifecycle, and joined cleanup
- optional installed or live probe only under explicit operator gating
- roadmap and front-door closeout

## Acceptance Criteria

- [x] no provider-specific behavior leaks into core, runtime, or shared
      protocol code
- [x] the driver claims only fixture-proven lifecycle and capabilities
- [x] local and remote authority remain distinct
- [x] default QA requires no live provider credential or paid inference
- [x] full QA passes or failures are recorded honestly

## Evidence

- the production descriptor claims only hosted-direct structured runs with
  task, blocking-work, time, network, and delegated-credential services
- one exact binding fixes instance, access profile, credential reference,
  execution host, region, and SDK credential provider before provider work
- the SDK config uses the host-approved endpoint and exact route/model, disables
  SDK retries with maximum attempts one, and reads no ambient AWS configuration
- the operation-private Tokio runtime lives inside joined host blocking work;
  cancellation and deadline wake the SDK future before credential release
- deterministic fake-SDK runs prove local and remote-authoritative topology,
  ordered output and usage, binding rejection, cancellation, deadline, and
  joined cleanup
- generated SDK fixtures continue to prove failure projection, semantic drift,
  redaction, and typed EventStream order without AWS access

## Validation Result

- 11 focused Bedrock tests pass
- focused warnings-denied clippy passes
- full `effigy qa` passes with 283 tests; three installed or live probes remain
  gated and ignored by default
- `git diff --check` passes
- `effigy doctor` retains the inherited 19 findings: 12 warnings and 7 errors

## Validation

- focused adapter and conformance tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- card 068 is incomplete
- implementation exposes a missing shared contract
- provider behavior has drifted from the frozen corpus

## Auto-Continuation

No. Return to the roadmap 020 planning checkpoint.
