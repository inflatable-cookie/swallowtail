# g01 Swallowtail Foundation

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Establish Swallowtail as an independent project, build its provider-neutral
contract and runtime kernels, prove real driver shapes, and begin consumer and
cross-adapter adoption without prematurely stabilizing the public API.

## Generation Runway

- standalone Northstar authority and repository QA
- cross-repository ownership and dependency direction
- portable identity, capability, model, reference, event, and error vocabulary
- deterministic conformance fixtures
- first Rust workspace validation and realized-architecture promotion
- multi-integration surface inventory before runtime decisions
- runtime-boundary reassessment before process or provider code
- provider-neutral runtime records, preflight, lifecycle, and host services
- synthetic cross-shape conformance
- separate Codex exec and app-server proof drivers
- later Soundcheck, Nucleus, and non-Codex proof roadmaps
- hosted transport and credential foundations
- OpenCode HTTP harness and Anthropic direct-inference proofs
- Gemini ACP and attached llama.cpp runtime proofs
- xAI connection-scoped direct inference and provider-billed-cost evidence
- provider-managed remote harness resources, durable sessions, event recovery,
  callbacks, interruption, and deletion truth
- realtime media portability and explicit provider-planned connection rollover

## Current Checkpoint

- standalone authority and repository QA are complete
- `swallowtail-core` and `swallowtail-testkit` exist
- the pure Contract 003 vocabulary compiles and passes focused unit tests
- reusable conformance fixtures exercise the public core API
- the Contract 003 implementation boundary is validated and closed
- runtime-boundary research and contract shaping are complete
- Nucleus and Soundcheck runtime requirements are mapped and promoted
- Contract 004 settles execution-host, Swallowtail, and consumer ownership
- operator scope now requires broad harness, API, SDK, CLI, protocol, and local
  runtime coverage
- Contract 005 separates integration family, adapter driver, transport,
  configured instance, and model route
- Contract 006 separates harness and direct-inference layers from operation
  shape, transport, credential, entitlement, endpoint audience, and support
  authority
- Research 003 covers Codex, Claude Code, OpenCode, Cursor, Pi, Kimi, Qwen
  Code, Gemini CLI, hosted APIs, ACP, and local serving runtimes
- GLM, Qwen, and DeepSeek hosted routes, open-weight artifacts, and self-hosted
  deployments remain independently identified
- Contract 007 separates artifacts, serving drivers, deployments, protocol
  facades, and model routes
- Spec 002 settles async-first scoped runtime semantics, capability-scoped host
  services, ordered bounded events, cancellation, cleanup, attachments,
  schemas, credentials, and extensions
- the conformance matrix covers one-shot CLI, long-lived RPC or ACP, hosted
  direct API, attached self-hosted, and owned self-hosted shapes
- the former two-consumer runtime-decision card is superseded before execution
- Contracts 008-011 promote registration/preflight, async lifecycle, host
  service/input, and conformance rules
- Rust compile probes settle the object-safe boxed-future posture
- runtime identity, access, instance, route, requirement, and parameterized
  capability records are realized in `swallowtail-core`
- pure dimensional preflight produces immutable bound plans and rejects stale
  instance revisions
- reusable testkit fixtures prove all Contract 008 rejection dimensions occur
  before recorded provider side effects
- roadmap 004 is complete
- roadmap 005 is complete
- `swallowtail-runtime` is realized without a global executor or concrete I/O
- separate dynamic driver roles consume immutable preflight plans
- bounded events, first-wins terminal outcomes, scoped cancellation, and
  attached-versus-owned serving authority are enforced in runtime primitives
- capability-scoped host ports and recording fixtures cover task, time,
  process, network, credential, resource, attachment, and diagnostic attempts
- cards 012-016 are complete
- five public, provider-free profile runners prove all common Contract 011
  assertions and their shape-specific lifecycle boundaries
- the runtime-kernel QA and dependency audit pass with 38 tests
- roadmap 006 is complete
- `swallowtail-host-local` resolves only host-approved executable,
  environment, and working-resource references
- bounded process I/O, EOF stop, force-stop, exit observation, and joined
  reader cleanup pass deterministic real-process fixtures
- runtime requests and outcomes now carry opaque, default-redacted operation
  content, and preflight plans expose their exact bound target and model
- `swallowtail-adapter-codex` implements text-only, read-only, ephemeral Codex
  exec runs through shared task and process ports
- deterministic JSONL, request translation, cancellation, redaction, and
  cleanup fixtures pass
- the Codex app-server driver adds local stdio initialization, paginated model
  discovery, open/resume sessions, streamed turns, native turn interruption,
  and joined process cleanup
- unsupported app-server callbacks and unowned inputs fail explicitly
- dual-driver tests prove distinct identities, transports, roles, capabilities,
  plan bindings, and lifecycle profiles behind one provider-neutral runtime
- core, runtime, testkit, and local-host crates remain free of Codex identity
  branches and reverse adapter dependencies
- 93 tests pass across the workspace
- current Soundcheck and Nucleus connector seams are mapped without modifying
  either consumer
- roadmap 007 sequences structured-run capability closure and a Soundcheck
  handoff; roadmap 008 preserves the later Nucleus callback/session lane
