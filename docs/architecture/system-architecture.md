# System Architecture

Status: active
Owner: Tom
Updated: 2026-07-28

## Realized State

Swallowtail has a twenty-three-crate Rust workspace plus its strict Northstar authority
spine:

- `swallowtail-core` owns pure provider-neutral contract records, including
  runtime identities, access state, configured instances, model routes,
  model-artifact identity and preflight bindings, reasoning and token-limit
  catalog evidence, source-scoped catalogue modality, lifecycle, streaming,
  inference, customization, and provider-extension observations, expanded
  interactive access and harness-isolation policy, parameterized requirements,
  owned remote-resource deletion kinds, durable-retention and managed-recovery
  capabilities, opaque provider-agent/version bindings, realtime audio format
  and bound requirements, disabled-or-positive-bounded planned connection
  rollover, positive locally continued direct-session bounds, fixed buffered/
  SSE attempt transports, explicit provider-automatic tool selection, exact
  model identity, provider-cache acceptance without cache management, and
  side-effect-free preflight; harness operations may separately bind ambient,
  provider-suppressed, or reserved host-scoped configuration posture;
  interactive sessions
  separately bind prohibited or durable-
  conversation-delete-on-close provider state, while structured harness
  operations may bind exact ambient,
  provider-enforced, or host-enforced isolation independently of session policy
  and remote ACP operations may bind exact transport, connection-affinity,
  bounded-state, and independent wire/RFD/SDK version evidence; portable
  unauthenticated access remains distinct from local topology
- `swallowtail-runtime` depends on core plus `futures-core` and `zeroize` and owns
  executor-neutral dynamic roles, lifecycle handles, bounded events, terminal
  outcomes, explicit operation policy, typed usage/rate/quota observations;
  usage is one cumulative operation snapshot at each emitted boundary,
  replacing provider-cumulative records or summing disjoint provider
  components once while keeping context occupancy and billed cost separate;
  scope- and audience-bound endpoint/credential ports, scoped
  materialization leases, read-only model-artifact leases, scoped owned-serving
  endpoint publication, a distinct working-resource callback I/O port, and
  portable runtime inputs; structured-run requests reuse bounded tool
  declarations, run handles may expose callback exchange with explicit run or
  turn ownership, operation policy
  separates durable retention from provider-managed recovery, carries exact
  structured-run harness isolation, rejects request/preflight posture mismatch,
  carries exact harness-configuration request policy independently from
  isolation, rejects request/preflight mismatch, and terminal
  outcomes keep deletion truth per owned remote resource; interactive open,
  load, and resume requests carry one explicit or plan-derived agreement for
  access, provider state, and harness configuration; safe preparation failures
  distinguish nine stages, already-authorized sessions may expose bounded
  immutable negotiated model options without becoming catalogue drivers, and
  access evidence retains observed or
  caller-asserted provenance; a separate
  realtime-media role owns resource-free requests, redacted zeroized chunks,
  exact append/commit/output sequencing, transcripts, observations, response
  handles, terminal reuse, session-ending interruption, and immutable
  planned-rollover request-plan agreement plus distinct locally continued
  direct-session requests, explicit attempt authorization, bounded redacted
  tool-call/result records, and private-continuation binding metadata that
  never carries provider reasoning bytes
- `swallowtail-testkit` depends on core and runtime and owns deterministic
  Contract 003, Contract 008, runtime-skeleton, and Contract 011 cross-shape
  fixtures and assertions, including distinct local and remote-authoritative
  execution-host identities plus pure remote-harness policy fixtures for
  durable retention, managed recovery, exact resource ownership, and a
  separate structured-harness native-bound and provider-conversation assertion
  packs plus an eleventh realtime-media direct-session profile, a twelfth
  locally continued direct-session profile, a thirteenth remote ACP harness
  profile, and a separate planned-rollover assertion pack over the unchanged
  realtime profile
- `swallowtail-host-local` depends on core and runtime and implements concrete
  host-approved local process, endpoint, credential, materialization, and
  monotonic deadline behavior behind capability-scoped runtime ports; it also
  provides per-host joined scoped tasks, inspectable exact service
  composition, and explicit executable approval returning one opaque discovery
  target
- `swallowtail-protocol-acp` is the provider-neutral ACP wire boundary; it owns
  bounded v1 NDJSON framing and message classification plus a fixture corpus
  pinned independently to Gemini CLI `0.51.0`/schema `v1.19.0` and Kimi Code
  `0.28.1`/schema `v1.19.1`; the Kimi corpus also freezes exact annotated-tag,
  source-commit, arm64 executable, isolated-state, and upgrade-gate evidence;
  a separate raw remote-transport corpus freezes HTTP/SSE and WebSocket
  lifecycle behavior against wire version 1, the Active transport RFD, and SDK
  `2.0.0` without depending on a production client
- `swallowtail-transport-acp-remote` depends on core, runtime, and the ACP
  protocol boundary; it keeps exact SDK, Tokio, HTTP/2, SSE, WebSocket, TLS,
  cookie, and transport errors private while exposing one operation-scoped
  bounded client over an exact preflight plan and network grant
- `swallowtail-protocol-openai-chat` owns bounded compatible Chat Completions
  request encoding, including null content and bounded structural message
  extensions, JSON envelope decoding, structural unknowns, SSE comments,
  data records, fragmentation, disconnect, and `[DONE]`; it has no endpoint,
  credential, provider selection, retry, lifecycle, runtime-event, host, or
  consumer authority
- `swallowtail-adapter-alibaba-model-studio` implements the 2026-07-22 Singapore
  workspace-dedicated Conversations and Responses subset through one session-
  scoped host-approved endpoint and API-key lease; exact provider-state policy,
  serial HTTP/SSE turns, usage, cancellation uncertainty, inventory, item-before-
  conversation deletion, and joined cleanup remain adapter-owned; a separate
  international control-plane driver lists bounded base and custom deployable
  model candidates without claiming Conversations compatibility
- `swallowtail-adapter-codex` depends on core and runtime and implements the
  read-only, ephemeral `codex exec` structured-run surface plus read-only and
  bounded-workspace app-server interactive sessions through runtime host ports
- `swallowtail-adapter-claude-agent` implements installed discovery,
  ambient read-write one-prompt structured runs and read-only interactive
  sessions for Claude Agent ACP `0.53.0..=0.61.0`, excluding `0.58.0`, over
  ACP v1 stdio; stable newer
  versions remain visible and unverified, while separate local-subscription
  and public-API-key profiles, model confirmation, ambient configuration,
  ambient-host isolation, permission rejection, cancellation, deadlines, and
  joined cleanup stay driver-owned
