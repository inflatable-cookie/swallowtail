# 111 Installed Executable Observation Records

Status: ready
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Add the narrow provider-neutral observation boundary needed to prove an exact
host-approved installed executable before configuration and preflight.

## Scope

- promote the installed-executable discovery rules from Research 025 into a
  new durable contract
- explicit opaque executable candidate supplied by the selected execution host
- exact interface-version observations and compatibility classification
- bounded deadline, cancellation, process completion, and joined cleanup
- safe absent, malformed, incompatible, timeout, cancellation, and cleanup
  outcomes
- local and remote-authoritative host identity preservation
- provider-neutral testkit assertions
- no Codex parser, adapter claim, ambient executable search, installation,
  update, authentication, or configured-instance promotion

## Acceptance Criteria

- [ ] discovery probes only the explicit host-approved candidate
- [ ] exact observations remain separate from maintained windows
- [ ] safe results carry no executable path, raw stdout, environment, or token
- [ ] discovery cannot create a configured instance or authorize execution
- [ ] remote-authoritative discovery runs on the authoritative host
- [ ] all work is deadline-aware and joined
- [ ] existing discovery drivers and profiles retain their behavior

## Validation

- focused core, runtime, testkit, and local-host tests
- focused warnings-denied clippy
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

Yes. Continue to card 112 when the shared observation boundary passes.