- cards 017-022 and roadmap 006 are complete
- provider-neutral reasoning metadata, explicit run network/search/reasoning
  policy, scoped materialization leases, temporary-resource cleanup authority,
  schema host service, and deadline observations are realized
- bounded local attachment/schema copies, scope-bound temporary working
  resources, explicit awaited lease release, and cancellable monotonic deadline
  waits are realized
- Codex exec binds image, JSON Schema output, reasoning, search/network, and
  deadline use exactly to the selected preflight plan and actual host services
- provider argv contains only host-materialized file references, explicit
  read-only/search configuration, and no ambient user configuration
- timeout, cancellation, process join, and materialization cleanup are proven
  across success and failure paths
- app-server model discovery carries supported reasoning modes and the provider
  default as catalog evidence
- a public structured-run parity fixture covers model/default/reasoning
  discovery, schema, image, search, progress, output, timeout, cancellation,
  cleanup, and redaction without consumer types
- the Soundcheck adoption handoff fixes one connector seam, exact-revision
  pinning, a temporary feature gate, host wiring, downstream validation,
  rollback, and legacy removal without editing the consumer
- roadmap 007 and cards 022-025 are complete
- Contract 012 and runtime records now fix session instructions, exact
  reasoning, bounded tool declarations, callback correlation, event ordering,
  wait abandonment, and consumer-owned execution authority
- the long-lived RPC profile proves callback response, timeout, and late
  response behavior without adding a generic executor
- Codex app-server translates exact session options and declared dynamic-tool
  callbacks through bounded, cancellable exchanges without executing tools
- current Codex app-server schema evidence does not permit dynamic-tool
  redeclaration on resume; tool-enabled resume fails before provider work
- host service sets carry their owning execution-host identity and cannot be
  substituted after preflight
- session resume bindings preserve provider session, configured instance,
  execution host, model route, and model identity
- local and remote-authoritative fixtures prove open, resume, callback wait,
  interruption, close, disconnect, redaction, and joined cleanup
- 102 tests pass across the workspace
- the Nucleus adoption handoff fixes the existing live-registry seam, an
  embedded-host first slice, callback projection, unsafe-resume constraint,
  exact-revision pinning, temporary gate, validation, rollback, and legacy
  removal without editing the consumer
- roadmap 008 and cards 026-029 are complete
- Soundcheck now routes model discovery and every structured Codex turn through
  Swallowtail; automated tests and authenticated catalogue discovery pass
- consumer feedback added typed search/reasoning progress and bounded catalogue
  deadlines without importing Soundcheck policy
- roadmap 009 and card 030 are complete with native consumer acceptance
- Nucleus Agent Chat now has native and authenticated parity through
  Swallowtail
- Contract 013 separates interactive resource, filesystem, approval, network,
  provider-request, deadline, and cleanup policy
- provider-neutral access records and preflight require exact agreement across
  policy, capability, host service, execution host, resource lease, and
  declared provider-request extensions before effects
- bounded Codex sessions map one host-resolved filesystem root into
  `workspace-write`/`workspaceWrite`, deny provider network, exclude ambient
  temporary roots, and retain approval posture `never`
- approval and user-input requests are correlated, rejected at the provider
  boundary, and surfaced as explicit observe-and-stop terminal outcomes
- local and remote-authoritative fixtures prove the same public seam without
  importing Nucleus identity or workflow types
- the Nucleus task-execution handoff fixes exact consumer mapping and
  downstream validation
- full repository QA passes with 119 tests
- roadmap 010 and cards 031-034 are complete
- provider evidence is refreshed in Research 004
- Contract 014 settles scoped endpoint grants, credential leases, direct
  streaming, catalogue limits, and provider usage or limit evidence
- roadmaps 011-015 compile the first cross-adapter, ACP, and self-hosted runway
- roadmap 019 completes the owned ephemeral llama.cpp lifecycle; roadmap 018
  is now active on the current Kimi successor boundary
- roadmap 011 and cards 035-038 are complete
- scoped endpoint grants and credential leases bind operation, reference, and
  audience and expose only redacted driver accessors
- structured direct runs no longer need a placeholder working resource; Codex
  retains its explicit resource requirement before process start
- model token limits, token usage, rate-limit state, and quota state are typed
  mutable evidence without retry or billing behavior
- the local host implements explicit endpoint, secret, and delegated-credential
  approvals with audience checks and tracked awaited lease release
- hosted Contract 014 conformance and full repository QA pass with 129 tests
- the versioned OpenCode `1.14.48` fixture freezes six attached HTTP/SSE routes,
  exact provider/model identity, deny-first read-only permissions, ordered
  success and failure events, abort, disconnect, and unknown-event handling
- card 039 is complete with six focused tests
- the production OpenCode driver uses host-approved libcurl blocking work,
  bounded SSE delivery, delegated credentials, exact provider/model identity,
  read-only sessions, abort, deadlines, and joined cleanup
- a sixth provider-neutral conformance profile covers attached HTTP/SSE
  harnesses without importing process or direct-inference behavior
- roadmap 012 and cards 039-041 are complete
- full repository QA passes with 150 tests
- the Anthropic `2023-06-01` public-API fixture freezes Models pagination,
  explicit message output bounds, SSE order, cumulative usage, request and rate
  evidence, errors, unknown events, local-only cancellation, and one attempt
