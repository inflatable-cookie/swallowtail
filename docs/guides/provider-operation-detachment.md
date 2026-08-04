# Provider Operation Detachment

Controlled detachment ends one qualified local observation attachment without
asking the provider to stop its work. It is not cancellation, completion,
stream reattachment, session resume, or crash reconciliation.

## Admission

Check the prepared plan for `Capability::ActiveOperationDetachment` and its
exact `OperationDetachmentScope`. A configured instance advertising the
capability is not enough. The selected profile must also bind durable provider
state and a reconciliation route.

The run or turn handle exposes `detachment()` only when its immutable plan
admits the operation. `None` means the route or selected profile cannot preserve
active work safely. Do not fall back to dropping the handle.

## Shutdown Sequence

For an admitted active operation:

1. Persist the session resume binding before dispatch can be lost.
2. Stop accepting new consumer work for the session.
3. Call `handle.detachment().unwrap().request()`.
4. Treat `Requested` and `AlreadyRequested` as local acknowledgement only.
5. Await the terminal outcome and require `TerminalStatus::Detached`.
6. Consume `handle.close()` and inspect its ordinary cleanup outcome.
7. Close the containing session attachment.
8. On restart, restore the exact binding and run provider-operation
   reconciliation before allowing another turn.

Do not record the provider turn as completed or cancelled. `Detached` says
only that Swallowtail stopped observing and joined its local work.

If cancellation already started, detachment fails. If both race, cancellation
wins. Ordinary close without a prior admitted detachment retains the route's
existing cancellation behavior.

## OpenCode

`OpenCodeSessionProfileInput::with_active_turn_detachment()` selects the first
production mapping. It is available only for read-only interactive sessions on
the qualified `opencode.http` range.

The selected plan binds:

- `ActiveOperationDetachment` with `ActiveTurn` scope
- durable provider-session preservation
- the exact OpenCode instance, route, model, resource, and access posture
- `SessionResumeBinding` persistence and session-scoped reconciliation

The driver closes and joins its SSE client without issuing `/abort`, another
prompt, a callback answer, load, resume, import, status, or deletion request.
The external OpenCode server and provider session remain outside Swallowtail's
ownership.

Callback-enabled sessions and structured runs cannot select detachment. Their
callback or delete-on-close lifecycles do not survive this boundary.

## Other Routes

No other production route currently exposes detachment. Research 100 records
the exact promotion gate for each route family. Consumers must not infer
support from background execution, session loading, stream reattachment, or
provider-managed recovery.

Contract 049 is authoritative for lifecycle truth. Contract 048 defines the
later observation-only reconciliation step.
