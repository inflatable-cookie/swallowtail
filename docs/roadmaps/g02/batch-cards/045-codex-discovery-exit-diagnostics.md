# 045 Codex Discovery Exit Diagnostics

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../014-consumer-scale-application-proof-and-hardening.md`
Contract ref: `../../../contracts/032-installed-executable-observation-and-discovery.md`

## Objective

Make non-zero Codex installed-version probes actionable without hardcoding
wrapper rejection or exposing raw process output.

## Scope

1. Capture bounded stderr independently from the exact stdout version bound.
2. Retain numeric exit status when the host supplies it.
3. Produce one bounded sanitized stderr excerpt for non-zero exit diagnostics.
4. Keep `swallowtail.codex.discovery_exit_failed` stable.
5. Cover status-only, detailed, redacted, and truncated non-zero exits.

## Acceptance Criteria

- [x] wrapper and direct executable targets use the same discovery path
- [x] non-zero exits expose status and safe stderr where available
- [x] path-, credential-, control-, and oversized material stays out
- [x] cleanup and preparation-stage classification remain unchanged
- [x] focused adapter tests and repository checks pass

## Stop Conditions

- the change requires executable-selection or wrapper policy
- raw stderr would enter a stable diagnostic
- the stable diagnostic code or failure stage would change

## Evidence

- the probe captures at most 1 KiB of stderr independently from exact-version
  stdout
- diagnostics retain numeric status and expose at most 240 sanitized characters
  plus an explicit truncation marker
- path, assignment, credential, email-like, long-token, ANSI, and control
  material is redacted or removed
- direct executables and wrappers retain the same host-selected discovery path
- all 93 Codex adapter tests pass, including non-zero status, redaction, and
  capture-bound coverage
- full `effigy qa` passes across all 22 production routes and the workspace
