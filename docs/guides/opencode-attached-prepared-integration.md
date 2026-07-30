# OpenCode Attached Prepared Integration

Use the prepared facade to connect to an operator-managed OpenCode HTTP
server. Swallowtail observes and invokes the selected service. It does not
start, stop, configure, authenticate, update, or recover the server.

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

## Version Posture

The guaranteed server window remains `1.14.48` through `1.18.10` across the
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
- prohibited provider-owned durable session state
- exact provider, model, route, endpoint target, and server version

`OpenCodePreparedSession::open_session` returns the unchanged interactive
session handle. Provider session identity, directory affinity, SSE ordering,
turn interruption, deadline behavior, and credential and resource cleanup
remain operation-scoped on the existing low-level lifecycle. Resume remains
unsupported. Closing a turn or session does not stop or dispose the attached
OpenCode service.

`OpenCodeSessionProfileInput::with_provider_callbacks()` enables the qualified
callback subset. Ordered OpenCode questions project into common typed harness
user input. Permission requests remain exact OpenCode extensions because
their one-shot authorization semantics are provider-specific. The consumer
answers both through the same correlated callback exchange.

## Structured Run

`prepare_run` requires a request identity, explicit provider and model route,
content, working-resource reference, and optional deadline. It derives a
separate `StructuredRun` plan with temporary provider retention and exact
operation-owned session deletion.

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
