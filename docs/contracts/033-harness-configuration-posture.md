# 033 Harness Configuration Posture

Status: active
Owner: Tom
Updated: 2026-07-23

## Purpose

Bind the configuration authority visible to one harness operation. Keep that
authority separate from process isolation, credentials, retained provider
state, working resources, and consumer policy.

## Postures

`HarnessConfigurationPosture` is operation-shape neutral across harness
structured runs and interactive sessions:

- `Ambient` — the consumer explicitly accepts the harness configuration sources
  ordinarily visible under the selected host invocation
- `ProviderSuppressed` — the selected driver and exact interface-version
  segment suppress provider configuration sources through a qualified
  invocation
- `HostScoped` — the harness sees only configuration supplied through a
  separately bound host lease

No posture grants configuration-file discovery, parsing, mutation, migration,
installation, or deletion authority. `Ambient` does not prove that any
configuration source exists. `ProviderSuppressed` does not claim suppression
of credentials or provider runtime state. `HostScoped` does not authorize an
adapter to create a temporary home or copy ambient files.

## Exact Binding

A migrated harness route binds one posture in its configured instance and
requires the same posture for each operation. The immutable preflight plan
retains that binding. The runtime request policy must agree exactly before any
provider or host effect.

An absent posture means the route has not migrated to this contract. It is not
an alias for `Ambient`. New and migrated harness routes bind a posture
explicitly. Direct-model inference cannot declare one.

There is no fallback between postures. Failure to realize
`ProviderSuppressed` or `HostScoped` rejects the operation; it never retries
with ambient configuration.

## Provider-Suppressed Evidence

`ProviderSuppressed` is valid only when the selected driver has qualified the
exact interface-version point or behavior segment and exact invocation that
suppresses provider configuration. A command name, installation method,
platform, clean test machine, or lack of observed customization is
insufficient.

The configured-instance posture records the qualified result. Provider-specific
flags, environment handling, and configuration-source vocabulary remain
adapter-private. Version dispatch remains governed by Contract 029.

## Host-Scoped Lease Gate

`HostScoped` requires a separate opaque, operation-scoped host configuration
lease and a capability-scoped host service. The lease must bind the execution
host and operation scope, expose no raw host path or secret, and define joined
cleanup independently from credential and working-resource leases.

The current runtime has no such lease or host service. Pure preflight therefore
rejects `HostScoped`, even when instance and requirement records agree. A later
contract may activate the posture by defining that port and its lifecycle.

## Independent Boundaries

Configuration posture does not imply or satisfy:

- `AmbientHost`, provider-enforced, or host-enforced process isolation
- delegated, API-key, OAuth, or other credential authority
- temporary or durable provider transcript retention
- provider-managed recovery or resume
- filesystem working-resource access
- network, search, approval, or tool policy

Those records remain independently bound and validated. In particular,
provider-suppressed configuration may still use an ambient credential store or
durable provider state when separate contracts permit them.

## Conformance

Provider-neutral fixtures prove:

- exact ambient and provider-suppressed agreement for structured-run and
  long-lived harness operation shapes
- configured-instance and operation-requirement mismatch rejects before effects
- direct inference rejects a harness configuration posture
- host-scoped configuration rejects until its lease boundary exists
- request policy cannot omit or change the preflight-bound posture
- changing configuration posture makes an immutable plan stale
- configuration, isolation, credential, and retention records remain
  independently selectable

Provider adapter fixtures additionally prove the exact invocation and
interface-version segment behind every `ProviderSuppressed` claim.

## Exclusions

This contract does not standardize provider configuration schemas, file names,
environment variables, discovery order, credential stores, transcript paths,
temporary homes, containers, or consumer routing policy. It grants no
workspace-write, provider fallback, or secret-copy authority.
