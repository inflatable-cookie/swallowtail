# 036 DeepSeek Direct Continuation Proof

Status: completed
Owner: Tom
Updated: 2026-07-22

## Purpose

Prove consumer-owned direct-inference continuation across provider tool calls
without turning Swallowtail into an agent loop or treating provider-private
reasoning as portable consumer state.

## Generation Runway

Keep g01 active. It contains 36 numbered roadmaps and remains inside the normal
30-50 roadmap range. This proof does not create g02.

## Evidence Gate

Research 022 selects DeepSeek V4 as the next direct-contract candidate. Current
official documentation exposes exact `deepseek-v4-flash` and
`deepseek-v4-pro` routes over OpenAI and Anthropic facades. Thinking-mode tool
chains require the assistant message's `reasoning_content` on later inference
attempts; omission produces a provider error. The retiring `deepseek-chat` and
`deepseek-reasoner` aliases are not eligible qualification points.

Research 023 selects the exact OpenAI-format facade and V4 Pro route. Contract
030 fixes a locally continued direct session with explicit attempt
authorization, consumer-owned tools, private ephemeral reasoning continuation,
provider-cache acceptance, and joined cleanup. The Anthropic facade, retiring
aliases, streamed tool-call assembly, V4 Flash, and beta features remain
excluded from the first proof.

## Goals

- [x] Separate one consumer operation from its multiple direct inference
      attempts and consumer-executed tool work.
- [x] Preserve provider-private continuation evidence without exposing it as
      portable reasoning or durable consumer memory.
- [x] Freeze one exact current DeepSeek V4 route and deterministic HTTP/SSE
      corpus.
- [x] Implement a separately registered direct driver only after the shared
      continuation contract is active.
- [x] Prove exact access, model, request, continuation, usage, cancellation,
      deadline, failure, redaction, and cleanup behavior under both host
      identities.

## Execution Plan

- [x] Currentness and direct-continuation evidence: card 102.
- [x] Shared contract, records, and frozen corpus: card 103.
- [x] Production driver, conformance, and closeout: card 104.

## Cards

- `batch-cards/102-deepseek-v4-direct-continuation-evidence.md` — completed
- `batch-cards/103-direct-continuation-contract-and-deepseek-corpus.md` — completed
- `batch-cards/104-deepseek-v4-driver-conformance-and-closeout.md` — completed

## Completion Evidence

The separately registered DeepSeek driver binds one exact facade, endpoint,
audience, API-key mechanism, model route, continuation configuration, and
provider-cache posture. It performs the authenticated catalogue request, one
buffered tool-bearing attempt, and two bounded SSE final attempts without
retry or fallback. Consumer tools remain outside Swallowtail. Provider-private
reasoning stays zeroizing, redacted, route-bound, and non-serializable.

Production fixtures prove exact replay, attempt authorization, usage and cache
evidence, finish and request evidence, cancellation, deadline, disconnect,
safe failure, both execution-host identities, joined transport work, and
credential-last cleanup. Full QA closes with 486 passing tests and three gated
probes ignored. Doctor remains at the inherited 19 findings.

## Bounded Candidate Shape

One local or remote-authoritative host uses a provider-supported DeepSeek API
key and one exact provider endpoint, facade, model route, and model. The
consumer remains responsible for prompts, tool declarations, tool execution,
authorization, and deciding whether to continue. Swallowtail performs one
explicit inference attempt at a time and transports only the provider-private
continuation evidence required for the next authorized attempt.

The candidate excludes automatic tool execution, autonomous loops, implicit
model aliases, provider or facade fallback, cross-provider continuation,
durable reasoning storage, raw reasoning diagnostics, credential acquisition,
account management, retries, and live authentication from default QA.

## Stop Condition

Stop if official evidence does not settle the exact facade or continuation
rules, if preserving required provider state would expose reasoning through
stable consumer surfaces, if one operation cannot bound attempts and tool
calls, or if implementation would require Swallowtail to choose or execute
consumer tools.
