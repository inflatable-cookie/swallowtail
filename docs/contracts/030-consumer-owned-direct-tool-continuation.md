# 030 Consumer-Owned Direct Tool Continuation

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Permit a direct model session to cross consumer-executed tool calls while
keeping orchestration downstream and provider-private continuation bounded,
redacted, route-bound, and ephemeral.

## Operation Shape

The operation is a resource-free `InteractiveSession` over
`DirectModelInference`. It is not a harness, structured run, provider-owned
conversation, connection-scoped continuation, or agent loop.

The session owns adapter-private provider-message history for its lifetime.
Each provider inference attempt uses one explicit HTTP request. A tool call
pauses the current turn for consumer input. It does not authorize Swallowtail
to execute a tool or start another inference attempt.

The consumer owns:

- system and user content
- tool declarations and authorization policy
- tool-call argument validation
- tool execution and result content
- the decision to submit a result, continue, cancel, or close
- transcript persistence, product memory, and consequences

Swallowtail owns exact protocol transport, correlation, private continuation
retention, one-attempt execution, event projection, deadline observation, and
joined cleanup.

## Identity

Configured instance, execution host, endpoint, endpoint audience, access
profile, credential, facade revision, model route, model, runtime session,
user turn, inference attempt, HTTP request, stream attachment, provider
completion, tool call, tool result, private continuation record, provider
cache entry, and common event sequence remain separate.

A tool call id is not an attempt, callback, turn, or provider completion id.
Provider cache evidence is not local continuation. A compatible message list
is not consumer transcript authority.

## Explicit Attempt Authorization

Starting a user turn authorizes exactly one inference attempt. A successful
tool-call result moves the turn into a bounded consumer-input wait. Submitting
all exact correlated tool results authorizes exactly one further attempt.

No timer, retry policy, provider error, usage observation, remaining capacity,
tool call, or successful result starts another attempt implicitly. Duplicate,
late, foreign, missing, oversized, or undeclared tool results fail before
network work.

Every session and turn declares positive maxima for user turns, total attempts,
declared tools, returned tool calls, result bytes, private continuation bytes,
history bytes, stream records, and output tokens. One active turn and one
active provider attempt are the default. Exceeding a bound ends the affected
session without fallback.

## Provider-Private Continuation

Provider-required reasoning or other private continuation fields remain
adapter-private. They:

- are captured only from the exact bound provider response
- are stored in bounded zeroizing memory
- retain exact order with the assistant content and tool-call envelope needed
  for replay
- may be sent only to the same configured instance, facade revision, access
  profile, model route, model, and runtime session
- never appear in public events, terminal output, callback exchange, resume
  bindings, serialization, `Debug`, `Display`, or stable diagnostics
- are destroyed on session close or invalidation

Normalized assistant text and bounded tool-call identity, name, and arguments
remain consumer-visible. Hiding private continuation cannot hide a tool call,
usage, finish reason, provider failure, or cleanup outcome.

The first boundary has no resume, export, import, fork, branch, durable
persistence, or reconstruction from a consumer transcript. A closed session
cannot be resumed.

## Tool Wait And Result Exchange

Direct tool exchange is distinct from a harness callback. The provider is not
waiting on an open RPC call; the prior inference attempt has ended. The
consumer receives a bounded pending-call record and later submits one exact
correlated result through the session.

The wait remains inside the turn's host-monotonic deadline. Cancellation,
deadline, close, or result failure abandons it and rejects late submissions.
Swallowtail never fabricates a result, approves a call, retries a tool, or
interprets the result.

## Streaming And Terminal Truth

An adapter may use non-streaming and streaming attempts in one turn only when
the exact request plan fixes which attempt shape is allowed. Switching is not
reconnect or fallback.

Every attempt has its own ordered provider evidence and cumulative usage.
Attempt usage is summed only into an explicit operation aggregate; a cumulative
snapshot within one attempt is replaced, not double-counted. Tool-call pause is
not turn completion. Exactly one final turn outcome follows final output,
cancellation, timeout, provider failure, host failure, or runtime failure.

Local cancellation or deadline closes and joins active network work. It claims
remote stop only when the selected API supplies and confirms native
cancellation. An uncertain active attempt invalidates the first session.

## Provider Cache And Retention

Local private continuation, provider response storage, provider conversation
retention, background execution, and provider-managed inference cache are
independent.

A route whose cache cannot be disabled requires explicit provider-managed
cache acceptance in the session request, requirements, configured-instance
policy, and immutable plan. Cache acceptance grants no read, list, delete,
resume, or persistence guarantee. Cache-hit usage is evidence only.

Provider data-processing or residency posture remains route-selection policy
owned by the consumer. Swallowtail cannot make a route acceptable, choose it
as fallback, or turn provider policy text into a technical deletion claim.