- `swallowtail-adapter-deepseek` implements the exact
  `deepseek-openai-chat-2026-07-22` V4 Pro locally continued session over
  host-approved HTTP/SSE, including authenticated catalogue, consumer-owned
  tool exchange, private reasoning continuation, and joined credential-last
  cleanup
- `swallowtail-adapter-opencode` implements version-bound OpenCode
  `1.14.48..=1.18.4` model discovery and ambient-host interactive sessions with
  read-only tool permissions over host-approved HTTP and bounded SSE; exact
  stable newer releases may run as visibly unverified without extending
  guaranteed support; a separate 45-release deletion corpus freezes two
  delete-schema revisions, recursive provider-defined descendants, missing-
  target rejection, inactive-target requirements, and post-dispatch
  uncertainty without yet advertising production deletion
- `swallowtail-adapter-anthropic` implements provider-supported `2023-06-01`
  Models catalogue and Messages direct inference over host-approved HTTP/SSE;
  its separate `managed-agents-2026-04-01` remote-harness driver binds one
  operator-owned agent version, one driver-owned limited environment and
  session, authoritative events, callbacks, bounded recovery, interruption,
  usage evidence, and ordered deletion
- `swallowtail-adapter-bedrock` pins the provider-supported
  `aws-sdk-bedrockruntime = 1.136.0` in-process Rust boundary and implements
  one exact `ConverseStream` production route; its native catalogue fixture
  boundary pins `aws-sdk-bedrock = 1.148.0`, the distinct regional control-
  plane audience, generated request, summary, lifecycle and error types, and
  bounded provider-neutral projection
- `swallowtail-adapter-gemini` implements the pinned Gemini CLI `0.51.0`
  ambient-host interactive subset with Plan Mode and bounded read callbacks
  over ACP v1 stdio and retains session-advertised model options as negotiated
  evidence; a separately qualified `0.51.0..=0.52.0` headless route sends one
  prompt over stdin, consumes bounded `stream-json`, reports usage, requires
  durable local transcript retention, and forces no sandbox; one public
  Gemini CLI facade requires explicit ACP or headless selection. A separate
  Developer API Models branch performs bounded paginated discovery, while the
  Gemini Live production driver binds exact
  `v1beta` preview setup, asymmetric audio, manual activity, output, usage,
  latest private handle, one planned raw-WebSocket rollover, local-only
  interruption, and joined two-generation cleanup under both host identities
- `swallowtail-adapter-kimi` implements exact Kimi Code `0.28.1` and
  `0.29.0..=0.29.2` behavior segments. Its ACP route owns ambient-host
  interactive new, load-with-replay, replay-free resume, bounded writes, and
  negotiated model evidence. Its separate headless route owns one default-
  engine stream-JSON prompt with durable provider retention and joined process
  cleanup. One installed facade requires explicit ACP or headless selection.
  The separate local-server route owns authenticated catalogue, retained
  one-prompt structured execution, interactive callbacks, archive, restore,
  attached topology, and an optional owned foreground child without a
  container or sandbox claim
- `swallowtail-adapter-kimi-platform` implements a separate public-platform
  API-key catalogue and exact `kimi-k3` direct route over host-approved HTTP/SSE;
  it shares only structural compatible-chat encoding and decoding with
  llama.cpp and owns its access, reasoning, error, usage, and lifecycle mapping
- `swallowtail-adapter-pi` implements the pinned Pi `0.80.10` restrictive
  ambient-host RPC subset over supervised strict-LF JSONL stdio with exact
  downstream provider/model routing, prompt, steering, follow-up, correlated
  extension UI, native abort, deadlines, joined credential-last cleanup, and
  a separate provider-suppressed `get_available_models` operation
- `swallowtail-adapter-llama-cpp` implements attached llama.cpp build `9910`
  readiness, catalogue, and bounded Chat Completions direct inference without
  owning the model artifact or server; its exact request and text-only semantic
  mapping now use the common compatible-chat framing and envelope codec
- `swallowtail-adapter-ollama` implements attach-only native Ollama API
  catalogue and text structured runs across qualified stable releases
  `0.14.0..=0.32.1`; exact runtime, installed and running model observations,
  NDJSON output, and inference-caused residency remain distinct, with no
  installation, model acquisition, cloud access, unload, or server ownership
- `swallowtail-adapter-xai` implements resource-free direct inference over one
  host-approved Responses WebSocket as either one bounded response without
  continuation or serial interactive turns with private continuation; both
  retain exact billed cost and connection-ending cancellation, while a
  separate language-models driver exposes bounded hosted catalogue evidence
- `swallowtail-adapter-openai` implements separate public-API drivers for
  background Responses and Realtime media. The background structured-run route
  owns explicit temporary retention, one create attempt, maximum-one cursor
  reattachment, bounded retrieve, native cancel, and joined HTTP/SSE cleanup.
  The Realtime role owns one host-approved GA WebSocket, exact
  `gpt-realtime-2.1` and PCM16 formats, two serial responses, native response
  cancellation, bounded typed events, and credential-last joined cleanup. It
  passes the realtime-media profile under local and remote-authoritative host
  identities; transport loss, provider failure, protocol drift, and
  cancellation uncertainty remain distinct. A separate public Models branch
  reports key-visible entries without inferring background or Realtime support
- `swallowtail-adapter-qwen` implements the pinned Qwen Code `v0.19.11`
  headless structured-run route with exact read-only argv, text stdin, bounded
  stream JSON, typed usage, explicit native budgets, durable local retention,
  redacted terminal classifications, host deadline and cancellation, joined
  process cleanup, and `AmbientHost` isolation without a sandbox claim; the
  production driver passes the provider-neutral one-shot profile under local
  and remote-authoritative host identities. Its separate safe-mode stream-JSON
  control operation verifies and calls `get_available_models`, then closes and
  joins the ephemeral child without opening a model session

There is no global async executor, durable credential store, or consumer
dependency. Concrete libcurl clients remain private to their adapters and run
only through host blocking-work ports.

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

provider adapter crates -> swallowtail-protocol-acp / swallowtail-protocol-openai-chat