- card 042 is complete with nine focused tests
- full repository QA passes with 159 tests
- the Anthropic direct driver adds an explicit output-token request bound,
  bounded catalogue pagination, one-attempt Messages SSE, typed request/rate/
  usage evidence, safe errors, local cancellation, and awaited secret cleanup
- the hosted-direct profile covers explicit output bounds; 18 focused adapter
  and profile tests pass without a live credential
- full repository QA passes with 168 tests; the separately gated OpenCode
  installed probe remains ignored by default
- roadmap 013 and cards 042-044 are complete
- ACP wire version 1 and stable schema release `v1.19.0` are distinct from SDK
  and agent artifact versions under Contract 015
- the Gemini CLI `0.51.0` fixture freezes a new-session, text-prompt, update,
  permission-cancel, filesystem-read, turn-cancel, and process-close subset
- authentication mutation, consumer membership, load/resume, mode/model
  mutation, MCP injection, writes, terminals, and native session close remain
  excluded from the first Gemini driver
- eight focused ACP fixture tests pass without a binary or credential
- the production ACP layer owns bounded v1 NDJSON framing and correlation with
  no Gemini identity branch
- `WorkingResourceIo` separates provider filesystem callbacks from consumer
  tools and rejects traversal or symlink escape under the leased read-only root
- the Gemini CLI `0.51.0` adapter pins API-key access identity, isolated
  process state, Plan Mode, new text sessions, updates, permission
  cancellation, native turn cancellation, and joined process close
- 29 focused adapter, protocol, and local-host tests pass without a Gemini
  binary or credential
- a seventh provider-neutral profile covers long-lived ACP without weakening
  the six existing transport and serving shapes
- deterministic Gemini conformance covers local and remote-authoritative
  topology, read callbacks, permission observation, cancellation, disconnect,
  terminal event closure, redaction, and joined cleanup
- prompt correlation now precedes waiter-task spawn, preventing cancellation
  from overtaking the active ACP request
- full repository QA passes with 190 tests; Gemini and OpenCode installed
  probes remain separately gated and ignored by default
- doctor remains at the pre-existing 19 findings with no new oversized-file
  debt
- roadmap 014 and cards 045-047 are complete
- llama.cpp build `9910`, its bounded native/OpenAI-compatible facade, and one
  operator-supplied 1.19 MB Stories 260K fixture are frozen without model or
  server ownership
- eight deterministic llama.cpp protocol tests pass without a server, model,
  credential, or network request
- the production llama.cpp driver observes readiness, exact build, template
  capabilities, modality posture, and route alias before catalogue or run work
- one bounded Chat Completions SSE attempt preserves output, usage,
  cancellation, deadline, failure, redaction, and joined cleanup while the
  attached server remains running
- local and remote-authoritative hosts pass the same public seam; 17 focused
  llama.cpp tests and full QA pass with 207 tests
- roadmap 015 and cards 048-050 are complete
- Research 005 revalidates xAI WebSocket, Kimi Code ACP `0.28.0`, current
  SDK wrappers, and owned self-hosted authority
- Contract 016 settles resource-free direct sessions, session-bound network
  and credential leases, connection-local continuation, no implicit recovery,
  and exact provider-billed-cost evidence
- xAI Responses WebSocket is selected as the next highest-information proof;
  Kimi ACP, owned llama.cpp, and SDK-native work remain sequenced behind it
- roadmap 016 and card 051 are complete
- the xAI evidence snapshot freezes the exact `/v1/responses` WebSocket
  upgrade and bearer boundary, serial `store=false` turns, private latest-
  response continuation, ordered text events, terminal usage, exact billed
  ticks, provider failures, disconnect, and cancellation by connection close
- eight focused fixture tests pass against a deterministic loopback WebSocket;
  no provider credential or external inference request is used
- resource-free session records now represent absence without a fake working
  resource; existing resource-bound sessions retain their explicit constructor
- exact provider-billed-cost evidence carries USD scale, cumulative-replacement
  semantics, turn, route, access profile, and provider-attempt identity
- the production xAI driver owns one host-approved WebSocket and API-key lease
  across serial `store=false` turns with private latest-response continuation
- deterministic driver tests prove concurrent rejection before a second frame,
  cancellation and deadline invalidation, disconnect, distinct provider
  failures, safe diagnostics, and task join before credential release
- 91 focused tests, strict focused clippy, all-target workspace compile,
  formatting, and diff checks pass; doctor remains at the known 19 findings
- an eighth provider-neutral profile covers connection-scoped direct sessions
  without importing xAI identity
- local and remote-authoritative xAI fixtures prove two chained turns, terminal
  event closure, exact turn cost, redaction, and joined connection cleanup
  before one awaited credential release
- full repository QA passes with 227 tests; the two installed/live probes
  remain separately gated and ignored by default
- roadmap 017 and cards 052-054 are complete
- Research 006 supersedes the Kimi `0.28.0` planning pin with provider release
  `0.28.1`, adapter package `0.3.4`, exact ACP SDK `0.23.0`, wire `1`, and
  stable schema artifact `v1.19.1`
