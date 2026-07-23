# 011 Runtime Conformance Profiles

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Make Contracts 008-010 testable before real provider adapters define the
runtime by accident.

## Testkit Boundary

`swallowtail-testkit` may add deterministic fixtures and assertions for core
runtime records and later depend on `swallowtail-runtime` when that crate
exists.

Fixtures use synthetic drivers and host services. They do not require provider
accounts, network access, installed harnesses, model artifacts, consumer
repositories, or real secrets.

The testkit exposes reusable fixture profiles rather than one monolithic test
harness. An adapter selects the profiles and optional capabilities it claims.

## Required Profiles

### One-Shot Structured CLI

Proves process start, ordered framed events, result, cancellation, deadline,
exit, graceful cleanup, and authorized force-stop.

### Long-Lived RPC Or ACP Harness

Proves initialization, session and turn identity, callbacks, declared
steering, active-turn cancellation, close, resume constraints, and joined task
scope. The same public profile must run against distinct local and remote-
authoritative host identities without exposing paths or moving authority to a
client.

### Persistent ACP Harness Extension

Composes with the long-lived ACP baseline only for agents that advertise the
additional lifecycle. Proves separately bound load and resume, ordered bounded
replay before load readiness, no replay on resume, exact read-write callback
authority, delegated harness authentication, explicit ambient execution, and
joined process, resource, and credential cleanup. Baseline ACP agents do not
inherit these claims.

### Hosted Direct API

Proves scoped credential use, endpoint policy, streaming, cancellation limits,
terminal outcome, usage and limit evidence, and absence of process-service
requirements. Direct inference does not require a placeholder working
resource.

### Connection-Scoped Direct Session

Proves resource-free interactive direct inference, one connection-bound
endpoint and credential lease, serial turns, private continuation, exact
turn-scoped billed cost, cancellation or deadline connection invalidation,
absence of resume, local and remote-authoritative topology, and joined
connection cleanup before credential release.

### Locally Continued Direct Session

Proves resource-free interactive direct inference over separate provider
requests, explicit consumer authorization for every attempt, consumer-executed
tool exchange, bounded adapter-private continuation, provider-cache posture,
serial turns, no resume, local and remote-authoritative topology, and joined
network and private-state cleanup before credential release. Contract 030
governs the first production proof.

### Attached Self-Hosted Runtime

Proves route discovery, observed model capabilities, optional local
credential state, endpoint attachment, and that cleanup never stops the
external service.

### Owned Self-Hosted Runtime

Proves artifact authority, start, dynamically observed endpoint binding,
readiness, route availability, resource failure, stop authority, and cleanup.
Contract 018's first production proof covers ephemeral ownership. Persistent
ownership remains synthetic until separately contracted.

## Common Assertions

Every applicable profile asserts:

- missing role, capability, constraint, or host service fails before side
  effects
- instance, route, access profile, ownership, and execution host remain bound
- stale preflight plans fail before side effects
- event sequence is ordered and has one start boundary
- exactly one terminal outcome is produced
- non-coalescible event overflow fails rather than dropping data
- cancellation and timeout are distinct and cleanup is joined
- cleanup failure remains visible beside provider outcome
- external processes and services are never stopped
- secrets, raw paths, and internal diagnostics stay out of public output
- attachments and temporary resources remain host-authorized and scoped
- consumer-owned resources and operation-scoped temporary resources retain
  distinct cleanup authority
- schema and attachment file leases are usable by drivers but redact their
  materialized host values
- reasoning mode and external-search requirements reject when unsupported
- external search cannot imply provider-side network authority
- deadline observation, consumer cancellation, and timed-out terminal state
  remain distinguishable
- schema transport does not imply domain validation
- unknown extensions follow explicit preserve or reject policy
- no implicit fallback crosses execution, access, support, billing, privacy,
  ownership, or topology boundaries
- exact interface-version points are bound; a multi-version claim covers its
  boundaries, known breakpoints, and exclusions under Contract 029
- endpoint and credential grants cannot cross operation scope or audience
- credential release is awaited after connection cleanup
- cumulative usage does not become repeated-attempt usage
- rate, quota, billing, and retry evidence remain separate
- a provider error inside a successful stream cannot complete successfully
- each locally continued inference attempt requires explicit authorization
- provider-private continuation stays redacted, route-bound, ephemeral, and
  distinct from consumer transcript or provider cache

## Fixture API Direction

The initial implementation runway may add:

- canonical driver descriptors, instances, access states, routes, and
  requirements
- recording host services with zero-side-effect counters
- scripted event and terminal-outcome sources
- deterministic task, time, cancellation, and cleanup controls
- assertions for preflight, event order, terminal uniqueness, redaction,
  ownership, and fallback boundaries
- profile runners assembled from smaller assertions

Fixtures exercise public core/runtime APIs. They do not reach into private
adapter state or duplicate provider wire schemas.

## Proof Order

1. pure record and preflight fixtures
2. runtime role-trait and handle fixtures
3. synthetic one-shot, RPC, hosted, attached, and owned profiles
4. real adapters reusing the applicable profiles

No real provider driver is considered ready until its claimed profile and
capability assertions pass.

## Acceptance

- each runtime contract rule maps to at least one deterministic assertion
- profiles can be composed without provider-specific dependencies
- a deliberate violation names the failed contract dimension
- testkit remains usable by future consumer and adapter crates
