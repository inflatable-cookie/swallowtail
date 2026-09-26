# 010 Execution Host Services and Inputs

Status: active
Owner: Tom
Updated: 2026-09-03

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

Ordinary scoped-task ownership remains joined: `spawn` returns a `JoinedTask`,
explicit `join` observes its completion, and a concrete host may retain
join-on-drop ownership. A caller-bound operation that reaches its deadline
while such a task is unfinished must not drop or synchronously join the handle
on the caller path.

Before a route starts work that may need that return path, the exact selected
task service grants one operation-scoped reap reservation, or an equivalent
atomic capability with the same guarantees. The grant is bound to one exact
execution host and `ScopeId` and reserves the capacity and lifecycle authority
needed to accept one later unfinished task. It is acquired before provider
work and before credentials, working resources, processes, or tasks are
acquired or started. Unsupported hosts, closed reservation admission, and
unavailable capacity reject at this point with no such effect.

Before its work is polled, the grant is bound atomically to one new task by an
additive reapable-task start operation, or by an equivalent combined reserve-
and-start operation. Once bound, reserved capacity cannot be released
independently from that task. Ordinary completion and join release it; accepted
relinquishment transfers it with the task. A reservation that was never bound
may be released without starting work.

Starting join on a reservation-backed task transfers the worker into
cancellation-safe host ownership before returning its observation future.
Dropping that future before polling or after a pending poll may abandon the
observation, but it cannot discard the live worker or release the shutdown
barrier. The host still joins or reaps the worker and settles the reservation.
Moving both worker and reservation only into a lazy join future is detached
execution when that future is cancelled. Ordinary unreserved join and concrete
join-on-drop behavior remain unchanged.

A service-kind check or boolean `supports_reap`-style probe is explicitly
insufficient. Host shutdown or capacity exhaustion can race the gap between a
probe and a later transfer. Only the held reservation, or an atomic
reserve-and-start operation with equivalent ownership, authorizes a route whose
caller may return before task completion.

`ScopedTaskService::relinquish` is the narrow ownership-transfer seam for the
reserved case. It accepts only an unfinished reservation-backed task created
under the same exact `ScopeId` and execution host. Success clears the caller's
task slot only after the host has accepted autonomous ownership and returns
`TaskRelinquishOutcome::AcceptedForReap`. The outcome means neither finished
nor joined. The host must reap the task after it finishes without a second
adapter call or adapter-global parking. A valid reserved exact-host/exact-scope
transfer cannot fail because shutdown began, capacity was consumed elsewhere,
or the host lifecycle advanced after the grant. Wrong-host, wrong-scope,
already-finished, repeated, forged, or released reservations fail closed and
leave ordinary task ownership with the caller.

The concrete selected-host composition retains and supervises accepted work and
its reapers through an outer lifecycle owner. Shutdown first stops new
reservations, then waits for every issued reservation to be released unused or
settled and for every accepted task to finish, then joins all retained reapers
outside the task tree. Task-service clones carry only weak reservation and
handoff authority and never join reapers on drop; discarding a reaper handle is
detached execution, not host ownership.

Relinquishment preserves the caller deadline; it is never cleanup-completion
evidence. An adapter may use it only to record that the selected host took
back ownership. It cannot derive `CleanupOutcome::Clean`, a joined task, or a
completed provider/session cleanup stage from acceptance for reap.

An unused reservation that was never bound to a task is released explicitly or
by its non-blocking drop path. Release does not join a task. A bound reservation
cannot be dropped away from its task. Ordinary `spawn`, explicit `join`, and
concrete join-on-drop behavior remain unchanged; the reserved path is additive
and cannot silently make every task detachable.

A concrete host composition may derive a deadline from its monotonic clock and
one caller-supplied duration. The caller still decides whether a deadline
exists and selects the duration. The host owns conversion into its monotonic
tick representation and saturates overflow at the latest representable
instant. This convenience supplies no default timeout and observes no deadline.

The same host time boundary may expose a UTC catalogue observation clock.
Drivers use it only to timestamp evidence observed by the current operation.
It remains distinct from provider-reported lifecycle timestamps and monotonic
deadline time. A host that cannot supply it fails that catalogue operation;
the driver does not substitute provider `modified_at`, request time, or an
ambient process clock.

## Caller-Bounded Interactive Cleanup

Every public `InteractiveSessionHandle::close` consumes one
`SessionCleanupRequest` and the exact `HostServices` set for that session. The
request carries one absolute caller-selected `Deadline`; it carries no
duration, default timeout, provider setting, or ambient-clock authority.

The close boundary covers the whole remaining session lifecycle: active-turn
interruption, provider-native close, required escalation, pump and task joins,
resource release, and credential release. A stage cannot select a later
deadline or keep the public close future pending after the shared boundary.
Post-open abort and cleanup after turn expiry use the same rule.

The runtime validates the execution-host identity and requires that host's
time service before polling cleanup. It observes host time before cleanup and
again before accepting a ready cleanup result. Missing or cross-host time, an
already elapsed deadline, or expiry while cleanup is pending returns an
honest failed cleanup outcome and drops the remaining cleanup future. Expiry
is never reported as clean. Duration-to-tick conversion, when wanted, remains
the caller's host-time operation described above.

Contract 037 permits a host crate to compose these services through one
inspectable per-host builder or result. Composition retains one exact host id,
registers only explicitly supplied services, and owns joined task work. It is
not a global executor and grants no service, process, network, credential, or
resource authority by default.

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
- report root exit and, where a concrete mechanism proves it, owned-tree
  completion
- report cleanup state