remote ACP operations -> swallowtail-transport-acp-remote -> core/runtime/protocol-acp
```

The active public release topology is recorded in
`release-and-package-topology.md` and governed by Contract 036. All 23
workspace libraries are separately consumable public packages under one
coordinated pre-1.0 version. The three-stage publication order is core plus
protocols, then runtime, then support, transport, and adapters. Current
manifests realize the compatible registry dependencies, resolver 3,
Rust-version floors, and package metadata rules. Roadmap g02.001 card 003 adds
the deterministic local metadata, API-declaration, documentation, MSRV,
content, clean-snapshot assembly, and extracted-package-family gates.

Crate status:

- `swallowtail-core` — realized
- `swallowtail-testkit` — realized with reusable contract-kernel, preflight,
  and callback fixtures, recording runtime host services, and thirteen composable
  provider-free conformance profile runners
- `swallowtail-runtime` — realized under Contracts 008-010, 012, and 026 with
  only core, `futures-core`, and `zeroize` dependencies
- `swallowtail-protocol-acp` — realized for bounded ACP v1 NDJSON framing,
  request/notification/response classification, safe error responses, and
  independent raw HTTP/SSE plus WebSocket remote-transport fixtures
- `swallowtail-transport-acp-remote` — realized under Contract 035 with
  explicit HTTP/2 SSE or WebSocket selection, bounded connection-private
  cookies, frames, streams, request/callback correlation, initialize-version
  validation, cancellation, deadline, disconnect invalidation, explicit
  close, and host-owned joined private runtime work
- `swallowtail-protocol-openai-chat` — realized under Contract 024 for bounded
  request JSON, SSE framing, common chunks, choices, deltas, model, finish,
  usage and error envelopes, and explicit bounded structural unknowns; the
  library depends only on `serde_json`
- `swallowtail-host-local` — realized with host-owned approvals, bounded piped
  I/O, supervised exit, graceful EOF stop, explicit force-stop, and joined
  reader cleanup; it also owns bounded attachment/schema copies,
  operation-scoped temporary working resources, explicit lease release, and
  cancellable monotonic deadline waits; exact endpoint and secret/delegated
  credential approvals remain scope- and audience-bound and redacted; per-host
  scoped task handles join explicitly or on drop, `LocalHostServices` composes
  the exact supported ports under one host identity, and installed executable
  approval returns only an opaque target
- `swallowtail-adapter-alibaba-model-studio` — realized under Contract 025 for
  one exact Singapore workspace-dedicated, general-API-key, pay-as-you-go
  `qwen3.7-plus-2026-05-26` session; the production driver creates one provider
  conversation, permits two serial synchronous Responses turns, rejects resume
  and unsupported inputs, keeps local cancellation distinct from remote stop,
  inventories and deletes every item before the conversation, and releases its
  credential only after transport and cleanup work join
- `swallowtail-adapter-codex` — realized for bounded exec runs plus local stdio
  app-server model discovery and interactive sessions
- `swallowtail-adapter-claude-agent` — realized for exact installed wrapper
  discovery and four private behavior revisions across the maintained ACP
  range, plus independent ambient read-write one-prompt structured and
  read-only interactive roles with explicit local-subscription or public-API-
  key access, no-argument process launch, bounded read callbacks, explicit
  structured-run edit acceptance, unexpected-permission rejection,
  cancellation, deadlines, disconnect classification, native close without
  deletion, and joined resource plus optional credential cleanup
- `swallowtail-adapter-deepseek` — realized for one exact opaque facade
  revision, V4 Pro catalogue and selection, buffered tool response, streaming
  finals, private continuation, cache usage, consumer-authorized attempts,
  failure, cancellation, disconnect, drift, and both host topologies
- `swallowtail-adapter-opencode` — realized for attached model catalogue and
  read-only interactive sessions over a six-route HTTP/SSE subset with exact
  delegated-auth, version, provider/model, abort, deadline, and cleanup bounds
- `swallowtail-adapter-anthropic` — realized for the frozen two-route public-
  API subset with API-key and version headers, bounded catalogue pagination,
  explicit output bounds, ordered SSE, usage, rate and request evidence,
  provider errors, unknown events, local cancellation, one inference attempt,
  joined work, and awaited credential release; a separately registered
  managed-harness driver realizes the frozen beta REST/SSE lifecycle with
  explicit durable retention, provider recovery, remote resource ownership,
  callback relay, one history reconciliation, per-resource deletion truth,
  local and remote-authoritative topology, and joined cleanup
- `swallowtail-adapter-bedrock` — realized under Contract 019 for one exact
  hosted-direct Bedrock Runtime `ConverseStream` route through the official
  Rust SDK, explicit region and host-approved endpoint, delegated cloud
  credential provider, one attempt, typed bounded EventStream projection,
  complete-stream deadline, cancellation, and joined operation-private Tokio;
  generated fixtures and default tests use no live AWS or ambient
  configuration; a separately registered Contract 020 control-plane driver
  uses `aws-sdk-bedrock` for one unfiltered `ListFoundationModels` request with
  its own instance, endpoint audience, region, access profile, delegated
  provider, deadline, one-attempt bound, observation projection, and joined
  private executor
- `swallowtail-adapter-gemini` — realized for pinned ACP initialization,
  isolated API-key process state, Plan Mode new sessions, text prompts,
  ordered updates, host-bounded reads, permission cancellation, active-turn
  cancellation, and joined process close; its separate headless driver owns
  exact installed discovery, one plan-mode structured run, native exit
  mapping, deadline, cancellation, and joined child cleanup with explicit
  ambient configuration and transcript retention; a separate direct
  realtime-media driver binds exact
  Gemini Live preview identity, host-approved query-key raw WebSocket access,
  asymmetric PCM, manual activity, two serial turns, one private-handle
  planned rollover, local-only interruption, and joined credential-last close
- `swallowtail-adapter-kimi` — realized for pinned initialization, isolated
  harness state, delegated membership auth, exact persistent bindings, new,
  load, ordered replay, resume, prompt, bounded text replacement, native turn
  cancellation, disconnect, and joined process/resource/credential cleanup;
  separate headless and local-server structured roles preserve their exact
  process versus REST/WebSocket lifecycle and durable-retention truth
- `swallowtail-adapter-kimi-platform` — realized for one authenticated bounded
  catalogue and one exact K3 Chat Completions attempt with explicit
  `low`/`high`/`max` reasoning, positive output bound, ordered reasoning,
  output, terminal usage and `[DONE]`, exact returned-model agreement, distinct
  safe failures, local cancellation/deadline, joined connection work, and
  awaited platform-key release without retry, recovery, or fallback; the
  unchanged hosted-direct profile proves exact instance, endpoint, access,
  route, model, and execution-host identity under local and remote-authoritative
  topology while catalogue observations imply no entitlement or invocation
  readiness
- `swallowtail-adapter-xai` — realized for one exact API-key WebSocket route
  with independent structured and interactive roles: one structured
  `store=false` response exposes no continuation, while interactive sessions
  retain one active turn and private continuation; both preserve ordered text
  and usage, exact USD ticks, distinct provider failures, cancellation,
  deadline, disconnect invalidation, and joined credential cleanup
- `swallowtail-adapter-openai` — realized under Contract 021 for one exact
  public-API Responses background structured run with API-key access, explicit
  model, output, deadline, retention, and reattachment policy, ordered SSE,
  bounded recovery, native cancel, provider cancellation evidence, and joined
  credential cleanup

Core, runtime, and testkit are realized and validated as one kernel. The thirteen
synthetic Contract 011 profiles use only public APIs and cover one-shot CLI,
long-lived RPC, long-lived ACP, attached network harness, hosted API, attached
self-hosted, owned self-hosted, connection-scoped direct-session, and a
persistent ACP extension, a provider-managed remote harness, plus a bounded
realtime-media direct session, a locally continued direct session, and a
remote ACP harness. The managed
profile adds exact agent binding, durable retention, managed recovery, bounded
reattachment, run callbacks, per-resource deletion truth, topology, and cleanup
ordering without widening the other profiles. The ACP extension composes
load, replay, resume, bounded write, delegated-auth, ambient-authority,
topology, and cleanup claims without widening the baseline ACP profile.
The realtime profile adds exact model and PCM format binding, bounded chunks,
two serial manual turns, transcript and provider evidence, interruption, and
joined cleanup without changing existing text operation shapes.
The locally continued profile adds separate provider requests, explicit user-
turn or correlated-tool-result authorization for every attempt, consumer-
executed tools, bounded adapter-private continuation, explicit provider-cache
posture, and request-scoped cleanup without widening harness callbacks,
connection continuation, or provider conversation state.
The remote ACP profile adds explicit HTTP/SSE or WebSocket selection, bounded
connection-private cookie affinity, separate version axes, no recovery or
fallback, callbacks, topology, and joined network work without creating a
generic provider or widening process ACP.
Structured-run policy now keeps attached execution, prohibited provider
retention, and disabled stream reattachment as the defaults. Provider-managed
background execution, temporary retention, and bounded reattachment are three
independent opt-ins backed by explicit capabilities. Existing structured-run
drivers reject that posture. Terminal outcomes may separately record confirmed,
completion-raced, or unconfirmed provider cancellation without replacing the
local terminal status.
Durable provider retention and provider-managed recovery are further
independent opt-ins. Interactive provider conversation retention is a separate
session-only opt-in and defaults prohibited. Owned environment, session,
conversation, and aggregate conversation-item deletion truth is keyed by
resource kind, so one confirmation cannot stand in for another. Structured-
run tools and callbacks reuse the existing bounded declaration and exchange
records; drivers that do not implement that subset reject them before work.
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

Claude Agent ACP structured runs retain the same reject-and-stop posture by
default. An explicit prepared-run opt-in binds
`acp/session/request-permission` in the immutable plan and installs the common
callback exchange. The adapter exposes only bounded one-shot options and
transports the consumer's exactly-once selection; it never selects approval or
admits persistent permission changes.

Interactive session requests now carry optional redacted developer
instructions, an exact reasoning selection, and bounded tool declarations.
Turn handles may expose a one-shot callback exchange with a bounded request
stream and object-safe response port. Callback requests bind a distinct
redacted callback id to one runtime run or turn id, event sequence, optional
monotonic deadline, and bounded opaque payload. Testkit proves response correlation,
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
The additive lifecycle corpus pins stable schema `v1.20.0` without rewriting
the historical Gemini or Kimi pins. Independent close-only, delete-only,
omitted, null, success, and error fixtures pass through the same bounded
message codec used by stdio and explicit remote ACP. Portable delete truth
remains history removal.
Gemini CLI ACP `0.51.0` is narrowed to new read-only sessions, text prompts,
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

The Claude Agent lifecycle corpus separately freezes close and delete at every
qualified behavior milestone plus the unpublished-package exclusion.
Qualified close tears down only active in-memory resources and preserves
history. Qualified delete tears down an active target when present, then uses
the exact Agent SDK path that removes the primary local transcript and sibling
session directory. That evidence supports provider-data deletion with
provider-defined descendants, but no hard-erasure or Anthropic API service-data
claim. Missing and repeated operations reject. Published `0.62.0` remains
unverified-newer rather than extending the guaranteed range.

The Claude Agent stdio driver now realizes the qualified mapping. Initialization
requires independently advertised close and delete capabilities before session
creation. Qualified handle cleanup sends native `session/close` as a separate
leg before closing and joining the owned process; an unverified-newer session
does not inherit that cleanup claim. Prepared sessions return an opaque
management binding without adding load or resume. The separate prepared delete
operation requires caller-asserted inactivity and explicit unverified-newer
acceptance, starts one fresh scoped ACP process, and reports
`ProviderDataDeleted` with `ProviderDefinedDescendants` only after an empty
success response. It never reads provider state paths or extracts credentials.
The completed portability matrix covers all qualified behavior segments, the
unpublished exclusion, unverified-newer opt-in, effect-boundary failures, and
credential-last cleanup. The real remote WebSocket ACP transport carries the
same initialize and delete records under both host topologies. Remote failure
has no process service, retry, reconnect, or stdio fallback. This proves
transport portability without claiming an authenticated remote Claude route.

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

Interface qualification now has its own provider-neutral boundary. Adapter,
package, SDK, wire, service, schema, facade, configured-instance, route, and
model versions remain independent. Configured instances and immutable plans
bind exact safe version points. Driver descriptors carry a maintained support
window per interface axis: an oldest supported baseline, latest-qualified
boundary, ordered behavior milestones, deprecated-but-supported segments, and
exact exclusions. Qualified points are the guaranteed support claim. An
ordered claim may separately permit an exact stable version above its latest
qualified boundary as `UnverifiedNewer`; preflight then retains that exact
version and private driver dispatch uses the latest qualified behavior
revision. Such execution is allowed, not guaranteed. Below-baseline points,
in-range gaps, explicit exclusions, non-qualified prereleases, malformed
values, and qualified-only claims remain incompatible. Moving the baseline or
latest-qualified boundary is an explicit later-release change. No open-ended
`latest` value participates in routing.

The OpenCode HTTP adapter has a closed qualified server-version boundary.
Tagged OpenAPI evidence for 45 stable releases from `1.14.48` through
`1.18.4` closes six selected operations through every transitive local schema
reference. Eighteen closed surfaces map to 20 contiguous segments so
unpublished patches and cross-minor synthetic versions remain unsupported.
The production descriptor publishes the `opencode.server` claim. Configured
instances, requirements, and immutable plans must bind one matching exact
release. Stable exact releases above `1.18.4` may execute as unverified through
surface 18 without widening the qualified range. `GET /global/health` produces
only that safe binding and three-way assessment; no endpoint, credential, raw
payload, configured instance, or execution authority enters the observation.
Catalogue and session work stop unless health matches the exact plan, and
created sessions must report the same release. Behavior selection remains
adapter-private. Cross-topology conformance proves qualified boundaries and
unverified-newer health and session work under local and remote-authoritative
host identities. The attached-network harness profile remains unchanged.
Its adapter-local prepared facade first authorizes one opaque endpoint,
acquires and releases one delegated credential lease, and observes exact
health. Separate prepared catalogue and read-only session values then delegate
to the unchanged low-level roles. The configured instance remains
`ExternalAttached`; ambient configuration and ambient-host isolation stay
visible, provider and model selection occurs only for sessions, and no server
lifecycle, authentication discovery, resume, remote-ACP fallback, or recovery
authority is added.

Installed-executable observation now has a separate additive discovery
boundary. One request binds a request id, operation scope, authoritative
execution host, opaque host-approved executable target, exact version axis,
monotonic deadline, and shared cancellation signal. Safe observations retain
only host identity, exact version binding, claim identity, and qualified,
unverified-newer, or incompatible classification. General discovery remains
unchanged; drivers
without the target-aware operation reject it explicitly. The local process
host resolves only the supplied opaque reference and existing process
completion remains joined. Testkit assertions exercise local and
remote-authoritative identities, exact classification, redaction, terminal
state separation, and process join without installing or invoking a provider
harness.

Harness configuration posture is now a separate provider-neutral preflight
boundary. Configured instances, operation requirements, immutable plans, and
runtime request policy distinguish explicit ambient configuration from an
exact provider-suppressed invocation. The posture applies to both structured
runs and interactive harness sessions and does not imply isolation,
authentication, retention, or working-resource authority. `HostScoped` is
represented but rejected before effects until a separate opaque host
configuration lease and capability-scoped service are contracted. Absent
posture remains unmigrated state, not an ambient alias; no posture falls back
to another.

The Pi RPC records compose with the existing long-lived RPC profile. One
restrictive policy binds one active operation, two completed prompts, one
pending steering message, one pending follow-up, no ambient customization, no
update, telemetry, package, or automatic-retry action, and explicit
`AmbientHost` read intent without a filesystem boundary. Command acceptance is
separate from model lifecycle. Correlated dialogs use callback exchange;
display-only UI becomes bounded semantic observation. The first adapter-private
corpus binds package `0.80.10`, strict LF JSONL, exact provider/model argv, and
offline startup without launching Pi or contacting a provider. The separate
production driver binds that exact point to one host-approved executable,
delegated harness credential, filesystem working resource, provider, model,
and `AmbientHost` read-intent policy. Its supervised connection keeps command
acknowledgement separate from model settlement, relays bounded extension UI,
uses native abort for cancellation and deadline requests without claiming
provider stop, and joins process work before resource and credential release.
The production fixture matrix passes the unchanged long-lived RPC profile and
the separate scheduling/UI assertion pack under local and remote-authoritative
host identities. It proves prompt-before-steering-before-follow-up ordering,
command acknowledgement before model settlement, deterministic callback
expiry and late-response rejection, distinct provider/retry/disconnect/format
failures, bounded prompt concurrency, redaction, and visible cleanup failure
without weakening terminal provider truth.

Pi provider-session continuity remains outside the production descriptor.
Research 053 checks every stable point from `0.80.10` through `0.82.1`.
Public RPC session switching recreates runtime services from the cwd stored in
the session file, checks only that the directory exists, and exposes no
effective cwd for host-resource corroboration. The current ephemeral profile
therefore stays provider-state prohibited. Load and resume remain paused until
a maintained public Pi interface can bind attachment to the exact
host-leased working resource; ambient execution does not weaken that identity
rule.

Research 042 corrects the original Pi tranche boundary. Exact Pi `0.80.10`
already exposes `get_available_models`; the omission was Swallowtail scope, not
an upstream limitation. The same prepared Pi installation now derives a
separate route-free model-catalogue operation. It starts one
provider-suppressed, tool-free, extension-free, offline, ephemeral RPC child,
issues the bounded catalogue command, projects configured provider/model
observations, including source-scoped reasoning-support evidence distinct from
named reasoning modes, then joins the child and releases delegated access. It
does not open a provider session, select a model, touch a working resource,
refresh catalogues, or infer entitlement.

ACP model selectors remain negotiated session evidence. Gemini and Kimi retain
bounded immutable options advertised while opening, loading, or resuming an
already-authorized session, while Claude Agent's selected subset echoes a
caller-supplied allowlist. None is promoted to a standalone catalogue by
creating a session solely for discovery. Kimi's separate local-server driver
implements exact authenticated `GET /api/v1/models` discovery without
refreshing providers, changing the default model, or widening ACP.

Research 043 closes the remaining catalogue audit. Qwen Code `0.19.11` and
`0.21.0` expose a stream-JSON `get_available_models` control request. Its
prepared catalogue starts an ephemeral safe-mode child, verifies the
advertised control capability, projects bounded model identity and
context-window evidence, then joins cleanup; safe mode is not an operating-
system sandbox claim. OpenAI, Gemini, and xAI expose separate hosted catalogue
drivers with their own configured instance, endpoint, access, role, and plan.
Alibaba Model Studio exposes a separate international deployment-candidate
control-plane catalogue with bounded base and custom pagination. None of these
catalogues constructs an inference route or implies compatibility with a
provider's background, realtime, Live, WebSocket, Conversations, region,
entitlement, or billing posture.

The DeepSeek production boundary composes Contract 030 with common
compatible-chat structure but keeps provider semantics private. Its opaque facade claim
qualifies only `deepseek-openai-chat-2026-07-22`, exact endpoint
`https://api.deepseek.com`, `/chat/completions`, and `deepseek-v4-pro`. The
corpus freezes one buffered tool-bearing attempt, two streaming final attempts,
later-turn reasoning continuation, cache hit/miss usage, status and provider-
stream failure, cancellation uncertainty, deadline posture, disconnect, model
mismatch, and unknown-field rejection. The separately registered driver uses
host-approved endpoint and API-key leases for authenticated catalogue and
completion requests. Its independent structured role sends one streamed,
tool-free request, exposes no session binding, and discards private reasoning
continuation at terminal completion. Its interactive role pauses after the
buffered tool call until the consumer submits the exact correlated result,
then streams the authorized continuation.
A later user turn replays bounded private history only into the same session,
facade, route, model, and access binding. Local and remote-authoritative
fixtures prove no provider request during tool wait, exact three-attempt
replay, per-attempt usage and cache evidence, finish and request evidence,
cancellation, deadline, safe failure, joined HTTP/SSE work, zeroization, and
credential-last cleanup without a live credential or paid inference.

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

