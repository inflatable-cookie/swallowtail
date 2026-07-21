# 021 Provider-Owned Background Run And Temporary Retention Boundary

Status: active
Owner: Tom
Updated: 2026-07-21

## Purpose

Represent a provider-owned asynchronous inference operation whose lifetime can
outlast one network attachment without making provider storage, recovery,
cancel authority, or inference retry implicit.

## Optional Operation Posture

Provider-managed background execution is an optional policy on one structured
direct-inference run. It is not a new operation shape, an interactive session,
a harness loop, or a default for structured runs.

The request selects these dimensions independently before endpoint or
credential effects:

- attached or provider-managed background execution
- prohibited or explicitly allowed temporary provider retention
- disabled or positively bounded stream reattachment

A driver declares matching background, temporary-retention, and stream-
reattachment capabilities and exact reattachment limits. Preflight fails when
requirements and capabilities differ. Driver validation fails before effects
when the selected policy is not an exact supported combination.

Temporary retention permits only the provider storage required to execute and
observe the selected operation. It does not authorize durable conversation
state, consumer persistence, later cross-process resume, files, webhooks, batch
jobs, or another provider feature. `store=false` cannot be described as no
provider retention when the provider still retains background data.

## Separate Identities And Lifecycles

The following remain separate:

- consumer request and Swallowtail runtime run
- provider response or operation reference
- create, retrieve, stream-attachment, and cancel HTTP requests
- each local SSE attachment
- provider event cursor
- common runtime event sequence

The provider reference is opaque and adapter-owned. It does not become a
session-resume binding. The provider cursor is transport progress for one
provider operation; it is neither a runtime sequence nor durable consumer
state.

One structured-run start creates at most one inference attempt. Retrieve,
reattach, and native-cancel requests manage that attempt. They cannot recreate
it, replay input, select another route, or count as transparent inference
retry.

## Attachment And Reattachment

A provider background operation may continue after an SSE attachment closes.
Local connection loss is therefore not terminal while an authorized bounded
reattachment remains and the provider reference and last accepted cursor are
known.

Each reattachment uses the same configured instance, endpoint audience,
credential lease, access profile, model route, provider operation reference,
deadline, and runtime run. It starts strictly after the last accepted provider
cursor. Duplicate, missing, decreasing, mismatched, malformed, or unknown
semantic events fail closed. Projected runtime events receive their own common
monotonic sequence.

The first OpenAI proof permits exactly one reattachment. Exhaustion is a
transport failure with potentially continuing remote work, not permission to
create another response. No reconnect loop, backoff, polling loop, replay, or
fallback is implicit.

If the create stream disconnects before a valid provider reference is known,
Swallowtail cannot recover or prove remote stop. It returns safe unconfirmed-
remote-state evidence without exposing request content, credentials, raw
payloads, or guessed provider identity.

## Status And Terminal Truth

Provider status is observed evidence, not the runtime lifecycle itself.
Queued and in-progress remain non-terminal. Completed, incomplete, failed, and
cancelled provider states map explicitly to the one runtime terminal outcome.
Unknown or contradictory states fail closed.

A cancellation request and its eventual provider result are separate facts.
Terminal evidence distinguishes:

- `Confirmed`: the provider reported the operation cancelled
- `RacedWithCompletion`: completion became authoritative before cancellation
- `Unconfirmed`: local work stopped but provider cancellation could not be
  established

Confirmed cancellation maps to runtime cancellation. When operator
cancellation races provider completion, completion maps to runtime completion.
When a local deadline has already won, the runtime remains timed out and
records provider completion as separate race evidence. Unconfirmed remote stop
may map to local cancellation, deadline, or failure according to the local
winner, but cannot claim that the provider stopped. Repeated provider cancel
requests may be protocol-idempotent; Swallowtail still sends no more than the
explicit bounded cancellation work.

Drop is not provider cancel. Dropping a handle cannot report confirmed cleanup
or confirmed remote stop.

## Access And Cleanup

Preflight fixes the configured instance, public-API endpoint reference and
audience, API-key credential reference, API billing boundary, provider support
authority, execution host, model route, and exact model before effects.

The endpoint grant and credential lease remain bound to the runtime run across
create, retrieve, reattachment, and cancel requests. They cannot be reacquired
from ambient state or exchanged for ChatGPT, Codex, subscription OAuth,
community OAuth, another endpoint, or another billing authority.

On terminal state, cancellation, deadline, or local failure, Swallowtail stops
and closes owned network work, joins every scoped task, then releases the
credential lease. Clean local cleanup does not prove remote cancellation.
Remote work may remain unconfirmed after every local task has joined.

No global executor, detached poller, reader, timer, callback, credential task,
or cleanup task is permitted.

## First OpenAI Responses Subset

The first proof binds:

- the provider-supported OpenAI public Responses API
- `POST /v1/responses` with `background=true`, `stream=true`, and `store=false`
- `GET /v1/responses/{response_id}` for bounded status retrieval
- streaming retrieval with `stream=true` and
  `starting_after=<sequence_number>` for one reattachment
- `POST /v1/responses/{response_id}/cancel` for native cancellation
- public API-key access, API billing, the exact `api.openai.com` audience, and
  provider support authority
- one exact configured model and positive maximum-output-token bound
- one explicit operation deadline
- text input and output, status, usage, rate, and request-correlation evidence

Temporary provider retention is required even with `store=false`. The subset
excludes tools, search, files, conversations, durable response recovery,
webhooks, Batch API, attachments, structured output, retry, fallback, model
defaults, ChatGPT or Codex login, subscription OAuth, and live access from
default QA.

## Conformance

Deterministic dated fixtures and loopback HTTP/SSE tests must prove:

- exact create, retrieve, reattach, and cancel request shapes
- no endpoint or credential effect before successful policy validation
- separate provider reference, provider cursor, runtime run, attachment, HTTP
  request, and common event identities
- queued, in-progress, completed, incomplete, failed, and cancelled states
- one inference attempt and one maximum reattachment
- duplicate, gap, mismatch, unknown-event, malformed-event, and disconnect
  failure
- confirmed, raced, and unconfirmed provider cancellation truth
- cumulative usage replacement plus safe rate and request correlation
- endpoint, credential, provider id, cursor, content, header, and payload
  redaction
- joined network and task cleanup before credential release

Default QA uses no credential, provider account, external request, or paid
inference. Live authentication remains separately gated.

## Acceptance

- background execution, temporary retention, and reattachment are opt-in
- `store=false` never masks required temporary provider retention
- management requests cannot become inference retry or replay
- provider cancellation truth survives local terminal mapping
- no provider, model, endpoint, credential, billing, topology, storage, retry,
  or support-authority fallback is implicit
