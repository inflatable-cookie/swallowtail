# 025 Provider-Owned Direct Conversation And Deletion Boundary

Status: active
Owner: Tom
Updated: 2026-08-05

## Purpose

Represent direct-inference interactive sessions whose context lives in one
provider conversation without making provider retention, load, deletion,
workspace choice, or a persistent network connection implicit. Keep the
operation-owned delete-on-close profile separate from a retained loadable
profile.

## Operation Shape

The operation is an `InteractiveSession` over `DirectModelInference`. The
provider owns conversation context but no agent loop, tools, workspace, or
harness lifecycle. Each turn may use a new HTTP/SSE connection; interactive
session shape does not require a connection-scoped transport.

The first session is resource-free. It carries no working resource,
filesystem policy, harness-isolation posture, callback exchange, or consumer
tool authority.

## Session Retention Profiles

Provider conversation retention is explicit at session open or load. The
request, requirements, capability manifest, and immutable preflight plan must
agree on one of two profiles.

The operation-owned profile requires:

- durable provider retention allowed for the session lifetime
- one driver-owned provider conversation
- delete-on-close authority for the conversation and its items
- no resume after local session close

The retained profile requires:

- `DurableProviderSessionPreserved`
- one exact resource-free `SessionResumeBinding`
- preservation of the provider conversation on ordinary attachment close
- bounded complete replay before a loaded handle becomes ready
- no delete-on-close authority

The profiles are distinct prepared operations. Neither can silently select or
fall back to the other. The existing structured-run `OperationPolicy` is not
reused as a session policy. Missing or mismatched retention, replay, or
deletion posture fails before endpoint, credential, or provider work.

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
references. They do not become consumer transcript ids, model routes, or
diagnostics. Only a successful retained open or exact retained load may place
the conversation reference inside the ordinary opaque resume binding.

That binding fingerprints configured instance, instance revision, execution
host, endpoint target, regional workspace audience, access profile and
policy, credential mechanism, facade and interface versions, deployment route
and revision, exact model, resource-free posture, provider-state policy, and
conversation identity. A copied id, list item, diagnostic, raw response, or
binding restored under drift grants no attachment authority.

## Access And Topology

Preflight binds the exact execution host, workspace-specific endpoint,
regional endpoint audience, access profile, credential mechanism, route, and
model before effects. Region, workspace, deployment scope, key, model list,
entitlement, metering, and support authority remain independent.

The session acquires one host-approved endpoint grant and one credential lease
for its full lifetime. Open or retrieve, every turn, replay listing, item
inventory, item deletion, and conversation deletion use that same scope and
audience. Network work and remote cleanup finish and join before awaited
credential release.

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

Retained attachment close instead stops and joins local work, releases access,
and preserves the provider conversation and its items. It reports no deletion
outcome. Preservation is not a guarantee of indefinite provider availability.

Destructive cleanup of a retained conversation is a separate Contract 038
operation. Successful retained open or load may issue a management binding for
that exact conversation. The persisted resume-binding record is not deletion
authority and cannot be relabelled as a management binding. Cleanup requires
an inactive target, complete bounded inventory, exact item-before-conversation
deletion, explicit consumer authorization, and separate uncertainty for item
and conversation outcomes. Cleanup-only restart from a resume record remains
unsupported unless management-binding persistence is separately qualified.

## Retained Load And Replay

Retained load performs these stages before returning readiness:

1. validate the resource-free resume binding against the exact prepared plan
2. acquire the same endpoint and credential posture
3. retrieve the exact conversation by opaque id and correlate the returned id
4. list items in ascending order with `limit=100`
5. follow only the returned last-item cursor while `has_more=true`
6. validate completed message identity, role, content shape, page bounds, and
   cross-page ordering
7. finish replay only after one final `has_more=false` page
8. return ordered `SessionReplayItem` values and one live continuation handle

The contracted adapter bounds one page at 100 items and 512 KiB, and the whole
load at 10 pages, 1,000 items, and 4 MiB of projected content. Empty
continuation pages, duplicate item ids, cursor non-progress, sequence overflow,
unsupported roles or content, incomplete items, mismatched first or last ids,
or excess bounds fail load and return no usable handle. Partial replay is not
readiness.

Missing or already-deleted conversations fail as missing without fallback.
Foreign or inaccessible conversations fail without converting access denial
into absence. Malformed provider data fails as protocol-invalid. A persisted
binding whose plan fingerprint drifted is stale and rejects before provider
work. Disconnect, deadline, cancellation, or ambiguous provider failure after
dispatch preserves uncertainty and returns no handle. No case creates a fresh
conversation, retries, changes workspace, region, endpoint, credential,
deployment, model, or retention profile.

Replay-free resume remains unsupported for the first retained profile. A live
continuation after successful load uses the retrieved conversation and the
same exact turn contract; it does not replay a prompt or infer interrupted-turn
state.

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

The realized operation-owned proof binds:

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

The retained contract additionally freezes evidence revalidated 2026-08-05:

- exact `GET /compatible-mode/v1/conversations/{id}` retrieval
- ascending bounded `GET /compatible-mode/v1/conversations/{id}/items`
  pagination using the last item as `after`
- strict user `input_text` and assistant `output_text` replay projection
- a separate preserved-state profile and separate explicit cleanup authority

Card 098 freezes this contract and deterministic corpus. It does not claim the
retained prepared operation is production-ready; implementation and route
truth remain cards 100-101.

The subsets exclude model catalogue, legacy DashScope and trial domains, other
regions, deployment-scope switching, Coding Plan, Token Plan, savings-plan
routing, aliases, previous-response continuation, stored-response retrieval or
deletion, replay-free conversation resume, metadata mutation, direct item
creation, provider tools, search, files, multimodal input, MCP, code execution,
context cache, background execution, retry, reattachment, and fallback.

## Conformance

Deterministic fixtures must prove:

- exact Singapore workspace, regional audience, API-key, route, model, and
  execution-host binding before effects
- explicit durable-retention and delete-on-close session policy
- explicit preserved-state retained policy with no delete-on-close
- exact resource-free resume-binding persistence and resource-free load request
- retrieve-before-list correlation and complete ordered bounded replay before
  readiness
- missing, deleted, foreign, stale, malformed, oversized, and uncertain load
  failures without fallback or usable handles
- retained close preservation and separately authorized cleanup
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
- retained and operation-owned profiles cannot widen or substitute for one
  another
- a persisted continuation binding cannot grant destructive cleanup
- provider response storage and context cache remain independently disabled
- conversation deletion cannot mask undeleted items
- local cancellation cannot claim native provider cancellation or remote stop
- no provider, region, workspace, endpoint, credential, model, billing,
  retention, retry, resume, or fallback choice is implicit
