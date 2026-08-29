# 2026-08-29 g05.003 Host-Process Watcher Supervision

Status: complete
Owner: Tom
Card: 014
Contract: 059
Research: 259

## Result

Default `LocalHostServices` now starts approved process-backed watchers through
the ordinary host process service. `WatcherOperationData` still resolves to a
private `ProcessRequest`; unapproved data rejects before work. Accepted starts
bind the owned `ProcessHandle` before returning watcher identity. Stop, wait,
and join target that handle. `ProcessHandle::wait` already joins the root,
cooperative process-group cleanup, output readers, and process supervisor; the
watcher monitor and registry join complete the Contract 059 lifecycle.

Public containment types are gone: `ProcessContainmentBackend`,
`ProcessContainmentLease`, `ContainedProcessStart`, and the builder injection
methods. Pre-1.0, no compatibility shims remain. Diagnostics no longer name a
containment backend. Process groups stay ordinary cleanup mechanics; escaped
`setsid` children remain Research 259's explicit non-claim.

## Evidence

- `swallowtail-host-local`: default watcher composition, process-handle stop
  and join, removed public containment API, host-local fixtures for completion,
  non-zero exit, model and operator stop, cancellation, timeout, output
  overflow, partial-start rollback, unapproved rejection, and cooperative child
  cleanup.
- Unreleased public-api baseline updated.
- Focused validation: 477 passed across core/runtime/host-local/testkit.

## Next

Card 010 stays planned until this PR merges. The orchestrator then assesses
Claude route binding. Cards 005-006 stay planned behind Research 256.
Contract 029 currentness remains standing.
