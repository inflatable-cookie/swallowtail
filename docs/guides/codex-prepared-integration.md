# Codex Prepared Integration

Use the prepared facade for normal Codex integration. Use the low-level drivers
when an application needs a profile the facade does not provide.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Choose A Route

Both routes are in `swallowtail-adapter-codex`; selection is explicit through
`CodexPreparedDriver`.

| Route | Driver ID and transport | Choose it for | Reject it when |
| --- | --- | --- | --- |
| `codex.exec` | `swallowtail.codex.exec`; structured CLI JSONL over stdio | one bounded structured run, optional reasoning, one image, JSON Schema output, or host-approved search | the application needs a reusable session, callbacks, discovery/import, reconciliation, or management |
| `codex.app-server` | `swallowtail.codex.app-server`; app-server JSONL RPC over stdio | model and thread catalogues, interactive sessions, tools, questions, plan mode, load/resume, newest-first history pages, reconciliation, or inactive-thread management | the application needs exec-only attachments, structured output, or external search |

There is no fallback between the drivers. A capability on one branch does not
belong to the other.

## Add The App-Server Connection

Only `codex.app-server` currently exports an addable descriptor.
`codex.exec` stays on the prepared-facade path below. Topology is
**installed**. It is not `ExecutionLayer`. Follow
[connection lifecycle](connection-lifecycle.md) before `prepare_codex`.

1. Assemble `AddableRouteCatalog` from
   `codex_app_server_addable_route_descriptor`. The row is `Available` when
   the host exposes the Process service; otherwise
   `Unavailable(HostService)`. Discovery of the executable stays Contract
   008 on the selected driver.
2. `admit_instance` writes the configured instance with opaque config refs
   for `binary_path` and `environment`. Admission does not prepare.
3. There is no credential field. ChatGPT access is cached local login.
   Swallowtail does not extract tokens, open a URL, or run hosted OAuth.
4. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement stays
   independent of 047 `Ready` / `NotReady`.
5. `observe_authenticated_subject` is `Absent`.
6. `observe_instance_update` reuses `codex_app_server_claim` and optional
   Contract 032 installed-executable observation.
7. Codex catalogue rows omit `provider_id`. Overlay keys instance plus
   model. Do not invent a catalogue provider id.
8. Build `CodexPreparationInput::from_admitted` from the admitted record, then
   call `prepare_codex` with its fixed `CodexPreparedDriver::AppServer` path.
   The constructor selects the stored `binary_path` and `environment` refs;
   the host resolves them during Contract 008 discovery.

The compile-tested
[`connection_lifecycle` example](../../crates/swallowtail-adapter-codex/examples/connection_lifecycle.rs)
shows catalog through prepare for `codex.app-server`. The canonical
route-map example remains
[`prepared_discovery`](../../crates/swallowtail-adapter-codex/examples/prepared_discovery.rs).

## Operator Prerequisites

The admitted record supplies the opaque binary-path and environment refs.
Preparation resolves them on the selected execution host, then requires the
configured-instance revision, caller-selected access profile, matching
observed or asserted access evidence, and the task, process, time, credential,
working-resource, and attachment services required by the chosen plan. Local
ChatGPT subscription access uses approved local login state without a
credential lease; API-key and enterprise access remain separate explicit
profiles.

Swallowtail does not install Codex, search `PATH`, log in, choose a model,
select billing, read an auth store, or infer a writable workspace. Exec admits
`0.80.0..=0.81.0`, `0.84.0..=0.107.0`, and `0.110.0..=0.149.1`; support is
deprecated through `0.121.0` and maintained from `0.122.0`. App-server admits
`0.80.0..=0.81.0`, `0.84.0..=0.107.0`, and `0.110.0..=0.149.1`; support is
maintained from `0.110.0`. Exact feature milestones remain narrower. Later
stable versions may remain visible `UnverifiedNewer` without gaining
capabilities.

## Normal Flow

1. The consumer selects the Codex driver: app-server or structured exec.
2. The execution host approves one exact executable and returns an opaque
   `InstalledExecutableTarget`.
3. The consumer supplies its stable instance identity, host identity, saved
   environment reference, access profile, and separate observed or explicitly
   asserted access evidence.
4. `prepare_codex` probes that exact target, classifies its installed version,
   and returns one `CodexPreparedIntegration`.
5. The consumer selects one named operation profile and inspects its evidence,
   plan, and request.
6. The prepared profile invokes its matching typed operation with services
   from the same authoritative host. That method delegates to the existing
   low-level driver.

The compile-tested
[`prepared_discovery` example](../../crates/swallowtail-adapter-codex/examples/prepared_discovery.rs)
shows the public constructors.

