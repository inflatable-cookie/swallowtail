# Nucleus Topology And Lifecycle Proof

Status: completed
Owner: Tom
Roadmap: 008 Nucleus Interactive-Session Readiness
Updated: 2026-07-19

## Goal

Prove the expanded session runtime works through host-owned resource and process
ports in local and remote-authoritative topologies.

## Scope

- opaque project-resource mapping
- local and remote execution-host selection fixtures
- open/resume identity binding
- turn, callback, interruption, session close, and recovery boundaries
- cleanup and safe diagnostics

## Out Of Scope

- Nucleus server persistence
- raw path transport
- automatic session recovery policy
- product task transitions

## Acceptance Criteria

- the selected execution host owns binary, credential, resource, and process
  authority
- resume never substitutes a provider session or configured instance
- callback waits and active turns remain cancellable
- close joins all session work in both topology fixtures
- no consumer or provider type enters core/runtime

## Validation

- topology-neutral host fixtures
- long-lived RPC profile
- dependency and redaction audit
- `effigy qa`

## Stop Condition

Stop if remote placement requires client paths, secrets, or process authority.

## Closeout

- runtime host-service sets now carry the execution-host id that owns every
  registered service; both Codex drivers reject post-preflight host
  substitution before host or provider work
- provider-neutral local and remote-authoritative fixtures use distinct opaque
  configured-instance, target, and working-resource references
- session handles return a resume binding across provider session, configured
  instance, execution host, model route, and model identities
- resume rejects a mismatched binding before process start and rejects a
  provider response that substitutes another session
- turn work rejects services from another host; notifications and callbacks
  from another provider session terminate safely
- callback cancellation and session close abandon pending waits, close streams,
  produce one terminal outcome, and join the process task in both topologies
- unexpected disconnect remains a host failure. No automatic recovery route,
  raw path transport, credential extraction, or consumer policy was added
- the long-lived RPC profile, redaction tests, topology integration fixtures,
  workspace tests, strict Clippy, formatting, and docs QA provide closeout
  evidence
