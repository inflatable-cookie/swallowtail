# 025 Provider-Owned Direct Conversation And Deletion Boundary

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Represent a direct-inference interactive session whose context lives in one
driver-owned provider conversation without making provider retention, resume,
deletion, workspace choice, or a persistent network connection implicit.

## Operation Shape

The operation is an `InteractiveSession` over `DirectModelInference`. The
provider owns conversation context but no agent loop, tools, workspace, or
harness lifecycle. Each turn may use a new HTTP/SSE connection; interactive
session shape does not require a connection-scoped transport.

The first session is resource-free. It carries no working resource,
filesystem policy, harness-isolation posture, callback exchange, or consumer
tool authority.

## Session Retention Policy

Provider conversation retention is explicit at session open. The request,
requirements, capability manifest, and immutable preflight plan must agree on:

- durable provider retention allowed for the session lifetime
- one driver-owned provider conversation
- delete-on-close authority for the conversation and its items
- no resume after local session close

The existing structured-run `OperationPolicy` is not silently reused as a
session policy. Card 088 may realize the minimum provider-state session record
needed by this boundary. Missing or mismatched retention and deletion posture
fails before endpoint, credential, or provider work.

Provider conversation retention is not consumer transcript persistence,
provider response storage, a background run, a harness session, connection-
local continuation, or permission to keep the resource after close.

## Identity

These identities remain separate:

- configured instance and execution host
- endpoint reference, endpoint audience, region, and workspace
- access profile and credential reference
- model route and exact model id
- runtime interactive session and turn
- provider conversation
- provider conversation item
- provider response
- HTTP request, SSE attachment, provider sequence, and runtime event sequence

Provider conversation, item, and response ids remain opaque adapter-owned
references. They do not become consumer transcript ids, model routes,
diagnostics, or implicit resume bindings.

## Access And Topology

Preflight binds the exact execution host, workspace-specific endpoint,
regional endpoint audience, access profile, credential mechanism, route, and
model before effects. Region, workspace, deployment scope, key, model list,
entitlement, metering, and support authority remain independent.

The session acquires one host-approved endpoint grant and one credential lease
for its full lifetime. Open, every turn, item inventory, item deletion, and
conversation deletion use that same scope and audience. Network work and
remote cleanup finish and join before awaited credential release.

No legacy domain, region, workspace, key, route, model, billing plan, or
support-authority fallback is permitted.

## Turn Lifecycle

The first proof permits one active turn and two successful turns maximum. A
second concurrent turn or a third turn fails before another provider request.
Each turn binds the same provider conversation, exact model, configured
instance, access profile, and execution host.

One turn produces one synchronous provider inference attempt and one SSE
attachment. Provider sequence is validated independently from runtime event
sequence. Completed output must agree with assembled deltas and the returned
model must agree with the selected route. Usage is cumulative evidence for
that attempt.

The driver constructs only its frozen request fields. A provider's policy of
ignoring unknown compatible fields grants no authority to pass through
unsupported consumer inputs. Unsupported tools, reasoning modes, output
bounds, storage, cache, background, retry, or extension fields fail before
effects.

## Retention And Deletion Truth

Provider response storage, provider conversation storage, and context caching
are independent. The first proof explicitly disables response storage and
context caching while allowing the provider conversation to retain turn input
and output until close.

Conversation close must:

1. stop and join current local turn transport work
2. obtain one complete bounded inventory of conversation items
3. delete every discovered item and validate each confirmation
4. delete the provider conversation and validate its confirmation
5. join all cleanup work
6. release the credential lease

Deleting a conversation cannot stand in for item deletion. Aggregate
conversation-item deletion is `Confirmed` only when inventory was complete and
every discovered item returned exact confirmation. Conversation deletion is a
separate outcome. Missing, contradictory, partial, overflowed, or failed
inventory and deletion evidence remains unconfirmed or failed cleanup.

No detached retry, deletion sweeper, background janitor, or credential task may
survive close. Drop remains best-effort and cannot report confirmed deletion.

## Cancellation And Deadline

When the selected provider surface defines no native response cancellation,
turn cancellation or deadline closes local connection work and joins it. It
does not claim that remote inference stopped.

A remote response may race item inventory or deletion after local disconnect.
The driver attempts the same bounded cleanup but cannot report confirmed item
or conversation removal when late remote mutation remains possible. Runtime
turn status, remote-stop truth, item deletion, conversation deletion, local
cleanup, and credential release remain separate evidence.

Cancellation and deadline end the affected session in the first proof. The
driver cannot continue a conversation whose remote turn state is uncertain.

## First Alibaba Model Studio Subset

The first proof binds:

- official Model Studio evidence observed 2026-07-22
- one separately registered Alibaba Model Studio direct-conversation driver
- one operator-approved Singapore workspace-dedicated endpoint at
  `{WorkspaceId}.ap-southeast-1.maas.aliyuncs.com`
- the exact Singapore workspace and region endpoint audience
- one general Model Studio API key, pay-as-you-go metering, and provider
  support authority
- exact model route `qwen3.7-plus-2026-05-26` in the International deployment
  scope
- `POST /compatible-mode/v1/conversations` at session open
- maximum two `POST /compatible-mode/v1/responses` text turns with
  `conversation`, `stream=true`, `store=false`, `reasoning.effort=none`, no
  tools, and no session-cache header
- bounded `GET /compatible-mode/v1/conversations/{id}/items`
- exact item deletion before exact conversation deletion
- local and remote-authoritative execution-host conformance

The subset excludes model catalogue, legacy DashScope and trial domains, other
regions, deployment-scope switching, Coding Plan, Token Plan, savings-plan
routing, aliases, previous-response continuation, stored-response retrieval or
deletion, conversation resume, metadata mutation, direct item creation,
provider tools, search, files, multimodal input, MCP, code execution, context
cache, background execution, retry, reattachment, and fallback.

## Conformance

Deterministic fixtures must prove:

- exact Singapore workspace, regional audience, API-key, route, model, and
  execution-host binding before effects
- explicit durable-retention and delete-on-close session policy
- one provider conversation, serial turns, two-turn maximum, and no resume
- exact request field table and rejection of ignored or unsupported inputs
- provider sequence, reasoning exclusion, ordered text, completed-output
  agreement, returned-model agreement, usage, safe errors, and unknown events
- one inference attempt and one attachment per turn
- local-only cancellation, deadline, disconnect, and remote-state uncertainty
- bounded item inventory, item-before-conversation deletion, per-kind truth,
  and cleanup races
- endpoint, credential, workspace, conversation, item, response, raw event,
  prompt, output, and provider diagnostic redaction
- joined network and cleanup work before credential release

Default QA uses no Model Studio account, workspace, API key, external request,
conversation resource, or paid inference. Live authentication remains
separately gated.

## Acceptance

- direct interactive sessions do not imply one persistent connection
- provider conversation retention is explicit and preflight-bound
- provider response storage and context cache remain independently disabled
- conversation deletion cannot mask undeleted items
- local cancellation cannot claim native provider cancellation or remote stop
- no provider, region, workspace, endpoint, credential, model, billing,
  retention, retry, resume, or fallback choice is implicit
