# Kimi Local Server Prepared Integration

Use this facade when an application needs Kimi Code's documented foreground
REST/WebSocket server, explicit approval and question exchange, or native
archive and restore. Use the separate Kimi ACP facade for the smaller stdio
path, provider load replay, or resume without local-server management.

The route is `kimi-code.local-server` in `swallowtail-adapter-kimi`, with
driver ID `swallowtail.kimi.local-server`. Its transport is authenticated
loopback REST plus WebSocket v2. It may attach to an operator-owned server or
own one foreground `kimi web` child; topology selection is explicit.

## Route Choice

| Need | Route |
| --- | --- |
| normal installed Kimi agent session over stdio | `kimi-code.acp` |
| load with provider replay | `kimi-code.acp` |
| REST/WebSocket v2 streaming | `kimi-code.local-server` |
| configured model catalogue | `kimi-code.local-server` |
| one retained prompt with explicit callback exchange | `kimi-code.local-server` structured run |
| explicit approval or structured-question callback exchange | `kimi-code.local-server` |
| exact interrupted-turn reconciliation after restart | attached `kimi-code.local-server` |
| controlled active-turn detachment | attached `kimi-code.local-server` without callbacks |
| archive or restore an inactive provider session | `kimi-code.local-server` |
| provider-session hard delete | neither route |

There is no fallback between the routes. Selecting one route does not authorize
the other.

## Attached And Owned Topology

Attached preparation requires:

- one configured-instance identity and revision
- one execution host and host-approved loopback endpoint target
- one opaque local-server bearer credential reference
- matching access evidence
- one explicit Kimi state-root identity
- one exact executable-version binding
- caller-owned probe cancellation and deadline

Call `prepare_kimi_local_server_attached`. Swallowtail probes health and exact
metadata, corroborates the executable and server release, releases the probe
credential, and returns an inspectable prepared integration.

Owned preparation takes the same input plus an approved Kimi executable target.
Call `start_kimi_local_server_owned`. Swallowtail starts:

```text
kimi web --no-open --host 127.0.0.1 --port <approved-port> --log-level info
```

The returned handle owns only that foreground child. `close` stops and joins
it. No container is required. Neither topology claims a sandbox. Kimi account
authentication and harness configuration remain Kimi's authority; the server
bearer only authenticates the approved loopback endpoint.

## Interactive Session

`prepare_session` requires:

- request identity
- explicit model route, revision, and model
- working-resource identity
- `Manual`, `Auto`, or `Yolo` permission mode
- optional reasoning mode
- optional `0.29.0` profile and disabled-tool controls
- explicit acceptance for a visible unverified-newer release

`open_session` returns the common interactive-session handle with exact Kimi
resume and archive/restore bindings. Prompt content remains a `TurnRequest`.
Output, reasoning, lifecycle, disconnect, resynchronization, and terminal
events remain ordered runtime events.

Take turn events, callbacks in `Manual` mode, and the terminal outcome
immediately and poll them concurrently. A blocked callback can block the turn;
an undrained semantic stream can exhaust its bounded buffer. Terminal status,
native provider state, local attachment cleanup, and owned-server cleanup are
separate truth.

`Manual` mode exposes provider approval and structured-question requests
through the callback exchange. The consumer owns authorization and response
policy. `Auto` and `Yolo` do not silently elevate an unexpected callback;
undeclared provider interaction fails visibly.

Structured questions use the common typed harness user-input request and
response. Manual approvals remain the exact
`kimi.local-server/approval-v1` provider extension. Both preserve the provider
request id and exactly-once callback correlation.

Cancellation sends Kimi's native WebSocket abort. Deadlines, provider
cancellation, transport loss, resynchronization, and runtime failure remain
distinct. Turn and session close join local work. Session close preserves
provider state.

### Checkpoint, Reconciliation, And Detachment

After Kimi exposes the exact provider turn, accepted runtime events carry a
`ProviderOperationCheckpoint`. Persist it with
`ProviderOperationCheckpoint::export_persisted` under the same prepared
session plan and `SessionResumeBinding`. The persisted bytes are opaque; do
not parse or rewrite the Kimi cursor.

After restart, construct `KimiLocalServerReconciliationInput` with the same
model selection, restored session binding, persisted checkpoint, and replay
bounds. `prepare_session_reconciliation` rejects another route, instance,
host, model, resource, access posture, session, runtime turn, or provider turn.
The prepared operation reads one finite retained WebSocket window and returns
exact `InterruptedTurnState` without prompt, abort, callback, resume, import,
or management authority.

For controlled shutdown, select
`KimiLocalServerSessionConfiguration::with_active_turn_detachment` before
preparation. The turn handle then exposes `detachment()`. Request detachment,
await the ordinary terminal outcome (`TerminalStatus::Detached`), and close
the handle to join local work. This closes only the WebSocket observer and
sends no provider abort.