## Prepared Integration And Bound Operation Layer

Contract 037 fixes the application-facing normal path without changing the
realized 23-crate dependency graph:

```text
consumer intent and explicit authority
  -> provider-adapter prepared profile
  -> expanded configured instance, requirements, access provenance, and plan
  -> typed bound operation for one existing runtime role
  -> existing low-level runtime role
  -> existing host services and provider driver
```

Provider-specific prepared surfaces belong to their adapter crates. Shared
plan-derived request, safe evidence, and diagnostic records belong to
`swallowtail-runtime`; common facade assertions belong to
`swallowtail-testkit`. Joined local service composition belongs to
`swallowtail-host-local`. Core, runtime, hosts, and low-level adapter roles
remain independently consumable. No umbrella crate constructs or selects a
provider.

Preparation binds adapter-owned facts and derives only state that repeats an
immutable plan. It cannot choose provider, model, target, credential, endpoint,
billing, topology, writable access, network, search, tools, prompts, workflows,
or persistence. Access status remains observed or visibly caller-asserted.
Every expanded profile is inspectable before effects.

Typed bound operations remove repeated driver and matching-request wiring, not
role semantics. Catalogue, structured run, interactive session, direct
session, background run, managed agent, realtime media, SDK, attached runtime,
and owned-serving operations remain separate types and methods. There is no
generic prompt method.

