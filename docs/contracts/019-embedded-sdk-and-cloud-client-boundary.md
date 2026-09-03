# 019 Embedded SDK And Cloud Client Boundary

Status: active
Owner: Tom
Updated: 2026-09-03

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
host resources, and credentials in contract order. Process isolation does not
by itself prove filesystem or network containment.

Where an upstream SDK launches further provider-owned processes, the nearest
sidecar is not the lifecycle boundary. The execution host owns termination of
the sidecar root and every descendant still reachable through its declared
tree authority. The launch recipe states exactly whether the host can attest
tree emptiness or only root completion. A route may support the latter posture
only as an explicit platform-qualified degraded boundary; it cannot describe
root-only evidence as a joined tree.

On an attested-tree platform, close joins every provider process in the tree,
not only the nearest child. Every platform bounds its claimed join, states the
bound, and escalates through host termination authority on expiry before
joining again. Close reports one explicit outcome: exited gracefully, exited
after escalation, or exit unconfirmed. A surviving descendant, discarded wait,
or result that cannot distinguish exit from expiry is never evidence of clean
close.

Root exit is not tree completion. A close outcome may report the descendant
tree as joined, or report cleanup `Clean`, only where the execution host attests
owned-tree emptiness from a concrete Contract 010 observation whose owned-tree
identity a descendant cannot escape by session change, descriptor drop, or
reparenting, and which denies migration out of the owned set.

An explicitly qualified root-only platform may still expose the route when the
host bounds interruption, disposal, escalation, and root join by the caller's
cleanup deadline. A confirmed root exit after the declared descendant
termination attempt returns an exact route-qualified `Degraded` outcome because
descendant emptiness remains unconfirmed. An unconfirmed root exit or an
observed surviving descendant returns `Failed`. Neither outcome may be promoted
to `Clean`, cached as tree-empty evidence, or used to widen another route or
platform. Applications may reject the degraded platform posture at selection.

Card 059 found no owned-tree-empty observation within current ordinary
host-local authority on macOS. The subscription-backed SDK route may therefore
qualify macOS only under the root-only degraded rule above. A future stronger
host mechanism may replace that posture after separate evidence; it is not
inferred from successful ordinary closes.

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

A route whose caller may return before that work completes must acquire
Contract 010's operation-scoped reap reservation from the exact selected task
service before provider work and before acquiring credentials, resources,
processes, or tasks. Missing support, closed admission, or unavailable reserved
capacity makes the host unsupported for that operation and rejects at this
boundary. Static service presence and a boolean capability probe do not qualify
the route because shutdown can race a later handoff.

Cancellation and deadline stop local SDK work, close owned transport state,
join the scoped work, then release credentials. A provider request may have
reached the service even when local cancellation succeeds; drivers state that
limit honestly.

Interactive-session close is additionally governed by Contract 010's one
caller-selected cleanup deadline. The same observable boundary covers SDK or
sidecar interruption, transport disposal, escalation, task and process joins,
provider cleanup, credential release, and host-resource release. No stage may
restart the timeout or extend the public close future. Once host time observes
expiry, cleanup returns `Failed` or an exact route-qualified degraded outcome;
it cannot return `Clean`.

For a foreign-language sidecar, one enclosing guardian owns the pump task,
process handle, resource lease, and credential lease. Its cleanup order is:
interrupt, provider-native close, force-stop when needed, root/process
observation, pump completion and join, resource release, then credential
release. Earlier stages may be not applicable, but later ownership cannot move
ahead of unfinished earlier work.

If the caller deadline arrives before that sequence completes, the caller
transfers the enclosing guardian task—not the pump—through the selected
execution host's `ScopedTaskService::relinquish` operation as the reservation-
backed task under its exact operation scope. The reservation makes a valid
exact-host/exact-scope transfer non-fallible for capacity and lifecycle reasons.
The host accepts autonomous reap ownership before the caller returns.
Acceptance is `AcceptedForReap`, not joined or cleanup-complete; the adapter
cannot use it to strengthen the session's cleanup result. The guardian keeps
the process and both leases until their ordered terminal cleanup. An outer
selected-host lifecycle owner keeps the reaper and explicitly joins it outside
the task tree after reservation admission closes and existing reservations and
accepted tasks settle. A task-service clone has only weak authority and never
joins a reaper on drop. Adapter-global parking, a discarded reaper handle, a
later adapter cleanup call, and release of leases around a transferred pump are
not substitutes.

After a turn deadline, session cleanup owns interruption of any still-active
turn. A structured projection must return any unfinished task to the selected
host before entering bounded session close, so an unresponsive task join
cannot strand the public operation outside the session boundary. It retains
the exact failed or degraded cleanup truth because accepted-for-reap is not
joined cleanup. An open path either validates every
fallible projection or management binding before provider work, or receives
the caller's cleanup request and closes an already-open session on abort.
Neither path may drop a live session as its cleanup mechanism.

Adapters do not retain the removed zero-argument close signature, synthesize a
route timeout, convert a duration into guessed ticks, or offer a compatibility
shim that reaches an unbounded cleanup path.

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

- unsupported host or reservation refusal rejects before credential, resource,
  process, provider, or task work
- a real selected-host reservation survives a shutdown race and lets caller
  expiry return the enclosing cleanup guardian to the exact host reaper without
  blocking or treating acceptance as joined cleanup
- guardian cleanup preserves interrupt, native close, force-stop,
  root/process observation, pump join, resource release, and credential release
  order after transfer
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
- a boolean reap-capability probe is insufficient; the operation holds the
  actual reservation before effects
