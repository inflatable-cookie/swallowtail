# Runtime Kernel Validation And Promotion

Status: completed
Owner: Tom
Roadmap: 005 Async Runtime and Conformance
Updated: 2026-07-19

## Goal

Validate the runtime kernel as one package, promote realized architecture, and
ready the first real-driver generation.

## Scope

- full Contract 008-011 acceptance review
- public API and dependency-direction audit
- object-safety and executor-leak compile checks
- failure, cancellation, cleanup, redaction, and side-effect audit
- architecture and roadmap promotion
- roadmap 006 readiness assessment

## Out Of Scope

- real provider implementation
- consumer adoption
- API stability promise beyond pre-1.0 contracts

## Acceptance Criteria

- full QA passes
- every runtime contract has fixture evidence
- no provider, consumer, Tauri, Tokio, transport-client, or secret-store type
  leaks into the runtime public boundary
- architecture matches the realized crate graph
- roadmap 006 starts with explicit driver surfaces and conformance profiles

## Validation

- `effigy qa`
- targeted dependency and source scans
- `git diff --check`

## Closeout

- Contracts 008-011 have deterministic fixture evidence across preflight,
  lifecycle, host-service, input, and cross-shape conformance boundaries.
- The runtime remains executor-neutral. Its only external dependency is
  `futures-core`; core has no dependencies, and testkit depends inward on core
  and runtime.
- Public-boundary scans found no provider, consumer, UI framework, executor,
  transport-client, or credential-store coupling.
- Full QA, rustdoc warnings-as-errors, dependency inspection, source scans,
  and diff hygiene pass. Roadmap 006 can begin at the local process host.
