# OpenCode Attached Prepared Integration

Use the prepared facade to connect to an operator-managed OpenCode HTTP
server. Swallowtail observes and invokes the selected service. It does not
start, stop, configure, authenticate, update, or recover the server.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The route is `opencode.http` in `swallowtail-adapter-opencode`, with driver ID
`swallowtail.opencode.http`. Use it for an already running OpenCode server when
the application needs HTTP/SSE catalogue, run, session, import, history
pages, reconciliation, callbacks, or inactive-session delete. Reject it when the
application must own server startup or shutdown, requires ACP, or cannot
provide an approved endpoint and delegated credential lease.

## Explicit Inputs

Preparation requires:

- one configured-instance identity and revision
- one execution host and host-approved endpoint target
- one maintainer-supported delegated-auth access profile, credential
  reference, and access evidence
- one probe scope, cancellation control, and host-monotonic deadline

The access profile describes the selected host arrangement. Preparation does
not inspect OpenCode configuration, discover `OPENCODE_SERVER_PASSWORD`,
extract provider credentials, select an account, or inject an authentication
fallback.

Call `prepare_opencode_attached` with `OpenCodePreparationInput`,
`OpenCodePreparationProbe`, and the selected host services. Preparation
authorizes only the opaque endpoint target, acquires and releases one scoped
delegated credential lease, and observes exact `GET /global/health` output.
It performs no catalogue request, session creation, prompt, inference, or
attached-service lifecycle action.

The result retains external-attached ownership, ambient harness configuration,
the exact server binding and compatibility assessment, access provenance, and
the low-level driver escape hatch.

Operation plans additionally bind task, time, HTTP, credential, attachment,
and working-resource services as needed. Default sessions are read-only.
Callback-enabled operations explicitly select ambient read-write authority.
Neither posture is a sandbox or host-containment claim.

## Version Posture

The guaranteed server window is `1.14.48` through `1.18.28` across the
qualified release segments and exact exclusions. A later exact stable release
may proceed as visibly unverified through the latest qualified behavior. It
does not expand guaranteed support.

Catalogue and session operations repeat the health check and require the exact
prepared version. Session creation must report that same release. Endpoint or
version drift fails instead of selecting another server or behavior.

## Catalogue

`prepare_catalogue` accepts only a request identity and optional deadline. It
derives a model-catalogue plan without a provider, model route, or session
access policy.

`OpenCodePreparedCatalogue::list_models` delegates to the existing
`GET /provider` operation. Catalogue entries are observations from the
selected OpenCode service. They do not prove access, entitlement, model
availability, or route selection.

## Read-Only Session

`prepare_session` requires a request identity, explicit provider and model
route, working-resource reference, and optional deadline. It derives:

- external-attached service ownership
- ambient-host isolation posture
- ambient OpenCode configuration
- read-only working-resource access
- prohibited provider-owned durable session state by default
- exact provider, model, route, endpoint target, and server version

`OpenCodePreparedSession::open_session` returns the unchanged interactive
session handle. Provider session identity, directory affinity, SSE ordering,
turn interruption, deadline behavior, and credential and resource cleanup
remain operation-scoped on the existing low-level lifecycle. Qualified
sessions support exact load with bounded oldest-first replay and exact resume
without replay. Closing a turn or session does not stop or dispose the
attached OpenCode service.

Call `with_image_attachments()` before preparation if later turns may carry at
most one PNG of declared size no greater than one MiB. The host materializes
the opaque reference within the prepared attachment and working-resource
bounds; the consumer does not pass provider-native image data.

`OpenCodeSessionProfileInput::with_provider_callbacks()` enables the qualified
callback subset. Ordered OpenCode questions project into common typed harness
user input. Permission requests remain exact OpenCode extensions because
their one-shot authorization semantics are provider-specific. The consumer
answers both through the same correlated callback exchange.

Take each turn's events, optional callback exchange, and terminal outcome
immediately and poll them concurrently. Responses are exactly once and scoped
to the requesting turn. Cancellation interrupts only that turn. Close the turn
and session to join local work while preserving the external server and
provider session; terminal and cleanup truth remain separate.

`OpenCodeSessionProfileInput::with_active_turn_detachment()` instead selects a
durable, callback-free session profile for controlled application shutdown.
Its active turn handle exposes `OperationDetachmentControl`. Request
detachment, await the local `Detached` terminal outcome, then consume ordinary
turn and session close. This joins the SSE client without `/abort` while the
external server and provider session remain preserved. Persist the resume
binding before dispatch and reconcile it after restart. See
[Provider Operation Detachment](provider-operation-detachment.md).

Detachment and provider callbacks are mutually exclusive. Structured runs
remain temporary and delete their private session on close, so they do not
expose detachment.

## External Session Catalogue And Import

`prepare_session_catalogue` derives a separate read-only, resource-scoped
catalogue plan. `OpenCodePreparedSessionCatalogue::list_sessions` uses only the
approved endpoint and resolved directory. It returns bounded candidates with
opaque references, titles, update times, activity state, and explicit import
availability. Pagination cursors remain bound to the exact catalogue plan.

