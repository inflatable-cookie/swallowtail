# 043 Anthropic Direct Driver

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../013-anthropic-direct-inference-proof.md`

## Objective

Implement Anthropic catalogue and one-attempt direct structured inference.

## Scope

- new Anthropic adapter crate
- provider-neutral explicit maximum-output-token input and capability
- scoped public-API credential, model listing, Messages SSE, cancellation,
  output, usage, rate, request correlation, failure, and cleanup

## Out Of Scope

- tool loop, retries, validation, billing, batches, files, or beta APIs
- consumer adoption

## Acceptance Criteria

- [x] no process or working-resource service is required
- [x] exact model, endpoint, audience, credential, and version stay bound
- [x] missing or unsupported maximum-output bound fails before host work
- [x] unknown top-level events do not hide unknown content semantics
- [x] mid-stream error fails once
- [x] secret release follows connection cleanup

## Evidence

- provider-neutral structured runs now carry an optional nonzero maximum-output
  bound; Anthropic requires the bound and matching capability before host work
- the production driver implements two-page bounded catalogue discovery and one
  Messages SSE attempt through host network, credential, task, blocking-work,
  and time ports only
- exact API-key, endpoint audience, model, provider, version, and request body
  remain bound; no SDK retry or model fallback exists
- output, keepalive, cumulative usage, rate remaining/limit, and opaque request
  correlation normalize separately
- mid-stream provider error, disconnect, invalid order, and unknown content
  semantics fail safely; unknown top-level events remain forward-compatible
- cancellation closes local connection work, joins the blocking job, then
  releases the zeroed credential lease
- 18 focused adapter and hosted-profile tests pass

## Validation

- focused adapter tests
- dependency/redaction scan
- `git diff --check`

## Stop Conditions

- adapter needs credential extraction from a harness
- implicit retry or model fallback appears

## Auto-Continuation

Yes, after card 044 is ready and focused validation passes.
