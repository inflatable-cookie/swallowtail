# Provider Session Management And Consumer Thread Boundary

Status: active
Owner: Tom
Created: 2026-07-26
Updated: 2026-08-05

## Purpose

Allow a consumer to archive, restore, or delete one bound persistent provider
session without confusing that action with consumer thread state, runtime
attachment close, or driver-owned remote-resource cleanup.

## Independent Lifecycle Planes

A consumer thread and a provider session are separate identities.

The consumer owns:

- thread archive and restore state
- thread deletion and product retention policy
- messages, attachments, task links, memory, persistence, and UI
- user confirmation and whether a provider action accompanies a local action

Swallowtail owns only the selected provider-session mechanism. It never
archives or deletes consumer data.

Local success does not imply provider success. Provider success does not imply
local success. Consumers must persist and present both outcomes separately
when both actions were requested.

## Lifecycle Separation

The following remain distinct:

- runtime attachment close
- provider-active session close
- reversible provider archive
- provider restore or unarchive
- provider history-list removal
- provider-declared data deletion
- provider-declared hard deletion
- driver-owned remote-resource cleanup

`InteractiveSessionHandle::close` ends one runtime attachment and joins its
owned work. A driver may send a qualified provider-native close during that
cleanup. Native close frees active resources and preserves durable provider
state unless the exact provider contract says otherwise. It cannot report
archive or deletion.

Owned environment, managed session, conversation, item, background-run, and
serving cleanup remain governed by their existing operation contracts.
`OwnedRemoteResourceDeletion` cannot authorize deletion of a consumer-selected
persistent provider session.

## Provider Session Management Binding

Management requires an opaque durable binding containing:

- provider session reference
- adapter driver and transport or facade identity
- configured instance identity and revision
- execution host identity
- exact bound interface versions and compatibility assessment
- endpoint, executable, or service target identity where applicable
- access profile and safe provenance
- working-resource scope when the provider session is resource-bound
- binding origin and lifecycle capabilities proven for that exact route

The management binding is independent from load or resume support. A session
may be manageable without being resumable.

A binding may be returned by new, load, or resume, imported through a later
explicit consumer-authorized flow, or returned after one successful structured
run that created the exact durable provider session. A structured-run binding
is unavailable before terminal completion, is take-once, and is never returned
for failed, cancelled, timed-out, or temporary-cleanup runs. A raw provider id,
list result, diagnostic, provider payload, or copied string is not a management
binding and grants no authority.

Bindings remain opaque and safely redacted. They contain no credential,
account, prompt, transcript, endpoint secret, or raw provider payload.

## Management Plan

Archive, restore, and delete use side-effect-free preflight. One immutable plan
fixes:

- management binding
- exact action
- capability and compatibility evidence
- target state required before the action
- deletion strength promised by the selected route
- target-only or provider-defined descendant scope
- deadline and cancellation posture
- host services and access needed for the action

The first common role accepts only an inactive target. The caller closes its
runtime handle before management. Swallowtail does not use a global registry
to discover, steal, cancel, or detach active handles. A later active-session
management feature requires a separate contract.

Archive, restore, and delete are separate typed actions. There is no generic
state string, lifecycle toggle, or delete-on-close shortcut.

## Capabilities

Provider session management capabilities are independent:

- archive
- restore
- delete
- provider-native active close

List, load, and resume remain separate capabilities. Listing is not required
to manage a binding already held by the consumer.

An adapter advertises only the actions proven for the exact driver, transport,
facade, interface-version segment, configured instance, and access profile.
One provider's CLI, SDK, private REST route, application UI, or alternate
transport cannot authorize the selected driver.

Unsupported or unadvertised actions fail before provider effects. The
consumer can still complete its local thread action.

## Version Qualification

Contract 029 governs every lifecycle action.

- qualified segments carry guaranteed action and outcome semantics
- older supported segments remain usable when an action is absent
- behavior milestones record method introduction, semantic changes,
  deprecations, and exclusions
- capability negotiation may narrow an exact qualified or unverified attempt
- an exact permitted unverified-newer point may execute the latest mapped
  action with visible unverified status
