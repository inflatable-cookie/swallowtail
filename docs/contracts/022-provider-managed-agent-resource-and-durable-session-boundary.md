# 022 Provider-Managed Agent Resource And Durable Session Boundary

Status: active
Owner: Tom
Updated: 2026-07-21

## Purpose

Define how Swallowtail may drive a provider-hosted agent harness whose remote
resources, state, retry behavior, and retention outlive one local network
attachment.

This contract does not authorize a consumer repository, provider filesystem,
external network, built-in tool, MCP server, skill, memory store, schedule, or
durable provider configuration mutation.

## Identity

The integration family, driver, transport, configured instance, access
profile, model route, model, provider agent definition, agent version,
environment, session, sandbox instance, persisted event, stream attachment,
runtime run, and common event sequence are distinct identities.

A provider agent definition is configuration, not a model route or runtime
session. An environment is a sandbox template, not the sandbox instance
created for one session. A provider session is not a Swallowtail runtime run,
network stream, consumer task, or resumable binding by implication.

Provider references remain opaque and redacted. Stable diagnostics expose no
remote identifier, event payload, prompt, tool input, file value, credential,
header, or provider error body.

## Ownership

Every provider resource has one explicit owner and cleanup authority.

- operator-owned resources may be retrieved and validated but are never
  created, updated, archived, or deleted by Swallowtail
- Swallowtail-owned ephemeral resources are created for one operation scope
  and deleted before that scope reports clean cleanup
- provider-owned internal resources may be observed only through the supported
  API and grant no mutation authority beyond the selected driver subset

An operator-owned versioned agent definition may bind the exact model and
consumer-owned configuration used by a run. The first proof never changes it.
The first proof owns one ephemeral environment and one session. Session
deletion precedes environment deletion. Deletion of either cannot affect an
independent agent, file, skill, vault, memory store, or other session.

## Retention And Deletion

Provider-managed durable retention is an explicit operation and capability
selection. It is distinct from prohibited retention and temporary background-
run retention. A driver that requires durable history or sandbox state rejects
any weaker policy before endpoint, credential, or provider effects.

Delete-on-close is part of the first driver contract, not a best effort. Clean
local cleanup requires supported provider confirmation that the owned session
and environment no longer exist. Failure, timeout, or ambiguity during delete
produces degraded cleanup and records remote removal as unconfirmed.

Deleting a runtime handle locally is not remote deletion. Drop is never proof
of interruption, deletion, or credential release. No durable cross-process
resume binding exists unless a later contract adds it explicitly.

## Provider-Managed Recovery

Provider-managed rescheduling or retry is separate from Swallowtail retry. A
driver must declare the capability and a request must accept it explicitly
before effects. Swallowtail still performs no implicit inference retry,
fallback, endpoint change, model change, or session replacement.

Provider statuses remain observed evidence. Running, idle, rescheduling, and
terminated cannot be flattened into local progress or success. A provider may
perform an unknown number of internal model attempts while rescheduling; the
driver reports only supported provider evidence and never invents an attempt
count.

## Event Authority And Recovery

Persisted provider events and connection-local previews are different
evidence. An authoritative event has a provider event identity and belongs to
one exact session. A best-effort preview may be partial, shed, unreplayable,
and absent from history. It cannot establish output or terminal truth.

The first proof consumes authoritative persisted events only. After stream
loss it performs one bounded history reconciliation for the same session,
deduplicates exact provider event identities, preserves provider order, and
reattaches without creating another session or task. Missing, duplicated,
foreign, contradictory, or unbounded history fails closed.

Stream loss does not imply provider stop. Event-history retrieval and stream
reattachment are management work for the same provider session, not retry.

## Callbacks And Authority

Declared custom tools use the existing bounded callback exchange. The
provider event id, callback id, runtime run, request, event sequence, and
deadline remain correlated and distinct. Swallowtail transports callback
requests and responses but never executes consumer tools.

A provider idle state with `requires_action` is a callback wait, not terminal
completion. Undeclared, unknown, duplicated, late, mismatched, or oversized
callbacks fail explicitly. Cancellation, deadline, or close abandons the wait,
interrupts supported remote work, rejects late responses, and continues owned
resource deletion.

