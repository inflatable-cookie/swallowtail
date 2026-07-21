# System Architecture

Status: active
Owner: Tom
Updated: 2026-07-21

## Realized State

Swallowtail has a twelve-crate Rust workspace plus its strict Northstar authority
spine:

- `swallowtail-core` owns pure provider-neutral contract records, including
  runtime identities, access state, configured instances, model routes,
  model-artifact identity and preflight bindings, reasoning and token-limit
  catalog evidence, expanded interactive access and harness-isolation policy,
  parameterized requirements, and side-effect-free preflight
- `swallowtail-runtime` depends on core plus `futures-core` and owns
  executor-neutral dynamic roles, lifecycle handles, bounded events, terminal
  outcomes, explicit operation policy, typed usage/rate/quota observations,
  scope- and audience-bound endpoint/credential ports, scoped
  materialization leases, read-only model-artifact leases, scoped owned-serving
  endpoint publication, a distinct working-resource callback I/O port, and
  portable runtime inputs
- `swallowtail-testkit` depends on core and runtime and owns deterministic
  Contract 003, Contract 008, runtime-skeleton, and Contract 011 cross-shape
  fixtures and assertions, including distinct local and remote-authoritative
  execution-host identities
- `swallowtail-host-local` depends on core and runtime and implements concrete
  host-approved local process, endpoint, credential, materialization, and
  monotonic deadline behavior behind capability-scoped runtime ports
- `swallowtail-protocol-acp` is the provider-neutral ACP wire boundary; it owns
  bounded v1 NDJSON framing and message classification plus a fixture corpus
  pinned independently to Gemini CLI `0.51.0`/schema `v1.19.0` and Kimi Code
  `0.28.1`/schema `v1.19.1`; the Kimi corpus also freezes exact annotated-tag,
  source-commit, arm64 executable, isolated-state, and upgrade-gate evidence
- `swallowtail-adapter-codex` depends on core and runtime and implements the
  read-only, ephemeral `codex exec` structured-run surface plus read-only and
  bounded-workspace app-server interactive sessions through runtime host ports
- `swallowtail-adapter-opencode` implements version-bound OpenCode `1.14.48`
  model discovery and ambient-host interactive sessions with read-only tool
  permissions over host-approved HTTP and bounded SSE
- `swallowtail-adapter-anthropic` implements provider-supported `2023-06-01`
  Models catalogue and Messages direct inference over host-approved HTTP/SSE
- `swallowtail-adapter-gemini` implements the pinned Gemini CLI `0.51.0`
  ambient-host interactive subset with Plan Mode and bounded read callbacks
  over ACP v1 stdio
- `swallowtail-adapter-kimi` implements the pinned Kimi Code `0.28.1`
  ambient-host interactive lifecycle with distinct new, load-with-replay, and
  replay-free resume plus bounded write callbacks over ACP v1 stdio
- `swallowtail-adapter-llama-cpp` implements attached llama.cpp build `9910`
  readiness, catalogue, and bounded Chat Completions direct inference without
  owning the model artifact or server
- `swallowtail-adapter-xai` implements resource-free interactive direct
  inference over one host-approved Responses WebSocket with serial turns,
  private continuation, exact billed cost, and connection-ending cancellation

There is no global async executor, durable credential store, or consumer
dependency. The concrete libcurl client remains private to the OpenCode
adapter and runs only through host blocking-work ports.

## Package Direction

The dependency direction is realized across core, runtime, hosts, and adapters:

```text
consumer applications
   |              |
host crates   provider adapter crates
   |              |
   +-----> swallowtail-runtime
                  |
          swallowtail-core

consumer and adapter tests -> swallowtail-testkit -> core/runtime contracts

provider adapter crates -> swallowtail-protocol-acp
```

Crate status:

- `swallowtail-core` — realized
- `swallowtail-testkit` — realized with reusable contract-kernel, preflight,
  and callback fixtures, recording runtime host services, and nine composable
  provider-free conformance profile runners
- `swallowtail-runtime` — realized under Contracts 008-010 and 012 with only
  core and `futures-core` dependencies
- `swallowtail-protocol-acp` — realized for bounded ACP v1 NDJSON framing,
  request/notification/response classification, and safe error responses