- unverified execution does not extend the guaranteed range or strengthen
  deletion truth
- unordered, malformed, prerelease, explicitly excluded, or incompatible
  points fail before effects

Absence of a negotiated capability cannot be overridden by version
assumption. Presence of a method name cannot substitute for behavioral
qualification.

## Archive And Restore

Archive is reversible provider state that hides or relocates a persistent
session under the provider's exact documented semantics. It is not deletion.

Restore reverses that provider archive operation. It is not load, resume, or
consumer-thread restoration.

An archive or restore outcome records:

- exact target binding
- resulting provider lifecycle state
- whether the target was already in that state
- affected target or provider-defined descendant scope
- qualified or unverified execution status
- provider request and rate evidence when available

An adapter without a native reversible operation must not simulate one through
local metadata, delete-and-recreate, provider list filtering, or consumer
state.

## Delete

Delete plans declare the strongest semantics the selected route can honestly
report:

- `HistoryRemoved` — absent from the provider's user-facing history or list;
  retained data may still exist
- `ProviderDataDeleted` — the provider declares the session and its associated
  data deleted without an explicit hard-erasure guarantee
- `ProviderHardDeleted` — the provider explicitly guarantees hard deletion for
  the qualified route

A weaker result cannot satisfy a stronger plan. ACP `session/delete` defaults
to `HistoryRemoved` unless separately qualified provider evidence supports a
stronger claim.

Delete outcomes also record whether:

- the action was applied
- the provider treated an absent target as already deleted
- the action failed before the effect boundary
- the request crossed the effect boundary but final provider truth is
  unconfirmed

Provider acknowledgement, missing-from-list observation, data deletion, hard
deletion, and descendant deletion remain separate evidence. An adapter cannot
infer a stronger result from HTTP success, empty JSON, connection close, local
file absence, or a provider notification alone unless its exact contract makes
that evidence authoritative.

### Claude Agent ACP

The qualified Claude Agent ACP `0.53.0..=0.64.0` range advertises independent
close and delete capabilities at every supported point. Exact tagged handler,
test, ACP SDK, and Agent SDK evidence qualifies:

- native close for one active session; it preserves persistent history
- delete for active or inactive sessions
- `ProviderDataDeleted` for the harness-owned primary session transcript
- `ProviderDefinedDescendants` for the recursively removed sibling session
  directory
- provider rejection for missing and repeated close or delete

This classification does not claim secure erasure or deletion of Anthropic API
service data, account analytics, logs, or backups. It is not
`ProviderHardDeleted`. Exact `0.62.0` retains the prior behavior, while exact
`0.63.0` and `0.64.0` add no stronger lifecycle authority. Later stable
versions remain visible and unverified.

### OpenCode HTTP/SSE

The qualified attached OpenCode `1.14.48..=1.18.10` range exposes
`DELETE /session/{sessionID}` at every supported point. Exact tagged schema,
handler, removal-core, middleware, and test evidence qualifies:

- `true` after provider-declared session-data deletion
- `ProviderDataDeleted`, never `ProviderHardDeleted`
- `ProviderDefinedDescendants` because the provider recursively deletes child
  sessions before the selected target
- `404` provider rejection for a missing or repeated target; absence is not
  reported as already-deleted success
- optional server Basic authentication with `401` rejection when configured
- unconfirmed provider truth for 5xx, disconnect, cancellation, or deadline
  after dispatch

The provider route has no busy guard. Releases `1.14.51` and later add
background-job cancellation during deletion, but no supported release proves
safe active-handle management. Swallowtail therefore requires the bound
target to be inactive across the full range.

This classification covers OpenCode-declared session, message, history, and
provider-defined descendant data. It does not claim secure erasure, deletion
of provider API service data, account analytics, logs, or backups, or
ownership of the attached server. Stable releases newer than `1.18.10` remain
visible and unverified under Contract 029; they do not extend the guaranteed
range.

### Kimi Code Local Server

Kimi Code local server is a separate driver and transport from Kimi ACP. The
first claim targets exact executable releases `0.28.1` and `0.29.0`,
`kimi web --no-open`, local REST, and WebSocket protocol version `2`.

