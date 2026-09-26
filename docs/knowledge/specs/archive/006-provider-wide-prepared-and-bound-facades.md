# 006 Provider-Wide Prepared And Bound Facades

Status: promoted and archived
Owner: Tom
Updated: 2026-07-25

## Purpose

Make the prepared integration path the normal application entry point for
every production driver without creating a lowest-common-denominator provider
API.

## Problem

Contract 037 and the Codex proof remove repeated preflight assembly, but the
other 20 production driver routes still expose only low-level descriptors,
bindings, drivers, and operation requests. A consumer integrating several
providers must learn and reproduce each route's construction sequence.

The problem is mechanical assembly, not provider diversity. Catalogue,
structured-run, interactive-session, realtime-media, retained-run, SDK, and
serving lifecycles must remain distinct.

## Production Inventory

The current 22 routes group into six implementation families:

1. installed harnesses — Codex exec, Codex app-server, Claude Agent ACP,
   Gemini CLI ACP, Kimi Code ACP, Pi RPC, and Qwen headless
2. attached harness network — OpenCode HTTP/SSE
3. hosted direct and provider-owned state — Anthropic Messages, Kimi Platform,
   DeepSeek, Alibaba Model Studio conversations, OpenAI background Responses,
   and Anthropic Managed Agents
4. realtime connections — xAI WebSocket, OpenAI Realtime, and Gemini Live
5. embedded SDK — Bedrock Runtime and Bedrock catalogue
6. local model runtimes — Ollama attached, llama.cpp attached, and llama.cpp
   owned ephemeral serving

Remote ACP is a transport used by compatible ACP adapters, not a 23rd provider
driver. It needs composable preparation inputs but does not become an
integration-family selector.

## Selected Shape

Keep two public layers:

1. low-level provider-neutral roles, records, transports, hosts, bindings, and
   drivers
2. adapter-local prepared facades with typed operation profiles and bound
   execution

Each adapter crate owns constructors for its production routes. A constructor
accepts one exact route, host, target, access source, and explicit
consumer-controlled options. It returns:

- safe preparation and compatibility evidence
- one expanded immutable plan or connection/start specification
- one typed prepared operation that invokes the matching low-level role
- access to the selected descriptor and low-level parts for advanced use

Bound execution removes repeated driver and request wiring. It does not hide
operation content or authority. Consumers still supply prompts, schemas,
attachments, callbacks, model selections, resource authority, deadlines, and
other inputs owned by the selected operation.

Do not add an umbrella crate. Consumers depend only on the adapter crates they
select. Shared facade records and assertions remain provider-neutral in
runtime and testkit.

## Facade Families

Family helpers may share construction and conformance only when their facts
are truly common:

- installed target observation and version classification
- approved endpoint and credential lease binding
- retained provider-resource ownership and deletion posture
- connection-scoped realtime setup and rollover posture
- explicit SDK client and delegated credential binding
- attached endpoint or owned-serving lifecycle binding

Family helpers never select a provider, driver, model, endpoint, credential,
topology, or fallback route. Adapter-private revisions and mappings remain in
adapter crates.

## Bound Operation Rule

A prepared value may offer typed methods such as catalogue observation,
structured-run start, session open, realtime connect, retained-run create, or
serving start only when the adapter implements the matching low-level role.

There is no universal `send_prompt`, `run`, or `session` method. Different role
semantics remain different Rust types and methods. Provider-specific
capabilities remain inspectable and may require provider-specific input.

The prepared value binds construction facts. Starting an operation still
requires explicit operation content and authority. Every returned handle keeps
the existing cancellation, deadline, terminal-truth, and joined-cleanup
contract.

## Version And Drift

Installed routes retain exact observed executable versions and Contract 029
compatibility assessment. Versions inside the guaranteed range are supported.
Explicit unverified-newer attempts remain allowed when the adapter claim
permits them; they are not guaranteed and are not hard-denied solely for being
newer.

Opaque hosted facades, SDK versions, and native runtimes retain their own
qualified axes. Preparation must not convert one version axis into another or
silently refresh a prepared binding after drift.

## First Representative Tranche

After shared records and Codex bound execution, prove the shape with:

- Kimi Code ACP — installed persistent harness, exact version, ambient
  configuration, callbacks, load/replay/resume, and optional isolation
- Anthropic Messages — hosted HTTP/SSE, endpoint and credential leases,
  catalogue observation, and direct streaming
- Ollama native attached — exact attached runtime observation, installed and
  running model inventory, and invocation-caused residency without server
  ownership

These routes exercise different authority, version, topology, connection, and
lifecycle shapes. They do not establish provider preference.

## Rollout

1. promote the provider-wide facade and bound-operation contract
2. add shared preparation evidence, traits, and conformance assertions
3. complete Codex bound execution and the representative tranche
4. cover the remaining installed and attached harness routes
5. cover hosted direct and provider-owned retained routes
6. cover realtime, SDK, attached-runtime, and owned-serving routes
7. publish a route matrix and compile-tested examples
8. run package-family and consumer-facing proof
9. return to replacement candidate freeze

## Non-Goals

- generic provider routing or a universal prompt API
- automatic provider, model, endpoint, target, credential, billing, or
  topology selection
- automatic sign-in, installation, update, or fallback
- flattening provider-native capabilities or lifecycle
- consumer prompts, tools, authorization, workflows, persistence, memory,
  retry policy, or UI
- making sandboxing mandatory
- removing low-level APIs
- adding a new crate solely for convenience

## Validation Strategy

- shared facade construction and safe-evidence assertions
- one deterministic operation through every prepared production route
- exact plan/specification agreement before effects
- version, access, topology, capability, drift, cancellation, deadline,
  failure, redaction, and joined-cleanup matrices
- local and remote-authoritative host proof where the underlying role supports
  both
- compile-tested route examples and package-family checks
- live installed or authenticated probes separately gated

## Stop Conditions

- a shared helper must choose a provider or consumer policy
- bound execution hides authority or changes terminal truth
- a facade merges distinct low-level operation roles
- adapter-private provider records would enter core or runtime
- a route needs a new durable lifecycle or access rule not covered by active
  contracts
- release mutation becomes necessary before complete provider-wide evidence

## Acceptance Criteria

- all 22 production routes have an adapter-local normal integration path
- consumers do not manually assemble adapter-fixed descriptors, bindings,
  plans, or matching request echoes
- every operation shape remains typed and distinct
- exact version and unverified-newer posture remain visible
- access and authority provenance remain honest
- low-level roles remain public
- deterministic facade conformance passes without credentials
- replacement candidate work resumes only after the route matrix is complete

## Promotion Targets

- revised Contract 037: provider-wide prepared facades and bound operation
  rules
- system architecture: route families, dependency direction, current Codex
  realization, and rollout gap
- roadmaps g02.007-g02.012 and cards 017-036
- front-door currentness and replacement-candidate hold

## Promotion Record

Promoted on 2026-07-25:

- Contract 037 now governs every adapter-local prepared facade and bound
  operation.
- System architecture records the 22-route family inventory and the
  implementation gap.
- Roadmaps g02.007-g02.012 sequence shared foundations, representative proofs,
  full route coverage, documentation, package proof, and candidate return.
- The former card 016 candidate freeze is held and superseded by card 036.