The current 26 production routes form six facade implementation families:
installed harness, attached harness network, hosted direct and provider-owned
state, realtime connection, embedded SDK, and local model runtime. Family
helpers may share host and preparation mechanics. They cannot select a
provider, model, target, credential, endpoint, topology, or fallback.

Codex is the first realized proof. Its exec structured-run, app-server
catalogue, and app-server interactive-session paths remain separate.
Read-only and bounded-workspace profiles remain separate. Exact installed-
version classification, local and remote-authoritative host identity,
cancellation, deadlines, callbacks, and joined cleanup continue through the
existing roles. Non-zero installed-version probes retain their stable failure
code while exposing numeric exit status and one bounded adapter-sanitized
stderr excerpt; raw stderr remains private.

The shared layer is realized: runtime owns plan-derived session agreement,
staged safe failures, access provenance, and `PreparedOperationEvidence`.
That record owns one immutable expanded plan and exposes safe driver, role,
layer, shape, instance, revision, host, opaque target, facade, access, and
exact interface-assessment evidence. Testkit proves the same record across
installed-harness, hosted-direct, and attached-runtime fixtures. It adds no
execution trait, provider selection, or operation request.

Host-local owns joined scoped tasks, exact service composition, and opaque
executable target approval. The Codex adapter owns an exact-target factory that
derives its discovery request, retains exact qualified, deprecated, or
unverified-newer evidence, preserves access provenance, and builds one
immutable configured-instance base from the same opaque target. It also owns
separate prepared catalogue, read-only session, bounded-workspace session, and
structured-exec values. Each retains the shared evidence and matching runtime
request. Model, reasoning, writable access, network, search, tools, schemas,
attachments, and deadlines remain explicit consumer inputs.

