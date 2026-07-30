# Kimi Local Server Prepared Integration

Use this facade when an application needs Kimi Code's documented foreground
REST/WebSocket server, explicit approval and question exchange, or native
archive and restore. Use the separate Kimi ACP facade for the smaller stdio
path, provider load replay, or resume without local-server management.

## Route Choice

| Need | Route |
| --- | --- |
| normal installed Kimi agent session over stdio | `kimi-code.acp` |
| load with provider replay | `kimi-code.acp` |
| REST/WebSocket v2 streaming | `kimi-code.local-server` |
| configured model catalogue | `kimi-code.local-server` |
| one retained prompt with explicit callback exchange | `kimi-code.local-server` structured run |
| explicit approval or structured-question callback exchange | `kimi-code.local-server` |
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
- interactive-handle close before archive or restore
- delete hidden or reported unsupported
- visible unverified-newer evidence with execution allowed only after the
  application's explicit mileage-may-vary policy

Swallowtail owns protocol, access leases, lifecycle effects, diagnostics, and
joined cleanup. Nucleus retains prompts, authorization, workflows, thread
archive/delete policy, memory, persistence, route preference, and UI.

Start with the compile-tested [attached preparation
example](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_attached.rs).
