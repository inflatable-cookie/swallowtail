# System Architecture

Status: active
Owner: Tom
Updated: 2026-07-20

## Realized State

Swallowtail has a five-crate Rust workspace plus its strict Northstar authority
spine:

- `swallowtail-core` owns pure provider-neutral contract records, including
  runtime identities, access state, configured instances, model routes,
  reasoning catalog evidence, expanded interactive access policy,
  parameterized requirements, and side-effect-free preflight
- `swallowtail-runtime` depends on core plus `futures-core` and owns
  executor-neutral dynamic roles, lifecycle handles, bounded events, terminal
  outcomes, explicit operation policy, host-service ports, scoped
  materialization leases, and portable runtime inputs
- `swallowtail-testkit` depends on core and runtime and owns deterministic
  Contract 003, Contract 008, runtime-skeleton, and Contract 011 cross-shape
  fixtures and assertions, including distinct local and remote-authoritative
  execution-host identities
- `swallowtail-host-local` depends on core and runtime and implements concrete
  host-approved local process, materialization, and monotonic deadline behavior
  behind capability-scoped runtime ports
- `swallowtail-adapter-codex` depends on core and runtime and implements the
  read-only, ephemeral `codex exec` structured-run surface plus read-only and
  bounded-workspace app-server interactive sessions through runtime host ports

There is no concrete network behavior, global async executor, credential
store, or consumer dependency. JSON deserialization is confined to the Codex
adapter.

## Package Direction

The dependency direction is realized for core, runtime, and testkit. Adapter
layers remain planned:

```text
consumer applications
   |              |
host crates   provider adapter crates
   |              |
   +-----> swallowtail-runtime
                  |
          swallowtail-core

consumer and adapter tests -> swallowtail-testkit -> core/runtime contracts
```

Crate status:

- `swallowtail-core` — realized
- `swallowtail-testkit` — realized with reusable contract-kernel, preflight,
  and callback fixtures, recording runtime host services, and five composable
  provider-free conformance profile runners
- `swallowtail-runtime` — realized under Contracts 008-010 and 012 with only
  core and `futures-core` dependencies
- `swallowtail-host-local` — realized with host-owned approvals, bounded piped
  I/O, supervised exit, graceful EOF stop, explicit force-stop, and joined
  reader cleanup; it also owns bounded attachment/schema copies,
  operation-scoped temporary working resources, explicit lease release, and
  cancellable monotonic deadline waits
- `swallowtail-adapter-codex` — realized for bounded exec runs plus local stdio
  app-server model discovery and interactive sessions

Core, runtime, and testkit are realized and validated as one kernel. The five
synthetic Contract 011 profiles use only public APIs and cover one-shot CLI,
long-lived RPC, hosted API, attached self-hosted, and owned self-hosted shapes.
The Codex exec driver proves a real provider adapter can consume opaque prompt
content and exact preflight-bound executable, model, environment, and working
resource references without depending on the concrete local host crate. It
normalizes JSONL events, preserves final output behind redacted wrappers, and
owns cancellation and joined cleanup. Optional image, JSON Schema output,
reasoning, external search, and deadline inputs must match exact capability
constraints and actual host services before provider work. Schema and image
arguments use only scoped host-materialized leases. Invocation ignores ambient
user configuration and rules, permits a host-approved non-Git resource, denies
approval prompts, prevents tool subprocess environment inheritance, and states
read-only sandbox and web-search policy explicitly. Deadline expiry and
operator cancellation remain separate terminal outcomes; both join the process
and release every lease.

The separate Codex app-server driver owns long-lived JSONL-RPC framing and
request correlation over a shared process handle. A joined reader task routes
responses, notifications, and declared dynamic-tool requests into model-
catalog, session, turn, and callback runtime records. Provider thread, turn,
and tool-call ids remain opaque and distinct from runtime ids. Active-turn
interruption uses the provider method; whole-session cancellation force-stops
the owned child. Unsupported server requests still receive explicit provider
errors. Model discovery translates the provider's current supported reasoning
modes, reasoning default, model description, and provider-default marker into
provider-neutral catalog evidence; it does not select a model or reasoning mode
for later operations.

Interactive preflight now binds an expanded access policy. Read-only remains
the default without changing its Codex request shape. A bounded workspace plan
must require one `WorkingResource` capability constrained to `ReadWrite` and a
filesystem representation, the working-resource host service, and every
provider-request extension it may observe. Provider network and external
search remain separate capabilities and neither is present in the bounded
workspace profile. Missing or mismatched policy, capability, host service,
extension, resource reference, access mode, representation, or execution host
fails before process start.