Codex prepared values now expose typed `list_models`, `start_run`,
`open_session`, and `resume_session` operations. Each constructs the exact
selected low-level Codex driver and delegates the immutable plan, explicit
request, and host services to the existing runtime role. Preflight, topology,
cancellation, deadlines, callbacks, terminal outcomes, and joined cleanup are
unchanged. `low_level_driver`, `plan`, `request`, and `into_parts` preserve the
advanced escape hatch and current consumer path. No new crate was needed.

Kimi Code is the second realized facade and first non-Codex proof. Its
installed facade requires explicit ACP or headless selection before discovery.
Both branches retain their exact qualified or unverified-newer evidence,
caller-supplied access provenance, isolated-state environment, target, host,
and configured instance. The ACP persistent-session profile derives ambient
harness configuration,
`AmbientHost` isolation, read-write resource authority, prohibited
Swallowtail-owned provider state, bounded writes, replay, resume,
active-turn interruption, and optional reasoning into one inspectable plan.
Bound new, load, and resume delegate to the existing ACP role. Prompt and
interruption continue through its returned session and turn handles. Load
returns provider replay; resume remains replay-free. Delegated credentials,
write callbacks, local and remote-authoritative topology, and ordered joined
cleanup remain unchanged. No sandbox or containment claim is introduced.
The headless branch instead derives one structured plan with durable provider
retention, starts the audited default print engine once, parses bounded
stream-JSON, exposes no reusable session identity, and joins the child on every
terminal path. Its prompt must occupy provider-required process arguments;
stable diagnostics and debug output remain redacted, but host process-table
visibility is not hidden.

Anthropic Messages is the third realized facade and first hosted-direct proof.
`prepare_anthropic_direct` binds one opaque host-approved endpoint target,
provider-supported `api.anthropic.com` API-key profile, access provenance,
execution host, externally owned configured instance, and dated
`anthropic-2023-06-01` facade without network or credential effects. Separate
prepared catalogue and inference-attempt values derive their own roles,
capabilities, immutable plans, and requests. Catalogue has no model route and
cannot select one. Inference requires an exact route, model, content, positive
output bound, and optional deadline; one `start_run` is one provider request.
The current text-only subset declares no tool or direct-continuation
capability. HTTP/SSE events, cancellation, deadline, usage and rate evidence,
safe failures, connection close, joined work, and credential-last cleanup
continue through the unchanged low-level driver under local or remote-
authoritative host identity.

Kimi Platform and DeepSeek now add two separate prepared surfaces over the
shared compatible-chat structure. Kimi preparation accepts only the
provider-supported `api.moonshot.ai` Platform API-key and pay-as-you-go profile;
Membership, Kimi Code, regional keys, and subscription metering fail before
effects. Separate catalogue and `kimi-k3` values bind explicit model,
reasoning, output bound, and one structured inference attempt. They expose no
tool or continuation capability even though newer Kimi documentation contains
compatible tool fields.

DeepSeek preparation accepts only the exact `https://api.deepseek.com` target,
`api.deepseek.com` Open Platform API-key profile, dated OpenAI facade, and
`deepseek-v4-pro` route. Catalogue stays route-free. Run preparation requires
high reasoning, an explicit output bound, and acceptance of unmanaged provider
caching; it emits one tool-free request and retains no continuation. Session
preparation requires high reasoning,
consumer-declared tools, and explicit acceptance of provider-managed cache
without management authority. Opening returns the existing direct-
continuation session. Each user turn authorizes its first attempt; only
correlated tool-result submission authorizes another. Private reasoning
replay, attempt bounds, cancellation, deadline, zeroization, and credential-
last cleanup remain in the low-level driver. Shared JSON and SSE structure
creates no cross-provider model, credential, lifecycle, or fallback path.

