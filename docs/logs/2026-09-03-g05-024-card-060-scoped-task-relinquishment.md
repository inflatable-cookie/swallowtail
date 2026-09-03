# 2026-09-03 g05.024 Card 060 Scoped Task Relinquishment

Status: complete; independent prerequisite PR; no merge
Owner: Tom

## Result

Exact-head review of PR 188 found one shared runtime gap after g05.023: a
caller deadline can expire while a host-scoped `JoinedTask` remains unfinished,
but local handle drop and join both wait for the worker. Adapter-global parking
would lose guaranteed reap ownership.

Card 060 adds `ScopedTaskService::relinquish` and
`TaskRelinquishOutcome::AcceptedForReap`. The selected host accepts only the
same exact execution host and `ScopeId`. Success clears the caller task slot
after a reaper accepts ownership. Rejection retains the slot. Acceptance says
nothing about task completion, join, or cleanup success.

## Local Host

`LocalJoinedTask` still joins explicitly and on drop. Its ordinary ownership
rule was not weakened. The new path starts one host-side reaper before moving
the unfinished worker. That reaper joins the worker after it finishes without
adapter-global state or another adapter call.

## Proof

Deterministic tests hold a task stalled while relinquishment returns, then
release it and observe host-side reap. Separate cases reject wrong scope, wrong
host, repeated transfer, and already-finished transfer. The finished task then
uses ordinary join; existing drop-join coverage remains intact.

The semantic API delta is one outcome enum and one method on the existing task
service. The handle-side host hook stays hidden from generated public API.

## Validation

- focused runtime and host-local validation: 362 tests passed
- affected-package source proof: runtime and host-local passed
- semantic API: 40-package v0.3.3 gate passed; no removal
- docs and Northstar gates: passed, including roadmap number/status checks
- god-file scan: inherited 386 findings; no task implementation or test file
  entered the threshold set
- `git diff --check`: passed

## Authority

- [g05.024](../roadmaps/g05/024-scoped-task-relinquishment.md)
- [card 060](../roadmaps/g05/batch-cards/060-scoped-task-relinquishment-and-host-reap.md)
- [Contract 010](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 019](../contracts/019-embedded-sdk-and-cloud-client-boundary.md)
