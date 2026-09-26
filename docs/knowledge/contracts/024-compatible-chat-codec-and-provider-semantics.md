# 024 Compatible Chat Codec And Provider Semantics

Status: active
Owner: Tom
Updated: 2026-07-21

## Purpose

Permit bounded reuse of OpenAI-compatible Chat Completions structure without
turning protocol similarity into provider, access, model, capability, or
lifecycle equivalence.

## Codec Authority

A compatible Chat Completions codec may own only wire structure:

- bounded JSON encoding and decoding for declared common request and response
  fields
- bounded SSE framing, comments, data records, and `[DONE]`
- structural chunk, choice, delta, finish, model, usage, and error envelopes
- correlation evidence needed by the provider adapter to validate one stream

The codec produces structural protocol records, not runtime events, catalogue
entries, terminal outcomes, provider failures, model routes, or access state.
It has no network, endpoint, credential, retry, timer, task, filesystem, or
provider identity authority.

## Provider Adapter Authority

Every provider adapter remains separately registered and owns:

- configured instance, endpoint path, audience, headers, credential mechanism,
  entitlement, billing posture, and support authority
- exact model route and agreement between requested and returned model identity
- accepted, required, ignored, mapped, and rejected request fields
- reasoning, tools, structured output, modalities, finish reasons, usage
  extensions, rate, quota, and error semantics
- model catalogue mapping and whether the provider exposes a catalogue at all
- cancellation truth, provider state, retention, retry, reconnect, and
  continuation behavior

Sharing the codec cannot widen a driver's capability profile. A field decoded
successfully is not a capability claim.

## Unknown And Drift Rules

The codec cannot silently discard an unknown semantic field. It either:

- returns one bounded structural unknown to the provider adapter for an exact
  dated decision, or
- fails the protocol record as unsupported or malformed

The provider adapter then accepts, namespaces, or rejects the field according
to its frozen corpus. Raw JSON, SSE data, provider ids, request ids, prompts,
outputs, reasoning, errors, endpoint values, and credentials remain private.

Each provider needs an independent dated fixture corpus. Passing one provider's
corpus does not qualify another provider or a newer version. A shared codec is
justified only after at least two independent facades exercise the same
structural boundary.

## Model And Fallback Rules

The operation binds one exact configured instance and model route before
network or credential work. A compatible field named `model` cannot select a
default or authorize an alias. The adapter validates returned model identity
when the provider supplies it.

Provider-side aliasing, compatibility mapping, endpoint failover, model
substitution, and ignored model names are fallback behavior. They are excluded
unless an exact configured route and provider corpus explicitly expose that
behavior and the consumer opts into it. The first proof permits none.

## Attempt And Lifecycle Rules

Compatibility examples, SDK defaults, and provider reconnection guides do not
authorize retry. One structured direct start produces one inference attempt
under Contract 014. Cancellation or deadline stops local connection work and
joins it; native provider cancellation is claimed only when the selected
surface defines it.

Chat Completions history remains consumer-supplied unless the exact provider
surface proves storage. A compatible codec owns no response retention,
continuation, conversation, background execution, polling, or deletion
authority.

## First Kimi Platform Subset

The first provider proof binds:

- Kimi Open Platform documentation observed 2026-07-21
- one separately registered Kimi Platform direct driver
- `https://api.moonshot.ai` through a host-approved endpoint grant
- an API key issued by `platform.kimi.ai`, pay-as-you-go metering, and the exact
  `api.moonshot.ai` audience
- authenticated bounded `GET /v1/models` catalogue observation
- one resource-free text structured run through `POST /v1/chat/completions`
- exact model route `kimi-k3`; no alias or model fallback
- explicit `low`, `high`, or `max` reasoning selection and one positive
  output-token bound
- `stream=true`, ordered reasoning and output deltas, terminal usage, returned-
  model agreement, and `[DONE]`
- one attempt, local cancellation and deadline, joined work, and awaited
  credential release

Kimi Membership, Kimi Code credentials, `platform.kimi.com` keys, subscription
metering, tools, structured output, multimodal input, files, batch, balance,
official tools, web search, Partial Mode, provider history, automatic
reconnection, and retry are excluded.

Catalogue presence remains source-scoped evidence under Contract 020. It does
not prove top-up, entitlement, route readiness, or invocation success.

## Reuse Proof

The first common package must pass both:

- the existing llama.cpp build-9910 compatible Chat Completions corpus without
  widening its text-only attached-runtime behavior
- the dated Kimi K3 corpus with Kimi-specific reasoning, usage, model, error,
  and catalogue mapping kept in the Kimi Platform adapter

Migration cannot change llama.cpp endpoint, deployment, request, cancellation,
terminal, catalogue, or serving-ownership behavior.

## Conformance

Deterministic fixtures must prove:

- bounded fragmentation, comments, data events, `[DONE]`, disconnect, and
  malformed framing
- common structure across llama.cpp and Kimi without provider identity branches
  in the codec
- provider-specific reasoning, usage extensions, finish reasons, errors, and
  unknowns remain adapter decisions
- exact endpoint, audience, credential, host, route, model, reasoning, and
  output-bound preflight before effects
- authenticated catalogue bounds and source-scoped observations
- one inference attempt with no SDK retry or provider reconnection
- ordered reasoning and output, terminal usage, returned-model agreement,
  provider failure, cancellation, deadline, redaction, joined connection work,
  and awaited credential release

Live catalogue and authentication checks remain separately gated.

## Acceptance

- reuse stops at structural protocol truth
- provider differences remain visible and testable
- no common `send_prompt`, provider router, default, retry, or fallback appears
- two independent corpora qualify the shared codec
- the first Kimi route cannot use Membership, Kimi Code, regional, or
  subscription credentials
- no raw provider payload or secret enters stable diagnostics