- current ACP and tagged Kimi source agree that load replays history before its
  response while resume reattaches without replay
- Contract 017 binds provider-owned sessions to exact route, host, resource,
  access, and policy identity and separates replay from live turns
- bounded text writes now require exact `ReadWrite` callback authority, but
  callback mediation remains separate from provider tool approval and process
  filesystem containment
- Kimi accepts both existing OAuth state and configured non-OAuth provider
  credentials; the first proof selects only an isolated pre-existing delegated
  harness route and launches no sign-in action
- the exact Kimi ACP fixture corpus pins provider, adapter, SDK, wire, schema,
  source hashes, access, and exclusions independently
- deterministic new, load, resume, prompt, cancellation, write, drift, auth,
  and disconnect transcripts pass 10 adapter-local assertions without changing
  the provider-neutral ACP decoder
- load replay must finish before its response, resume must not replay, and
  wrong-session or write-authority mismatches fail closed
- cards 055-056 are complete with the second-agent protocol corpus
- Research 007 finds no current local mechanism that meets Contract 017:
  Landlock alone is incomplete, macOS App Sandbox needs signed deployment
  authority, `sandbox-exec` is deprecated, and the Windows process-sandbox API
  is experimental
- Contract 017 now rejects partial, best-effort, deprecated, private,
  experimental, and locally fabricated remote containment claims
- the original local-only card 057 stopped because the current host has no
  qualifying dynamic process-filesystem containment mechanism
- Research 009 records the operator's native macOS App Sandbox helper
  selection: one persisted user-selected project grant, isolated Kimi state,
  inherited descendants, and no container or broad home access
- Research 010 repairs the successor currentness error: `0.28.1` already is the
  maintained TypeScript Kimi Code line, not the wound-down Python line
- annotated tag objects and peeled source commits are now distinct for both
  Kimi Code and ACP schema `v1.19.1`
- the exact official arm64 archive, extracted executable, upstream signature,
  dynamic-code entitlements, isolated state root, exclusions, and upgrade gate
  are frozen for the deployment helper proof
- current tagged source retains separate load-with-replay and resume-without-
  replay; the existing protocol transcripts remain current
- Research 008 pins llama.cpp `b10069`, proves its race-free loopback port-zero
  bind, and excludes downloads, router mode, tools, UI, public listeners, and
  persistent serving
- Contract 018 promotes read-only model-artifact leases, readiness-before-
  handle, safe host-scoped endpoint handoff, and joined child cleanup before
  endpoint and artifact release
- provider-neutral artifact identities and preflight bindings are now distinct
  from opaque host references and request attachments
- the runtime exposes read-only artifact leases plus host-scoped serving
  endpoint publication without a global registry or generic inference handle
- owned start substitution and host-service drift fail before recorded process,
  network, artifact, or endpoint effects
- full repository QA passes with 230 tests; doctor remains at the inherited 19
  findings, including 7 errors
- the local host now binds exact approved regular-file artifacts to one
  execution host and verifies their SHA-256 digest before issuing read-only
  serving leases without taking deletion authority
- only exact nonzero loopback HTTP sockets can become dynamic scoped endpoint
  grants, and awaited release invalidates their network authority
- owned conformance records artifact acquisition, process start, endpoint
  publication, process stop and join, endpoint release, and artifact release in
  contract order under both local and remote-authoritative host identities
- 50 focused local-host and testkit tests pass; full QA passes with 236 tests
- the distinct b10069 owned driver launches only one host-approved artifact on
  exact loopback port zero with offline, no-UI, and no-agent flags
- bounded startup supervision remains active through health, exact build, and
  single-route readiness; no handle appears before all evidence agrees
- deterministic failure fixtures prove malformed, duplicate, non-loopback,
  early-exit, build-mismatch, graceful-stop, forced-stop, joined process,
  endpoint-release, artifact-release, and redaction behavior
- 25 focused llama.cpp tests and all 254 repository tests pass; warnings-denied
  clippy passes and doctor remains at the inherited 19 findings
- the common owned profile now passes beside the production b10069 driver, and
  local plus remote-authoritative fixtures preserve serving scope, host,
  ownership, and ordered cleanup through the public handle
- deterministic readiness timeout and route-mismatch fixtures join the child
  before endpoint and artifact release and keep endpoint values and artifact
  paths out of diagnostics
- the b9910 attached suite remains distinct and proves cleanup cannot stop its
  external server
- 28 focused llama.cpp tests and all 257 repository tests pass; two installed
  or live probes remain gated, warnings-denied clippy passes, and doctor
  remains at the inherited 19 findings
- roadmap 019 and cards 060-064 are complete
- card 065 is complete with 22 focused ACP fixture tests
- full repository QA passes with 259 tests; doctor remains at the inherited 19
  oversized-file findings, including 7 errors
- Research 011 proves dynamic security-scoped bookmark propagation through a
  compatible inherited App Sandbox helper, including shell and background
  descendants and denied outside-root access
- the exact Kimi `0.28.1` artifact crashes during V8 initialization under the
  documented helper signature; JITless then stalls on an extracted native
  module and is not a supported route
- Contract 017's exact-runtime stop condition fired before portable records or
  a host-enforced provider mapping