- `swallowtail-host-local` — realized with host-owned approvals, bounded piped
  I/O, supervised exit, graceful EOF stop, explicit force-stop, and joined
  reader cleanup; it also owns bounded attachment/schema copies,
  operation-scoped temporary working resources, explicit lease release, and
  cancellable monotonic deadline waits; exact endpoint and secret/delegated
  credential approvals remain scope- and audience-bound and redacted
- `swallowtail-adapter-codex` — realized for bounded exec runs plus local stdio
  app-server model discovery and interactive sessions
- `swallowtail-adapter-opencode` — realized for attached model catalogue and
  read-only interactive sessions over a six-route HTTP/SSE subset with exact
  delegated-auth, version, provider/model, abort, deadline, and cleanup bounds
- `swallowtail-adapter-anthropic` — realized for the frozen two-route public-
  API subset with API-key and version headers, bounded catalogue pagination,
  explicit output bounds, ordered SSE, usage, rate and request evidence,
  provider errors, unknown events, local cancellation, one inference attempt,
  joined work, and awaited credential release
- `swallowtail-adapter-gemini` — realized for pinned initialization, isolated
  API-key process state, Plan Mode new sessions, text prompts, ordered updates,
  host-bounded reads, permission cancellation, active-turn cancellation, and
  joined process close
- `swallowtail-adapter-kimi` — realized for pinned initialization, isolated
  harness state, delegated membership auth, exact persistent bindings, new,
  load, ordered replay, resume, prompt, bounded text replacement, native turn
  cancellation, disconnect, and joined process/resource/credential cleanup
- `swallowtail-adapter-xai` — realized for one exact API-key WebSocket route,
  one active turn, private `store=false` continuation, ordered text and usage,
  exact USD ticks, distinct provider failures, cancellation, deadline,
  disconnect invalidation, and joined credential cleanup

Core, runtime, and testkit are realized and validated as one kernel. The nine
synthetic Contract 011 profiles use only public APIs and cover one-shot CLI,
long-lived RPC, long-lived ACP, attached network harness, hosted API, attached
self-hosted, owned self-hosted, connection-scoped direct-session, and a
persistent ACP extension. The extension composes load, replay, resume, bounded
write, delegated-auth, ambient-authority, topology, and cleanup claims without
widening the baseline ACP profile.
The owned profile now binds one exact model artifact into preflight, rejects
artifact substitution before host effects, acquires a distinct read-only
artifact lease, publishes a redacted execution-host endpoint binding, and
retains stop authority only on the owned handle. The local host resolves only
exact approved regular-file artifacts, verifies their digest, and publishes
only scoped nonzero loopback endpoints under the bound execution host.
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

Interactive preflight now binds an expanded access policy. Generic local
harness requests default to explicit `AmbientHost`; provider- or host-enforced
isolation is opt-in. Codex selects its provider-enforced read-only profile
explicitly without changing the provider request shape. A bounded workspace
plan must require one `WorkingResource` capability constrained to `ReadWrite`
and a filesystem representation, the working-resource host service, and every
provider-request extension it may observe. Provider network and external
search remain separate capabilities and neither is present in the bounded
workspace profile. Missing or mismatched policy, isolation posture,
capability, host service, extension, resource reference, access mode,
representation, or execution host fails before process start.

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

Hosted transport foundations now expose one non-empty host-approved endpoint
value only through a redacted driver accessor. Network and credential grants
bind operation scope, opaque reference, and endpoint audience. Secret and
delegated credential leases require explicit awaited host release; the local
host tracks issued leases and rejects foreign scope/reference/audience cleanup.
It does not scan ambient credential stores.

Structured-run resources are optional at the common operation boundary so a
direct API needs no fake workspace. Codex exec keeps its prior requirement and
fails before process start when the resource is absent. Optional model token
limits remain mutable catalogue evidence. Token usage, rate limits, and quota
are distinct semantic observations and carry no retry, fallback, or billing
behavior. Codex exec now emits typed usage instead of formatted progress text.

Access profiles may now retain one opaque credential reference inside the
immutable preflight binding. Plans expose that reference, its credential
mechanism, and its endpoint audience to drivers without exposing secret bytes.
For network drivers, the configured-instance target converts directly to the
host endpoint reference. Harness catalogue entries and model routes may retain
a separate provider id instead of forcing it into model identity.

