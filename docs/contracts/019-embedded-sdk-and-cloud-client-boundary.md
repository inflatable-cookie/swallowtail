# 019 Embedded SDK And Cloud Client Boundary

Status: active
Owner: Tom
Updated: 2026-09-02

## Purpose

Permit provider-maintained SDKs inside adapters without hiding process bridges,
ambient configuration, concrete executors, automatic retries, or cloud access
authority behind an `SDK` label.

## SDK Identity And Placement

An SDK-native driver links one maintained language-native package into the
adapter process. CLI wrappers, subprocess clients, FFI libraries, containers,
and language sidecars retain their real transport and lifecycle identities.
They are not SDK-native because a wrapper package starts them.

SDK package identity and version remain separate from:

- integration family and adapter driver
- provider service API and protocol version
- HTTP, TLS, EventStream, or other wire transport
- configured instance, endpoint, region, and execution host
- access profile, credential mechanism, and support authority
- model provider, route, artifact, and mutable catalogue evidence

A version pin proves the fixture and build boundary. It does not promise that
future versions preserve behavior.

## Foreign-Language SDK Sidecars

A maintained SDK in another language may run in a host-owned sidecar. That
route is not SDK-native: it retains a process boundary, a Swallowtail-owned
wire, a language runtime, and sidecar lifecycle alongside the upstream SDK.

The configured instance and preflight plan bind these identities separately:

- adapter driver and route
- source-tagged sidecar revision and launch recipe
- sidecar wire and behavior revision
- exact language runtime
- exact upstream SDK package and version
- downstream provider, model, credential, resource, and execution host

The application provisions the approved runtime, sidecar entry point, and SDK
dependency. Swallowtail does not search for, install, upgrade, repair, or
publish them during discovery or execution. An application-local source asset
does not become an independent package release merely because the launch
recipe loads an npm or other language package.

The sidecar wire is strict, versioned, bounded, and private to the driver. It
uses correlated commands, responses, events, and terminal failures; rejects
unknown semantics; and keeps SDK types, raw payloads, paths, and credentials
outside portable records. Transport acceptance does not imply provider
acceptance.

Sidecar construction suppresses ambient extensions, skills, prompts, context,
themes, settings, model aliases, update checks, catalogue refresh, retry, and
network discovery unless an active contract and exact host authority admit
them. Configuration is in-memory or supplied through explicit host-owned
references. The sidecar reports the effective resource binding before ready;
stored provider state cannot replace the host-leased binding.

Cancellation and close abort SDK work, dispose SDK/session state, drain the
bounded wire, join the sidecar process, then release provider state,
credentials, and host resources in contract order. Process isolation does not
by itself prove filesystem or network containment.

Where an upstream SDK launches further provider-owned processes, the nearest
sidecar is not the lifecycle boundary. The execution host owns and can
terminate the full descendant process tree rooted at that sidecar. The launch
recipe proves descendant enrollment or containment on every supported
platform. A platform where that cannot be proved is unsupported for the route,
not best-effort.

Close joins every provider process in the tree, not only the nearest child. A
bounded join states its bound and escalates through host termination authority
on expiry before joining again. Close reports one explicit outcome: exited
gracefully, exited after escalation, or exit unconfirmed. A surviving
descendant, discarded wait, or result that cannot distinguish exit from expiry
is cleanup failure. It is never evidence of clean close.

Root exit is not tree completion. A close outcome may report the descendant
tree as joined only where the execution host attests owned-tree emptiness from
a concrete Contract 010 observation. Root-only evidence closes the root and
leaves the tree unconfirmed, so a route that requires the full tree stays
unavailable until a host on that platform can make the observation.

## Runtime And Task Ownership

An adapter may privately depend on an SDK's async runtime, HTTP client, TLS
stack, generated types, and error types. None may leak into core, runtime,
testkit, or consumer-facing public records.

Every SDK operation runs inside the selected execution host's scoped task or
blocking-work authority. If the SDK requires a concrete executor, the adapter
may create an operation-scoped private executor inside that joined work. It
cannot install or depend on a global executor, detach background work, or let
an SDK refresh, retry, reader, timer, or connection task survive operation
close.

Cancellation and deadline stop local SDK work, close owned transport state,
join the scoped work, then release credentials. A provider request may have
reached the service even when local cancellation succeeds; drivers state that
limit honestly.

## Explicit SDK Configuration

Preflight fixes the configured instance, adapter driver, execution host,
endpoint reference and audience, access profile, credential reference, model
route, and model before SDK construction.

Every SDK configuration source used by an operation is explicit. A driver
cannot silently inherit:

- environment variables
- user or system configuration files
- default profiles
- instance, container, or workload metadata endpoints
- region or partition discovery
- endpoint or proxy discovery
- model aliases or fallback
- retry, adaptive rate, timeout, or background-refresh defaults

Adapter-specific safe configuration may bind a region, partition, service
signing name, and other SDK-required values. They must agree with the exact
host-approved endpoint and selected configured-instance revision. They cannot
be inferred from mutable catalogue data during execution.

## Delegated SDK Credentials