- Research 012 finds T3 Code and similar orchestrators map harness-native
  permission or sandbox controls without imposing one portable outer sandbox
- the operator selected optional explicit isolation: `AmbientHost`,
  `ProviderEnforced`, and `HostEnforced` remain distinct with no fallback
- generic harness sessions now default to ambient disclosure; Codex retains
  explicit provider-enforced read-only and bounded-workspace profiles
- Gemini ACP and attached OpenCode are reclassified as ambient because their
  working resources and permission rules do not contain every process path
- card 057 is complete as negative capability evidence; card 066 rebaselines
  shared records and card 058 is ready for ambient Kimi ACP
- full repository QA passes with 262 tests; installed/live probes remain gated
- doctor remains at the inherited 19 oversized-file findings, including 7
  errors
- the production Kimi Code `0.28.1` ACP driver is an explicit `AmbientHost`
  route with isolated provider state and an opaque delegated membership lease
- provider-owned load returns bounded ordered replay before a ready handle;
  resume remains a separate replay-free operation
- durable session bindings now fix working resource and expanded access policy
  beside provider, instance, host, route, and model identity
- one-MiB text replacement callbacks require an exact preflight capability and
  `ReadWrite` filesystem lease; callback authority makes no process-containment
  or provider-tool approval claim
- deterministic Kimi fixtures prove new, load, resume, prompt, write,
  cancellation, pre-effect binding rejection, and joined resource/credential
  cleanup without adding a Kimi branch to shared ACP framing
- card 058 is complete; card 059 is complete with cross-agent conformance and
  roadmap 018 closeout
- full repository QA passes with 269 tests; Gemini and OpenCode installed
  probes remain separately gated and ignored by default
- one provider-neutral ACP decoder accepts pinned Gemini and Kimi corpora;
  Kimi-only lifecycle methods do not widen Gemini's baseline claims
- a ninth provider-neutral profile composes persistent load, replay, resume,
  bounded text write, delegated auth, ambient authority, topology, redaction,
  and ordered cleanup
- the production Kimi lifecycle passes under local and remote-authoritative
  host ids; its installed `0.28.1` check remains explicitly gated
- roadmap 018 and cards 055-059, 065-066 are complete
- Research 013 selects provider-supported Bedrock Runtime `ConverseStream` as
  the first real in-process Rust SDK route; Contract 019 fixes explicit SDK,
  delegated cloud credential, private executor, and one-attempt boundaries
- the fixture-first Bedrock adapter pins `aws-sdk-bedrockruntime = 1.136.0` and
  proves generated typed text, stop, usage, failure, semantic-drift, and retry
  configuration behavior without AWS access
- the production Bedrock Runtime driver binds one exact host, endpoint, region,
  delegated AWS credential provider, route, model, positive output bound, and
  one SDK attempt without ambient AWS configuration
- typed bounded EventStream output and usage, cancellation, complete-stream
  deadline, and credential release finish inside joined operation-private
  Tokio work
- local and remote-authoritative deterministic fixtures pass the hosted-direct
  profile without a credential, account, network request, or paid inference
- Research 014 fixes the separate native Bedrock catalogue to
  `aws-sdk-bedrock = 1.148.0` and one non-paginated
  `ListFoundationModels` request; the distinct Mantle `/models` audience stays
  outside roadmap 021
- Contract 020 keeps bounded catalogue lifecycle, modality, streaming,
  inference, customization, and provider-extension observations separate from
  runtime capability, entitlement, and route selection
- provider-neutral observations preserve source, absent-versus-empty evidence,
  lifecycle transitions, advertised streaming, and bounded namespaced drift
  without creating provider ids, model routes, or runtime capabilities
- the Bedrock adapter pins `aws-sdk-bedrock = 1.148.0`; generated unfiltered
  request, single-response, summary, lifecycle, error, explicit-config, one-
  attempt, bound, and redaction fixtures pass without AWS access
- roadmap 020 and cards 067-069 are complete; cards 070-071 establish the
  separate control-plane boundary and generated SDK corpus
- full repository QA passes with 283 tests; three installed/live probes remain
  gated and ignored by default
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors
- the separately registered Bedrock catalogue descriptor exposes only the
  model-catalogue role over `rust-sdk-control-plane`; its instance, endpoint,
  region, access profile, delegated provider, and permission boundary remain
  independent from Bedrock Runtime inference
- one unfiltered, non-paginated, one-attempt SDK request runs inside joined
  host blocking work; deadline signalling finishes the operation-private Tokio
  executor before credential release
- local and remote-authoritative fixtures prove exact endpoint use, bounded
  observation projection, drift rejection, redaction, provider failure, and
  cleanup without creating a model route or provider identity
- the new core and adapter catalogue files were split before closeout; doctor
  remains at the inherited 19 findings with no new structural debt
- full repository QA passes with 291 tests; three installed/live probes remain
  gated and ignored by default
- Research 015 selects OpenAI Responses background mode as the next proof: a
  provider-owned async run, required temporary retention, recoverable SSE
  cursor, and native cancellation through one public-API access boundary
- ChatGPT, Codex, harness, subscription, and community OAuth access remain
  separate; no endpoint, model, provider, or consumer route becomes a default
