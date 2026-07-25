# 005 Prepared Consumer Integration

Status: promoted and archived
Owner: Tom
Updated: 2026-07-24

## Purpose

Give applications a small, hard-to-misconfigure Swallowtail integration path
without weakening exact version, access, lifecycle, capability, or topology
contracts.

## Problem

The public crates expose the complete low-level contract kernel but require
ordinary consumers to assemble adapter descriptors, configured instances,
access state, requirements, version bindings, configuration posture, host
services, preflight plans, and matching runtime requests.

That construction is repetitive and permits plans that compile but must fail
at runtime. Nucleus and Soundcheck independently reproduce the same Codex
setup logic.

## Target Operating Model

Swallowtail exposes two layers:

1. low-level provider-neutral records and role traits for advanced consumers,
   custom hosts, remote topology, and conformance
2. provider-specific prepared integrations that bind adapter-owned facts and
   derive plan-echoed request state while leaving consumer intent explicit

A prepared integration:

- accepts one host-approved target and service set
- observes and classifies the exact interface version when required
- accepts observed access state or an explicit caller assertion
- constructs one internally consistent configured instance, requirements set,
  route, and immutable preflight plan
- derives access, provider-state, and configuration agreement wherever the
  runtime request merely repeats the plan
- exposes the expanded safe plan and compatibility assessment
- owns no product prompt, tool execution, workflow, persistence, or routing
  preference

## Selected Shape

- Keep the current 23-package graph. Do not add an umbrella or convenience
  crate in the first tranche.
- Add shared request-preparation primitives to `swallowtail-runtime`.
- Add joined local task and host-service composition to
  `swallowtail-host-local`; no global executor or detached task.
- Add Codex-specific prepared exec and app-server surfaces to
  `swallowtail-adapter-codex`.
- Keep exec structured runs, app-server catalogues, and app-server interactive
  sessions as separate operation paths.
- Make read-only and bounded-workspace profiles named and inspectable.
  Writable, network, search, and tool authority remain opt-in inputs.
- Remove implicit plan-echo defaults from the unreleased pre-1.0 request API.
  Use plan-derived or explicit construction instead of a compatibility shim.
- Keep target search outside installed-executable discovery. A local host may
  offer an explicitly selected resolution policy, then approve one exact
  opaque target for discovery and execution.
- Preserve safe diagnostics while adding typed failure stages and one primary
  error chain.

## Consumer Input

Consumers still supply:

- adapter route: Codex exec or app-server
- stable configured-instance identity and revision
- authoritative execution host and exact target-selection policy
- access profile and observed or explicitly asserted access state
- model and working resource where required
- reasoning selection
- product tool declarations and callback execution
- external network and search policy
- read-only or bounded-workspace authority
- prompts, schemas, attachments, deadlines, persistence, and UI

## Swallowtail-Owned Preparation

The prepared Codex layer owns:

- exact version probe and compatibility classification
- selected private behavior revision
- facade, ownership, support, and driver identity
- adapter-required ambient configuration binding
- capability and host-service requirements implied by the selected profile
- consistent instance, requirements, route, plan, and runtime request policy
- safe staged preparation diagnostics
- joined failure cleanup

## Access Evidence

Preparation cannot convert “configured” into “ready” silently.

The first implementation accepts either:

- a safe `AccessStatus` obtained through an existing host or consumer
  observation, or
- an explicit caller assertion whose provenance is visible in the prepared
  result

If current contracts cannot represent this distinction honestly, roadmap
g02.002 must promote the missing record before facade implementation.

No prepared profile discovers credentials, signs in, changes account, selects
billing, or treats a failed provider request as permission to try another
access mechanism.

## Diagnostics

Preparation failures retain one safe primary code plus a typed stage:

- target selection or resolution
- process spawn
- bounded output
- process exit
- version parse
- compatibility classification
- access evidence
- preflight
- cleanup

Raw paths, environment, stdout, stderr, credentials, account identity, and
provider payloads remain excluded from stable formatting.

## Consumer Migration

After the facade passes deterministic conformance:

1. Nucleus migrates catalogue, read-only Agent Chat, bounded task execution,
   and smoke paths.
2. Nucleus deletes duplicated host, discovery, preflight, and policy-copy
   helpers while retaining product tools, authority, receipts, persistence,
   and UI.
3. Soundcheck migrates catalogue and structured exec.
4. Soundcheck deletes duplicated host and preflight construction while
   retaining prompts, schemas, evidence, validation, review, and application.
5. Both consumers run deterministic runtime preparation tests and separately
   gated live authentication checks.

Consumer edits, branches, commits, and releases remain owned by their
repositories.

## Release Consequence

The current unpublished `0.1.0` candidate remains held. After both migrations:

- candidate validation must execute deterministic runtime preparation, not
  only `cargo check`
- package and API evidence is regenerated
- consumer handoffs describe the facade path and rollback
- a replacement `0.1.0` candidate is frozen because no external release
  exists

Publication, registry, tag, push, workflow, and release mutations remain
separately operator-gated.

## Non-Goals

- generic `send_prompt`
- one lowest-common-denominator provider facade
- hidden provider, model, credential, endpoint, billing, or topology fallback
- automatic sign-in or credential discovery
- automatic writable, network, search, or tool authority
- consumer prompt, workflow, persistence, retry, or UI ownership
- removal of low-level records and role traits
- new container or sandbox prerequisite

## Validation Strategy

- provider-neutral plan-derived request assertions
- deterministic local and remote-authoritative host fixtures
- staged discovery and cleanup failure matrix
- Codex exec, catalogue, read-only session, bounded-workspace, tools, version
  range, and unverified-newer conformance
- compile-tested public examples
- isolated Nucleus and Soundcheck deterministic runtime smokes
- separately gated installed and authenticated probes
- full package, MSRV, API, docs, and repository QA before candidate freeze

## Stop Conditions

- simplification requires hidden authority or fallback
- access readiness cannot be represented without fabrication
- the prepared facade merges exec and app-server lifecycle
- local convenience compromises remote-authoritative topology
- a consumer-owned concept enters a Swallowtail public type
- consumer migration needs policy changes beyond replacing integration
  mechanics
- release mutation becomes necessary before the replacement candidate gate

## Acceptance Criteria

- normal Codex integration does not manually construct adapter-fixed preflight
  records
- plan and runtime request cannot drift through an implicit default
- exact version and compatibility status remain visible
- access provenance remains honest
- low-level capabilities remain available
- Nucleus and Soundcheck delete duplicated integration glue
- deterministic runtime consumer proof closes the compile-only gap
- the replacement candidate retains every existing safety and authority rule

## Promotion Targets

- system architecture: prepared integration layer and dependency direction
- new Contract 037: preparation, plan derivation, access evidence, diagnostics,
  and low-level escape hatch
- Contracts 008-010, 032-033 only where existing wording needs a narrow
  consistency update
- g02 roadmaps 002-006
- release candidate and consumer handoffs after implementation

## Promotion Record

Promoted on 2026-07-24:

- Contract 037 fixes the prepared integration boundary.
- System architecture records the contracted dependency direction and current
  realization gap.
- Contracts 008-010, 032-033, and 036 carry narrow consistency rules.
- Roadmaps g02.002-g02.006 retain implementation, consumer, and release work.
