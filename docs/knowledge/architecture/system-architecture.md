# System Architecture

Status: active
Owner: Tom
Updated: 2026-11-21

This file owns the system shape: which crate families exist, what each family
owns, and which way dependencies run. It names no route, version window,
capability cell, or validation recipe; those have other owners.

| Fact | Owner |
| --- | --- |
| Production routes, version windows, capability cells | [Provider Route Matrix](../../guides/provider-route-matrix.md) and its [solution feature CSV](../../guides/provider-solution-feature-matrix.csv) |
| Durable behaviour rules | [Contracts](../contracts/README.md), indexed in [contract-index.md](../contracts/contract-index.md) |
| Public package roster, version shape, MSRV, release authority | [Release And Package Topology](release-and-package-topology.md) |
| Consumer and repository boundaries | [Repository Authority Map](repository-authority-map.md), [Contract 002](../contracts/002-repository-authority.md) |
| Per-route consumer guidance and compiling examples | [Integration Guide Map](../../guides/integration-guide-map.md) |
| Route, feature, and guide checking | `effigy qa:routes`, `effigy qa:guides` |

## Crate Families And Dependency Direction

The workspace is thirty-nine current-source packages: one contract kernel, one
runtime, three support crates, three protocol and transport crates, and
thirty-one provider adapters. Every adapter carries at least one production
route; no package exists only as scaffolding. The root manifests own the exact
roster, version shape, and dependency topology, and
[Release And Package Topology](release-and-package-topology.md) records them.

```text
consumer applications
   |              |
host crates   provider adapter crates
   |              |
   +-----> swallowtail-runtime ----> swallowtail-idioms
                  |
          swallowtail-core

provider adapters -> swallowtail-protocol-acp | swallowtail-protocol-openai-chat
HTTP/SSE adapters -> swallowtail-transport-http -> runtime
remote ACP operations -> swallowtail-transport-acp-remote -> core/runtime/protocol-acp
consumer and adapter tests -> swallowtail-testkit -> core/runtime contracts
```

- **Contract kernel.** `swallowtail-core` holds the provider-neutral record
  vocabulary and nothing provider-specific: identity, access, configured
  instances, requirements, lifecycle, streaming, inference, customization,
  extensions, retention and deletion strengths, management capabilities,
  isolation and harness-configuration posture, and exact interface evidence.
  Its only dependency is `semver`.
- **Runtime.** `swallowtail-runtime` depends on core, `swallowtail-idioms`,
  `futures-core`, `zeroize`, and `sha2`. It owns executor-neutral dynamic
  roles, scoped handles, bounded events, terminal outcomes, usage and quota
  observations, operation policy, plan-derived request and safe evidence
  records, scope- and audience-bound host ports, and the shared prepared
  evidence record. It never names a provider.
- **Idioms.** `swallowtail-idioms` depends on core only. It owns portable
  idiom records, deterministic confidence decay, merge outcomes, bounded
  selection, and a fail-soft recorder behind host ports. It composes no prompt
  and carries no transport.
- **Host services.** `swallowtail-host-local` composes the exact supported
  ports under one host identity: bounded piped I/O, supervised process
  completion, joined reader cleanup, scoped deadlines, approvals, endpoint and
  credential leases, bounded attachments and schemas, operation-scoped working
  resources, scoped task reaping, the local watcher service, and the
  operation bridge listener. Local service composition belongs here; no
  adapter owns process or host policy.
- **Protocol codecs.** `swallowtail-protocol-acp` and
  `swallowtail-protocol-openai-chat` are bounded wire codecs over `serde_json`
  alone. They classify framing, requests, responses, notifications, events,
  and structured unknowns. They hold no runtime projection and no provider
  policy.
- **Remote ACP transport.** `swallowtail-transport-acp-remote` owns explicit
  HTTP/SSE or WebSocket selection, connection-private affinity, bounded frames
  and streams, correlation, initialize-version validation, cancellation,
  deadlines, and host-owned joined cleanup.
- **Bounded HTTP transport.** `swallowtail-transport-http` owns the one
  provider-neutral `curl` exchange shared by every HTTP/SSE adapter: endpoint
  resolution, bounded responses and SSE framing, blocking-worker bridging,
  backpressure that never abandons a live stream, connect and finite-transfer
  timeouts, cancellation on close and drop, and zeroized secret headers.
- **Test support.** `swallowtail-testkit` depends on core, runtime, and idioms.
  It provides contract-kernel, preflight, and callback fixtures, recording
  host services, and composable provider-free conformance profiles. Tests
  depend on it; production code does not.