The route may qualify:

- reversible archive through
  `POST /api/v1/sessions/{session_id}:archive`
- reversible restore through
  `POST /api/v1/sessions/{session_id}:restore`

It does not qualify deletion. A deprecated provider schema name that aliases
the archive response is not delete evidence.

The configured instance binds one exact server endpoint, exact server metadata,
Kimi executable release, execution host, Kimi state-root resource identity,
and opaque bearer credential lease. Attached and owned-foreground server
topologies remain distinct. Owned execution starts one foreground child,
waits for health and exact metadata, and joins it on close. Attached execution
never stops the external server.

An ACP management or resume binding cannot be relabelled as a local-server
binding. One explicit consumer-authorized import may issue a new local-server
management binding only after proving:

- the exact Kimi release and execution host match
- both routes bind the same opaque Kimi state-root resource
- the authenticated server reports the expected configured-instance metadata
- the exact target session exists through the local-server route
- the requested local-server lifecycle capability is qualified

A provider family match, raw session id, list result, path, diagnostic, or
copied binding is insufficient. Import grants only the capabilities of the new
route. It does not grant load, resume, archive, restore, or delete authority to
ACP.

### Gemini CLI Stored Transcript

Gemini CLI ACP and headless remain separate transports. ACP does not advertise
ACP `session/delete`. The installed executable exposes a separate
`--delete-session <identifier>` control across the qualified
`0.51.0..=0.52.0` headless range.

The first Gemini management binding may be issued only after successful
terminal completion for a transcript whose session id was selected by that
Swallowtail headless run. `RunHandle::take_management_binding` returns it at
most once. It binds:

- the exact Gemini executable observation and compatibility assessment
- the headless driver and stored-transcript management role
- execution host and approved executable target
- exact working resource and project-scoped storage identity
- the Swallowtail-selected opaque session id

An ACP session id, list index, list output, copied string, filesystem path, or
arbitrary CLI argument is not a management binding. Cross-transport ACP import
is outside this claim. The opt-in temporary-cleanup profile never returns a
management binding.

The intended route outcome is `HistoryRemoved`, but the qualified
`0.51.0..=0.52.0` absence check is not side-effect free. The provider command
prints a success line after its storage helper returns, while storage cleanup
catches some local unlink failures. Exact source also shows that
`--list-sessions` runs summary generation before listing and may issue a model
request plus append summary or scratchpad metadata to retained transcripts.
It therefore cannot be described or used as read-only reconciliation.

Process exit, the success line, and a stateful list result are each
insufficient to confirm removal. Swallowtail therefore advertises no Gemini
provider-session management binding or `HistoryRemoved` outcome. The opt-in
operation-owned cleanup profile issues one exact delete attempt, performs no
list confirmation, and reports removal unconfirmed. A future confirmation path
must be explicitly qualified as side-effect free and must not expose the first
user message in stable diagnostics.

The exact route has no archive or restore claim. It requires an inactive
target. A target still present after the command is not deleted. Cancellation,
deadline, process loss, malformed output, or failed reconciliation after
dispatch leaves history-removal truth unconfirmed. No retry or direct
filesystem deletion is permitted.

## Destructive Authority

Swallowtail does not decide whether deletion should occur. The consumer:

- obtains user or workflow authorization
- chooses local-only, provider-only, or both
- orders its own persistence effects
- handles partial success and retry policy
- decides whether unverified-newer provider deletion is allowed

Swallowtail receives one explicit requested action. It performs no implicit
deletion during archive, close, load, resume, disconnect, upgrade, fallback,
garbage collection, or credential release.

Deletion never selects another provider, account, endpoint, executable,
transport, model, credential, or topology after failure.

## Cancellation, Deadline, And Effect Truth

Every management operation is scoped and joined.

- cancellation or deadline before dispatch produces no provider effect
- after dispatch, local cancellation does not prove provider cancellation
- a lost response after the effect boundary yields unconfirmed provider truth
- no retry occurs unless a later contract and exact idempotency evidence allow
  it
- all network, process, protocol, timer, resource, and credential work joins
  before the outcome returns