- Cursor Cloud Agents remain later behind explicit GitHub, repository,
  provider-VM, artifact, and durable-agent deletion authority
- roadmap 022 and card 073 select roadmap 023's background Responses proof
- Contract 021 makes provider-managed background execution, temporary
  retention, and bounded reattachment independent opt-ins; ordinary structured
  runs remain attached, retention-prohibited, and non-reattaching
- provider cancellation truth now preserves confirmed, completion-raced, and
  unconfirmed outcomes separately from the local terminal status
- the dated OpenAI public-API fixture corpus fixes create, retrieve, one SSE
  reattachment, cancel, all background statuses, usage, rate, failure,
  identity separation, redaction, and one-attempt loopback behavior
- the production OpenAI background driver binds the public API endpoint,
  API-key lease, exact model route, output bound, explicit deadline, temporary
  retention, one create attempt, and one maximum cursor reattachment
- ordered SSE phase and cursor validation, output agreement, bounded retrieve,
  native cancel, usage, rate, request correlation, redaction, and provider
  cancellation truth pass deterministic local and remote-authoritative tests
- roadmap 023 and cards 074-075 are complete
- full repository QA passes with 314 tests; three installed/live probes remain
  gated, and doctor stays at the inherited 19 findings
- Research 016 selects Claude Managed Agents as the next high-information
  shape without requiring repository authority or a local container
- Contract 022 fixes operator-owned agent configuration, driver-owned remote
  environment and session resources, durable retention, provider-managed
  rescheduling, authoritative persisted events, callbacks, interruption,
  deletion truth, metering, and joined cleanup
- roadmap 024 and card 076 are complete; roadmap 025 is complete
- card 077 realizes durable-retention, managed-recovery, owned-resource
  deletion, structured-run tool/callback records, exact preflight rejection,
  and the dated Managed Agents REST/SSE and loopback corpus
- card 078 adds the separately registered production remote-harness driver,
  exact agent/version preflight identity, structured-run callback ownership,
  bounded recovery, interruption, usage evidence, and ordered resource cleanup
- nine deterministic production-driver tests cover success, callback,
  rescheduling, reconnect, cancellation, deadline, provider failure, deletion
  ambiguity, redaction, and lease release without live access
- 181 affected-crate tests and focused warnings-denied clippy pass; doctor is
  restored to the inherited 19 findings after splitting new lifecycle files
- card 079 adds the tenth provider-neutral conformance profile and proves the
  production driver under local and remote-authoritative host identities
- roadmap 025 and cards 077-079 are complete
- full repository QA passes with 330 tests; three installed/live probes remain
  gated, and doctor stays at the inherited 19 findings
- Research 017 selects stable Qwen Code `v0.19.11` headless after rechecking
  remote ACP, Cursor Background Agents, and the experimental Qwen daemon
- Contract 023 makes harness isolation explicit for structured runs without
  treating safe mode, tool permissions, native budgets, or optional sandboxing
  as process containment
- card 080 adds exact common requirement/runtime-policy isolation bindings and
  pure mismatch validation while leaving older structured drivers consistently
  unbound until migration
- the fixture-only `swallowtail-adapter-qwen` crate freezes text stdin,
  stream-JSON output, a registry-level read-only tool allowlist, native exits,
  durable local retention, redaction, and explicit `AmbientHost` posture
- 110 focused core, runtime, testkit, and Qwen fixture tests pass without a
  Qwen binary, credential, provider request, or container
- the separately registered Qwen headless production driver binds one exact
  executable, environment, working resource, delegated harness access profile,
  provider, model, deadline, durable-retention acceptance, and `AmbientHost`
  policy before process start
- exact frozen argv and stdin content transport, bounded stream JSON, typed
  usage, native budget exits, provider/protocol failure, cancellation, timeout,
  redaction, process wait, and task join pass 13 focused Qwen tests
- no sandbox, container, resume, transcript deletion, background execution,
  provider fallback, or direct-inference authority is claimed
- doctor remains at the inherited 19 findings after splitting the new parser
  and test files
- Qwen passes the unchanged provider-neutral one-shot structured-CLI profile;
  a separate Contract 023 assertion pack covers ambient authority, explicit
  durable retention, native-budget independence, and no transcript-deletion
  claim without creating an eleventh profile
- local and remote-authoritative production fixtures prove success, native
  exits, provider and protocol failure, disconnect, cancellation, timeout,
  redaction, pre-effect isolation rejection, process wait, and task join
- full repository QA passes with 360 tests; three installed/live probes remain
  separately gated, and doctor stays at the inherited 19 findings
- roadmap 026 and cards 080-082 are complete; roadmap 027 and card 083 then
  opened the direct Kimi Platform, DeepSeek, Z.AI, and Alibaba/Qwen
  compatibility evidence checkpoint
- Research 018 revalidates Kimi Platform, DeepSeek, Z.AI, and Alibaba Model
  Studio from official sources and keeps their public, subscription, plan,
  region, workspace, and product audiences separate
- Kimi Platform K3 is selected as the first direct-provider breadth proof: one
  stateless Chat Completions stream, one authenticated catalogue, exact
  `api.moonshot.ai` pay-as-you-go access, and no Membership or Kimi Code key
