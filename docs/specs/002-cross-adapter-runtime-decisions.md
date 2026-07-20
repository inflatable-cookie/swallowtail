# 002 Cross-Adapter Runtime Decisions

Status: promoted
Owner: Tom
Updated: 2026-07-19

## Purpose

Settle runtime semantics against one-shot CLI, long-lived RPC or ACP, hosted
API, SDK, and self-hosted serving shapes before public Rust traits are drafted.

This specification is promoted through Contracts 008-011. Names describe
roles, not final Rust identifiers.

## Runtime Shape

Swallowtail runtime is a capability-led coordinator around explicit driver
instances. It is not a universal provider client and does not own routing
policy.

The minimum flow is:

```text
consumer requirements
  -> explicit configured instance and model route
  -> capability and host-service preflight
  -> scoped operation or session handle
  -> ordered events plus one terminal outcome
  -> joined cleanup result
```

The runtime has no implicit default provider, model, access profile, fallback,
or execution host.

## Async Posture

- Runtime execution is async-first. Network streams, subprocess streams,
  callbacks, cancellation, and deadlines must remain concurrently observable.
- Swallowtail does not initialize or own a global executor.
- The execution host supplies a task scope. Every spawned task belongs to an
  operation, session, owned instance, or discovery probe and is joined when
  that owner closes.
- No detached background task may outlive its owning scope.
- Blocking provider or SDK calls run through a host blocking-work facility or
  an adapter-private equivalent whose work is still joined by the owning
  scope.
- A synchronous consumer facade may be added outside the primary runtime
  contract. It must drive the same lifecycle rather than define different
  semantics.
- Contract 009 fixes object-safe boxed `Send` futures, `futures-core` streams,
  and an executor-neutral public boundary from compile-probe evidence.

## Driver And Instance Discovery

A driver descriptor declares:

- stable driver identity and version
- integration family and transport family
- supported execution layers and operation shapes
- required host services
- supported discovery and sign-in actions
- namespaced extensions it may emit or accept

A configured instance remains host-owned configuration. It identifies the
driver, endpoint or binary reference, execution host, ownership mode, access
profile references, and instance policy. Core records contain no secrets.

Discovery is explicit and bounded:

- a host enables drivers and discovery scopes
- a driver may probe only authorized binary locations, endpoints, services,
  or SDK installations
- probe results distinguish absent, discovered, incompatible, and failed
- discovery does not authenticate, download artifacts, start a persistent
  service, mutate configuration, or select a default route
- readiness, auth, entitlement, and model-catalog probes remain separate
- discovered candidates become usable only after the host creates or approves
  a configured instance

## Host Service Set

Host services are capability-scoped. A driver declares only what it needs;
missing required services fail preflight. No universal host object requires
every implementation to support processes, networks, files, or credentials.

### Task And Time

The host supplies scoped task execution, blocking-work placement, monotonic
time, sleeps, and deadlines. Wall-clock timestamps may annotate diagnostics
but never drive timeout correctness.

### Process Authority

Process drivers receive a narrow process service that can:

- resolve an approved executable reference
- spawn with approved working resources, arguments, environment references,
  and stdio configuration
- stream stdin, stdout, and stderr under size and redaction limits
- request graceful termination, force termination when authorized, and wait
  for exit
- report whether cleanup succeeded

Consumer or renderer input cannot nominate arbitrary executable paths,
environment variables, or working directories.

### Network Authority

Network drivers receive approved endpoint and connection policy rather than
unrestricted destination selection. The host controls endpoint allow-listing,
proxy, TLS, timeout, and remote placement.

Swallowtail does not force every adapter through one byte-level HTTP trait.
An SDK or provider client may own protocol implementation internally, but it
must use the configured endpoint and host policy and expose cancellation and
diagnostics through the common lifecycle.

### Credentials And Access

Public configuration carries opaque credential references and access-profile
metadata only. A credential service may provide one of:

- a short-lived scoped secret lease for the selected endpoint audience
- delegated authentication owned by a harness, SDK, cloud environment, or
  credential helper
- an explicit sign-in action the host may initiate with operator consent

Secret leases are non-serializable, redacted, audience-bound, and released
with the owning scope. Adapters do not inspect unrelated provider credential
stores.

Access status remains dimensional:

- credential state
- entitlement and quota state
- endpoint authorization
- runtime readiness
- support authority

There is no single authenticated or ready boolean that hides these states.

### Working Resources And Attachments

Consumers provide opaque resource and attachment references. The host resolves
them into scoped readable or writable capabilities on the execution host.
Adapters may request an allowed representation such as stream, bounded bytes,
temporary file, or provider upload. Raw client paths and arbitrary URLs are
not portable attachment identifiers.

Attachment descriptors carry safe metadata: media type, optional display name,
known length, optional digest, and declared role. Size, media, count, and
representation requirements participate in preflight.

Temporary materializations belong to the operation scope and are cleaned up
after provider/process cleanup.

### Event Delivery And Diagnostics

Adapters emit into an operation-scoped runtime event port. An operation handle
exposes the corresponding bounded ordered event stream to the execution host.
There is no process-global consumer callback or unbounded queue.

Provider payload normalization and default redaction happen before an event
enters the common stream. Restricted internal diagnostics may be sent to a
host diagnostic observer under explicit policy; they never enter public events
by accident.

## Operations And Handles

### Structured Run

Starting a structured run returns an owned run handle containing:

- stable Swallowtail request and run identity
- ordered event stream
- cancellation control
- one terminal-outcome future

The handle does not imply a provider session. A one-shot CLI may map the run to
one child process; a direct API may map it to one request; a harness may create
an internal provider thread or turn and expose opaque references.

### Interactive Session

Opening or resuming an interactive session returns an owned session handle.
The handle may start one active turn at a time unless an instance explicitly
advertises concurrency. Each turn has its own identity, event sequence,
cancellation scope, and terminal outcome.

Session close joins active work and releases owned resources. Resume requires
the exact configured instance and opaque provider session reference unless a
driver advertises and validates an explicit migration extension.

### Serving Instance

Self-hosted serving drivers distinguish:

- external attached instance
- host-owned ephemeral instance
- host-owned persistent instance

Cleanup may stop only an owned instance and follows its ownership mode.
Dropping a handle never stops an external attached service.

## Capability Preflight

Before provider work, the runtime intersects:

1. requested execution layer and operation shape
2. configured-instance capabilities
3. selected model-route capabilities
4. access-profile constraints
5. available host services
6. requested options and extension namespaces

Missing or incompatible requirements return structured failures before a
process, network request, SDK call, upload, or model load begins.

Boolean feature flags alone are insufficient where parameters matter.
Parameterized capability detail may describe attachment media and size,
schema dialect, event granularity, cancellation scope, context limits,
concurrency, tool modes, or reasoning support.

## Event Contract

- Sequence numbers are monotonic within one run or turn, not global.
- The runtime emits a start event before provider progress or output.
- Semantic events are never silently dropped.
- An event kind may declare itself coalescible; only those events may be
  replaced under pressure.
- Bounded delivery applies backpressure to the adapter boundary. Driver-specific
  readers may use a separately bounded ingress buffer when a provider cannot
  pause safely.
- Overflow of non-coalescible events is a terminal runtime failure, not silent
  loss or unbounded allocation.
- Exactly one terminal outcome completes the handle. Terminal outcomes are not
  inferred from the last progress event.
- Events arriving after a terminal outcome are quarantined as internal
  diagnostics.
- Runtime event replay is not guaranteed. Consumers persist normalized events
  when durable replay is product policy.

## Cancellation And Deadlines

Cancellation is cooperative first and forceful only within owned authority.

Supported scopes are:

- structured run
- active turn
- whole interactive session
- owned serving instance, when separately authorized

Rules:

- cancellation requests are idempotent
- request acknowledgement is distinct from terminal cancellation
- a deadline is an absolute monotonic deadline propagated through child work
- adapters use native interrupt or request cancellation when available
- otherwise they stop local consumption or request process termination as the
  driver's contract permits
- after an explicit cleanup grace period, the host may force-stop owned
  processes or connections
- external processes and services are never killed by generic cleanup
- the terminal outcome distinguishes completed, cancelled, timed out,
  provider failed, host failed, and cleanup failed
- cleanup failure remains visible even when the provider operation already
  produced output

## Tools, Permissions, And Steering

Tool calls, permission requests, filesystem callbacks, terminal callbacks, and
mid-turn steering are separate negotiated capabilities.

- A consumer supplies declarations and owns execution authority.
- Swallowtail transports calls and correlated responses only.
- Callback ids are distinct from operation, session, and provider ids.
- Missing callback support fails preflight when required.
- A provider waiting for a callback remains cancellable and deadline-bound.
- Unknown callback kinds remain namespaced extensions or fail explicitly.
- No generic runtime tool executor is introduced.

## Structured Output

Schema transport and output validation remain separate:

- a request may carry a bounded schema document or host-resolved schema
  reference plus media type, dialect, and optional stable digest
- a driver advertises accepted schema dialects and transport limits
- Swallowtail verifies transport compatibility and preserves the requested
  result media type
- provider-native schema rejection becomes a provider failure
- schema conformance, domain meaning, repair, ranking, and acceptance remain
  consumer-owned

Swallowtail may later offer an optional generic validation helper. It is not a
runtime success condition unless the consumer explicitly composes it.

## Extensions

Provider-specific operations and event data use stable driver-owned namespaces.
Core callers may preserve or reject unknown extensions. Extension support is
declared during discovery and preflight; protocol compatibility never implies
extension compatibility.

An extension cannot weaken credential audience, host authority, redaction,
cancellation, or ownership rules.

## Conformance Matrix For Contract Promotion

Card 009 must express fixtures for these representative shapes:

| Shape | Representative evidence | Required fixture behavior |
| --- | --- | --- |
| One-shot structured CLI | Codex `exec`, Claude/Cursor/Qwen headless | process start, ordered JSONL, output, cancel/deadline, exit and forced cleanup |
| Long-lived RPC or ACP harness | Codex app-server, Pi RPC, ACP agents | initialize, session/turn ids, callbacks, steering where declared, active-turn cancel, close and join |
| Hosted direct API | OpenAI, Anthropic, xAI, Kimi, GLM, Qwen, DeepSeek | scoped credential, endpoint policy, streamed response, no process requirement, provider cancellation limits |
| Attached self-hosted runtime | Ollama, llama.cpp, vLLM, SGLang | route discovery, observed capabilities, no provider credential where local, external service never stopped |
| Owned self-hosted runtime | host-launched llama.cpp or later Monkey route | readiness, model route, resource failure, owned cleanup and stop authority |

Every shape must prove applicable cases from this set:

- unsupported capability and missing host service fail before side effects
- execution layer, instance, route, access profile, and ownership remain intact
- event order and exactly-one terminal outcome hold
- bounded delivery does not silently lose semantic events
- cancellation and deadline produce distinct outcomes and joined cleanup
- external resources are not stopped by cleanup
- credential and internal diagnostics do not enter public output
- attachment authority and cleanup stay on the execution host
- schema transport does not imply domain acceptance
- unknown extensions follow explicit preserve or reject policy
- fallback never crosses execution, access, billing, support, or topology
  boundaries without consumer policy

## Promotion

- registration, configured-instance, access, requirements, and preflight rules:
  Contract 008
- object-safe async roles, handles, events, cancellation, outcomes, and cleanup:
  Contract 009
- host services, credentials, resources, attachments, schemas, and diagnostics:
  Contract 010
- deterministic fixture profiles and assertions: Contract 011
- implementation sequencing: g01 roadmaps 004-006

No provider adapter implementation is authorized by this specification alone.
