# 018 Owned Ephemeral Model Serving Lifecycle

Status: active
Owner: Tom
Updated: 2026-07-20

## Purpose

Define the narrow host authority, artifact lease, readiness binding, and joined
cleanup required when a serving driver launches an ephemeral model server.

## Scope

This contract governs `HostOwnedEphemeral` serving instances. Persistent
servers, fleet scheduling, model acquisition, artifact mutation, router-mode
catalogues, and adjacent serving systems remain outside this proof.

Contract 007's identities remain separate:

- model artifact
- serving driver
- configured deployment
- protocol facade
- model route
- serving instance
- execution host

An executable approval does not approve an artifact. An artifact approval does
not approve an endpoint, model route, or serving process.

## Model Artifact References And Leases

Portable configuration uses an opaque model-artifact reference. Safe metadata
may identify format, revision, quantization, expected digest, provenance, and
license posture. It never carries a raw host path.

The execution host may resolve that reference into a serving-scope, read-only
artifact lease. The lease is bound to the exact scope and execution host and
exposes its materialized value only through a redacted driver accessor.

Artifact leases are distinct from operation attachments:

- they identify model-serving material, not request content
- their lifetime covers serving start, use, and stop
- consumer-owned artifact material is never deleted by lease release
- Swallowtail does not download, convert, license, relocate, select, evict, or
  mutate the artifact

Release is explicit and awaited after the owned process is joined. Cross-scope
or cross-host use fails before process start.

## Launch Authority

Preflight binds the serving role, `HostOwnedEphemeral` ownership, exact driver,
configured instance revision, model route, execution host, artifact reference,
and required host services before effects.

The host separately approves the executable, arguments, environment,
filesystem representation, resource posture, and endpoint audience. A driver
cannot turn a portable string into an executable, artifact path, or bind
address.

The first llama.cpp proof uses the runtime's own race-free ephemeral bind and
accepts only the reported loopback address. This does not create a general
ambient port-allocation or public-listener authority.

Missing artifact, process, time, task, blocking-work, or network authority
fails before launch. A configured external endpoint cannot be silently reused
as owned-launch authority.

## Startup Observation And Readiness

Process output remains bounded host I/O. A driver may normalize a documented
startup record into a safe endpoint binding. It must reject malformed,
duplicate, non-loopback, unexpected-scheme, or out-of-scope bindings. Raw
stdout, stderr, paths, arguments, and pids remain internal.

Start carries one absolute host-monotonic deadline. Success requires:

1. process creation under owned-child authority
2. one accepted endpoint binding
3. provider-specific health readiness
4. exact serving-build observation
5. exact configured model-route observation

Provider-specific probes stay in the adapter. The common handle records that
readiness completed and exposes only the safe endpoint binding needed by later
catalogue or inference operations on the same execution host.

Start returns no usable handle before readiness. It performs no automatic
retry, artifact fallback, executable fallback, endpoint widening, model
selection, or route substitution.

## Owned Handle And Handoff

An owned serving handle carries:

- serving-instance identity
- `HostOwnedEphemeral` ownership
- exact execution-host identity
- safe endpoint reference and audience
- child-process ownership
- artifact-lease ownership

The endpoint reference is host-issued and remains usable only while the handle
is alive. Consumers may use it to build an exact configured attachment or
inference binding. The handle does not itself become a lowest-common-
denominator inference API.

Attached and owned handles remain different types. An owned handle may stop its
child; an attached handle may only release its attachment.

## Failure And Cleanup Ordering

After process creation, every startup failure follows one joined cleanup path:

1. request graceful child stop
2. escalate only after the bounded grace period and only under owned-child
   authority
3. wait for process exit and join reader or waiter tasks
4. invalidate and release the endpoint binding
5. release the artifact lease

Normal stop uses the same ordering. Cleanup is explicit, awaited, and reports
clean, degraded, or failed state. Provider failure never hides cleanup failure.

The endpoint and artifact cannot outlive an unjoined child. Cleanup cannot stop
an external service or delete consumer-owned artifacts. There is no detached
task or global serving registry.

## llama.cpp b10069 Proof Boundary

The first production proof is one exact `b10069` single-model server using an
operator-supplied GGUF artifact, loopback port zero, offline mode, explicit
alias, disabled UI, and disabled agent tools.

It excludes:

- downloads and repository model references
- router mode and dynamic model load or unload
- sleep or warm-server lifecycle
- mutable properties
- Web UI, MCP proxy, built-in tools, and media directories
- public binds, TLS termination, and server API-key management
- persistent ownership and automatic restart

These exclusions are capability and preflight facts, not undocumented driver
conventions.

## Conformance

The owned-self-hosted profile must prove:

- an artifact lease is distinct from an attachment lease
- scope, execution host, artifact, route, and ownership mismatches fail before
  process start
- no handle is returned before readiness
- malformed, duplicate, or non-loopback startup endpoints fail safely
- early exit, readiness timeout, build mismatch, and route mismatch stop and
  join the child before lease release
- success exposes a redacted host-scoped endpoint binding
- stop is authorized only for the owned child
- endpoint release precedes artifact release and both follow process join
- consumer-owned artifact material is not deleted
- diagnostics expose no artifact path, pid, raw process output, or request
  content

Live model tests remain separately gated. Deterministic fixtures lead.

## Failure Codes

Stable failures should distinguish at least:

- artifact unavailable or incompatible
- artifact authority mismatch
- serving launch rejected
- serving process exited before readiness
- serving endpoint invalid
- serving readiness timed out
- serving build mismatch
- serving route mismatch
- serving cleanup degraded or failed

Safe diagnostics retain driver, instance, route, ownership, and execution-host
identity without exposing host material.

## Acceptance

- owned start has explicit artifact, deadline, endpoint, and cleanup semantics
- no attachment API stands in for model-artifact authority
- no dynamically observed endpoint crosses execution-host or serving scope
- provider-specific readiness does not leak into the provider-neutral handle
- Monkey and external serving lifecycles remain outside owned llama.cpp
  authority
