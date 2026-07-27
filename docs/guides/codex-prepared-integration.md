# Codex Prepared Integration

Use the prepared facade for normal Codex integration. Use the low-level drivers
when an application needs a profile the facade does not provide.

## Normal Flow

1. The consumer selects the Codex driver: app-server or structured exec.
2. The execution host approves one exact executable and returns an opaque
   `InstalledExecutableTarget`.
3. The consumer supplies its stable instance identity, host identity, saved
   environment reference, access profile, and observed or explicitly asserted
   access evidence.
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

## Named Profiles

| Prepared path | Driver | Consumer choices |
| --- | --- | --- |
| `prepare_catalogue` | app-server | request identity and optional deadline |
| `prepare_read_only_session` | app-server | model route, model, working resource, instructions, reasoning, tools |
| `prepare_bounded_workspace_session` | app-server | the same session inputs plus explicit writable-profile selection |
| `prepare_archive_session` | app-server | request identity, inactive management binding, optional deadline, explicit unverified-newer acceptance |
| `prepare_restore_session` | app-server | request identity, inactive management binding, optional deadline, explicit unverified-newer acceptance |
| `prepare_delete_session` | app-server | request identity, inactive management binding, optional deadline, explicit unverified-newer acceptance |
| `prepare_structured_exec` | exec | content, model route, model, working resource, network, search, reasoning, deadline, schema, attachment |

Catalogue, interactive session, and structured run remain different runtime
roles. Read-only and bounded workspace remain different methods. There is no
generic prompt method.

`CodexPreparedCatalogue`, `CodexPreparedSession`, the three lifecycle values,
and `CodexPreparedExec` retain:

- exact installed-version and compatibility evidence
- access status and whether it was observed or caller-asserted
- the expanded immutable `PreflightPlan`
- the matching runtime request

Use `evidence()`, `plan()`, and `request()` before effects. Catalogue, session,
and exec values also provide `into_parts()`. Lifecycle values retain the
prepared environment internally for their typed `execute` method.

## Bound Operations

Prepared profiles expose only the runtime role they implement:

| Prepared value | Bound operation |
| --- | --- |
| `CodexPreparedCatalogue` | `list_models(services)` |
| `CodexPreparedExec` | `start_run(services)` |
| `CodexPreparedSession` | `open_session(services)` |
| `CodexPreparedSession` | `resume_session(request_id, binding, services)` |
| `CodexPreparedArchive` | `execute(services)` |
| `CodexPreparedRestore` | `execute(services)` |
| `CodexPreparedDelete` | `execute(services)` |

`resume_session` derives and validates the resume request before returning the
runtime future. Unsupported dynamic-tool redeclaration therefore fails before
provider effects.

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

## Explicit Limits

- A model is always explicit. Catalogue defaults are display evidence, not
  route selection.
- App-server dynamic tools are consumer declarations. Swallowtail transports
  callbacks but never executes the tools.
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
