# DeepSeek V4 Continuation Selection

Date: 2026-07-22

## Changed

- promoted Research 023 from current official DeepSeek evidence
- selected the OpenAI-format `https://api.deepseek.com` facade and exact
  `deepseek-v4-pro` route for the first proof
- promoted Contract 030 for consumer-authorized direct attempts,
  consumer-executed tools, private local continuation, cache posture, and
  joined cleanup
- added a twelfth locally continued direct-session conformance profile
- completed card 102 and made card 103 the sole ready task

## Decision

The operation is a resource-free interactive session over direct model
inference. Each user turn authorizes one provider attempt. A tool call pauses
the turn. The consumer validates and executes it, then an exact correlated
result authorizes one further attempt. Swallowtail does not choose tools or run
an automatic loop.

DeepSeek-required `reasoning_content` remains bounded zeroizing adapter state.
It is replayed only into the same session, facade, route, model, access, and
host binding. It never becomes portable reasoning, a public event, a durable
resume binding, consumer memory, or a diagnostic.

The Anthropic facade is excluded because it maps unsupported models to V4
Flash and ignores request fields. The first tool-bearing attempt is non-
streaming because current official material does not freeze streamed tool-call
assembly clearly enough. Final attempts use SSE.

Provider disk context caching is enabled by default and has no first-proof
deletion surface. The consumer must explicitly accept that route posture.
Swallowtail cannot turn cache expiry or provider privacy text into deletion
truth.

## Bounds

- two user turns, three provider attempts
- eight declared functions, one returned tool call, one result
- 8,192 output tokens per attempt
- 64 KiB arguments, 64 KiB result, 256 KiB continuation field
- 1 MiB private history or encoded record, 4,096 SSE records per attempt
- one active turn and request, one turn deadline, no retry

## Validation

- current official API, model, facade, thinking, tool, cache, rate, error, and
  privacy material reviewed without live access
- configured docs and Northstar QA pass
- explicit new-document link checks pass
- `git diff --check` passes
- doctor baseline: inherited 19 findings, 7 errors and 12 warnings

## Next

Card 103 is ready. Realize the provider-neutral records, pure preflight,
twelfth profile, and exact V4 Pro offline corpus before production transport.