`prepare_session_import` accepts only an available candidate from that
catalogue plus an explicit model route and the same working resource. Import
repeats the exact health, lookup, directory, title, update-time, server
revision, root, archive, and idle-status checks before issuing a normal
`SessionResumeBinding`. The consumer then calls the existing prepared
`load_session` to receive bounded replay or `resume_session` to continue
without replay.

Import support is guaranteed only for qualified `1.14.48..=1.18.28` server
revisions. Visible unverified-newer servers do not inherit it. Child, active,
archived, incompatible, missing-status, changed, or missing sessions issue no
binding.

The catalogue is not synchronization. A provider reference is not attachment
authority, and a consumer must not construct a binding from a raw OpenCode
session id. The consumer owns selection, persistence, duplicate handling, and
when to repeat discovery. Swallowtail never scans other projects or
directories and never starts, stops, or updates the attached server.

For restart continuity, export the issued binding with
`SessionResumeBinding::export_persisted(prepared_session.plan())` and store the
opaque bytes with the consumer thread. After restart, re-prepare the exact
OpenCode route and restore with `SessionResumeBinding::restore_persisted` using
that plan, working resource, and access policy. Compaction retains the same
OpenCode session identity. Invalid or drifted records fail before HTTP work and
must not trigger fresh-session fallback.

For an interrupted attached turn, build
`OpenCodeSessionReconciliationInput` with that exact binding, model, runtime
turn, optional provider turn, bounds, and deadline. Then call
`prepare_session_reconciliation` and `reconcile`. The read-only operation
classifies retained session state without prompting, aborting, answering a
callback, or granting attachment or management authority. A settled result
may compose with the matching prepared session through
`prepare_settled_session_restoration`; active or ambiguous work stays
observational. This is qualified only on the maintained version range.

For newest-first history browse without attaching a handle, build
`OpenCodeSessionHistoryInput` with that exact binding, model, history id,
page/cursor/snapshot bounds, and optional deadline. Then call
`prepare_session_history` and `page_history`. Older pages use
`older_page_request` plus `page`. The path walks session messages through the
same replay helper as reconciliation, slices portable pages in-process, and
issues no prompt, abort, status observation, or management call. Unverified
newer servers do not prepare history. See
[Provider Session History Pages](provider-session-history.md).

## Structured Run

`prepare_run` requires a request identity, explicit provider and model route,
content, working-resource reference, and optional deadline. It derives a
separate `StructuredRun` plan with temporary provider retention and exact
operation-owned session deletion.

Optional `OpenCodeRunProfileInput` controls are one bounded PNG, provider
callbacks, a catalogue-supported model reasoning variant, and one inline JSON
Schema 2020-12 object. Reasoning and schema require the exact bound
`ModelCatalogEntry`; structured output uses zero hidden retries. The route
does not infer support from a model name.

`OpenCodePreparedRun::start_run` creates one private provider session,
subscribes to its exact SSE terminal stream, submits one prompt, closes the
turn, deletes only that session, then releases the working-resource and
credential leases. The terminal outcome reports session deletion as confirmed
or unconfirmed independently from provider completion and cleanup.

The run exposes no provider run, interactive handle, resume binding, or
provider-session management binding. It never starts, stops, updates, or
recovers the attached OpenCode server.

`OpenCodeRunProfileInput::with_provider_callbacks()` enables the same typed
question and provider-specific permission subset for the structured run.

Drain events, callbacks, and terminal concurrently. Deletion confirmation and
local cleanup remain independent from model completion.

## Delete, Failures, And Unsupported Capabilities

An interactive handle exposes an opaque management binding. After closing the
handle, pass that binding to `prepare_delete_session`; a raw OpenCode session
ID cannot authorize deletion. The typed effect reports provider-data deletion
with provider-defined descendant scope. Unknown, active, drifted,
cross-instance, and cross-resource bindings fail closed. Unverified-newer
execution requires explicit acceptance and does not inherit import,
reconciliation, or detachment.

Handle failures through portable classification and retain the exact
`swallowtail.opencode.*` diagnostic for support. Do not parse HTTP bodies, SSE
frames, server logs, question text, permission display, or provider prose to
infer retry, auth, terminal, or cleanup truth.

OpenCode exposes no archive, restore, native close, external search,
consumer-tool exchange, output-token limit, billed-cost evidence, provider
managed retry, owned server lifecycle, or public subagent control. New claims
require an exact server surface, qualified release evidence, prepared-plan and
access binding, bounded fixtures, lifecycle tests, and route-matrix coverage.

## Transport Separation

OpenCode HTTP/SSE is a provider-specific harness interface. It is not ACP.
`swallowtail-transport-acp-remote` remains a separate explicit transport for
provider adapters that implement ACP over a selected HTTP/SSE or WebSocket
endpoint. Neither route probes, upgrades, falls back to, or recovers through
the other.

`plan`, `request`, `evidence`, `server`, `low_level_driver`, and `into_parts`
remain available for inspection and advanced use.

See the compile-tested
[`prepared_opencode_attached` example](../../crates/swallowtail-adapter-opencode/examples/prepared_opencode_attached.rs).

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-opencode
effigy check:examples
```

No live server, credential use, prompt, import, reconciliation, or delete is
required. Operator live checks must remain separately gated.