Alibaba Model Studio now adds a prepared provider-conversation surface over
the existing Singapore workspace driver. Preparation binds the exact regional
audience, general pay-as-you-go API-key profile, configured instance revision,
host-approved workspace endpoint, route, model, and access provenance.
Conversation preparation requires explicit durable-provider-retention and
delete-on-close posture. Opening delegates provider conversation creation to
the existing driver; returned session turns retain serial Responses semantics.
The independent run preparation sends one `store=false` Responses request with
no conversation or previous-response identity and prohibits provider
retention.
Close still joins active work, obtains a bounded item inventory, deletes every
item, deletes the conversation separately, joins cleanup, then releases the
credential. Provider conversation state remains distinct from consumer memory,
and neither deletion outcome can stand in for the other.

OpenAI Responses background mode now has a separate prepared public-API
surface. Preparation binds `https://api.openai.com`, API-key pay-as-you-go
access, provider support authority, the exact dated facade and GPT-5.6 route,
host services, and access provenance without endpoint or credential effects.
Background creation, required temporary provider retention, and maximum-one
cursor reattachment remain explicit inputs. `store=false` does not become a
no-retention claim.

Starting the prepared value delegates to the existing structured-run driver.
One create request remains one inference attempt. Cursor reattachment, bounded
retrieve, and native cancel manage that attempt and cannot replay input or
select another route. Provider response identity, stream cursor, runtime run,
local attachment, cancellation request, and terminal provider truth remain
separate. The facade adds no cross-process recovery, polling loop, retry,
fallback, durable consumer storage, ChatGPT access, Codex access, subscription
OAuth, or community OAuth route.

Anthropic Managed Agents now has a prepared provider-hosted harness surface
separate from Anthropic Messages direct inference. Preparation binds the
first-party API-key audience, exact beta facade, operator-owned agent identity
and numeric version, model route, host-approved endpoint target, access
provenance, and `HarnessInteraction` layer. It creates or mutates no provider
resource.

Run preparation requires durable retention, provider-managed recovery, and
one authoritative-history reattachment. Starting delegates to the existing
managed driver, which creates one limited-network environment and one session,
reconciles persisted event history after one disconnect, relays correlated
custom-tool callbacks without executing them, and interrupts active work on
cancel or deadline. Cleanup deletes the session before the environment, joins
owned work, then releases the credential. The operator-owned agent is never
deleted. Repository, provider filesystem, built-in tools, external sandbox
network, MCP, skills, vaults, memory, schedules, webhooks, files, and local
containers remain excluded.

Realtime routes now expose three separate prepared connection surfaces. xAI
Responses WebSocket requires an explicit caller-selected model and retains
serial text turns, connection-private continuation, billed cost, and
whole-session invalidation. OpenAI Realtime binds `gpt-realtime-2.1`, mono
24 kHz PCM, manual input commit, native response cancellation, and no planned
rollover. Gemini Live binds `gemini-3.1-flash-live-preview`, 16 kHz input,
24 kHz output, local interruption truth, and exactly one provider-planned
rollover at an idle boundary. Each retains bounded chunks, two serial
responses, endpoint and credential leases, connection cleanup, and the
unchanged low-level driver. No common method hides their model selection,
media events, cancellation truth, billed evidence, retry posture, or rollover.
Capture, playback, conversion, pacing, privacy, and played-position truth
remain downstream.

Bedrock Runtime and control-plane catalogue now expose separate prepared SDK
surfaces. Each requires an exact region and an already-selected opaque
credential provider through `BedrockCloudClientConfig`; neither consults the
ambient AWS region, credential, profile, file, container, or instance-metadata
chains. Runtime binds `aws-sdk-bedrockruntime = 1.136.0`,
`ConverseStream`, one exact model route and underlying provider, bounded text
output, and one structured attempt. Catalogue binds
`aws-sdk-bedrock = 1.148.0`, `ListFoundationModels`, its own access and
regional control-plane target, and no model route. Their prepared evidence
retains separate SDK and service interface axes. Bound operations delegate to
the unchanged one-attempt drivers; private SDK work joins before credential
release. Catalogue observations cannot construct Runtime capability,
entitlement, availability, or route truth.

Ollama native is the fourth realized facade and first attached-runtime proof.
`prepare_ollama_attached` binds one host-approved native endpoint, configured
instance, selected route, native model tag, expected manifest digest, and
local-unauthenticated access evidence. It observes exact runtime version,
installed inventory, running inventory, and selected-model detail without
inference or model mutation. Prepared inventory and one-attempt inference stay
separate. Inference declares runtime-managed residency but grants no pull,
unload, restoration, process, or server authority. Exact endpoint and runtime
drift fail before operation effects. The guaranteed `0.14.0` through `0.32.1`
window, exact `0.32.2` exclusion, prerelease closure, and visibly unverified
later stable execution remain explicit.

llama.cpp completes the local-runtime family with deliberately separate
prepared types. `prepare_llama_cpp_attached` binds one host-approved external
endpoint, exact b9910/f5525f7e7 runtime identity, local-unauthenticated access,
and separate catalogue or one-attempt inference plans. It exposes no serving
start or stop authority; closing inference leaves the external server running.

`prepare_llama_cpp_owned` instead binds one host-approved executable, exact
b10069/178a6c449 runtime identity, one GGUF artifact, one route and alias, and
host-owned ephemeral lifecycle authority. Its typed serving selection couples
artifact and route before preflight. Bound start delegates to the existing
owned driver, which acquires the artifact, starts offline loopback serving,
observes and publishes the endpoint, verifies health, properties, and
catalogue, then returns the owned handle. Stop joins the child, invalidates
endpoint authority, and only then releases the artifact. Acquisition,
persistent serving, and Monkey ownership remain outside Swallowtail.

The provider-wide prepared contract is now realized across all 26 production
routes. The g02.008 cross-shape review accepted the common
`PreparedOperationEvidence`, adapter-local evidence, two-phase construction,
safe preparation stages, and typed low-level delegation without a new durable
rule. The prepared-facade authoring guide records that pattern. Roadmaps
g02.009-g02.011 complete every remaining route. Roadmap g02.012 now owns the
exact route matrix, packaged proof, and replacement candidate evidence.

## Provider Session Management Boundary

