# 016 Operation-Scoped Watcher HTTP Bridge Core

Status: ready
Owner: Tom
Created: 2026-08-30
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: completed cards 008-009 and 014-015; Contract 060

## Goal

Implement the provider-neutral Contract 060 host bridge from one
operation-scoped HTTP/MCP lease to the exact turn-owned `WatcherHostService`.

## Scope

Deliver one coherent core, runtime, local-host, and testkit tranche:

1. Add the optional stable watcher-bridge host service kind and an object-safe
   runtime port. Opening a lease binds the exact host, operation, turn, and
   watcher service; registration alone binds nothing.
2. Keep endpoint and bearer capability values driver-only, non-serializable,
   default-redacted, and absent from public events, diagnostics, records, and
   errors. Generate fresh operation-private authority for every lease.
3. Implement the local-host lease on an ephemeral loopback endpoint. Open must
   report ready before a future provider spawn. Close must freeze admission and
   join accept, connection, dispatch, and cancellation work.
4. Decode only the minimum bounded HTTP/MCP initialization, tool listing,
   reserved watcher start/inspect/list/wait/stop, and completion-gate query.
   Unknown or malformed protocol fails before watcher work.
5. Dispatch model calls into the same registered watcher service used by
   operator controls. Preserve requester identity, exact correlation, host
   approval, watcher bounds, and Contract 059 lifecycle truth.
6. Implement the terminal barrier and deterministic races. No admitted call
   may create or revive watcher work after completion admission freezes.
7. Add deterministic provider-free fixtures for authentication, correlation,
   bounds, redaction, omission, concurrent wait/stop/complete/close, every
   terminal path, and joined cleanup. Update public API baselines and realized
   architecture when the implementation lands.

Choose the smallest maintainable HTTP implementation consistent with existing
workspace dependencies and Rust-quality rules. Do not create a generic server,
transport abstraction, provider helper, or reusable arbitrary MCP framework.

## Acceptance Criteria

- [ ] omission creates no listener, task, private material, or new host-service
      requirement
- [ ] open binds a ready endpoint and fresh authority to one exact host,
      operation, turn, and watcher service
- [ ] endpoint, token, paths, headers, bodies, commands, PIDs, and raw watcher
      output stay out of public records and default formatting
- [ ] missing, wrong, stale, cross-lease, duplicate, foreign, malformed,
      oversized, unknown, and post-terminal requests fail before watcher work
- [ ] only the reserved watcher operations and completion query reach the same
      registry as operator controls
- [ ] listener admission freezes before successful completion and no race can
      create post-barrier work
- [ ] cancellation, deadline, failure, explicit close, and normal completion
      join watcher and bridge work and release private material exactly
- [ ] no container, sandbox, public endpoint, sign-in-port reuse, ambient
      configuration, Claude binding, or route capability claim enters the diff

## Stop Conditions

- the implementation needs a new generic HTTP/MCP or arbitrary tool product
  surface rather than the closed Contract 060 family
- endpoint or authentication material must enter argv, ambient environment,
  shared configuration, durable records, events, diagnostics, or default
  formatting
- the bridge cannot bind the same watcher service and exact host, operation,
  and turn used by operator controls
- a listener, connection, dispatch task, wait, watcher, or private resource can
  survive operation cleanup
- a new provider, remote-topology, container, sandbox, TLS, firewall, or
  consumer policy decision is required

Stop and return the exact contract gap. Do not widen authority or start Claude
adapter work.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Return one reviewable PR. The orchestrator must reassess cards 010-011,
current Claude version evidence, and the separately authorized live gate after
the provider-neutral bridge lands.