Do not request detachment until the newest event checkpoint is durably stored.
The adapter rejects a request made before the first recoverable checkpoint is
available.

Detachment is available only on qualified externally attached servers in
`Auto` or `Yolo` mode. Manual callbacks, owned foreground servers, structured
runs, and unverified-newer versions remain excluded. Calling ordinary close
without requesting detachment keeps the existing native-abort behavior.

See the compile-tested
[`interactive example`](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_interactive.rs).

## Retained Structured Run

`prepare_run` requires an explicit model, prompt content, writable working
resource, deadline, permission mode, and optional reasoning mode. It derives a
`StructuredRun` plan; it does not expose an interactive-session plan or handle.

The driver creates one operation-private Kimi session, submits one prompt,
relays the same qualified events and manual approval or question callbacks,
awaits one terminal outcome, then closes and joins local resources. The Kimi
thread remains. `DurableAllowed` is mandatory. Close does not archive, delete,
or expose reusable session-management authority.

The run input requires `accept_managed_recovery()`. An attached run may also
select `with_one_stream_reattachment()` for at most one same-turn cursor
reattachment after transport loss. It never replays the prompt and preserves
callback and cancellation truth. Reattachment is not automatic retry, a new
run, or authority to adopt another provider session.

Attached topology preserves the external server. Owned topology joins the run
before the caller may stop the foreground child. Neither path requires a
container or claims a sandbox.

See the compile-tested [structured-run
example](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_structured.rs).

## Model Catalogue

`prepare_catalogue` requires only a request identity, optional deadline, and
explicit unverified-newer acceptance when applicable. `list_models` performs
one authenticated `GET /api/v1/models` against the prepared attached or owned
server and projects bounded configured aliases. It does not refresh providers,
change Kimi's default model, open a session, or convert the local-server bearer
into Kimi account authority.

## Archive And Restore

Close the interactive handle before preparing archive or restore. The exact
management binding returned by the session is opaque and route-bound.
`prepare_archive_session` and `prepare_restore_session` require that binding
plus an explicit request identity. Their typed operations perform one native
REST effect and report before-dispatch versus uncertain-after-dispatch truth.

Kimi's selected server revisions expose no hard-delete operation. Swallowtail
does not inspect or delete Kimi state files.

An ACP-created session can gain local-server management authority only through
`prepare_binding_import`. The adapter requires its own ACP source authority,
matching host, executable version, state root, endpoint target, access
profile, and authenticated target lookup. A raw session id, filesystem path,
or list result cannot mint authority.

See the compile-tested [owned lifecycle
example](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_owned_lifecycle.rs)
and [binding-import
example](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_imported_management.rs).

## Failures, Unsupported Capabilities, And Promotion

Handle failures through portable classification and retain the exact
`swallowtail.kimi.local_server.*` diagnostic for support. Keep preparation,
provider terminal status, callback abandonment, reconciliation result,
management effect truth, and cleanup separate. Do not parse WebSocket frames,
REST bodies, server logs, Kimi prose, or state files in consumer code.

The route exposes no hard delete, JSON Schema output, attachments, consumer
tools, external search, billed-cost evidence, public subagent control, or host
sandbox claim. Provider task and child activity remains observational.
Unverified-newer servers do not inherit reconciliation, detachment, binding
import, or management capabilities merely because endpoints respond.

Promotion requires an exact server/executable behavior milestone, attached
and owned topology evidence where applicable, immutable plan and access
binding, bounded REST/WebSocket fixtures, lifecycle tests, and route-matrix
coverage.

## Deterministic Validation

The linked examples cover attached preparation, interactive and structured
operations, owned lifecycle, and ACP management-binding import. Run:

```sh
effigy validate:focused swallowtail-adapter-kimi
effigy check:examples
```

No Kimi login, live server, prompt, archive, or restore is required.

## Nucleus Adoption Inputs

Nucleus can add this route without changing its consumer-owned thread model.
The integration boundary needs:

- explicit ACP versus local-server selection
- attached versus owned-foreground topology selection
- endpoint target, bearer reference, state-root identity, and exact version
  binding
- model and permission-mode selection
- optional isolation shown as provider or host capability, never assumed
- callback UI for manual approvals and structured questions
- storage of opaque resume and management bindings beside, not inside,
  Nucleus thread identity
- atomic persistence of the newest opaque operation checkpoint carried by a
  runtime event
- explicit detach disposition followed by restart reconciliation before
  admitting another turn on the same consumer thread
- interactive-handle close before archive or restore
- delete hidden or reported unsupported
- visible unverified-newer evidence with execution allowed only after the
  application's explicit mileage-may-vary policy

Swallowtail owns protocol, access leases, lifecycle effects, diagnostics, and
joined cleanup. Nucleus retains prompts, authorization, workflows, thread
archive/delete policy, memory, persistence, route preference, and UI.

Start with the compile-tested [attached preparation
example](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_attached.rs).