## Access, Version, And Catalogue

Contract 014 endpoint and credential leases remain exact and audience-bound.
The session holds them across all authorized attempts, tool waits, and close.
Connection work and private continuation cleanup finish before release.

Contract 020 catalogue evidence remains source-scoped. Catalogue presence
does not select a model or prove entitlement, balance, capacity, or invocation.

Contract 029 binds the exact unversioned hosted facade through one dated
evidence revision. Model-route revisions remain separate. An alias,
provider-side model mapping, ignored model, alternate facade, proxy, gateway,
or compatible third-party endpoint cannot substitute for the bound route.

## First DeepSeek V4 Subset

The first proof binds:

- official evidence observed 2026-07-22
- facade revision `deepseek-openai-chat-2026-07-22`
- exact OpenAI-format endpoint `https://api.deepseek.com` and
  `POST /chat/completions`; no appended `/v1`
- one public Open Platform bearer API-key lease, usage-billed balance, and
  provider support authority
- authenticated `GET /models` observation and exact
  `deepseek-v4-pro` model route
- explicit thinking enabled, reasoning effort `high`, provider-automatic tool
  selection, and a positive maximum-output-token bound
- maximum two user turns, three attempts, eight declared function tools, one
  returned tool call, and one correlated result
- maximum 8,192 output tokens, 64 KiB arguments, 64 KiB result, 256 KiB one
  private continuation field, 1 MiB private session history or encoded record,
  and 4,096 SSE records per attempt
- one non-streaming tool-bearing attempt and streaming final attempts
- one active turn, one active attempt, one turn deadline, no retry, and joined
  credential-last cleanup
- explicit acceptance of provider-managed best-effort disk context caching
  without read or deletion authority

The driver omits `tool_choice` on the wire for thinking mode while the public
request explicitly selects provider-automatic behavior. It rejects
temperature, top-p, penalties, developer role, logprobs, JSON mode, beta
strict tools, prefix completion, FIM, multiple tool calls, a fourth attempt,
and a third user turn before effects.

The mutable `deepseek-chat` and `deepseek-reasoner` aliases, V4 Flash,
Anthropic facade, `/v1`, beta endpoint, third-party compatible endpoints,
DeepSeek app credentials, OAuth, account or balance management, streamed tool-
call assembly, provider tools, automatic execution, retry, resume, durable
history, and live access in default QA are excluded.

## Failure And Cleanup

The adapter preserves authentication rejection, insufficient balance, account
concurrency, invalid request, provider overload, provider failure, disconnect,
protocol drift, model mismatch, continuation rejection, overflow,
cancellation, deadline, and cleanup failure as distinct safe evidence.

Close order is:

1. reject new turns, attempts, and tool results
2. abandon pending tool input
3. close and join active HTTP/SSE work
4. zeroize private continuation and local provider-message history
5. join every session task and timer
6. release endpoint and credential authority

No reader, timer, retry, cache, cleanup, or credential task detaches.

## Conformance

Contract 011 gains a locally continued direct-session profile. Deterministic
fixtures must prove:

- exact execution layer, operation shape, facade, endpoint, audience, access,
  route, model, host, cache posture, and version binding before effects
- explicit attempt authorization and no provider request during tool wait
- tool-call/result correlation and consumer-owned execution
- private continuation replay across the immediate result and later user turn
- continuation redaction, zeroization, route binding, and non-serialization
- fixed non-streaming tool attempt plus streaming final attempts
- ordered assistant text, tool call, final output, finish, per-attempt usage,
  cache-hit/miss usage, rate, and request evidence
- alias, Anthropic mapping, ignored fields, beta fields, `/v1`, second tool
  call, fourth attempt, third turn, and incompatible version rejection
- provider failure, continuation 400, insufficient balance, rate limit,
  overload, disconnect, malformed data, unknown semantic field, cancellation,
  deadline, and cleanup failure
- local and remote-authoritative hosts, bounded buffers, joined work, endpoint
  and credential release, and safe diagnostics

Default QA uses no DeepSeek account, credential, external request, balance,
cache entry, or paid inference. Live authentication remains separately gated.

## Acceptance

- consumer-owned tool orchestration remains outside Swallowtail
- each further inference attempt requires an explicit consumer action
- provider-private reasoning never becomes portable reasoning or durable state
- local continuation cannot cross session, route, facade, model, access, or
  host boundaries
- provider cache is explicit and grants no deletion truth
- one locally continued direct session is distinct from structured run,
  connection continuation, provider conversation, and harness callback
- no provider, model, endpoint, facade, credential, billing, retry, tool,
  privacy, cache, topology, or fallback choice is implicit