ACP v1 evidence now has a separate protocol package boundary. Wire version,
schema artifact version, SDK version, and agent version remain distinct.
Gemini CLI `0.51.0` is narrowed to new read-only sessions, text prompts,
updates, native turn cancellation, permission cancellation, and bounded read
callbacks. Authentication mutation, load/resume, mode or model switching, MCP
injection, writes, terminals, and native session close are not claimed.
Contract 015's `WorkingResourceIo` service is realized separately from
consumer tools. The local host canonically resolves provider locators under
the exact leased root, applies line and byte bounds, and rejects traversal,
symlink escape, wrong representation, and unapproved resources before content
is returned. The Gemini adapter advertises reads only when that host service is
present.

The Gemini ACP proof passes deterministic production-driver fixtures for
successful text/update/read flow, permission observe-and-stop, native prompt
cancellation, disconnect, event-stream closure, and joined cleanup. Its
working resource scopes callbacks and working-directory selection but is not a
process sandbox. The immutable policy therefore records `AmbientHost`. Local
and remote-authoritative host identities exercise the same public driver seam.
ACP prompt correlation is installed and written before its joined waiter task
starts, so immediate cancellation cannot overtake the provider request. The
installed `0.51.0` probe is separately gated and ignored by default.

The Kimi corpus and production adapter prove the maintained TypeScript successor's new, load, resume,
replay, prompt, cancellation, write-callback, drift, auth-failure, and
disconnect shapes without changing shared framing. It pins `0.28.1` source and
ACP artifact identities separately and rejects ambient executable discovery,
ambient state, and self-upgrade paths. The local process host sets a working directory
but does not sandbox descendants. Current platform evidence disqualifies
Landlock alone as incomplete, deprecated macOS `sandbox-exec` as unsupported,
and experimental Windows process-sandbox APIs as unstable. A native arm64
probe proves that a security-scoped project grant propagates through a
compatible inherited App Sandbox helper to shell and background descendants.
The exact Kimi `0.28.1` single executable cannot retain V8 and extracted-
native-module runtime behavior under the documented helper signature, so that
`HostEnforced` profile is unavailable. Harness communication uses an explicit
`AmbientHost` Kimi mapping that makes no bounded filesystem, descendant, or
provider-tool network claim. One shared ACP decoder accepts both pinned agent
corpora. Local and remote-authoritative fixtures prove exact persistent
binding, replay-before-readiness, replay-free resume, bounded write authority,
redaction, cancellation, disconnect, and process-before-resource-before-
credential cleanup. The installed Kimi probe is separately gated and ignored
by default.

The llama.cpp adapter is an attached direct-inference driver, not a model or
server manager. Its configured instance binds build `9910`, an external
loopback endpoint, local unauthenticated access, and the bounded
`llama.cpp.openai-chat-completions.b9910` facade. The operator-supplied GGUF,
`llama-server`, configured deployment, facade, and model alias remain distinct.

Catalogue and structured-run operations acquire a host-approved endpoint and
observe `/health` plus `/props` before model or inference work. Build, alias,
ChatML capabilities, and text-only modalities must match the frozen
deployment. The driver then uses `/v1/models` or one streaming
`/v1/chat/completions` attempt. It exposes no provider id, process service,
credential service, serving-lifecycle role, artifact mutation, retry, tool,
reasoning, schema, or multimodal claim. Cancellation and deadline stop local
connection work; run close joins owned work and never stops the external
server. Local and remote-authoritative execution-host identities use the same
public driver seam.

A separate llama.cpp owned driver binds build `10069`, one host-approved GGUF,
one host-approved executable, `HostOwnedEphemeral` ownership, loopback port
zero, offline mode, disabled UI and agent tools, and one exact alias. It retains
bounded stderr supervision through health, build, and route readiness and
returns no handle before those checks pass. Readiness timeout, startup drift,
early exit, build mismatch, and route mismatch all use the same joined cleanup
path before endpoint and artifact release. Local and remote-authoritative
fixtures preserve scope and execution-host identity. Its stop authority reaches
only its owned child; the build-9910 attached driver retains no serving-
lifecycle role and leaves its external server running.

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