Process completion evidence separates two facts. Root exit reports whether the
one spawned root process ended and with which platform code. Owned-tree
completion reports whether the host observed that no member of its exact owned
descendant tree remains. A host reports the attested state only from a
concrete mechanism that made that observation. Exit code, a graceful stop
request, a successful force-stop request, and a successful nearest-child wait
are never that observation, and no host infers it from a platform name. A bare
process-group number, an inherited descriptor's end-of-file, and an ancestry
walk are each insufficient: a descendant escapes them by calling `setsid`, by
closing or not inheriting the descriptor while alive, or by being reparented
after its intermediate parent exits. A sound observation needs a mechanism whose
owned-tree identity a descendant cannot leave by any of those, with exclusive
host ownership and denied migration out of the owned set. A host whose mechanism
cannot observe emptiness reports root-only evidence. Root-only evidence is
honest, not a failure, and does not weaken that host's enrollment, termination,
or cleanup authority.

Executable selection and host approval may occur before installed discovery.
That host action produces one opaque approved target. It does not change
Contract 032: the discovery role receives one target and performs no ambient
search or fallback.

One executable reference may resolve to either one native program or one
host-private launch recipe. A launch recipe contains:

- one exact interpreter or native program
- bounded immutable prefix arguments, such as one exact script path
- optional bounded bootstrap environment owned by the execution host

The host appends driver-supplied process arguments after the immutable prefix.
It clears ambient environment, applies bootstrap values first, then applies
explicit environment references carried by the process request. The composed
argument vector remains subject to the same host limits as a native launch.
Bootstrap environment is only launcher authority. It cannot carry credentials,
provider configuration, model selection, working-resource authority, or
consumer prompt content.

The recipe remains behind the opaque executable reference. Stable records,
default formatting, events, and diagnostics expose no program, script path,
prefix argument, or bootstrap value. A process host does not invoke a shell,
search `PATH` at execution time, infer a provider-specific launcher inside an
adapter, or substitute another recipe after a request begins. Native execution
is the zero-prefix, empty-bootstrap case.

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

## Interactive Sign-In Ports

Contract 057 sign-in loops require optional host service kinds that do not
collapse into Credential, Process, or Network:

- open a host-approved URL
- bind a loopback callback for one sign-in operation
- display a device code

Spawning an approved login helper stays process authority. These ports never
return secret bytes to portable records, never embed a browser or keychain,
and never start sign-in by being registered. A loop that needs a missing port
fails closed.

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

`DiagnosticObserver` is optional host registration. When present, adapters and
runtime helpers may emit:

- `Diagnostic` records that carry safe public fields plus optional restricted
  `internal_detail`
- structured `DebugObservation` records governed by Contract 053

When absent, emission is a no-op. Observer registration is never required for
ordinary preparation or execution. Observer sink failure must not alter
lifecycle, classification, cleanup, or route selection. Contract 053 owns the
observation vocabulary, bounds, redaction, and non-interference rules.

## Extensions And Fallback

Host-service and input extensions use stable namespaces and cannot weaken
authority, ownership, access audience, redaction, cancellation, or cleanup.

Contract 059 adds one optional stable watcher service kind. It owns only
turn-scoped watcher requests accepted under host policy, bounded status and
summary output, model and operator stop, deadline propagation, and joined
cleanup. Registration alone starts nothing and grants no arbitrary process or
PID authority. A process-backed watcher may use the ordinary process service
when the host binds its root handle and available process-tree cleanup before
publishing watcher identity, retains stop authority, and joins process and
supervision work before turn completion. This is lifecycle ownership, not a
security-containment claim. Deliberately detached or daemonized work is outside
the watcher contract and must not be approved as a watcher operation.

Contract 060 adds a separate optional stable watcher-bridge service kind. It
opens one operation-scoped, provider-neutral listener lease bound to the exact
execution host, runtime operation, turn, and registered watcher service.
Registration binds nothing. Endpoint and authentication material are
driver-only, non-serializable, and redacted. The bridge carries only the
bounded reserved watcher protocol and completion query, freezes admission
before successful completion, and joins listener and dispatch work before
releasing private material. It cannot reuse the sign-in loopback callback,
network service, serving-endpoint publication, or watcher registration as
listener authority.

No host service performs route fallback. Consumers explicitly authorize any
change of execution layer, credential mechanism, entitlement, endpoint,
billing, support authority, privacy posture, ownership, or topology.

## Acceptance

- caller-bounded relinquishment returns after exact-host/scope acceptance,
  without waiting for unfinished task completion
- unsupported or closing hosts reject the reservation before credential,
  resource, process, provider, or task work
- a held reservation makes later exact-host/scope relinquishment immune to
  shutdown and capacity races; a boolean capability probe does not
- the reservation binds to one task before its work is polled and cannot be
  released independently while that task remains live
- wrong-host, wrong-scope, finished, repeated, forged, and released-reservation
  relinquishment retain ordinary caller ownership; accepted-for-reap never
  means joined
- selected-host shutdown stops reservation admission, waits existing
  reservations and accepted tasks, then joins every retained reaper outside
  the task tree even when a worker captures a task-service clone
- ordinary spawn, explicit join, and join-on-drop behavior remain unchanged
- a hosted API driver executes without process service
- a one-shot CLI fails preflight without process service
- delegated harness authentication does not require secret extraction
- interactive sign-in ports do not embed a browser, keychain, or secret
- a 057 sign-in loop fails closed when a required port is missing
- raw paths and secrets are absent from portable requests and diagnostics
- native and interpreted launches both clear ambient environment
- immutable launcher prefix arguments participate in host argument limits
- launcher recipes expose no program, path, prefix, or bootstrap value
- attachment and temporary-resource cleanup remains operation-scoped
- consumer-resource cleanup remains consumer-owned
- materialized host paths remain redacted and are available only through
  explicit driver accessors on scoped leases
- structured-output transport does not imply consumer validation
- service extensions cannot bypass host authority