- **Provider adapters.** Thirty-one adapter crates, one per provider surface.
  Each owns its provider's raw envelopes, exact version mapping, prepared
  surfaces, driver role implementations, and route claims. Adapters remain
  opt-in: selecting one installs no harness, grants no credential, selects no
  model, and widens no route.

## Cross-Cutting Roles

Roles are separate, typed, and named by capability. A route that implements one
does not gain another.

| Role | Owning crates | Contract |
| --- | --- | --- |
| Portable vocabulary and interface identity | core | [003](../contracts/003-portable-contract-kernel.md), [005](../contracts/005-integration-identity-and-transport-diversity.md) |
| Operation lifecycle, host authority, conformance | runtime, host-local, testkit | [009](../contracts/009-async-operation-lifecycle.md), [010](../contracts/010-execution-host-services-and-inputs.md), [011](../contracts/011-runtime-conformance-profiles.md) |
| Prepared integration and typed bound operations | adapters (adapter-local), runtime (shared evidence), testkit (assertions) | [037](../contracts/037-prepared-consumer-integration.md) |
| Configured-instance catalogue, admission, readiness, sign-in | core, runtime, host-local store adapters | [047](../contracts/047-configured-provider-instance-catalogue.md), [057](../contracts/057-route-readiness-and-connection-admission.md) |
| Consumer route, feature, and control projection | core, runtime | [061](../contracts/061-consumer-route-feature-and-control-projection.md) |
| Observable activity and disclosure | core, runtime, adapters | [044](../contracts/044-observable-agent-activity-and-disclosure.md) |
| Subagent topology and attribution | runtime, adapters | [045](../contracts/045-subagent-topology-observation-and-control.md) |
| Provider session management | core, runtime, adapters, testkit | [038](../contracts/038-provider-session-management-and-consumer-thread-boundary.md) |
| Paged provider session history | runtime, adapters | [054](../contracts/054-paged-provider-session-history.md) |
| Provider session catalogue and explicit import | runtime, adapters | [046](../contracts/046-provider-session-catalogue-and-explicit-import.md) |
| Interrupted work reconciliation and restoration | runtime, adapters | [048](../contracts/048-cross-process-active-operation-reconciliation.md), [050](../contracts/050-working-state-restoration-facade.md) |
| Controlled operation detachment | runtime, adapters | [049](../contracts/049-controlled-shutdown-active-operation-detachment.md) |
| Retention, recovery, and cleanup truth | core, runtime, adapters | [021](../contracts/021-provider-owned-background-run-and-temporary-retention-boundary.md), [025](../contracts/025-provider-owned-direct-conversation-and-deletion-boundary.md), [042](../contracts/042-harness-managed-recovery-and-active-turn-reattachment.md) |
| Portable failure classification | core, runtime | [051](../contracts/051-portable-failure-classification.md) |
| Opt-in debug observation | runtime, host-local sink, adapters | [053](../contracts/053-opt-in-debug-observation.md) |
| Learned idioms and route-path opt-in | idioms, runtime | [055](../contracts/055-pluggable-learned-idioms.md), [056](../contracts/056-route-path-idioms-opt-in.md) |
| Operation-scoped watchers and their bridge | runtime, host-local, adapters | [059](../contracts/059-operation-scoped-process-watchers.md), [060](../contracts/060-operation-scoped-watcher-http-bridge.md) |
| Skill visibility and bounded inventory | runtime, host-local, adapters | [058](../contracts/058-effective-harness-skill-visibility.md), [062](../contracts/062-bounded-skill-inventory.md) |

Which route implements which role is a route fact. The
[Provider Route Matrix](../../guides/provider-route-matrix.md) records it, and
`effigy qa:routes` checks that record against the adapters.

## Prepared Integration

Contract 037 fixes the application-facing path:

```text
consumer intent and explicit authority
  -> provider-adapter prepared profile
  -> expanded configured instance, requirements, access provenance, and plan
  -> typed bound operation for one existing runtime role
  -> existing low-level runtime role
  -> existing host services and provider driver
```

Preparation binds adapter-owned facts and derives only state that repeats an
immutable plan. It cannot choose provider, model, target, credential, endpoint,
billing, topology, writable access, network, search, tools, prompts, workflows,
or persistence. Access stays observed or visibly caller-asserted. Every
expanded profile is inspectable before effects, and every prepared value keeps
one typed path plus a low-level escape hatch. There is no generic prompt method
and no umbrella crate that constructs or selects a provider.

## Event And Activity Ownership

Two event vocabularies coexist by recorded disposition, not duplication:

- `core::EventEnvelope` and `EventKind` form the portable consumer-facing
  envelope: kind, content, and policy-governed provider extensions.
- `runtime::RuntimeEvent` and `RuntimeEventKind` form the runtime-internal
  sequenced event: typed payloads plus the delivery policy that drives
  buffering.

The kind concept overlaps. The envelope carries extension governance; the
runtime event carries typed evidence and delivery policy. Consolidation in
either direction loses one of those responsibilities.

Observable activity rides the existing run and turn streams. The portable layer
owns operation-local activity identity, lifecycle and disclosure fidelity,
bounded provider labels, typed content streams, correlation, and redacted
formatting. The portable durable key is the composite `ActivityKey` (one
consumer-unique run or turn owner plus one operation-local activity id), not a
provider message reference. Adapters keep raw provider envelopes and exact
version mapping. Consumers keep durable projection, transcription, grouping,
and presentation. Warning or error activity may carry the same safe portable
failure classification; display content never becomes diagnostic text.

## Provider Session Roles

Provider work splits into separate read-only or mutating roles with exact
authority each:

- **Load, resume, and replay.** Contract 017 bindings attach to one provider
  session. Load replays provider history; resume does not. Neither grants
  management authority.
- **Management.** Contract 038 archives, restores, and deletes one inactive
  bound session with explicit deletion strength and after-dispatch
  uncertainty. Inactivity is caller-asserted; no registry exists.
- **History paging.** Contract 054 reads newest-first pages with the same
  replay projection and opaque cursors. It grants no load readiness or
  reconciliation.
- **Catalogue and import.** Contract 046 observes bounded candidates and
  revalidates one explicit selection into an ordinary resume binding. A list
  result is never authority.
- **Reconciliation.** Contract 048 observes one interrupted turn or run and
  returns no handle, retry, resume, or control.
- **Detachment.** Contract 049 stops local observation without provider
  cancellation and reports local detachment truth only.
- **Cleanup.** Operation-owned cleanup deletes only its own private resource
  and records deletion truth separately from inference truth.

None of these roles implies another. A route qualifies each one by its own
evidence.

## Consumer Documentation Layer

The provider route matrix and its solution feature CSV are the canonical route
and feature inventory. The
[Integration Guide Map](../../guides/integration-guide-map.md) binds each route
and feature family to non-authoritative usage guidance and compiling examples.
Contract 052 requires that traceability and forbids guide coverage from
widening capabilities, version claims, access, lifecycle, cleanup, or recovery.
Deterministic examples compile without live credentials; live probes stay
separately operator-gated.

## Dependency Rules

- consumers depend toward Swallowtail; Swallowtail never depends on consumers
- core depends on no runtime, host, or provider adapter
- runtime depends on no provider adapter
- shared protocol, transport, idiom, host, and test crates depend on no
  provider adapter
- provider adapters depend on core, runtime, and at most the shared protocol
  and transport crates they use
- provider extensions remain namespaced and optional
- no umbrella crate selects a provider, model, target, or fallback
- UI frameworks and consumer persistence stay outside the crate graph
- execution happens on the host chosen by the consumer

## Architecture Promotion Rule

Realized sections name only implemented and validated structure. A separately
labelled contracted section may record an active contract and dependency
direction before implementation. It must state the realization gap and the
owning lane. Route, version, and capability facts belong in the route matrix,
never here.

## Contracted Registered-Tool Boundary — 2026-09-07

[Contract 063](../contracts/063-registered-tool-operation-bridge.md) and the
Contract 060 amendment define the next implementation seam; this section
records ownership, not realized generic MCP support. Swallowtail owns the
sole immutable registration snapshot and operation bridge kernel. Longhorn
supplies the production MCP server (Longhorn Contract 022 `agent-control`),
with Longhorn's earlier linked dispatch/validation counterpart withdrawn for
that role by Longhorn
[`7ea44d23`](https://github.com/inflatable-cookie/longhorn/commit/7ea44d23);
consumers own domain schemas, policy, durable admission ids, bounded context,
and app startup.

One internal kernel serves distinct watcher and registered-tool profiles:
[Contract 060](../contracts/060-operation-scoped-watcher-http-bridge.md) owns
the realized closed watcher profile, and
[Contract 063](../contracts/063-registered-tool-operation-bridge.md) owns the
contracted registered-tool profile. Neither replaces the other, and neither
becomes an independent second stack. The existing WatcherBridge stays closed
and source-compatible. No second listener, registry authority, daemon, or
product schema enters the producer runtime.
