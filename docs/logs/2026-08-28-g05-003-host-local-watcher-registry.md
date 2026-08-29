# 2026-08-28 g05.003 Host-Local Watcher Registry

Status: complete
Owner: Tom
Card: 009
Contract: 059
Research: 259

## Result

The host-local watcher registry provides the Contract 059 enforcement layer
behind the provider-neutral core. The pre-1.0 start seam uses bounded,
redacted `WatcherOperationData` interpreted under host policy. Only the host
selects progress or terminal summaries.

`LocalHostServices` registers the watcher service without a process-containment
backend. Process-backed starts reject before work with
`swallowtail.local_watcher.containment_unavailable`. An exact injected
`ProcessContainmentBackend` must bind a containment lease before a watcher id
returns; stop, cancel, deadline, and join target that lease and prove empty
scope before clean turn cleanup. Process groups are ordinary cleanup mechanics
only and do not satisfy the watcher capability gate. Tests inject a recording
containment backend; no non-conforming backend is shipped in the public API.

Approved operation data, capacity failures, closed turns, foreign ids, stale
lifecycle operations, wait controls, wake-all, panic notification, rollback,
and bounded retirement remain fail-closed. Escaped `setsid` children can
outlive ordinary process-group cleanup; that gap is Research 259 evidence, not
ownership.

## Evidence

- Restacked PR 117 onto pushed `main` with Research 259 and revised Contracts
  010/059 preserved.
- `swallowtail-host-local`: containment lease seam, default absence, probe
  injection, lease-targeted stop/join, removed descendant poller.
- Focused validation: 473 passed across core/runtime/host-local/testkit.
- Affected-package proof, public-api unreleased baselines, `git diff --check`.

## Next

Return to the orchestrator for exact containment-backend selection and proof.
Keep card 010 gated until that composition exists. Do not merge without
operator authorization.
