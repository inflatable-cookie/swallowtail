# 2026-09-03 g05.024 Card 060 Scoped Task Relinquishment

Status: complete; independent prerequisite PR repaired at exact-head review; no merge
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
rule was not weakened. Exact-head review found that the first implementation
dropped each per-transfer reaper handle. The repaired path starts one reaper
per transfer and registers its handle under an outer selected-host lifecycle
owner before worker handoff. Task-service clones keep only weak handoff
authority. Explicit selected-host shutdown outside the task tree joins every
retained reaper. There is no adapter-global state or second adapter call.

## Proof

Deterministic tests hold a task stalled while relinquishment returns, then
release it and observe host-side reap. Separate cases reject wrong scope, wrong
host, repeated transfer, and already-finished transfer. The finished task then
uses ordinary join; existing drop-join coverage remains intact. A load-bearing
shutdown test holds accepted work stalled while the worker owns a captured
task-service clone. It proves outer shutdown cannot finish early, releases the
work, observes the worker drop its clone without deadlock, then observes both
reap and shutdown completion. Moving join ownership back to service-clone drop
would make this bounded counterexample fail.

The semantic API delta is one outcome enum, one method on the existing task
service, and one explicit local host reaper-shutdown operation. No task handle
or handle-side host hook enters generated consumer API.

## Validation

- focused runtime and host-local validation: 365 tests passed
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
