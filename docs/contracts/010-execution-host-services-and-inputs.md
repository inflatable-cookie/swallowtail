# 010 Execution Host Services and Inputs

Status: active
Owner: Tom
Updated: 2026-07-23

## Purpose

Define capability-scoped host authority for tasks, time, processes, networks,
credentials, working resources, attachments, schemas, events, and diagnostics.

## Service Registration

The execution host registers object-safe services by stable service kind. One
service set carries the stable execution-host id that owns every registered
service. Drivers declare required kinds per role. Preflight rejects missing
services, and execution rejects a service set from a different host before
using it.

There is no mandatory god trait requiring every host to support process,
network, credential, or filesystem operations. A typed service set or registry
may carry optional service trait objects while preserving static service kinds
for preflight.

## Task And Time Services

The task service owns scoped async and blocking work and joins all child tasks.
The time service provides monotonic time and deadline observation. A
`wait_until` future resolves to a `DeadlineObservation`; it does not request
cancellation or choose a terminal outcome. Wall-clock time may annotate safe
records but never controls timeout correctness.

The same host time boundary may expose a UTC catalogue observation clock.
Drivers use it only to timestamp evidence observed by the current operation.
It remains distinct from provider-reported lifecycle timestamps and monotonic
deadline time. A host that cannot supply it fails that catalogue operation;
the driver does not substitute provider `modified_at`, request time, or an
ambient process clock.

## Operation Policy

`OperationPolicy` keeps provider-side access separate from route transport:

- `ExternalNetworkPolicy::Denied` or `HostApproved`
- `ExternalSearchPolicy::Disabled` or `Enabled`
- an optional exact `ReasoningMode`

External search cannot be enabled while provider-side external network access
is denied. Host-approved means the selected execution host may apply its own
network policy; it is not unrestricted network authority. Transport access to
a selected hosted API endpoint does not enable provider tools or web search.
Catalog defaults never silently fill operation policy.

## Process Service

The process service may:

- resolve a host-approved executable reference
- run one Contract 032 target-aware installed-executable version probe without
  ambient search
- spawn with approved arguments, environment references, working resources,
  stdio, and limits
- exchange bounded stdin, stdout, and stderr
- request graceful stop, force-stop owned children, and wait for exit
- report cleanup state

Renderer or remote-client data cannot establish authority by naming an
executable path, environment variable, or working directory. Process output is
not a safe diagnostic until normalized and redacted.

Drivers may write opaque operation content to bounded stdin. Executables,
environment, and working directories remain host-owned references. Default
formatting must not expose argument, environment, stdin, stdout, or stderr
bodies.

Long-lived process drivers may concurrently read framed output and write
correlated requests through one shared process handle. The driver owns framing,
correlation, and protocol state; the host retains process ownership, I/O
bounds, and stop authority.

## Network Service

The network service resolves host-approved endpoint references and policy for
destination, proxy, TLS, connection timeout, and execution-host placement.

Swallowtail does not force SDKs and provider clients through one lowest-common
denominator byte transport. A driver may own its protocol client internally,
but it must use approved instance configuration, remain cancellable through the
common lifecycle, and expose safe normalized diagnostics.

Contract 014 makes authorization operation-scoped and audience-bound and
requires one redacted driver-usable endpoint value. Route transport remains
separate from provider-side network and search policy.

## Credential Service

Public records use opaque credential references. The credential service may
return:

- a scoped secret lease bound to one endpoint audience
- delegated authentication owned by a harness, SDK, cloud environment, or
  credential helper
- a supported sign-in action requiring host and operator authorization

Secret leases are non-serializable, redacted in `Debug` and `Display`, scoped
to the operation or instance, and released during cleanup. Drivers cannot scan
unrelated credential stores or replay credentials across endpoint audiences.

Contract 014 requires exact operation-scope and audience binding plus an
explicit awaited release boundary. Delegated harness authentication still
exposes no secret.

## Working Resources

Consumers identify product resources. The host resolves them into scoped
read/write capabilities on the execution host. Public requests do not carry
arbitrary paths. Resolution receives the owning operation scope.

The working-resource service may also create a temporary resource for one
operation. A `ResourceLease` records access, representation, and cleanup
authority:

- a resolved consumer resource has `Consumer` cleanup authority and must not
  be deleted by Swallowtail
- a host-created temporary resource has `OperationScope` cleanup authority and
  is released only after provider/process cleanup

Every resource lease records its owning scope. `WorkingResourceService::release`
is the explicit awaited cleanup boundary: consumer-owned leases return
`NotApplicable`; operation-scoped leases remove only material owned by the same
host and scope and report cleanup failure rather than hiding it.

Driver requests for working-resource representation and access mode participate
in preflight. A write-capable harness cannot run against a read-only resource
capability without an explicit failure.

Contract 015 adds a distinct `WorkingResourceIo` service for bidirectional
protocols whose agent calls back into the client filesystem. The callback port
does not expose general filesystem authority. Each operation repeats the owning
scope and resource lease, resolves under that lease, applies content bounds,
and rejects traversal or cross-host use before I/O. A resolved filesystem path
alone does not authorize a callback implementation inside an adapter.

Contract 017 extends that port with bounded text replacement under an exact
`ReadWrite` filesystem lease. Callback mediation does not approve a provider
tool or contain other process filesystem paths. A driver whose harness can
bypass the callback requires separately preflight-bound and tested provider or
execution-host containment before claiming bounded filesystem access.

## Attachments

A portable attachment contains safe metadata and an opaque host reference:

- media type
- optional display name
- declared role
- known length when available
- optional digest

The attachment service may materialize an approved stream, bounded bytes,
temporary file, or provider upload according to driver capability. Raw client
paths and arbitrary URLs are not portable attachment references. Requests are
bound to an operation scope.

For file-oriented drivers, `materialize_file` accepts the safe attachment
descriptor and returns an `AttachmentFileLease`. The lease exposes one
host-authorized `MaterializedFileRef` through an explicit driver accessor,
redacts it from formatting, and fixes cleanup authority to `OperationScope`.
`AttachmentService::release_file` is awaited after provider/process work and
must finish removal before returning `Clean`.

Count, size, media, representation, and transport limits participate in
preflight. Temporary materialization and uploads belong to the operation scope
and clean up after provider/process work.

## Model Artifacts

Model artifacts are not attachments. Contract 018 adds opaque artifact
references and serving-scope, read-only leases for owned ephemeral servers.
Artifact resolution stays on the execution host, driver accessors remain
redacted, and release follows owned-child join without deleting consumer-owned
model material.

## Schemas And Results

A structured-output request carries a bounded schema document or opaque schema
reference plus media type, dialect, and optional digest. Drivers declare
accepted dialects and transport limits. `SchemaService::materialize_file`
resolves either document form within the owning scope and returns a redacted
operation-scoped `SchemaFileLease` usable by a file-oriented driver.
`SchemaService::release_file` provides the matching awaited cleanup boundary.
Hosts reject release attempts for material they do not own or for a different
operation scope.

Swallowtail owns transport compatibility and result media preservation. The
consumer owns schema meaning, validation, repair, ranking, and acceptance. An
optional validation helper may be composed later but is not implicit runtime
success.

## Event And Diagnostic Services

The runtime supplies an operation-scoped event-emission port to adapters and
exposes a bounded stream on the operation handle. There is no global event
callback.

Provider normalization and default redaction occur before common event
delivery. A host diagnostic observer may receive restricted internal details
under explicit policy. Raw provider payloads, prompts, outputs, stderr, tokens,
secret values, credential-store paths, and sensitive host paths never enter
public events or default formatting.

## Extensions And Fallback

Host-service and input extensions use stable namespaces and cannot weaken
authority, ownership, access audience, redaction, cancellation, or cleanup.

No host service performs route fallback. Consumers explicitly authorize any
change of execution layer, credential mechanism, entitlement, endpoint,
billing, support authority, privacy posture, ownership, or topology.

## Acceptance

- a hosted API driver executes without process service
- a one-shot CLI fails preflight without process service
- delegated harness authentication does not require secret extraction
- raw paths and secrets are absent from portable requests and diagnostics
- attachment and temporary-resource cleanup remains operation-scoped
- consumer-resource cleanup remains consumer-owned
- materialized host paths remain redacted and are available only through
  explicit driver accessors on scoped leases
- structured-output transport does not imply consumer validation
- service extensions cannot bypass host authority