Local hosts can approve the target and compose joined services together:

```rust
let (builder, target) = LocalProcessHost::builder(limits)
    .approve_installed_executable(executable_ref, codex_axis, executable_path);
let local = builder.build_services(execution_host_id.clone());
let services = local.services();
```

Pass `services.clone()` to `prepare_codex` when the same service set will
execute the resulting profile. The facade never searches `PATH`, chooses a
credential, or changes host topology.

When a caller already has an explicit timeout duration, the local composition
can derive the corresponding monotonic deadline without choosing that
duration:

```rust
let deadline = local.deadline_after(Duration::from_secs(15));
```

## Access Profiles

For provider-supported Codex access through a cached ChatGPT login, use
`codex_chatgpt_subscription_access_profile(profile_id)`. It encodes only the
fixed route facts: interactive OAuth, subscription allowance, the `codex`
audience, provider support, and no credential reference.

The helper does not inspect local login state, discover credentials, assert
readiness, or create `AccessStatus`. Supply observed or explicitly asserted
status separately through `PreparedAccessEvidence`. Its profile id must match
the chosen profile.

API-key login and enterprise access tokens remain separate explicit profiles.
The ChatGPT helper does not authorize the public OpenAI API or substitute one
billing route for another. See the compile-tested example for the profile and
status composition.

## Named Profiles

| Prepared path | Driver | Consumer choices |
| --- | --- | --- |
| `prepare_catalogue` | app-server | request identity and optional deadline |
| `prepare_session_catalogue` | app-server | catalogue identity, exact working resource, page/traversal/content/reference bounds, optional deadline |
| `prepare_session_history` | app-server | history identity, exact durable binding, page/cursor/snapshot bounds, optional deadline |
| `prepare_read_only_session_import` | app-server | one candidate from the prepared catalogue plus the future model, resource, and read-only session options |
| `prepare_bounded_workspace_session_import` | app-server | the same explicit candidate and future session inputs plus bounded-workspace selection |
| `prepare_read_only_session` | app-server | model route, model, working resource, instructions, reasoning, plan mode, tools, optional typed user-input exchange |
| `prepare_bounded_workspace_session` | app-server | the same session inputs plus explicit writable-profile selection |
| `prepare_archive_session` | app-server | request identity, inactive management binding, optional deadline, explicit unverified-newer acceptance |
| `prepare_restore_session` | app-server | request identity, inactive management binding, optional deadline, explicit unverified-newer acceptance |
| `prepare_delete_session` | app-server | request identity, inactive management binding, optional deadline, explicit unverified-newer acceptance |
| `prepare_structured_exec` | exec | content, model route, model, working resource, network, search, reasoning, deadline, schema, attachment |

Catalogue, interactive session, and structured run remain different runtime
roles. Read-only and bounded workspace remain different methods. There is no
generic prompt method.

`CodexPreparedCatalogue`, `CodexPreparedSessionCatalogue`,
`CodexPreparedSessionHistory`, `CodexPreparedSessionImport`,
`CodexPreparedSession`, the three lifecycle values, and `CodexPreparedExec`
retain:

- exact installed-version and compatibility evidence
- access status and whether it was observed or caller-asserted
- the expanded immutable `PreflightPlan`
- the matching runtime request

Use `evidence()`, `plan()`, and `request()` before effects. Lifecycle values
retain the prepared environment internally for their typed `execute` method.

## Bound Operations

Prepared profiles expose only the runtime role they implement:

| Prepared value | Bound operation |
| --- | --- |
| `CodexPreparedCatalogue` | `list_models(services)` |
| `CodexPreparedSessionCatalogue` | `list_sessions(services)` and bounded cursor continuation through `next_page_request` plus `list_page` |
| `CodexPreparedSessionHistory` | `page_history(services)` and older-page continuation through `older_page_request` plus `page` |
| `CodexPreparedSessionImport` | `import_session(services)` |
| `CodexPreparedExec` | `start_run(services)` |
| `CodexPreparedSession` | `open_session(services)` |
| `CodexPreparedSession` | `load_session(request_id, imported_binding, services)` |
| `CodexPreparedSession` | `resume_session(request_id, binding, services)` |
| `CodexPreparedArchive` | `execute(services)` |
| `CodexPreparedRestore` | `execute(services)` |
| `CodexPreparedDelete` | `execute(services)` |

`load_session` and `resume_session` derive and validate their requests before
returning runtime futures. Unsupported dynamic-tool redeclaration therefore
fails before provider effects.

