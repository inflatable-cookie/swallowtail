# 103 Direct Continuation Contract And DeepSeek Corpus

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../036-deepseek-direct-continuation-proof.md`

## Objective

Implement the minimum provider-neutral direct-continuation records and freeze
the selected DeepSeek V4 HTTP/SSE corpus.

## Readiness Gate

Card 102 must promote current evidence and an active contract that settles
continuation ownership, bounds, redaction, and the exact first route.

## Scope

- contract-backed operation, inference-attempt, tool-call, tool-result, and
  provider-private continuation records
- exact request-plan agreement and pure preflight
- bounded continuation storage and safe diagnostics
- twelfth locally continued direct-session profile required by Contract 030
- dated exact DeepSeek V4 request, stream, continuation, usage, rate, failure,
  cancellation, and drift corpus
- one non-streaming tool-bearing attempt followed by streaming final attempts
- explicit provider-cache acceptance and no read or deletion authority
- no production network transport or live authentication

## Acceptance Criteria

- [x] shared records contain no DeepSeek naming or consumer tool policy
- [x] provider-private continuation cannot be read as portable reasoning
- [x] attempts, tool calls, bytes, events, and time are positively bounded
- [x] unsupported versions and exact corpus drift fail before effects
- [x] the twelfth profile proves only the new locally continued session shape
- [x] the other eleven profiles remain unchanged
- [x] fixtures run without a credential, account, provider request, or retry

## Evidence

- core binds positive turn, attempt, tool, byte, record, token, transport,
  tool-selection, cache, and exact-model requirements into pure preflight
- runtime adds redacted direct-attempt, tool-call, tool-result, continuation-
  binding, and explicit-authorization records without reusing harness callbacks
- provider-private payload bytes remain inside the adapter-private DeepSeek
  protocol module; public continuation records expose only redacted binding and
  byte-count evidence
- `swallowtail-protocol-openai-chat` adds bounded per-message structural
  extensions and null content without acquiring DeepSeek semantics
- `swallowtail-adapter-deepseek` freezes the exact dated facade, opaque version
  claim, V4 Pro request plans, buffered tool response, two SSE final attempts,
  later-turn continuation, cache usage, error, rate, cancellation, disconnect,
  and drift fixtures
- 148 focused core, runtime, testkit, common-codec, and DeepSeek tests pass;
  focused warnings-denied clippy passes
- runtime records and authorization state are split into bounded modules; the
  doctor report remains at the inherited 19 findings rather than adding debt
- full repository QA passes; the final inventory contains 482 tests, with 479
  runnable and three separately gated ignored probes

## Validation

- focused tests for core, runtime, testkit, compatible chat, and DeepSeek
- focused warnings-denied clippy
- full repository QA at batch closeout: 479 passed, three ignored
- `git diff --check`

## Auto-Continuation

No. Confirm contract and corpus before production mapping.
