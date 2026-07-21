# 014 Hosted Transport, Credential, And Evidence Boundary

Status: active
Owner: Tom
Updated: 2026-07-20

## Purpose

Make host-approved network transport, scoped credentials, direct streaming,
model catalogues, and provider usage or limit evidence concrete enough for the
first hosted harness and direct-inference drivers.

## Endpoint Authorization

Preflight binds the configured instance, execution host, endpoint reference,
endpoint audience, route, access profile, and required network service before
network work.

Network authorization receives the operation scope, opaque endpoint reference,
and exact endpoint audience. It returns a non-serializable grant containing:

- the same operation scope
- the same endpoint reference and audience
- one host-approved driver endpoint value
- host-owned transport constraints needed by the driver

The endpoint value is redacted from `Debug`, `Display`, diagnostics, events,
and serialized public records. Only an explicit driver accessor exposes it.
Drivers cannot replace its scheme, authority, base path, proxy, TLS policy, or
execution-host placement.

The driver may own its HTTP, SSE, or WebSocket protocol client. Swallowtail does
not force all clients through a generic byte transport. Every request still
uses the bound grant and common cancellation, deadline, diagnostic, and cleanup
rules.

Route transport to the selected endpoint is independent from provider-side
external-network permission and external search. Authorizing one never enables
the others.

## Credential And Delegated Authentication

Credential acquisition receives the operation scope, opaque credential
reference, and exact endpoint audience. It returns either:

- a secret lease bound to that scope, reference, and audience
- delegated authentication owned by the selected harness, SDK, cloud
  environment, or credential helper

A secret lease exposes bytes only through an explicit driver accessor. It is
non-cloneable, non-serializable, redacted, zeroed when released or dropped, and
never becomes a stable diagnostic. Delegated authentication exposes no secret.

Credential release is an explicit awaited cleanup boundary. Provider and
connection cleanup happen before release. Dropping a lease is best-effort
redaction, not proof of clean release. Scope, reference, audience, or execution-
host mismatch fails before a request.

Harness-owned authentication never authorizes extracting its credential for a
direct endpoint. Public API keys, workload identities, subscription OAuth,
server basic auth, and provider credentials delegated to a harness remain
separate access mechanisms.

## Structured Direct Inference

Structured run remains an operation shape, not a synonym for harness
execution. A direct-inference structured run may omit a working resource. A
driver that requires a resource declares the corresponding capability and host
service; it rejects an absent binding before side effects. A driver that does
not require a resource cannot demand a placeholder path or lease.

One direct-inference start produces one provider inference attempt. The
consumer owns retry, repair, tool-loop, validation, and acceptance policy under
Contract 006. A future explicit retry policy may authorize more attempts. It
must expose attempt and usage evidence and cannot cross route, credential,
billing, support, or execution-layer boundaries.

Provider-required generation bounds remain consumer inputs. When a direct
route requires `max_tokens` or an equivalent maximum-output bound, the
structured-run request carries one explicit positive value and the driver
declares support for that input. The driver cannot invent a default, copy a
mutable catalogue maximum, or infer a value from the selected model. Missing,
unsupported, zero, or out-of-range bounds fail before credential or network
work.

Internal catalogue pagination is not inference retry. It remains bounded,
cancellable, deadline-aware, and fixed to the selected instance and route.

## Streaming

HTTP success does not imply provider success. SSE or WebSocket drivers must:

- preserve provider order while emitting common monotonic event sequence
- distinguish output deltas, completed output, usage observations, keepalive,
  and terminal state
- turn a provider error after connection success into `ProviderFailed`
- preserve or reject unknown semantic events under declared extension policy
- stop local reads, close owned connection work, and join all reader tasks on
  cancellation or deadline
- state honestly when cancellation only stops local consumption

Raw headers, event bodies, prompts, output bodies, and provider error payloads
never become public diagnostics.

## Model Catalogue Evidence

Catalogues report observed instance reality. Each entry keeps stable model
identity separate from mutable presentation and may add typed observations for:

- maximum input tokens
- maximum output tokens
- supported operation capabilities and constraints
- provider release or availability metadata when safe

Absence means unknown, not unlimited or unsupported. Catalog evidence does not
select a model, populate an operation, promise entitlement, or prove that a
compatible facade implements every advertised provider feature.

Provider and model ids remain separate where the harness exposes both. A
combined display label does not collapse configured instance, provider route,
model route, or model identity.

## Usage, Rate, Quota, And Correlation Evidence

The runtime may expose safe typed observations for:

- input, output, cache-read, and cache-write tokens
- provider-defined usage dimensions through namespaced extensions
- request or token rate limit, remaining capacity, and reset evidence
- quota or entitlement exhaustion
- opaque provider request correlation

Usage is not cost. Rate limits are not quota. Quota is not entitlement.
Remaining capacity is mutable evidence, not routing or retry authority. Billing
and cost calculations remain consumer policy unless a later contract defines a
provider-authoritative billing record.

Observations record their source boundary and apply only to the operation,
route, access profile, and provider response that produced them. Cumulative
stream usage replaces earlier snapshots for the same provider attempt; it is
not summed as a new attempt.

Provider request ids remain opaque references. They may support operator
correlation without exposing raw headers or payloads.

## Failure Mapping

Drivers preserve at least these safe distinctions when provider evidence
supports them:

- authentication rejected or expired
- endpoint or model permission denied
- billing or entitlement unavailable
- rate limited
- quota exhausted
- provider overloaded
- invalid request
- transport failed
- protocol failed
- provider stream failed after connection success

Provider messages are not stable error codes. Unknown provider failures remain
provider failures with a safe adapter-owned diagnostic.

## Conformance

Deterministic fixtures lead live checks. The hosted direct profile and network
harness profile must prove:

- endpoint and credential scope, audience, route, and execution-host binding
- no network or credential side effect before successful preflight
- endpoint and secret redaction
- awaited connection, task, and credential cleanup
- ordered stream events and mid-stream error handling
- catalogue pagination and unknown-limit behavior
- cumulative usage replacement
- rate, quota, billing, and retry separation
- cancellation and deadline behavior without detached work

Live authentication checks are separately gated and never required for default
repository QA.

## Acceptance

- a hosted driver receives one driver-usable approved endpoint without a raw
  endpoint in public records
- a credential cannot cross scope or endpoint audience
- delegated harness auth exposes no secret
- direct inference runs without a fake working resource
- a provider-required output-token bound stays explicit and consumer-owned
- no implicit inference retry occurs
- catalog limits remain observed mutable metadata
- mid-stream provider errors cannot complete successfully
- usage and limit evidence cannot silently trigger retry or fallback
