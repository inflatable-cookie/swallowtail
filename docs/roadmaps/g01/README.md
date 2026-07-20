# g01 Swallowtail Foundation

Status: active
Owner: Tom
Updated: 2026-07-20

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

## Batch Shape

- cards 010-011 form the runtime-record and preflight batch
- cards 012-014 form the runtime-role, lifecycle, and host-service batch
- cards 015-016 form the synthetic conformance and validation batch
- cards 017-021 form the first process, Codex proof, and adoption-readiness lane
- cards 022-025 form the Soundcheck shared-readiness and handoff lane
- cards 026-029 form the later Nucleus shared-readiness and handoff lane
- card 030 closed the first real Soundcheck consumer feedback loop
- cards 031-034 completed the bounded interactive workspace-access lane

Run validation after each complete batch, not after each small card.

## Generation Boundary

g01 is a long-lived generation. It normally remains active until it contains
roughly 30-50 numbered roadmap files. Batch cards do not count toward that
range. New implementation phases extend g01 rather than creating a generation
per phase.