Provider-built tools, provider filesystem tools, MCP tools, permission grants,
skills, multiagent delegation, and external network remain outside the first
subset.

## Access And Metering

Preflight fixes the execution host, endpoint reference and audience,
credential reference and mechanism, entitlement, metering authority, support
authority, provider agent id and version, model route, and model before
effects.

The endpoint grant and credential lease remain held across validation,
environment creation, session creation, event send, streaming, history
reconciliation, interruption, session deletion, and environment deletion.
They cannot be reacquired from ambient state or exchanged for subscription,
consumer login, another cloud marketplace, endpoint, or billing authority.

Token usage and provider-running time are separate observations. Local elapsed
time is not exact billed runtime. A driver emits exact provider-billed cost
only when the provider supplies exact cost evidence under Contract 016.

## First Claude Managed Agents Subset

The first proof binds:

- the first-party Claude API and `managed-agents-2026-04-01` beta authority
- one public API-key lease, API billing boundary, exact endpoint audience, and
  provider beta support authority
- one operator-owned agent id pinned to one exact version and model route
- one driver-owned cloud environment with limited network posture, no package-
  manager or MCP allowance, and no consumer file resource
- one driver-owned provider session and sandbox instance
- one structured text task, explicit deadline, and bounded optional custom
  tool declarations
- authoritative buffered event history, one bounded reconnect reconciliation,
  native interruption, cumulative token usage, safe rate and request evidence,
  session deletion, and environment deletion
- explicit durable-retention and provider-managed-rescheduling acceptance

The subset excludes provider agent creation or mutation, built-in tools,
provider filesystem authority, GitHub, files, vaults, MCP, skills, memory,
multiagent, outcomes, schedules, webhooks, preview deltas, external sandbox
network, cross-process resume, provider defaults, retry by Swallowtail, and
live access from default QA.

If the provider cannot create the exact limited environment without an
external host allowlist, the fixture gate stops production. It cannot widen to
unrestricted networking silently.

## Cancellation, Deadline, And Cleanup

Operator cancellation and deadline are distinct local winners. When a
provider session is running, the driver sends the supported interrupt and
observes the resulting authoritative state when possible. It then deletes the
session and environment regardless of interrupt confirmation.

Remote interruption, local task join, remote deletion, and credential release
are separate facts. Cleanup order is:

1. stop accepting new local input and callback responses
2. close active event input and network work
3. interrupt running provider work when supported
4. reconcile bounded authoritative terminal evidence when possible
5. delete the owned session and confirm removal
6. delete the owned environment and confirm removal
7. join every operation-scoped task and blocking worker
8. release the credential and endpoint leases

No global executor, detached poller, reader, timer, callback, deletion task, or
cleanup task is permitted.

## Conformance

Deterministic dated fixtures and loopback tests must prove:

- exact beta header, endpoint audience, API-key boundary, agent version, model
  route, environment posture, and exclusions
- no provider effect before exact policy and binding validation
- separate agent, version, environment, session, sandbox, event, attachment,
  request, runtime, callback, and common sequence identities
- running, idle end-turn, idle requires-action, rescheduling, terminated,
  provider failure, cancellation, deadline, and disconnect behavior
- authoritative event order, bounded history reconciliation, deduplication,
  foreign-event rejection, and no preview-as-output path
- declared custom callback round trip and late or mismatched rejection
- no second session or Swallowtail retry after disconnect or rescheduling
- cumulative usage, safe rate and request evidence, and redaction
- session-before-environment deletion and joined work before lease release
- degraded cleanup when interruption or either deletion is unconfirmed

Default QA uses no credential, provider account, external request, remote
sandbox, or paid inference. Live authentication remains separately gated.

## Acceptance

- durable provider retention and provider-managed recovery are explicit
- operator-owned and driver-owned provider resources cannot be confused
- authoritative history, preview, stream attachment, and runtime events remain
  separate
- remote deletion truth survives local cleanup projection
- the first proof grants no repository, provider filesystem, built-in tool,
  external network, MCP, skill, memory, schedule, or webhook authority
- no provider, model, endpoint, credential, billing, retry, topology,
  retention, support, or cleanup fallback is implicit
