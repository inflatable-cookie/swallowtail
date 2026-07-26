# Changelog

All notable Swallowtail changes will be recorded here.

The project has not published a release.

## Unreleased

### Release Candidate

- prepared one non-published 23-package candidate at coordinated version
  `0.1.0`, replacing the superseded compile-only candidate
- declared Rust `1.93` for 22 packages and Rust `1.94.1` for the Bedrock
  adapter; current-stable validation uses Rust `1.97.1`
- added compatible `0.1.0` registry requirements to every internal normal
  dependency while retaining workspace paths
- added deterministic package metadata, public-declaration, documentation,
  MSRV, content, checksum, source-bundle, reproducibility, all-route prepared-
  facade, and isolated consumer runtime gates
- classified `0.1.0` as the initial pre-1.0 public API and guaranteed-behavior
  baseline; there is no prior published Swallowtail release
- kept provider and harness version guarantees separate from the crate version

### Changed

- Codex installed-version probe failures now retain their stable diagnostic
  code while reporting numeric exit status and bounded sanitized stderr when
  available
- interactive session requests now require one explicit or plan-derived
  agreement for access, provider state, and harness configuration; implicit
  policy defaults and post-construction setters are removed before first
  publication
- replaced the superseded unreleased `0.1.0` declaration baseline across core,
  runtime, local host, testkit, and all 16 adapters; the two protocol crates
  and remote ACP transport retain their prior declaration hashes
- made adapter-local preparation and typed bound operations the normal public
  path for all 22 production routes while retaining low-level public roles

### Added

- standalone strict-Northstar repository authority
- initial vision, architecture, contracts, specs, and generation roadmap
- `swallowtail-core` provider-neutral identity, capability, model, reference,
  event, extension, and diagnostic records
- `swallowtail-testkit` canonical Contract 003 fixtures and reusable public
  conformance assertions
- native Effigy Rust formatting, checking, and test selectors
- multi-integration identity and transport-diversity contract
- integration-landscape research gate before runtime API decisions
- provider-neutral runtime identity, access, configured-instance, model-route,
  requirement, and parameterized-capability records
- pure dimensional preflight with immutable bindings and stale-plan rejection
- reusable Contract 008 zero-side-effect fixtures and assertions
- executor-neutral `swallowtail-runtime` with separate dynamic driver roles
- scoped run, session, turn, attached-serving, and owned-serving handles
- bounded ordered event channels, first-wins terminal outcomes, and scoped
  idempotent cancellation
- capability-scoped host-service ports, opaque portable inputs, redacted secret
  leases, and recording test fixtures
- typed preparation stages and safe causal failures plus observed or
  caller-asserted access provenance
- composable conformance runners for one-shot CLI, long-lived RPC, hosted API,
  attached self-hosted, and owned self-hosted integration shapes
- inspectable reports covering all 14 common Contract 011 assertions plus
  shape-specific process, session, and serving-lifecycle boundaries
- `swallowtail-host-local` with host-approved executable, environment, and
  working-resource resolution
- per-host joined scoped tasks, inspectable exact local service composition,
  and explicit executable approval returning one opaque discovery target
- Codex prepared discovery that binds one driver, host, opaque executable,
  exact version assessment, access provenance, and configured-instance base
  without selecting model or operation authority
- separate Codex prepared catalogue, read-only session, bounded-workspace
  session, and structured-exec values retaining exact evidence, inspectable
  immutable plans, and matching runtime requests
- deterministic local and remote-authoritative Codex prepared-facade
  conformance plus public getting-started, explicit-limit, diagnostic, and
  low-level escape-hatch guidance
- bounded redacted process I/O, graceful EOF stop, explicit force-stop, exit
  records, and joined child/output cleanup
- opaque default-redacted operation content and exact target/model preflight
  bindings for driver execution
- `swallowtail-adapter-codex` with text-only, read-only, ephemeral Codex exec
  request translation, JSONL normalization, cancellation, and joined cleanup
- Codex app-server model catalog and interactive-session roles with JSONL-RPC
  correlation, opaque provider references, streamed turns, native turn
  interruption, explicit callback rejection, and joined process cleanup
- dual-driver conformance proving separate Codex exec/app-server registrations,
  plan bindings, capabilities, transports, lifecycle profiles, and unsupported
  input boundaries
- current Soundcheck and Nucleus connector-seam evidence plus separate g01
  structured-run and interactive-session adoption-readiness roadmaps
- explicit structured-run network, search, and reasoning policy plus mutable
  model-catalog reasoning evidence
- scoped working-resource, attachment-file, and schema-file lease contracts
  with redacted materialized references and distinct cleanup authority
- deadline observations and deterministic preflight/host fixtures for the
  expanded structured-input boundary
- bounded local attachment/schema materialization, operation-scoped temporary
  working resources, explicit lease release, and cancellable monotonic deadline
  waits
- capability- and preflight-bound Codex exec image, JSON Schema, reasoning,
  external-search, and deadline support using only host-materialized paths
- distinct timeout and operator-cancellation outcomes with joined process and
  materialization cleanup across every terminal path
- Codex app-server model metadata carrying supported reasoning modes and the
  provider-reported default
- Codex catalog metadata for safe model descriptions and provider-default
  markers
- controlled non-Git exec invocation with ambient rule/config isolation,
  non-interactive approval policy, and no tool subprocess environment
  inheritance
- generic public-API structured-run parity coverage and a bounded Soundcheck
  dependency, feature-gate, validation, rollback, and legacy-removal handoff
- preflight-bound Codex app-server developer instructions, reasoning effort,
  and bounded dynamic-tool declarations using the current provider schema
- correlated dynamic-tool callback exchange with bounded queues, exactly-once
  responses, cancellation and deadline abandonment, safe provider rejection,
  and no Swallowtail-owned tool execution
- execution-host identity on runtime service sets, with pre-provider rejection
  when services do not match the immutable preflight host
- session resume bindings across provider session, configured instance,
  execution host, model route, and model identities
- local and remote-authoritative Codex app-server proofs for open, resume,
  callbacks, interruption, active-session close, disconnect, and joined cleanup
- bounded Nucleus live-session adoption handoff covering the existing registry
  seam, embedded-host rollout, callback bridge, safe fresh-session migration,
  dependency gate, consumer validation, rollback, and legacy removal
