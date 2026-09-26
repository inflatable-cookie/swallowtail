# 046 Provider Session Catalogue And Explicit Import

Status: active
Owner: Tom
Created: 2026-08-01

## Purpose

Allow a consumer to discover provider-owned harness sessions and explicitly
import one for ordinary load or resume without treating provider history as a
consumer database or granting authority from a raw provider identifier.

## Separate Identities

The following identities remain separate:

- consumer thread
- provider session
- catalogue observation
- catalogue candidate
- import operation
- durable session resume binding
- runtime session attachment
- provider-session management binding

A consumer may map one imported provider session to one local thread. That
mapping, its uniqueness rules, and its persistence remain downstream.
Swallowtail does not infer identity across consumers, accounts, configured
instances, drivers, transports, working resources, or provider families.

## Separate Roles

Provider-session catalogue and provider-session import are independent driver
roles and operation shapes.

Catalogue is read-only. It returns bounded candidates and pagination evidence.
It creates no provider state and grants no load, resume, archive, restore, or
delete authority.

Import revalidates one explicitly selected candidate and may issue the same
`SessionResumeBinding` used by an ordinary provider-created session. It sends
no model prompt, replays no history, and creates no consumer thread.

Load and resume remain governed by Contract 017. Provider archive, restore,
delete, and native close remain governed by Contract 038. One role cannot be
used as a fallback for another.

## Catalogue Plan

Side-effect-free preparation fixes:

- integration family, driver, transport, and protocol facade
- configured instance and exact target revision
- authoritative execution host
- access profile and safe provenance
- exact interface versions and compatibility assessments
- discovery scope
- page and entry bounds
- optional deadline and cancellation posture
- required host process, endpoint, credential, or resource services

The first common discovery scope is one exact configured instance and one
host-approved working resource. A consumer may request several independent
catalogues and merge their presentation downstream.

Configured-instance-wide, account-wide, state-root-wide, archived, descendant,
search, or cross-resource listing requires a separately advertised exact
scope. Absence of a scope cannot be widened by omitting a provider filter.

## Candidate Records

One candidate contains only bounded portable observations:

- operation-local opaque candidate identity
- provider-session reference held as redacted authority material
- configured instance, route, host, and working-resource association
- optional provider title or preview as explicit content
- optional provider update time
- optional lifecycle or activity observation
- exact source and compatibility evidence
- import availability and its reason

Title, preview, timestamps, and provider labels are provider content. They are
available only through the operation result, carry explicit bounds, and never
enter stable diagnostics by default. Raw paths, prompts, transcripts,
credentials, account identifiers, provider payloads, and arbitrary metadata
remain excluded.

The provider-session reference may be retained privately inside an opaque
candidate. Its `Debug`, display, diagnostic, and serialized public forms stay
redacted.

## Pagination And Snapshot Truth

Catalogue pagination preserves provider ordering only when the exact route
documents it. A cursor is opaque, scoped to one prepared catalogue, and
bounded in size. It cannot be reused across drivers, instances, hosts,
resources, filters, versions, or access profiles.

Each page is a snapshot observation. It does not promise a complete account
history, stable ordering, absence of concurrent change, or continued target
existence. Duplicate candidates within one bounded traversal are rejected or
reported explicitly; silent deduplication cannot invent completeness.

Cancellation or deadline stops the current catalogue operation and joins its
work. It does not delete or modify provider sessions. Partial pages are not
returned as complete success.

## Explicit Import

Import requires an exact candidate produced by a compatible prepared
catalogue plus explicit consumer selection. Side-effect-free import preflight
binds:

- candidate identity and source catalogue
- integration, driver, transport, facade, configured instance, and target
- execution host and access profile
- interface version and compatibility assessment
- model route and model selected for the future attachment
- exact working resource and session access policy
- provider-state and harness-configuration posture
- load and resume capabilities required after import
- deadline, cancellation, and host-service requirements

Execution performs only the qualified read-only provider checks needed to
prove the candidate still exists and matches the planned attachment. A route
may use exact lookup, metadata read, or equivalent provider evidence. It may
not inspect provider storage directly when an integration interface exists.

Successful import returns one opaque `SessionResumeBinding` with an explicit
import origin. It does not return a credential, raw provider id, path, prompt,
transcript, endpoint secret, or provider payload.

Candidate disappearance, staleness, resource mismatch, access drift, version
drift, model incompatibility, missing replay support, or failed lookup returns
no binding. Import has no retry, provider fallback, transport substitution,
account substitution, model fallback, or working-resource fallback.

## Load, Replay, And Continuation

Import alone does not create a usable interactive handle. The consumer passes
the returned binding to the exact prepared load or resume operation.

The first import profile requires load with ordered historical replay so a
consumer can build its initial local projection. Replay retains Contract 017's
phase, sequence, bounds, correlation, and failure rules. A route with only
resume may be discoverable but cannot satisfy this first profile.