- Contract 024 limits common Chat Completions reuse to bounded structural wire
  truth; provider access, model, capability, error, lifecycle, retry, and
  fallback semantics remain in separate adapters
- `swallowtail-protocol-openai-chat` now realizes that structural boundary with
  only a `serde_json` dependency; independent llama.cpp and Kimi corpora pass
  the same fragmented SSE and JSON decoder
- the llama.cpp build-9910 request and semantic mapping use the common codec;
  all attached, owned-serving, cancellation, usage, and conformance behavior
  remains green
- the dated offline K3 corpus freezes platform access, catalogue, exact request,
  reasoning/output/usage, errors, unknowns, model mismatch, disconnect, fixed
  parameter omissions, and exclusions without a credential or network request
- the separate Kimi Platform production adapter binds one exact public API-key
  audience, authenticated catalogue, `kimi-k3` route, reasoning selection,
  output bound, and one streaming attempt before host effects
- ordered reasoning, output, usage, returned-model agreement, provider error,
  unknown semantics, disconnect, cancellation, deadline, redaction, joined
  connection work, and awaited credential cleanup pass 12 focused tests
- the unchanged hosted-direct profile proves the Kimi Platform production
  route under local and remote-authoritative execution-host identities
- exact topology, one-attempt inference, source-scoped catalogue truth,
  connection-before-credential cleanup, and no fallback or detached work pass
  deterministic conformance
- 93 focused Kimi Platform, compatible-chat, llama.cpp, and testkit tests pass;
  full repository QA passes with 384 tests and three gated probes ignored
- doctor remains at the inherited 19 findings with no new oversized file
- roadmap 028 and cards 084-086 are complete
- Research 019 revalidates DeepSeek V4, Z.AI GLM-5.1, and Alibaba Model Studio
  from official 2026-07-22 evidence
- Alibaba Model Studio's Singapore workspace-dedicated Conversations and
  Responses route is selected for provider-owned direct-session context and
  separate conversation-item deletion truth
- Contract 025 fixes explicit session retention, exact regional workspace
  access, local-only cancellation, and item-before-conversation cleanup
- cards 087-089 and roadmap 029 are complete; the production Alibaba driver
  realizes the exact two-turn conversation and ordered deletion boundary
- local and remote-authoritative fixtures preserve the same instance, access,
  route, lifecycle, redaction, cleanup, and joined-release truth
- full repository QA passes with 404 tests; three installed or live probes
  remain separately gated
- doctor remains at the inherited 19 findings with no new oversized file
- Research 020 inventories all sixteen production drivers and ten common
  profiles, then selects OpenAI Realtime GA as the missing duplex-media shape
- Contract 026 fixes a separate realtime-media direct-session role, exact
  formats, bounded chunks, consumer playback ownership, cancellation, and
  joined cleanup
- card 091 realizes the separate role, records, pure preflight, eleventh common
  profile, and frozen OpenAI corpus without changing the sixteen production
  routes
- full repository QA passes with 416 tests; three installed or live probes
  remain separately gated, and doctor remains at the inherited 19 findings
- card 092 adds the seventeenth production route: one exact OpenAI Realtime GA
  WebSocket session with host-approved public API-key access, two serial audio
  responses, native cancellation, typed evidence, and joined credential-last
  cleanup
- all 23 focused OpenAI adapter tests and warnings-denied clippy pass; doctor
  remains at the inherited 19 findings with no new oversized file
- card 093 proves exact plan identity, serial and parallel behavior, provider
  failure, unknown semantics, format drift, disconnect, confirmed and
  unconfirmed cancellation, deadline, redaction, and credential-last cleanup
  under local and remote-authoritative hosts
- the eleventh common profile records separate provider usage, rate, quota,
  and request evidence; the production driver passes without shared provider
  branches
- roadmap 031 and cards 091-093 are complete; full repository QA passes with
  430 tests and doctor remains at the inherited 19 findings
- roadmap 032 and card 094 complete the post-realtime provider-coverage
  checkpoint
- Research 021 selects Gemini Live preview; Contract 027 separates one
  opt-in planned connection rollover from reconnect, reattachment, retry,
  consumer resume, and durable provider state
- roadmap 033 and cards 095-097 own exact Gemini Live records, corpus, driver,
  portability conformance, and closeout; card 095 is the sole ready task
- Grok Build remains later harness breadth; remote ACP remains behind
  implementation and hardening evidence; Cursor remains beta and policy-heavy

## Milestones

- [001 Standalone Authority Foundation](001-standalone-authority-foundation.md)
  — completed
- [002 Portable Contract Kernel](002-portable-contract-kernel.md) — completed
- [003 Integration Landscape and Runtime Boundary](003-integration-landscape-and-runtime-boundary.md)
  — completed
- [004 Runtime Records and Preflight](004-runtime-records-and-preflight.md) — completed
- [005 Async Runtime and Conformance](005-async-runtime-and-conformance.md) — completed
- [006 Codex Proof Drivers](006-codex-proof-drivers.md) — completed
- [007 Soundcheck Structured-Run Readiness](007-soundcheck-structured-run-readiness.md)
  — completed
- [008 Nucleus Interactive-Session Readiness](008-nucleus-interactive-session-readiness.md)
  — completed
