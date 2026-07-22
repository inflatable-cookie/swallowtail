# 008 Runtime Registration and Preflight

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Define the safe records and preflight boundary between consumer requirements,
registered driver roles, configured instances, model routes, access state, and
execution-host services.

## Public Record Boundary

`swallowtail-core` may add pure provider-neutral records for:

- integration-family and transport-family identity
- configured-instance and execution-host identity
- execution layer and operation shape
- instance ownership mode
- model-route identity separate from model display metadata
- safe access profile and dimensional access status
- exact safe interface-version bindings and qualification-claim identity
- required host-service kinds
- parameterized capability requirements and observed constraints
- driver descriptors and safe discovery outcomes
- operation requirements and preflight failures

These records remain free of async runtimes, transport clients, provider wire
types, secrets, executable paths, filesystem paths, and consumer policy.

Existing `AdapterIdentity` is the stable adapter-driver identity unless an
implementation card proves a rename materially improves the public boundary.
Pre-1.0 compatibility shims are not required.

## Driver Registration

A runtime registration contains one safe driver descriptor and one or more
role implementations. Roles are explicit:

- discovery
- model catalog
- structured run
- interactive session
- serving-instance lifecycle
- optional provider extension roles

Absence of a role means unsupported. A single catch-all trait with stub
methods returning unsupported is not the registration model.

Model catalog is distinct from discovery. Discovery reports possible driver
instances without authenticating or configuring them. Catalog reads models
from one already configured and preflight-approved instance. Catalog results
carry stable model identity and mutable safe metadata; pagination remains a
driver concern unless the public request explicitly exposes it.

Reasoning catalog metadata contains driver-owned mode names, the observed set
supported by that model, and an optional observed default. This is mutable
selection evidence only. It does not populate an operation request, alter a
route, or authorize fallback. Consumers select an exact `ReasoningMode` when
they require one.

The descriptor declares:

- driver identity and version
- integration and transport families
- supported execution layers and operation shapes
- required host-service kinds per role
- supported discovery and sign-in actions
- declared extension namespaces

Registration does not select a default instance, model, credential, route, or
fallback.

## Configured Instance

A configured instance safely identifies:

- stable instance id
- registered driver id
- execution-host id
- endpoint, executable, SDK, or service reference owned by the host
- external-attached, host-owned-ephemeral, or host-owned-persistent ownership
- access-profile references and support authority
- enabled protocol facade and instance policy

Provider secrets and raw host paths are never part of the public instance
record. Host-owned references remain opaque outside the host service that
resolves them.

A discovered candidate is not a configured instance. Discovery may report
absent, discovered, incompatible, or failed without authenticating, mutating
configuration, starting persistent services, or choosing defaults.

## Access State

Access status is a record with independent dimensions:

- credential: not required, unknown, required, ready, expired, or rejected
- entitlement: unknown, available, unavailable, exhausted, or restricted
- endpoint authorization: unknown, allowed, or denied
- runtime readiness: unknown, ready, degraded, or unavailable
- support authority from Contract 006

Provider-specific detail remains a namespaced extension. No aggregate
`authenticated` or `ready` boolean replaces dimensional state.

## Capability Requirements

Capability support has two layers:

1. a stable named capability such as structured run, resume, attachments, or
   interruption
2. optional constraints such as cancellation scope, accepted media type,
   maximum size or count, schema dialect, event granularity, context limit,
   concurrency, tool mode, or reasoning mode

The first implementation adds only constraints required by the first
conformance fixtures. Unknown constraints are preserved or rejected under
extension policy; they are not treated as satisfied.

Reasoning selection is the named `ReasoningSelection` capability with the
selected `ReasoningMode` as a constraint. Provider-side external search is the
separate `ExternalSearch` capability. Transport networking, provider-side
external-network permission, and external search are distinct; satisfying one
never implies either of the others.

## Preflight

Preflight receives explicit:

- configured instance
- model route when the operation requires one
- execution layer and operation shape
- required capabilities and constraints
- acceptable access-profile constraints
- required extension namespaces
- available host-service kinds

It checks driver role, instance, route, access, support authority, ownership,
topology, host services, capability constraints, and extension policy before
provider side effects.

An operation using a selected reasoning mode must require that exact mode. An
operation enabling external search must require `ExternalSearch`, explicit
host-approved provider-side network access, and the host services declared by
the driver for that access. Request policy and the immutable plan must agree;
a driver rejects a mismatch before process or provider work.

Failure is structured and identifies the unsatisfied dimension without
exposing secrets or internal paths. Preflight never spawns a process, opens a
network request, loads a model, uploads an attachment, starts sign-in, mutates
configuration, or invokes a provider SDK.

Successful preflight produces an immutable execution plan bound to the exact
driver registration, instance revision, model route, requirements, access
profile, ownership, and execution host. Execution rejects a stale plan when a
material bound revision changes.

When an operation requires an executable, package, SDK, wire, service, schema,
or facade version, preflight binds the exact configured-instance point and
checks the selected driver's qualified support under Contract 029. A range is
never substituted for the exact point reached by execution.

The runtime host-service set identifies the execution host that owns its
services. A driver rejects a service set whose host id differs from the
preflight-bound execution host before invoking a task, process, resource,
credential, network, or time service.

The selected driver may read the exact bound instance target reference and
model identity needed to execute that plan. The target remains opaque and
redacted. Reading a binding does not grant permission to replace it, resolve
it outside the relevant host service, or choose another route.

## Routing Boundary

Swallowtail validates one explicitly selected route. Consumers may compose
several preflight attempts into routing policy, but Swallowtail does not choose
among routes or silently cross execution, access, support, billing, privacy,
ownership, or topology boundaries.

## Acceptance

- missing role fails before provider work
- missing host service fails before provider work
- unsupported parameterized capability fails explicitly
- discovered but unconfigured candidates cannot execute
- stale execution plans cannot execute
- instance, model route, access profile, ownership, and execution host remain
  observable in safe results
- execution uses the exact target reference and model identity bound by
  preflight
- execution services identify and match the preflight-bound host
- no public record contains secret material or raw host paths