Contract 038 adds a provider-session management role above opaque persistent
session bindings. The provider-neutral record, runtime role, and public
cross-host conformance layers are realized. Codex app-server, Claude Agent
stdio ACP, and OpenCode HTTP/SSE now implement the role; Claude lifecycle
records also pass explicit remote-ACP transport conformance.

`swallowtail-core` now owns independent archive, restore, delete, and native
close capabilities plus typed action, lifecycle state, deletion strength,
affected scope, effect truth, and exact interface compatibility evidence.
`swallowtail-runtime` owns the redacted management binding. It requires an
exact driver descriptor, configured instance, host target, access evidence,
interface versions, management capability, origin, and optional resource
scope; a raw provider reference is insufficient.

`swallowtail-runtime` now exposes one immutable management plan, independent
archive, restore, and delete request types, a low-level driver role, exact
effect outcome, validation against host services and preflight evidence, and
shared prepared evidence. Inactivity is caller-asserted; no active-session
registry exists. Cancellation posture and deadlines are plan-bound, and
after-dispatch uncertainty carries no confirmed deletion strength.

`swallowtail-testkit` exposes composable local and remote-authoritative
fixtures plus one reusable assertion pack. It covers action and deletion
strength, qualified and unverified-newer execution, unsupported and
incompatible routes, exact binding and plan drift, pre/post-dispatch
cancellation and deadlines, uncertainty, and joined release ordering. Exact
method mapping, version milestones, prepared operations, and provider truth
remain in each adapter.

Consumer thread archive, restore, deletion, persistence, and UI stay
downstream. Runtime attachment close, provider-native active close, reversible
archive, restore, history removal, provider data deletion, provider hard
deletion, and driver-owned resource cleanup remain separate.

The first common role targets one inactive bound provider session. It contains
no global session registry, provider-history browser, arbitrary-id authority,
implicit deletion, retry, or fallback. Codex app-server, Claude Agent ACP, and
OpenCode HTTP/SSE are the first applicable implementations. Kimi and Gemini
ACP retain explicit unsupported mappings until their selected routes qualify
the methods. Other operation shapes remain consumer-local or keep their
existing driver-owned cleanup.

Research 040 and Contract 038 define a separate contracted
`kimi-code.local-server` route inside `swallowtail-adapter-kimi`. It uses the
provider-documented foreground `kimi web --no-open` process, local REST,
WebSocket protocol version `2`, exact server metadata, and an opaque bearer
credential lease. Attached and owned-foreground topologies remain distinct.
The implementation qualifies reversible archive and restore across exact Kimi
Code `0.28.1`, exact `0.29.0`, and the audited `0.29.1..=0.29.2` range. The
later range has a separate behavior revision for global WebSocket event fan-out
and filtered configured-model discovery. It does not qualify deletion or
change the unsupported ACP mapping. Attached preparation preserves the
external server. Owned preparation launches one approved foreground child on
the approved loopback port and joins it on close. Both require exact metadata,
state-root identity, and an opaque local bearer lease. Archive and restore
cross one POST effect boundary without retry; delete fails before dispatch.

The optional ACP-to-local-server management import is realized as two
adapter-local opaque values. A prepared ACP session plus its matching resume
binding issues source authority only when ACP preparation bound an opaque Kimi
state root. Local-server preparation issues a target snapshot containing the
exact configured instance, endpoint, access evidence, server observation, and
state root. Side-effect-free import preflight requires exact release, host,
state-root, target, lifecycle capability, and unverified-newer agreement.
Execution performs one authenticated read-only target lookup. An exact
unarchived result issues a new local-server archive/restore binding; it never
widens ACP. Matching provider family or raw session identity is not authority.
The later interactive tranche reuses the same route identity and transport
without replacing ACP or adding a provider-neutral prompt facade.

The local-server route also registers `StructuredRun` independently. One
operation creates a private retained session, submits one prompt, relays only
qualified events and callback exchange, awaits terminal truth, and closes
local resources. The structured plan carries no interactive-session policy or
handle. `DurableAllowed` is mandatory; close claims no archive or deletion.

Roadmap g02.015 owns the completed shared foundation. Roadmaps g02.016-g02.019
own the first provider realization and provider-wide acceptance. Roadmap
g02.020 owns the additive Kimi local-server proof. Research 046 and g02.024
own the exact `0.29.2` currentness extension.

## Realized Provider-Retention Closure Tranche

Research 054-055 and Contracts 021, 038-039 define the realized
provider-retention tranche.

- Gemini CLI stored-transcript management is a separate installed-executable
  role across `0.51.0..=0.52.0`. A successful durable headless run can return
  one take-once bound management capability after terminal completion.
  Deletion uses the exact bound id, joins the delete process, performs one
  bounded `--list-sessions` reconciliation, and reports only
  `HistoryRemoved`; Gemini ACP remains unsupported.
- Gemini CLI and Claude Agent expose separate opt-in temporary-retention
  structured profiles. Each deletes only the operation-private transcript or
  session, records deletion truth separately from inference truth, and leaves
  its existing durable profile unchanged.
- OpenAI background Responses perform at most one terminal response-delete
  attempt before credential release and report the operation-owned `Response`
  resource separately from inference status.

Focused exact-range fixtures cover completion, cancellation, deadline,
rejection, acknowledgement loss, mismatched deletion, retained-history
reconciliation, and joined cleanup. OpenCode structured cleanup was already
realized; card 102 owns its stale matrix correction and package-wide closeout.

## Realized Kimi Recovery And Reattachment

Research 056-057 and Contract 042 define the realized Kimi harness lifecycle
repair.

Kimi headless and local-server structured preparation now require explicit
managed-recovery acceptance before effects. Their qualified retry records are
validated as ordered provider evidence. Swallowtail performs no retry and
exposes no provider error text.

Kimi local-server structured runs may separately opt into one active-turn
WebSocket replacement. The replacement retains the same session, prompt,
turn, runtime, endpoint, credential lease, model, deadline, and
`{seq, epoch}` cursor. It submits no prompt or session creation. The failed
attachment joins, cancellation control follows the replacement socket, and
the final attachment joins before access release. ACP and interactive-session
preparation inherit no capability.

## Dependency Rules

- consumers depend toward Swallowtail; Swallowtail never depends on consumers
- core does not depend on runtime or provider adapters
- runtime does not depend on provider adapters
- provider extensions remain namespaced and optional
- UI frameworks and consumer persistence stay outside the crate graph
- execution happens on the host chosen by the consumer

## Architecture Promotion Rule

Realized sections name only implemented and validated structure. A separately
labelled contracted section may record an active contract and dependency
direction before implementation. It must state the realization gap and owning
roadmap.
