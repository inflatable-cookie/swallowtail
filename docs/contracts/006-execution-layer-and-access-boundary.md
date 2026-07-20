# 006 Execution Layer and Access Boundary

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Keep harness control, direct model inference, operation shape, and provider
access separate enough to support both zero-setup local use and explicit API
connections safely.

## Execution Layers

Swallowtail exposes two distinct runtime layers:

1. **Harness interaction** — drives a provider-owned agent loop, session,
   process, or service. The harness may own context assembly, tools, workspace
   actions, approvals, compaction, recovery, and other agent behavior.
2. **Direct model inference** — calls a model endpoint or local inference
   runtime. The consumer owns orchestration, prompt assembly, tool loop,
   validation, retry, and product consequences.

Using only a small subset of a harness does not make it direct inference.
Adding tools, streaming, or provider-side state to a model API does not make it
an agent harness without lifecycle evidence.

A driver may implement both layers only when its supported surface genuinely
exposes both and its support authority is visible. It advertises and tests them
separately. There is no third generic `hybrid` layer.

## Operation Shape Is Independent

Interactive session and bounded structured run remain the operation shapes
defined by Contract 004. Either may be implemented over either execution layer
when capabilities support it.

Examples:

- a multi-turn coding session through Codex is harness interaction
- one schema-oriented tagging turn through Codex is a harness-backed structured
  run
- one Responses or Messages request is direct inference
- a consumer-owned tool loop over a model API remains direct inference

Operation semantics must name the execution layer used. Swallowtail does not
silently substitute a harness for direct inference or the reverse.

## Access Profiles

Access is not one enum. A configured driver instance declares an access profile
with independent dimensions:

1. **Credential mechanism** — interactive OAuth, device OAuth, automation
   token, API key, workload or service identity, cloud-provider identity,
   gateway helper, local unauthenticated access, or a provider-specific method.
2. **Entitlement and metering** — subscription allowance, prepaid or bundled
   credits, pay-as-you-go usage, cloud-account billing, local compute, or an
   unknown/custom arrangement.
3. **Endpoint audience** — the product, API, account, workspace, or gateway for
   which the credential was issued.
4. **Support authority** — provider-supported, integration-maintainer-supported,
   experimental/observed, or prohibited.

Credential mechanism does not imply commercial entitlement. An API key may be
subscription-metered, as with Kimi Code, or usage-billed, as with many public
model APIs. OAuth may authorize a harness, a product-scoped inference endpoint,
or both.

Credential state, entitlement state, endpoint authorization, and runtime
readiness are distinct. A valid credential does not promise a model
entitlement; a discovered binary does not promise a valid credential.

## Subscription Boundary

- Subscription-backed access may drive either execution layer when the
  credential is valid for the selected endpoint audience.
- A subscription credential for one product is not automatically valid for a
  provider's public API, even when request bodies look compatible.
- Direct inference includes product-scoped model endpoints as well as public
  model APIs; their credential and entitlement mechanisms may differ.
- An adapter must not extract, replay, or translate cached provider credentials
  into another endpoint audience without evidence that the use is authorized.
- When a CLI, SDK, app-server, or helper owns credential refresh, an adapter may
  delegate to it instead of reading its credential store.
- A provider-published route is preferred. A route documented and maintained by
  an integration may be supported with that lower authority made visible.
- Experimental or observed routes require explicit consumer opt-in, carry no
  stability promise, and must be isolated from stable adapters.
- Swallowtail does not ship a route that the provider explicitly prohibits.
- Provider version, account, workspace, role, plan, quota, and policy may alter
  capabilities. Discovery reports observed instance reality without promising
  a plan-wide feature set.

## Host And Consumer Ownership

The execution host owns:

- secret storage and credential references
- interactive sign-in placement and user consent
- credential injection, rotation, revocation, and redaction policy
- permission to use local subscription state on behalf of an application
- authority to place credentials on a local or remote execution host

Swallowtail may expose safe auth status, supported sign-in actions, access
profile metadata, and credential requirements. Stable public records never
contain secret material.

The consumer owns:

- route preference and acceptable execution layers
- acceptable credential, entitlement, endpoint-audience, and support-authority
  combinations
- cost, quota, privacy, and fallback policy
- operator UI and confirmation when an access or billing boundary changes

No automatic fallback may cross credential mechanism, entitlement or billing
authority, endpoint audience, execution host, support authority, or execution
layer without explicit consumer policy.

## Failure Rules

The runtime distinguishes at least:

- driver unavailable
- sign-in required
- credential rejected or expired
- subscription or account not entitled
- model unavailable under the selected access profile
- quota or rate limit reached
- organization or administrator policy rejection
- provider transport failure

These failures remain safe diagnostics. Raw credentials, tokens, provider
responses, and credential-store paths are not public error detail by default.

## Deferred Decisions

This contract does not choose concrete Rust traits, access-profile records,
credential-host ports, router policy, or adapter packages. Research 003 and
Spec 002 supply the wider evidence; Contracts 008-010 promote the concrete
runtime boundaries.
