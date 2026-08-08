# Shared Discovery Debug Emissions

Date: 2026-08-08
Roadmap: g03.056
Card: 172

## Outcome

Shared installed discovery and plan-family readiness emit failure-path debug
observations. `HostServices::emit_failure_debug` and
`failure_debug_observation` land the helper shape; probe outcomes emit
`HostProcess` / `InterfaceVersion` / `Cleanup`; missing host services emit
`Lifecycle`.

## Validation

- focused runtime: 167 passed
- affected package proof and regenerated v0.3.0 API baseline passed