For a run or turn, take its event stream, optional callback exchange, and
terminal outcome immediately and poll them concurrently. Leaving semantic
events or callbacks undrained can stall or fail a bounded operation. A
terminal outcome does not imply cleanup success; close the run, turn, and
session handles in their documented order and retain cleanup separately.

Exec emits assistant, tool, usage, and terminal evidence for one run.
App-server additionally projects provider activity, plans, task lists, child
topology, consumer-tool callbacks, and the opt-in typed question exchange when
the selected version and profile qualify them. Provider approval observations
do not grant response authority.

## External Thread Import

Codex app-server versions `0.105.0..=0.107.0` and
`0.110.0..=0.149.1` expose the complete selected import chain. Earlier
supported app-server versions keep their existing operations without
advertising catalogue or import. Later versions remain visible as unverified
newer but do not gain this exact capability.

Keep the phases separate:

1. Prepare one catalogue for one exact host-approved working resource and
   explicit bounds.
2. Call `list_sessions`. Present its bounded title, preview, update time,
   activity, and availability downstream. Follow only cursors returned by the
   same prepared catalogue.
3. After explicit user selection, pass the complete opaque candidate to
   `prepare_read_only_session_import` or
   `prepare_bounded_workspace_session_import`. A raw Codex thread id is not
   accepted.
4. Call `import_session`. Swallowtail reads and validates the exact thread id,
   cwd, source, update time, inactive availability, and bounded history before
   returning an `ExplicitlyImported` resume binding.
5. Prepare the matching ordinary session profile and call `load_session` with
   that binding. Load returns ordered history before the handle is ready.
6. Close or continue the loaded handle normally. Later attachments use the
   existing `resume_session` path and return no replay.

The catalogue lists only non-archived `cli`, `vscode`, and `appServer` threads
under the exact materialized cwd. Missing, changed, active, substituted,
wrong-resource, malformed, oversized, cancelled, timed-out, disconnected, or
unclean operations return no binding. Import does not create a consumer thread,
persist history, synchronize databases, send a prompt, or grant archive/delete
authority.

Applicable prepared app-server session handles expose
`management_binding()`. Clone that opaque binding while the handle is
available, close the handle, then pass the binding to one exact lifecycle
preparation method. The binding carries the provider thread identity, exact
driver, instance, host, target, access evidence, interface compatibility,
working resource, origin, and supported lifecycle capabilities. A raw thread
id cannot authorize management.

Archive, restore, and delete do not resume or discover a session and never
auto-close a runtime handle. The caller asserts that its attachment is
inactive by choosing the lifecycle operation after close. A mismatched binding
fails during preparation before app-server work.

Codex lifecycle support remains version-specific:

- archive starts at `0.80.0` and guarantees only the target
- restore starts at `0.92.0`
- matching archive and restore notifications are expected from `0.104.0`
- best-effort descendant archive from `0.123.0` does not widen the guarantee
- hard delete starts at `0.140.0` and reports provider-defined descendants

Unknown and repeatedly fully deleted targets remain failures. Swallowtail does
not turn them into already-absent success. A response lost after dispatch,
post-dispatch cancellation or deadline, or a malformed success response
returns unconfirmed effect truth. Cleanup and notification disagreement remain
separate diagnostics.

These methods clone the immutable plan and explicit request, construct the
matching Codex low-level driver from the prepared environment reference, and
delegate execution. They do not change preflight, host validation,
cancellation, deadlines, callbacks, terminal outcomes, or cleanup.

Use these bound operations for normal integration. Catalogue, session, and
exec values retain `into_parts()` for advanced consumers that need to separate
the exact prepared request from its driver. Extracting those parts only to
reconstruct the same low-level role adds integration work and is not a second
normal path.

## History Browse, Restart, Reconciliation, And Management

Persist an app-server `SessionResumeBinding` only through its opaque export
under the same prepared plan. On ordinary attachment, `load_session` returns
bounded ordered replay and `resume_session` returns no replay.

For UI scroll-back without treating browse as load readiness, prepare
`CodexSessionHistoryInput` from the exact binding and page bounds, then call
`prepare_session_history` and `page_history`. Later pages use the opaque
older cursor from the same plan. Codex synthesizes newest-first pages over one
bounded `thread/read(includeTurns: true)` under existing replay ceilings; it
does not send turn start, interrupt, resume, archive, restore, or delete.
See [Provider Session History Pages](provider-session-history.md).

When the consumer has durable interrupted-turn evidence, build
`CodexSessionReconciliationInput` from the exact binding and optional exact
provider-turn reference, then call `prepare_session_reconciliation` and
`reconcile`. The operation reads provider state without sending a prompt,
answering a callback, interrupting work, or replacing root turn ownership. A
settled result may compose with the same prepared session through
`prepare_settled_session_restoration`; active or ambiguous work stays
observational. Unknown, stale, cross-instance, cross-resource, and
post-terminal evidence fails closed.