Contract 014 delegated authentication may authorize one exact SDK credential
provider. The provider object is configured with the adapter on the selected
execution host and is bound to the same configured instance, access profile,
credential reference, endpoint audience, and execution host as the preflight
plan.

The runtime credential service still grants operation-scoped use and returns a
delegated credential lease. The lease exposes no secret. The adapter may call
only the already-bound provider after validating the lease scope, reference,
audience, and host binding.

The provider cannot be discovered by scanning ambient SDK configuration. A
consumer or host may explicitly construct a provider backed by a named
profile, workload identity, helper, or static test credential, but that choice
is outside the driver and remains visible through the selected access profile
and support authority.

Credential caching or refresh belongs to the authorized provider. It does not
authorize another endpoint audience, region, service, configured instance, or
operation. Provider cleanup and SDK work finish before the delegated lease is
released.

## Attempts, Timeouts, And Streaming

Contract 014's one-attempt rule applies inside the SDK. A driver must override
an SDK default that performs retries, redirects, endpoint failover, adaptive
rate delay, replay, or hedging. One consumer operation cannot become several
provider inference attempts merely because the SDK considers an error
retryable.

The common operation deadline bounds client construction, credential
resolution, connection, response headers, and full stream consumption. An SDK
timeout that stops at response headers is insufficient for a streamed result.
The adapter retains the outer deadline and cancellation authority until the
terminal provider event or failure.

Typed SDK events are still provider events. The adapter must:

- preserve provider order through a bounded runtime stream
- fail closed on unknown semantic variants that could change output
- distinguish output, stop, usage, provider failure, transport failure, and
  local cancellation
- keep SDK error bodies, request objects, credentials, and raw payloads out of
  stable diagnostics
- join all SDK and projection work before reporting clean close

## Service And Catalogue Separation

A cloud SDK may publish separate clients for runtime inference, model
catalogue, account management, or other control planes. Each client retains
its own driver role, service API, endpoint audience, permissions, and
capabilities. Authorization for inference does not authorize catalogue access,
and catalogue presence does not prove model entitlement or invocation success.

A gateway may expose models from several underlying providers. The gateway
integration family, cloud service, underlying model provider, model route, and
model identity remain separate. A gateway model id cannot become an implicit
provider or consumer routing preference.

## First Bedrock Runtime Subset

The first proof binds:

- official `aws-sdk-bedrockruntime = 1.136.0`
- direct structured inference through `ConverseStream`
- one exact host-approved Bedrock Runtime endpoint and AWS region
- provider-supported cloud-provider identity with cloud-account billing
- one host-authorized delegated AWS credential provider
- one exact configured model route
- text input and output
- one explicit positive consumer-owned maximum-output-token bound
- one SDK attempt and bounded typed EventStream projection

The SDK default credential chain is prohibited. Maximum attempts is one. The
operation deadline covers the complete stream. The adapter-private Tokio,
Hyper, rustls, `aws-lc-rs`, SigV4, and EventStream implementation stays behind
the driver boundary.

The first proof excludes model catalogue, cross-region inference profiles,
global routing, provider tools, guardrails, prompt resources, attachments,
images, documents, system-managed prompt storage, automatic retry, and live
authentication from default QA.

## Access Evidence

The first route reports these dimensions independently:

- credential mechanism: cloud-provider identity
- endpoint audience: exact Bedrock Runtime service audience
- entitlement metering: cloud-account billing
- support authority: provider-supported AWS SDK and Bedrock API
- endpoint and IAM authorization
- model access and regional availability
- rate, quota, and runtime readiness

Credential readiness does not prove IAM permission. Catalogue presence does
not prove model access. Region support does not prove quota. A provider model's
own public API credential or subscription does not authorize its Bedrock route.

## Conformance

Deterministic SDK fixtures must prove:

- exact SDK version and typed event variants
- no SDK or credential work before successful preflight
- exact endpoint, audience, region, credential-provider, route, model, and host
  binding
- one attempt with SDK retries disabled
- ordered text, stop, usage, throttling, model-stream error, unknown variant,
  disconnect, cancellation, and deadline behavior
- bounded projection and joined SDK work
- credential, endpoint, SDK error, request, and payload redaction
- no catalogue, profile, environment, metadata, region, endpoint, model, or
  retry fallback

Live AWS authentication and paid inference remain separately gated.

Foreign-language sidecar fixtures additionally prove:

- exact runtime, sidecar, wire, SDK-package, and behavior identity
- explicit construction with all ambient loaders and automatic work disabled
- strict correlated framing, bounds, unknown-message failure, and redaction
- exact host-leased resource agreement before session readiness
- cancellation, disposal, process join, and lease cleanup ordering
- deterministic default QA without package installation, provider access, or
  mutable network discovery

## Acceptance

- SDK-native means in-process Rust embedding for the first proof
- a foreign-language SDK sidecar retains explicit process, wire, runtime, and
  package identities and is never relabeled SDK-native
- concrete SDK runtimes and types remain adapter-private
- no ambient SDK configuration participates in an operation
- delegated credential use remains exact, scoped, and secret-free
- one Swallowtail direct run produces at most one provider inference attempt
- runtime inference and cloud control-plane catalogues remain separate drivers
- no provider, model, region, credential, billing, or topology fallback is
  implicit
