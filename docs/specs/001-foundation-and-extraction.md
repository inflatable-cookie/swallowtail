# 001 Foundation and Extraction

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Shape Swallowtail as an independent shared runtime before moving live code from
Nucleus or Soundcheck.

## Initial Consumers

### Nucleus

Needs persistent interactive sessions, streaming events, tool exchange,
interruption, resume, model selection, and remote host placement. Nucleus keeps
its projects, tasks, goals, memory, authority, tools, and UI.

### Soundcheck

Needs bounded structured runs, progress, cancellation, schema-oriented output,
and reliable diagnostics. It should use an existing signed-in local harness
without additional setup, while allowing a configured direct model route for a
smaller inference-only path. Soundcheck keeps plugin discovery, tagging
prompts, taxonomy, review, routing policy, and persistence.

### Future Providers

CLI, API, SDK, local, and remote adapters must fit without making Codex
assumptions part of core vocabulary.

The target is broad integration coverage rather than one generic harness
wrapper. Initial inventory candidates include Codex, Claude Code, OpenCode,
Cursor, Pi, Kimi, xAI/Grok, and local model runtimes. Each supported exposure
may need a separate driver: a provider's CLI, app server, ACP endpoint, SDK,
and direct API are not interchangeable merely because they share a brand.

## Proposed Crates

- `swallowtail-core`
- `swallowtail-runtime`
- `swallowtail-testkit`
- provider and transport adapter crates, namespaced after the integration
  inventory settles package boundaries

The first implementation boundary permits only `core` and `testkit`. Contract
003 is translated into concrete types first, then reusable fixtures.

## Current State

The workspace and both first-boundary crates now exist. `swallowtail-core`
contains the Contract 003 record vocabulary with no external dependencies.
`swallowtail-testkit` depends only on core and exposes reusable fixtures and
assertions through the core public API. Runtime and adapter crates remain
proposals.

## Source Inventory

Nucleus source areas to inspect, not copy wholesale:

- live runtime records and lifecycle vocabulary
- provider identity, capability, route, and event records
- Codex registry, process supervision, and transport boundaries
- downstream chat tools and product policy as negative-boundary evidence

Soundcheck source areas to inspect:

- Codex CLI connection and cancellation
- structured output and progress behavior
- product tagging logic as negative-boundary evidence

## Migration Stages

1. establish standalone authority and first generation
2. implement and validate the portable contract kernel
3. inventory current harness, API, SDK, CLI, protocol, and local-runtime
   surfaces
4. define runtime traits and host ownership against several materially
   different adapter shapes
5. extract Codex structured-run and interactive-session mechanisms behind
   separate drivers
6. adopt from Soundcheck as the structured-run proof
7. adopt from Nucleus as the interactive-session proof
8. prove materially different harness, direct-API, and local-runtime drivers
9. expand integration coverage without stabilizing a lowest-common-denominator
   facade

## Settled Decisions

- project name: Swallowtail
- dedicated standalone repository
- Rust-first shared foundation
- mechanisms stay separate from application intent and durable state
- interactive sessions and structured runs remain distinct shapes
- first code boundary is provider-neutral records plus test fixtures
- execution hosts own binary, credential, filesystem, cancellation, cleanup,
  event-delivery, and redaction authority
- Swallowtail owns reusable runtime mechanisms; consumers own prompts, tools,
  product schemas, validation, repair, persistence, and consequences
- Swallowtail is a multi-adapter ecosystem, not a Codex-only shared connector
- one integration family may require multiple transport-specific drivers
- harness control and direct model inference remain distinct route categories
- execution layer, operation shape, transport, credential mechanism,
  entitlement, endpoint audience, and support authority are independent
- provider-supported, integration-maintained, experimental, and prohibited
  access routes remain distinguishable
- API keys may carry subscription entitlement; OAuth may authorize direct
  product-scoped inference
- consumers explicitly control fallback across execution, access, billing,
  privacy, and topology boundaries
- runtime execution is async-first without a Swallowtail-owned global executor
- every background task and resource belongs to a joined operation, session,
  discovery, or owned-instance scope
- drivers declare required host services; process, network, credential,
  resource, time, and event behavior is capability-scoped
- operations expose ordered bounded events and exactly one terminal outcome
- cancellation, deadlines, and cleanup preserve ownership and distinguish
  external from host-owned runtimes
- attachment references resolve on the execution host; raw client paths are
  not portable authority
- schema transport is shared mechanism; schema meaning and result validation
  remain downstream

## Open Decisions

- adapter crate grouping: per family, per transport, or a justified hybrid
- exact Rust trait syntax, object-safety, executor interoperability, and
  dependency choices
- concrete parameterized capability and access-status record shapes
- concrete attachment, schema, terminal-outcome, and cleanup records
- serialization guarantees for core records
- two-consumer conformance matrix and release cadence

## Acceptance Criteria

- Swallowtail can build and test without consumer repositories
- first records satisfy Contract 003 fixture tests
- no provider process or transport is introduced in the first code batch
- consumer concepts remain absent from core public types
- later adoption can replace duplicated connector mechanics incrementally

## Promotion Targets

- realized crate structure to `docs/architecture/system-architecture.md`
- new stable rules to `docs/contracts/`
- delivery sequencing to `docs/roadmaps/`
- extraction findings and decision evidence to `docs/logs/`