Archive, restore, and delete require the opaque inactive management binding
from a matching app-server handle. They are separate effects, not cleanup on
session close. Exec exposes no continuation, history pages, reconciliation,
or management. See
[Provider Operation Reconciliation](provider-operation-reconciliation.md) and
[Working-State Restoration](working-state-restoration.md).

## Explicit Limits

- A model is always explicit. Catalogue defaults are display evidence, not
  route selection.
- App-server dynamic tools are consumer declarations. Swallowtail transports
  callbacks but never executes the tools.
- `CodexSessionProfileInput::with_user_input_exchange()` opts a session into
  app-server question callbacks. Each turn can expose ordered typed questions
  and accepts correlated typed answers. It does not enable approval requests.
- `SessionOptions::with_harness_mode(HarnessMode::Plan)` opts the whole
  app-server session into Codex plan mode. The adapter retains the choice and
  sends the exact collaboration-mode preset on every turn. This is qualified
  from Codex `0.88.0`; older prepared versions reject it before provider work.
  It does not change the session access policy or grant approval authority.
- Dynamic tools cannot be redeclared on Codex resume.
- App-server session-open deadlines are currently unsupported and fail during
  preparation. Turn deadlines remain available on `TurnRequest`.
- Structured exec supports no declared tools, at most one image attachment,
  JSON Schema structured output, and either offline execution or host-approved
  external search.
- Bounded workspace is version-gated. It never replaces the read-only default.
- Qualified, deprecated, and unverified-newer executable observations remain
  visible. An unverified-newer version is permitted with mileage-may-vary
  evidence; it is not silently promoted into the guaranteed range.
- Destructive lifecycle preparation requires
  `CodexSessionManagementInput::allow_unverified_newer()` before an
  unverified-newer executable may run.

Codex exposes no portable output-token limit. Exec has no interactive tools or
callbacks; app-server has no image attachment, JSON Schema result, or external
search operation. New claims require an exact provider surface, versioned
behavior evidence, prepared capability and preflight binding, deterministic
corpus coverage, and a route-matrix update. Provider prose or a CLI flag alone
is not a promotion gate.

## Failures

Preparation failures expose a safe stage:

- target selection
- process spawn
- bounded output, cancellation, or timeout
- process exit
- version parse
- compatibility classification
- access evidence
- preflight
- cleanup

Stable formatting excludes raw executable paths, environments, operation
payloads, credentials, and provider payloads. A non-zero version probe keeps
`swallowtail.codex.discovery_exit_failed` and may add its numeric status plus a
bounded sanitized stderr excerpt. Keep `PreparationStage` and the safe
diagnostic code when projecting failures into application errors.

A malformed app-server notification or protocol message keeps its exact
`swallowtail.codex.app_server.malformed_notification` or
`swallowtail.codex.app_server.malformed_message` code and appends the
notification method plus a bounded sanitized excerpt of the raw line. A
protocol terminal failure may also append a bounded sanitized app-server
stderr tail. The offending payload itself stays out of the safe message.

When the host registers a `DiagnosticObserver`, the same malformed-inbound
path also emits correlated restricted
[debug observations](debug-observation.md) (`WireInbound`, `ProtocolParse`,
and `StderrRing` when a stderr tail was retained). Ordinary integrations
leave the observer unregistered; debug emission never changes the safe code,
classification, or poisoned-session behavior.

## Low-Level Escape Hatch

The facade is additive. `CodexExecDriver`, `CodexAppServerDriver`, discovery,
descriptors, claims, request records, preflight, and runtime handles remain
public. Catalogue, session, and exec values also expose `low_level_driver()`
when an advanced consumer needs the exact selected Codex driver. Lifecycle
prepared values delegate to that same public driver's management role.

Use those surfaces when a consumer has a legitimate operation profile that the
named facade does not express. The consumer then owns exact configured-instance
construction, access and interface-version bindings, requirements, immutable
request agreement, and drift rejection. Do not bypass a preparation failure by
rebuilding a weaker low-level plan.

## Deterministic Validation

The compile-tested
[`prepared_discovery` example](../../crates/swallowtail-adapter-codex/examples/prepared_discovery.rs)
covers the public prepared branches. Validate without provider work:

```sh
effigy validate:focused swallowtail-adapter-codex
effigy check:examples
```

Authenticated Codex prompts, destructive lifecycle calls, and account checks
are optional operator-gated evidence, never deterministic acceptance.