- [009 Soundcheck Consumer Adoption](009-soundcheck-consumer-adoption.md) — completed
- [010 Bounded Workspace Session Access](010-bounded-workspace-session-access.md) — completed
- [011 Hosted Transport Foundations](011-hosted-transport-foundations.md) — completed
- [012 OpenCode HTTP Harness Proof](012-opencode-http-harness-proof.md) — completed
- [013 Anthropic Direct Inference Proof](013-anthropic-direct-inference-proof.md) — completed
- [014 Gemini ACP Proof](014-gemini-acp-proof.md) — completed
- [015 llama.cpp Attached Runtime Proof](015-llama-cpp-attached-runtime-proof.md) — completed
- [016 Post-Tranche Coverage Checkpoint](016-post-tranche-coverage-checkpoint.md) — completed
- [017 xAI Responses WebSocket Proof](017-xai-responses-websocket-proof.md) — completed
- [018 Kimi Code ACP Portability Proof](018-kimi-code-acp-portability-proof.md) — completed
- [019 Owned llama.cpp Serving Proof](019-owned-llama-cpp-serving-proof.md) — completed
- [020 Post-Portability Coverage Expansion](020-post-portability-coverage-expansion.md) — completed
- [021 Bedrock Control-Plane Catalogue Proof](021-bedrock-control-plane-catalogue-proof.md)
  — completed
- [022 Post-SDK Coverage Checkpoint](022-post-sdk-coverage-checkpoint.md) — completed
- [023 OpenAI Background Responses Proof](023-openai-background-responses-proof.md)
  — completed
- [024 Post-Background Coverage Checkpoint](024-post-background-coverage-checkpoint.md)
  — completed
- [025 Claude Managed Agent Remote Harness Proof](025-claude-managed-agent-remote-harness-proof.md)
  — completed
- [026 Qwen Headless Structured Harness Proof](026-qwen-headless-structured-harness-proof.md)
  — completed
- [027 Direct Provider Compatible-Codec Checkpoint](027-direct-provider-compatible-codec-checkpoint.md)
  — completed
- [028 Kimi Platform K3 Direct Inference Proof](028-kimi-platform-k3-direct-inference-proof.md)
  — completed
- [029 Remaining Direct Provider Breadth](029-remaining-direct-provider-breadth.md)
  — completed
- [030 Post Direct Provider Breadth Coverage Checkpoint](030-post-direct-provider-breadth-coverage-checkpoint.md)
  — completed
- [031 OpenAI Realtime Media Direct Session Proof](031-openai-realtime-media-direct-session-proof.md)
  — completed
- [032 Post-Realtime Coverage Checkpoint](032-post-realtime-coverage-checkpoint.md)
  — completed
- [033 Gemini Live Realtime Portability Proof](033-gemini-live-realtime-portability-proof.md)
  — active

## Batch Shape

- cards 010-011 form the runtime-record and preflight batch
- cards 012-014 form the runtime-role, lifecycle, and host-service batch
- cards 015-016 form the synthetic conformance and validation batch
- cards 017-021 form the first process, Codex proof, and adoption-readiness lane
- cards 022-025 form the Soundcheck shared-readiness and handoff lane
- cards 026-029 form the later Nucleus shared-readiness and handoff lane
- card 030 closed the first real Soundcheck consumer feedback loop
- cards 031-034 completed the bounded interactive workspace-access lane
- cards 035-038 form the hosted transport foundation lane
- cards 039-041 form the OpenCode HTTP harness proof
- cards 042-044 form the Anthropic direct-inference proof
- cards 045-047 form the Gemini ACP proof
- cards 048-050 form the attached llama.cpp proof
- card 051 closed the post-tranche provider-coverage checkpoint
- cards 052-054 form the xAI Responses WebSocket proof
- cards 055-059 form the Kimi Code ACP portability proof
- cards 060-064 form the owned llama.cpp serving proof
- card 065 repairs the Kimi Code successor pin before production mapping
- card 066 makes harness isolation optional and explicit before Kimi mapping
- cards 067-069 form the post-portability evidence, selected-boundary, and
  production-proof lane
- cards 070-072 form the separate Bedrock control-plane catalogue lane
- card 073 closes the post-SDK coverage checkpoint
- cards 074-075 form the OpenAI background Responses proof
- card 076 closes the post-background coverage checkpoint
- cards 077-079 form the Claude Managed Agents remote-harness proof
- cards 080-082 form the Qwen Code headless structured-harness proof
- card 083 opens the direct-provider compatible-codec evidence checkpoint
- cards 084-086 form the Kimi Platform K3 direct-inference proof
- cards 087-089 form the Alibaba direct-conversation selection, records,
  fixtures, and production-proof lane
- card 090 closes the post-direct-provider breadth evidence checkpoint
- cards 091-093 form the OpenAI Realtime media records, driver, and conformance
  lane
- card 094 closes the post-realtime provider-coverage checkpoint
- cards 095-097 form the Gemini Live rollover records, driver, and portability
  conformance lane

Run validation after each complete batch, not after each small card.

## Generation Boundary

g01 is a long-lived generation. It normally remains active until it contains
roughly 30-50 numbered roadmap files. Batch cards do not count toward that
range. New implementation phases extend g01 rather than creating a generation
per phase.
