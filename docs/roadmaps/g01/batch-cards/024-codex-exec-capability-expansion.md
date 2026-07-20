# Codex Exec Capability Expansion

Status: completed
Owner: Tom
Roadmap: 007 Soundcheck Structured-Run Readiness
Updated: 2026-07-19

## Goal

Expand the Codex exec driver to the structured inputs proven necessary for the
Soundcheck transport seam.

## Scope

- accepted JSON Schema transport
- optional image attachment
- explicit reasoning and search/network policy
- host-enforced deadline and cancellation cleanup
- richer safe model metadata through the existing catalog role
- JSONL progress and terminal-output parity

## Out Of Scope

- Soundcheck product orchestration
- automatic validation or repair
- unrestricted sandbox or implicit web access
- arbitrary attachment kinds

## Acceptance Criteria

- exact options are capability- and preflight-bound
- provider argv uses only host-materialized references
- cancellation and timeout kill and join the owned process
- unsupported input combinations fail before process start
- the one-shot profile retains every common assertion

## Validation

- scripted process fixtures for each claimed option and failure path
- one-shot Contract 011 profile
- `effigy qa`
- dependency and redaction audit

## Stop Condition

Stop if current Codex CLI evidence contradicts the promoted contract or needs
authority not granted by the selected host services.

## Closeout

- Codex exec now accepts one bounded image, one JSON Schema output contract,
  exact reasoning selection, explicit external-search policy, and an optional
  deadline only when the selected preflight plan binds the same capabilities
  and required host services
- every provider-visible image and schema path comes from a scoped host lease;
  drivers never accept a consumer path
- invocation ignores ambient user configuration and rules, permits an approved
  non-Git resource, denies approvals, prevents tool subprocess environment
  inheritance, and passes explicit read-only sandbox and web-search settings
  while retaining the selected authenticated Codex installation
- timeout and operator cancellation remain distinct; both force-stop and join
  the owned process, and every terminal path awaits materialization cleanup
- app-server model discovery preserves current supported reasoning modes and
  the provider default as catalog evidence
- deterministic fixtures cover exact argv, early rejection, missing execution
  services, timeout, deadline-wait cancellation, cleanup, and the unchanged
  one-shot conformance profile
