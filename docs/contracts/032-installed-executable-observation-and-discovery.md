# 032 Installed Executable Observation And Discovery

Status: active
Owner: Tom
Updated: 2026-07-23

## Purpose

Observe one exact host-approved harness executable before configuration or
preflight. Keep candidate selection, version evidence, compatibility
classification, configuration, and execution authority separate.

## Explicit Target

An installed-executable discovery request binds:

- one request and operation scope
- one authoritative execution-host id
- one opaque executable reference approved by that host
- one exact interface-version axis
- one host-monotonic deadline
- one operation-scoped cancellation signal

The executable reference is not a path. Drivers cannot search `PATH`, scan
install locations, inspect unrelated package managers, install, update,
downgrade, or substitute another executable. A renderer or remote client
cannot create authority by supplying a path-like value.

The selected driver owns one fixed, bounded, non-authenticating version
command and its parser. Provider-specific output syntax stays inside that
adapter. The probe uses no credential, sign-in flow, model request, provider
request, working resource, or consumer prompt.

## Exact Observation

A successful parse produces one safe installed-executable observation:

- authoritative execution-host id
- exact interface-version binding
- exact compatibility-claim id used for classification
- either the matching behavior revision and maintained or deprecated support
  status, or an explicit incompatible classification

The observation carries no executable reference, path, argument, environment,
stdout, stderr, token, manifest, credential, or provider payload. Exact
observations remain evidence only. A maintained window remains a separate
driver claim under Contract 029.

An observation cannot create a configured instance, revise an instance,
authorize execution, select a route, authenticate, or prove model access,
entitlement, catalogue freshness, or provider availability. Promotion remains
an explicit consumer or host configuration action outside discovery.

## Outcomes

Installed-executable discovery keeps these terminal states distinct:

- absent: the approved candidate is unavailable
- discovered: an exact compatible version was observed
- incompatible: an exact version was observed but the selected claim rejects
  it
- malformed: bounded output did not contain one valid exact version
- timed out: the host monotonic deadline won
- cancelled: the operation-scoped cancellation request won
- failed: process start, I/O, exit, parser, or other host work failed
- cleanup failed: stop or process join did not complete cleanly

Only discovered and incompatible outcomes carry an exact observation.
Diagnostics are normalized and redacted. Raw process output never becomes a
stable diagnostic.

## Lifecycle

The discovery call does not complete until its owned process has exited and
been joined. Success, incompatibility, malformed output, provider-independent
failure, cancellation, and timeout all close stdin where applicable, request
graceful stop, use force-stop only under host ownership, and await process
completion.

The caller may request cancellation through the request's shared
operation-scoped signal. Cancellation acknowledgement is distinct from the
terminal cancelled outcome. A deadline is selected only from a host
`DeadlineObservation`; inspecting a wall clock cannot produce a timeout.

Cleanup failure remains visible even if an exact version was parsed. No task
or process detaches. Dropping the discovery future is not successful cleanup;
the owning host task scope remains responsible for join.

## Topology

The request host id and `HostServices` id must match before effects. The
process service on that authoritative host resolves and executes the opaque
candidate. A remote-authoritative host performs discovery remotely and returns
only the safe observation.

Local and remote-authoritative discovery use the same records. A local
renderer must not probe its own filesystem or `PATH` on behalf of a remote
execution host.

## Existing Discovery

General provider, endpoint, or configured-instance discovery remains
available. Installed-executable observation is an additive target-aware
operation on the discovery role. Drivers that do not implement it reject it
explicitly without changing existing discovery behavior.

## Conformance

Deterministic default QA proves:

- only the exact opaque candidate reaches the process service
- request, scope, axis, deadline, cancellation, and host identity are retained
- exact compatible and incompatible versions remain separate from the claim
- absent, malformed, timeout, cancellation, failure, and cleanup failure stay
  machine-distinct
- process completion and host task cleanup are joined on every path
- local and remote-authoritative host identities do not substitute
- formatting and diagnostics expose no executable value or raw output
- no outcome creates a configured instance or authorizes execution

Live installed-binary probes remain separately gated.

## Acceptance

- installed discovery never performs ambient executable search or fallback
- exact observations and maintained windows remain independent
- remote-authoritative hosts execute their own probes
- cancellation and deadline paths retain joined cleanup
- provider-specific parsing stays in adapters
- public records expose no path, raw output, environment, token, or payload
- discovery grants no configuration, authentication, route, or execution
  authority
