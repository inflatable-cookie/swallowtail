# 021 Anthropic Direct Prepared Facade

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../008-representative-cross-shape-facades.md`

## Objective

Prove the facade against provider-supported Anthropic Models and Messages
HTTP/SSE.

## Governing Refs

- Contracts 014, 020, 029-030, and 037
- Anthropic direct driver fixtures
- card 020

## Scope

1. Prepare one exact endpoint audience, API credential lease source, access
   profile, route, model, and host.
2. Bind catalogue and streaming direct-inference operations separately.
3. Preserve consumer-owned tool continuation and explicit attempt authority.
4. Expose source-scoped catalogue and usage evidence without implying
   entitlement.
5. Reuse current HTTP/SSE cancellation, deadline, redaction, and cleanup.

## Acceptance Criteria

- [x] no endpoint, credential, model, or retry fallback exists
- [x] catalogue observation does not select a route
- [x] each provider attempt remains explicit
- [x] tool execution stays downstream
- [x] credential release occurs after joined network work

## Validation

- deterministic direct fixtures
- hosted-direct and direct-tool conformance
- local and remote-authoritative hosts
- low-level driver regression

## Evidence

- `prepare_anthropic_direct` binds one provider-supported public API-key
  profile, `api.anthropic.com` audience, opaque endpoint target, host, dated
  facade, configured instance, and access provenance without provider effects.
- Prepared catalogue and inference-attempt values retain separate roles,
  plans, requests, evidence, and typed bound execution.
- The text-only inference profile fixes one offline, attached, non-retained
  Messages attempt and declares neither tool calls nor direct continuation.
  Every further attempt requires another explicit consumer call.
- Deterministic fixtures pass against local and remote-authoritative host
  identities. Catalogue returns source-scoped observations without a route;
  inference makes one request and releases its credential after joined stream
  work.
- The complete Anthropic adapter suite and hosted-direct plus locally
  continued direct-session conformance pass. The latter remains a separate
  operation shape rather than being flattened into Anthropic structured run.
- The compile-tested example, public guide, all-target check, and
  warnings-denied Rust lint pass.
- Full repository QA passes with 661 deterministic tests and four gated live
  checks ignored.
- Doctor remains at the same 19 pre-existing oversized-file findings; no new
  prepared-facade file appears in the report.
- Public-API comparison reports the expected additive Anthropic adapter drift
  alongside the already held core, runtime, testkit, Codex, and Kimi facade
  drift. Card 036 owns the replacement baseline.

## Auto-Continuation

Yes. Continue to card 022 after deterministic validation.