- access is released only after transport and cleanup work finish

An unconfirmed destructive outcome remains unconfirmed. A later observation
may reconcile it through a separately authorized operation; diagnostics cannot
silently promote it.

## Prepared Integration

Contract 037 applies.

An applicable adapter exposes:

- inspectable management evidence and plan
- typed archive, restore, or delete operations only where supported
- exact compatibility and capability status
- the unchanged low-level management role

There is no central provider-session router and no common `manage`,
`set_state`, or `send_prompt` method. Prepared facades may derive
adapter-fixed method, endpoint, capability, and deletion-semantic facts. The
consumer still supplies the target binding, requested action, deadline,
authorization decision, and any explicit unverified-newer acceptance.

## Route Applicability

The first applicable route set is:

- Codex app-server
- Claude Agent ACP
- OpenCode HTTP/SSE

Kimi Code local server is a separate applicable archive/restore route. Gemini
CLI stored-transcript management is a separate installed-executable route;
it does not change the unsupported Gemini ACP mapping. Kimi Code ACP remains
unsupported. The local-server and installed management routes cannot change
their ACP classifications.

Codex exec, Pi RPC, Qwen headless, direct one-attempt and locally continued
inference, realtime sessions, catalogues, SDK inference, and attached or owned
model runtimes have no user-managed persistent provider session under their
current contracts. Their consumer threads remain local-only.

Alibaba's operation-owned delete-on-close conversation and Anthropic Managed
Agent cleanup retain their driver-owned resource contracts. A separately
prepared retained Alibaba conversation may issue this user-directed delete
authority after successful open or load. Its management binding targets the
exact resource-free conversation and retains Contract 025's complete item-
before-conversation deletion semantics. Its persisted resume binding is not
management authority; cleanup-only restart from that record remains
unsupported. Production route truth remains gated on cards 100-101.

## Diagnostics

Stable diagnostics may include:

- safe action and stage
- driver, transport, capability, compatibility, and host dimensions
- target binding identity through redacted wrappers
- qualified, unverified, unsupported, before-effect, or after-effect state
- safe provider status, request, and rate evidence already permitted by the
  selected adapter

They never expose raw provider ids, paths, account identity, credentials,
prompts, transcripts, response bodies, or provider payloads.

## Conformance

Provider-neutral fixtures prove:

- consumer thread lifecycle is absent from Swallowtail records and effects
- arbitrary session ids cannot create management authority
- every binding and plan dimension rejects drift before effects
- archive, restore, delete, close, load, and resume remain distinct
- unsupported capability stops before dispatch
- inactive-target enforcement needs no global session registry
- qualified and unverified-newer evidence remains visible
- each deletion strength is preserved without promotion
- already-absent and applied outcomes remain distinct
- cancellation and deadline before dispatch cause no effect
- loss after dispatch returns unconfirmed truth
- no retry, fallback, provider substitution, or transport substitution occurs
- operation work joins before access release
- diagnostics stay bounded and redacted

Adapter fixtures additionally prove exact wire or SDK mapping, version
milestones, capability negotiation, descendant scope, authoritative
acknowledgement, idempotency behavior, and provider-specific deletion
semantics.

## Exclusions

This contract does not add:

- consumer thread persistence or UI
- provider history browsing, import, search, or synchronization
- provider session export, compaction, retention scheduling, or garbage
  collection
- active-handle discovery or a global session registry
- automatic provider deletion when a consumer thread is deleted
- automatic local deletion after provider success
- hard-erasure claims for soft-delete protocols
- provider, model, credential, endpoint, account, transport, or topology
  fallback
- direct filesystem deletion of provider state behind a harness interface

## Acceptance

- local and provider lifecycle outcomes cannot be confused
- close cannot masquerade as archive or deletion
- one bound inactive provider session is the only target
- exact capability and version evidence precedes effects
- deletion strength and descendant scope remain explicit
- unsupported routes preserve local-only consumer behavior
- unverified-newer execution remains allowed but visibly unguaranteed
- destructive uncertainty is retained
- all work joins and diagnostics remain safe
