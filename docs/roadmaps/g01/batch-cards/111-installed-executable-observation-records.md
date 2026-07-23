# 111 Installed Executable Observation Records

Status: completed
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

- [x] discovery probes only the explicit host-approved candidate
- [x] exact observations remain separate from maintained windows
- [x] safe results carry no executable path, raw stdout, environment, or token
- [x] discovery cannot create a configured instance or authorize execution
- [x] remote-authoritative discovery runs on the authoritative host
- [x] all work is deadline-aware and joined
- [x] existing discovery drivers and profiles retain their behavior

## Validation

- focused core, runtime, testkit, and local-host tests
- focused warnings-denied clippy
- `effigy doctor` delta review
- `git diff --check`

## Evidence

- Contract 032 fixes one request, scope, authoritative host, opaque executable
  target, exact version axis, deadline, cancellation signal, classified safe
  observation, and joined process lifecycle.
- core classifies one exact version against one claim without carrying the
  executable reference or raw process material.
- runtime adds an additive installed-target discovery method. Existing general
  discovery remains unchanged and unsupported drivers fail explicitly.
- target-aware requests require task, time, and process services from the
  matching execution host.
- testkit assertions cover local and remote-authoritative topology, compatible
  and incompatible exact points, safe terminal-state separation, redaction,
  and process join.
- the local host proves only an explicitly approved target starts; an
  unapproved reference fails without ambient search.
- 165 focused core, runtime, testkit, and local-host tests pass. Focused
  warnings-denied clippy and `git diff --check` pass.

## Auto-Continuation

Yes. Continue to card 112 when the shared observation boundary passes.
