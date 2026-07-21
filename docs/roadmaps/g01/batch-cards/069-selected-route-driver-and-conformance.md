# 069 Selected Route Driver And Conformance

Status: blocked
Owner: Tom
Updated: 2026-07-21
Milestone: `../020-post-portability-coverage-expansion.md`

## Objective

Implement card 067's selected route against card 068's promoted boundary, then
prove its claimed provider-neutral profiles and close roadmap 020.

## Scope

- one distinct production adapter driver
- explicit registration, access, capability, topology, and lifecycle mapping
- deterministic local and remote-authoritative conformance where applicable
- cancellation, disconnect, drift, redaction, and joined cleanup
- optional installed or live probe only under explicit operator gating
- roadmap and front-door closeout

## Acceptance Criteria

- [ ] no provider-specific behavior leaks into core, runtime, or shared
      protocol code
- [ ] the driver claims only fixture-proven lifecycle and capabilities
- [ ] local and remote authority remain distinct
- [ ] default QA requires no live provider credential or paid inference
- [ ] full QA passes or failures are recorded honestly

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
