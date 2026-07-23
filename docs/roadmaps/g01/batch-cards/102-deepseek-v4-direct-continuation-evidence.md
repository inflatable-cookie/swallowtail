# 102 DeepSeek V4 Direct Continuation Evidence

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../036-deepseek-direct-continuation-proof.md`

## Objective

Freeze the current DeepSeek V4 direct-inference continuation boundary and
compile only the shared contract and implementation lane justified by current
official evidence.

## Readiness Gate

Roadmap 035 and card 101 are complete with the Pi RPC harness proof passing
production conformance. Research 022 identifies DeepSeek reasoning/tool
continuation as the next missing direct-inference pressure.

## Governing References

- vision 001
- system architecture and repository authority map
- Contracts 005, 006, 008-011, 014, 019, 020, 022, 025, and 029
- Research 018-022
- roadmap 036

## Scope

- revalidate exact DeepSeek V4 model ids, alias retirement, endpoint facades,
  API-key audience, billing, support authority, model catalogue, and service
  currentness against official sources
- map non-thinking and thinking HTTP/SSE behavior, tool-call correlation,
  assistant content, provider-private reasoning continuation, finish reasons,
  usage, cache, rate, quota, and failure evidence
- distinguish one consumer-owned operation, multiple explicit inference
  attempts, consumer-executed tools, and provider-required continuation state
- decide whether the first proof uses the OpenAI or Anthropic facade and one
  exact V4 model; record unresolved equivalence instead of silent fallback
- define bounded attempt, tool-call, stream, payload, deadline, cancellation,
  retry, retention, redaction, and cleanup limits
- compare existing hosted-direct and compatible-chat records before adding any
  provider-neutral surface
- promote a research delta, the minimum durable contract, and cards 103-104
  only when the boundary is settled

## Acceptance Criteria

- [x] every claim is dated and backed by current official DeepSeek material
- [x] endpoint, facade, credential, entitlement, metering, model, and support
      authority remain separate
- [x] provider-private continuation is not exposed as portable reasoning,
      consumer memory, or a generic agent loop
- [x] consumer tool choice, authorization, and execution remain downstream
- [x] exact unsupported, ignored, preview, deprecated, and retiring behavior is
      recorded without compatibility inference
- [x] no implicit model, facade, endpoint, retry, credential, or provider
      fallback enters the lane
- [x] one contract-and-corpus card is ready, or the exact evidence blocker is
      recorded and implementation stays closed
- [x] the roadmap spine retains one sole next task

## Completion Evidence

- Research 023 records the dated provider, access, cache, catalogue, facade,
  model, request, streaming, tool, reasoning, usage, rate, failure, privacy,
  and lifecycle evidence
- the first route uses `https://api.deepseek.com`, OpenAI-format
  `/chat/completions`, and exact `deepseek-v4-pro`; it is not a consumer
  default
- the Anthropic facade is excluded because unsupported-model mapping and
  ignored fields conflict with exact route and failure behavior
- provider-private `reasoning_content` stays bounded, zeroizing, route-bound,
  ephemeral, non-serializable, and absent from stable consumer surfaces
- Contract 030 makes every further inference attempt consumer-authorized and
  keeps tool validation, execution, result creation, and loop policy downstream
- the first corpus uses one non-streaming tool attempt and streaming final
  attempts; streamed tool-call assembly remains an evidence gap
- automatic disk context caching is an explicit provider posture with no
  Swallowtail read or deletion authority
- card 103 is ready for records, pure preflight, the twelfth locally continued
  direct-session profile, and frozen offline corpus

## Evidence Required

- official DeepSeek API start, change log, model/pricing, model catalogue,
  Chat Completions, thinking-mode, tool-call, error, and rate-limit material
- an explicit mapping against current direct-operation, hosted transport,
  compatible-codec, usage, lifecycle, and version-qualification contracts
- a field-level continuation ownership and redaction table
- a bounded first-proof recommendation with exclusions

## Validation

- configured docs and Northstar QA
- explicit new-document link checks
- `git diff --check`
- `effigy doctor` retains the inherited 19 findings: 7 errors and 12 warnings

## Stop Conditions

- official surfaces disagree on required continuation replay
- choosing a facade or model would set consumer product policy rather than an
  integration proof boundary
- required reasoning cannot remain provider-private and safely redacted
- the useful subset needs automatic tool execution, retry, or an unbounded
  inference loop

## Auto-Continuation

No. Promote the contract boundary before corpus or production work.
