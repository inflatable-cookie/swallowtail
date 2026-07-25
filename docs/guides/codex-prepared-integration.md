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
| `prepare_structured_exec` | exec | content, model route, model, working resource, network, search, reasoning, deadline, schema, attachment |

Catalogue, interactive session, and structured run remain different runtime
roles. Read-only and bounded workspace remain different methods. There is no
generic prompt method.

`CodexPreparedCatalogue`, `CodexPreparedSession`, and `CodexPreparedExec`
retain:

- exact installed-version and compatibility evidence
- access status and whether it was observed or caller-asserted
- the expanded immutable `PreflightPlan`
- the matching runtime request

Use `evidence()`, `plan()`, and `request()` before effects. `into_parts()`
transfers all three without discarding evidence.

## Bound Operations

Prepared profiles expose only the runtime role they implement:

| Prepared value | Bound operation |
| --- | --- |
| `CodexPreparedCatalogue` | `list_models(services)` |
| `CodexPreparedExec` | `start_run(services)` |
| `CodexPreparedSession` | `open_session(services)` |
| `CodexPreparedSession` | `resume_session(request_id, binding, services)` |

`resume_session` derives and validates the resume request before returning the
runtime future. Unsupported dynamic-tool redeclaration therefore fails before
provider effects.

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
payloads, credentials, and provider payloads. Keep `PreparationStage` and the
safe diagnostic code when projecting failures into application errors.

## Low-Level Escape Hatch

The facade is additive. `CodexExecDriver`, `CodexAppServerDriver`, discovery,
descriptors, claims, request records, preflight, and runtime handles remain
public. Each prepared value also exposes `low_level_driver()` when an advanced
consumer needs the exact selected Codex driver.

Use those surfaces when a consumer has a legitimate operation profile that the
named facade does not express. The consumer then owns exact configured-instance
construction, access and interface-version bindings, requirements, immutable
request agreement, and drift rejection. Do not bypass a preparation failure by
rebuilding a weaker low-level plan.