For writable Codex sessions the host resolves one opaque resource into a
redacted filesystem lease. The adapter maps only that root into thread
`workspace-write` and turn `workspaceWrite`, denies network, excludes ambient
temporary roots, and keeps approval at `never`. The session handle retains and
releases the lease after provider cleanup. The request API has no raw-path or
secondary-root input. Local and remote-authoritative fixtures retain their
distinct service-set identities through preflight, resource resolution, open,
and joined close.

Declared Codex approval and user-input server requests normalize to bounded,
redacted provider extensions with distinct callback, runtime turn, provider
request, namespace, sequence, and deadline correlation. Observation grants no
authority and accepts no response: the adapter rejects the provider request,
interrupts the turn, and terminates with `ProviderRequestObserved`. Undeclared,
unknown, malformed, or mismatched callbacks remain explicit runtime failures.

Interactive session requests now carry optional redacted developer
instructions, an exact reasoning selection, and bounded tool declarations.
Turn handles may expose a one-shot callback exchange with a bounded request
stream and object-safe response port. Callback requests bind a distinct
redacted callback id to one runtime turn id, event sequence, optional monotonic
deadline, and bounded opaque payload. Testkit proves response correlation,
exactly-once state, timeout abandonment, late-response rejection, and matching
callback event order. The Codex driver translates preflight-bound developer
instructions, reasoning effort, and inline JSON Schema tool declarations into
the current app-server protocol. It opts into Codex's experimental API only
when an opened session carries dynamic tool declarations or the declared
user-input observation extension; other tool-free catalogue and session
connections do not inherit that provider capability. Its bounded
callback bridge accepts only declared tools, preserves independent turn and
callback observation, rejects late or mismatched responses, and abandons
provider waits on cancellation or deadline. Swallowtail never executes the
tool. The current provider schema cannot redeclare dynamic tools on
`thread/resume`, so tool-enabled resume is rejected before provider work
instead of silently losing declarations.

Every runtime host-service set now carries the execution-host id that owns its
task, process, resource, credential, network, and time ports. Both Codex
drivers reject a service set that does not match the immutable preflight plan
before host or provider work. Interactive session handles expose a resume
binding that keeps the opaque provider session reference attached to its
configured instance, execution host, model route, and model. Codex resume
rejects a mismatched binding before process start and rejects a provider that
returns a different session id. Turn events and callback requests must also
belong to the bound provider session.

Soundcheck's first consumer adoption exposed and closed two shared gaps. Codex
exec now emits distinct normalized external-search and safe-reasoning progress
while preserving agent activity, terminal structured output, and usage
snapshots. App-server model-catalogue requests may carry a host-monotonic
deadline; expiry closes input and joins the owned connection instead of
leaving discovery unbounded.

Provider-neutral fixtures run the same open, resume, callback cancellation,
active-callback close, interruption, unexpected disconnect, and joined cleanup
behavior against local and remote-authoritative host identities. Opaque target
and working-resource references reach only the selected host process port; no
raw client path or secret is introduced.

A generic public-API parity fixture now composes both Codex drivers with only
core/runtime records and host traits. It covers the complete first-consumer
transport seam without importing consumer types or policy. Soundcheck now uses
that seam for model discovery and every structured Codex turn. Product prompts,
schemas, validation, review, settings, and mutation remain downstream.

Both Codex surfaces coexist through ordinary provider-neutral dynamic
registration. They share only their integration family, access/route records,
host-service ports, diagnostics, and conformance vocabulary. Exec registers a
structured-run role over a structured CLI transport. App-server registers
model-catalog and interactive-session roles over JSONL-RPC stdio. Cross-bound
preflight plans reject before process work; neither surface inherits the
other's capabilities or lifecycle.

The structured-input boundary now distinguishes route transport from explicit
provider-side network and search policy. Reasoning selection is carried on the
operation and checked against exact preflight constraints; model-catalog
defaults remain evidence only. Working-resource, attachment-file, and
schema-file leases record cleanup authority and redact materialized host
values. The time port returns deadline observations without collapsing them
into consumer cancellation. The local host now resolves only approved opaque
attachment, schema, and working-resource references, bounds copied content,
rejects cross-scope lease use, removes host-owned material before reporting
clean release, and joins cancelled deadline waiters.

## Dependency Rules

- consumers depend toward Swallowtail; Swallowtail never depends on consumers
- core does not depend on runtime or provider adapters
- runtime does not depend on provider adapters
- provider extensions remain namespaced and optional
- UI frameworks and consumer persistence stay outside the crate graph
- execution happens on the host chosen by the consumer

## Architecture Promotion Rule

Move a planned package or boundary into this document only after it exists and
validation proves the dependency direction.