After successful load, subsequent continuation uses the ordinary session
handle and resume binding. Import does not change model, access, isolation,
callback, tool, working-resource, retention, or cleanup authority.

## Consumer Ownership

The consumer owns:

- whether provider sessions are shown
- user selection and confirmation
- creation of a local thread
- message, activity, attachment, title, and task persistence
- replay merge, deduplication, and conflict policy
- local-to-provider mapping and uniqueness
- presentation of stale, active, unsupported, or failed candidates
- any later refresh or explicit re-import

Swallowtail does not automatically insert catalogue results into a consumer
database. It does not watch provider storage, poll in the background, merge
histories, select a preferred copy, or delete either side after import.

## Concurrent And Changed Sessions

An optional provider activity state is evidence, not a lock. A route may
reject import or attachment when it authoritatively observes active work. A
route without that evidence must not claim exclusive ownership.

Swallowtail has no global handle registry and does not steal, detach, cancel,
or coordinate another client. Concurrent external continuation can make a
consumer projection stale. Refresh, reconciliation, and user conflict policy
remain explicit later work.

## Version And Capability Truth

Contract 029 applies independently to catalogue, lookup, replay, load, and
resume behavior. A route advertises import only when one exact compatible
segment proves the complete selected chain.

Method presence, optional capability advertisement, current documentation,
or a newer package cannot extend a historical claim. Unverified-newer
execution remains visible and requires the same explicit acceptance posture as
the underlying route; it does not strengthen completeness or history truth.

ACP `session/list` remains optional. Stable ACP wire support cannot qualify an
agent that omits listing or whose load/replay behavior is not independently
qualified.

## Failure, Diagnostics, And Cleanup

Failures distinguish:

- unsupported catalogue or import capability
- incompatible or unverified interface
- access or host-service rejection
- malformed or oversized provider data
- invalid or expired cursor
- missing or changed candidate
- resource, route, model, or policy mismatch
- provider rejection
- cancellation or deadline
- transport or process failure
- cleanup degradation

Diagnostics use stable codes and bounded sanitized detail. Candidate titles,
previews, raw ids, cwd values, transcripts, provider bodies, stderr,
credentials, and state paths are absent unless an exact existing safe-content
contract permits them.

All readers, child processes, connections, cursor state, resource leases,
endpoint grants, and credential leases are joined or released in owner order.
Catalogue and import completion never hides cleanup failure.

## Conformance

Common deterministic fixtures prove:

- catalogue preparation is effect-free
- exact scope, page size, total candidates, cursor bytes, and content bytes are
  bounded
- unsupported capability fails before provider work
- candidate and cursor identity reject every plan dimension mismatch
- malformed, duplicate, oversized, and cross-scope results fail closed
- titles and previews never enter diagnostics
- a raw provider id or copied candidate cannot load or resume
- import revalidates before issuing one binding
- stale, missing, active-when-forbidden, or mismatched candidates issue none
- successful import binds exact route, host, access, version, model, resource,
  and policy
- load replay follows import and completes before readiness
- resume emits no replay
- cancellation, deadline, disconnect, and cleanup leave no detached work
- no consumer persistence, background synchronization, provider mutation, or
  implicit lifecycle action occurs

Adapter fixtures additionally prove exact list, lookup, history, load, resume,
pagination, filter, ordering, and activity-state behavior at every qualified
version milestone.

## First Production Tranche

The selected proof order is:

1. Codex app-server, using exact thread list/read/resume behavior
2. stable ACP records plus Kimi Code ACP exact list/load/resume behavior
3. OpenCode attached HTTP using exact session, message, load, and resume routes
4. provider-wide classification and a consumer adoption handoff

Kimi local server, Claude Agent, Cursor, Pi, Qwen, Antigravity, Gemini, Grok,
and direct routes gain no capability from this contract. They require their
own complete evidence.

## Exclusions

This contract does not add:

- a Swallowtail or global session database
- consumer thread persistence, UI, routing, merge, or deduplication
- automatic import or local-thread creation
- continuous polling or bidirectional synchronization
- account-wide discovery by default
- ambient filesystem or provider-state scanning
- arbitrary raw-id load or resume
- active-handle discovery, locking, stealing, or cancellation
- provider archive, restore, delete, fork, rename, export, or compaction
- provider-session management-binding persistence
- any persistence form other than Contract 017's separate ordinary resume-
  binding record
- provider, account, credential, endpoint, model, transport, or topology
  fallback

## Acceptance

- catalogue observation is not attachment authority
- import is explicit, read-only, and revalidated
- load and resume remain separate after import
- consumer and provider persistence remain separate
- exact resource and access binding precedes attachment
- provider content stays bounded and out of diagnostics
- unsupported routes fail before provider work
- no route gains support from another transport or provider family
