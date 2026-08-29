# 014 Host-Process Watcher Supervision

Status: ready
Owner: Tom
Created: 2026-08-29
Updated: 2026-08-29
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: completed card 009; revised Contract 059; Research 259

## Goal

Make the host-local watcher registry run ordinary host-approved processes and
publish truthful joined lifecycle without requiring a container or claiming
security containment of deliberately detached descendants.

## Scope

1. Replace pre-1.0 public and internal containment terminology that promises
   sandbox-grade descendant control with managed watcher-process supervision
   terminology matching revised Contract 059.
2. Compose process-backed watcher starts from the existing host-approved
   `ProcessRequest` and `LocalProcessHost`. Default local composition must not
   require an injected container, VM, platform containment service, or test
   backend.
3. Bind the owned process handle and its existing process-group cleanup before
   returning watcher identity. Preserve pre-work rejection for unknown or
   unapproved `WatcherOperationData` and rollback any partially started work.
4. Keep model and operator status, wait, and idempotent stop on the same
   registry. Terminal and joined truth must include root-process exit,
   cooperative process-group cleanup, bounded output-reader completion, watcher
   monitor completion, and supervisor join.
5. Keep the limitation exact: process groups are ordinary cleanup mechanics,
   not a sandbox. Do not add process-table polling or claim control of a child
   that deliberately daemonizes, calls `setsid`, or otherwise detaches.
6. Add deterministic host-local fixtures for normal completion, non-zero exit,
   model/operator stop, cancellation/timeout cleanup, cooperative child cleanup,
   output failure, partial-start rollback, and unchanged rejection when the
   operation is not approved.
7. Update public API baselines, package docs, card/log evidence, and affected
   Northstar front doors. Do not wire Claude Code or publish a production route
   claim in this PR.

## Acceptance Criteria

- [ ] default host-local composition can start an approved process-backed
      watcher without an injected containment backend
- [ ] unapproved operation data rejects before process work
- [ ] watcher identity exposes no executable, command, path, environment, raw
      output, PID, or process-group identity
- [ ] normal completion waits for process, output readers, watcher monitor, and
      supervision join
- [ ] model and operator stop remain distinct request paths against one owned
      watcher and are idempotent
- [ ] cancellation, timeout, failure, and close stop and join managed work
- [ ] cooperative child processes in the owned group are stopped during
      watcher cleanup
- [ ] no container, VM, cgroup, Job Object, privileged helper, process-table
      poller, or arbitrary shell authority is added
- [ ] public names, diagnostics, and docs do not imply sandbox-grade descendant
      containment
- [ ] deliberately detached descendants remain an explicit non-claim rather
      than a hidden success condition
- [ ] existing portable watcher lifecycle and bounded-summary behavior remain
      unchanged

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- ordinary host-process supervision cannot provide truthful root-process,
  output-reader, monitor, and join state through the current host boundary
- the fix requires a container, VM, privileged helper, platform-specific hard
  containment, process-table ownership, or caller-supplied PID
- watcher start requires a public arbitrary command, executable path, argument
  vector, environment, or working-directory authority surface
- the repair changes route binding, injected watcher skill behavior, or Claude
  completion interception assigned to card 010
- validation exposes a contract or architecture decision beyond the revised
  managed-process boundary

## Auto-Continuation

No. Return one reviewable PR and the exact head. After merge, the orchestrator
will close card 014 and assess card 010 readiness.
