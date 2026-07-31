# Observable Agent Activity And Disclosure

Status: active
Owner: Tom
Updated: 2026-07-31

## Purpose

Expose provider-visible intermediate agent activity through the existing
ordered operation stream without exposing hidden reasoning, flattening route
differences, or moving consumer presentation and persistence into
Swallowtail.

This contract applies to structured runs and interactive turns. Direct
inference, harness execution, realtime media, catalogue operations, and model
serving retain their separate operation shapes.

## Separate Concepts

The following remain independent:

- provider-native event
- portable runtime event
- observable activity
- assistant message
- final operation output
- provider-visible reasoning summary
- hidden provider reasoning
- callback or direct-tool exchange
- consumer transcript
- consumer work-log projection
- raw provider payload

An ordered stream does not imply rich activity. A tool-capable route does not
imply harness-owned tool visibility. An interactive session does not imply
provider item lifecycle. A direct model response does not imply agent work.

## Transport Boundary

Run and turn handles continue to expose one event stream. No second observable
operation, global event bus, detached recorder, or consumer-specific facade is
introduced.

`StreamingEvents` continues to mean ordered bounded delivery. A separate
observable-activity capability and prepared activity profile describe semantic
fidelity.

Terminal outcome, provider stop truth, callback settlement, event-stream
completion, and joined cleanup remain separate under Contracts 009 and 012.

## Activity Identity

Each portable activity has:

- one non-empty operation-local activity id
- one runtime run or turn owner
- the event sequence of each observation
- one activity kind
- an optional bounded provider-intended display label
- optional opaque provider item reference
- optional callback, request, or direct-tool correlation

The same native item maps to the same activity id for its whole observed
lifecycle. Provider item references are opaque, bounded, redacted in default
formatting, and never accepted as cross-route authority.

An adapter may mint a runtime activity id when the provider supplies no stable
item id. Minting identity does not invent provider lifecycle or continuity.

The display label is not activity identity and may refine during one native
item's lifecycle. When a later update omits it, an adapter that observed an
earlier label carries the latest non-empty value forward. Labels are bounded,
redacted by default, and permitted only within provider-display or
adapter-summary disclosure. They are never reconstructed from activity
content. Consumers may replace them with product presentation, but do not
split payload content to recover a provider label.

## Activity Kinds

The portable vocabulary covers:

- assistant message
- reasoning summary
- plan
- command execution
- file change
- provider-owned tool call
- consumer-owned tool call
- external search
- image view
- subagent or collaborative-agent action
- review transition
- context compaction
- task
- hook
- warning or error activity
- bounded namespaced unknown activity

Provider-specific categories remain visible through exact typed detail or a
bounded namespace. They are not forced into an unrelated common kind.

Catalogue, discovery, serving lifecycle, access, rate, quota, usage, billed
cost, and general provider observations remain their existing typed evidence.
They do not become agent activity merely to fill a timeline.

## Lifecycle Fidelity

Activity observations may be:

- started
- updated
- completed

Each route profile declares one exact fidelity per supported activity kind:

- complete lifecycle
- update and completion
- completion only
- unavailable

The adapter emits only phases supplied or safely derived from the selected
documented interface. A completion-only source emits no synthetic start. A
single terminal item emits no invented intermediate updates.

Within one activity:

- sequences increase
- status cannot regress
- completion occurs at most once
- later deltas after completion fail closed
- foreign, contradictory, malformed, or uncorrelatable identity fails closed

Provider-authoritative replacement snapshots may replace earlier state only
when the exact interface documents that behavior. Deltas and replacement
snapshots remain distinct.

## Task-List Snapshots

A plan or task activity may carry one typed task-list replacement snapshot in
addition to readable activity content. This is the portable sidebar surface;
it is not plan-mode state and does not grant Swallowtail task execution or
persistence authority.

Each snapshot contains an ordered bounded collection of:

- non-empty task content
- pending, in-progress, or completed status
- optional high, medium, or low provider priority

The owning activity id is the list identity. Individual items have no portable
identity because current qualified interfaces do not consistently supply one.
Consumers replace the whole displayed list on every snapshot and may use
position and content only as presentation hints, not durable identity.

An empty snapshot authoritatively clears the list. Omission means no task-list
observation; it does not clear prior state. Snapshots may originate from plan
updates, todo tools, or another exact structured task source, but only
`Plan` and `Task` activity may carry them. Free-form plan text, tool display
content, and labels are never parsed to manufacture a list.

Task content is bounded operation data and redacted by default. Status and
priority are portable metadata. A prepared kind profile explicitly declares
task-list snapshot support; ordinary plan or task activity alone does not
imply it.

## Content Streams

Portable activity content distinguishes:

- intermediate assistant text
- final-answer text
- readable reasoning-summary text
- plan text
- command output
- file-change output or diff
- provider tool display content
- normalized activity summary

Every delta names its owning activity and content stream. Independent
assistant messages retain separate identities and phases. Final operation
output remains explicit even when it is also the completed final assistant
message.

When a qualified older interface supplies an assistant item without classifying
it as commentary or final answer, the adapter retains a provider-unspecified
assistant phase with identity-only disclosure. It does not guess a content
stream or finality.

Content is bounded operation data, not a safe diagnostic. Default `Debug`,
`Display`, diagnostics, preparation evidence, and provider observations redact
it. Consumers must treat it as potentially sensitive task data. An adapter
truncates qualified display content at its declared activity-content bound on a
valid text boundary. Oversized display content does not make an otherwise
well-formed provider update malformed. Transport frame bounds remain separate
from activity-content bounds.

## Tool And Request Correlation

Consumer callbacks and direct-tool continuations retain their existing
bounded opaque exchanges.

Observable activity may carry:

- the matching callback, request, or direct-tool id
- documented tool identity and status
- bounded provider-intended display arguments or result content
- bounded command output, exit status, and duration when supplied
- an adapter-normalized safe summary

It must not duplicate raw callback bodies, credential material, authorization
headers, endpoint secrets, or an uninterpreted provider envelope.

A provider-request correlation retains the portable request reference's
qualified scalar representation. A text id and signed-integer id with equal
visible content do not identify the same request. Activity projection uses the
same representation-aware reference as callback admission and resolution; it
does not maintain a weaker display-only identity.

Standardized field names do not make uninterpreted content safe. ACP
`rawInput`, `rawOutput`, and untyped `_meta` remain excluded unless the exact
adapter qualifies and maps a bounded typed subset. ACP tool content
collections are replacement snapshots, not append-only deltas.

Harness-owned tools may be observed without becoming consumer-executed tools.
Consumer-executed tools may be correlated without granting the harness
execution authority. Approval, question, tool invocation, tool result, and
provider completion remain separate states.

## Reasoning Disclosure

The portable surface exposes only provider-intended readable reasoning
summaries or thought updates.

It does not expose:

- hidden chain-of-thought
- raw reasoning blocks not intended for client display
- provider-private continuation state
- model scratchpads
- undocumented internal traces

The portable kind is `ReasoningSummary`, never a claim of complete reasoning.
A provider-visible thought chunk may map to this kind only when the qualified
interface defines it as client-display content.

ACP `agent_thought_chunk` is a client-facing display channel, but the shared
protocol decoder does not decide its portable activity kind. An exact adapter
must distinguish readable model thought from provider use of the same channel
for warnings or other display text.

ACP plans are authoritative full replacements. ACP mode, command,
configuration, session-info, and usage updates retain their existing typed
metadata or evidence roles and do not become agent activity.

## Disclosure Strength

Each activity kind in a prepared route profile declares one maximum disclosure
strength:

- provider display content
- adapter-normalized summary
- identity and lifecycle only
- unavailable

The profile is a maximum, not a promise that every activity contains content.
The same applies to optional labels. No route upgrades itself from summary to
provider content because a newer unverified event happens to contain more
fields.

Raw provider envelopes remain adapter-private. Provider extensions may be
used internally for round-trip behavior but are not a stable transcript or
diagnostic surface.

## Prepared Route Profile

Every prepared structured run or ordinary interactive turn with runtime
streaming events exposes an immutable activity profile derived from:

- exact adapter and driver
- operation shape
- selected transport
- observed interface version and compatibility segment
- selected interface options that change emitted activity, including explicit
  partial-message or preview-event flags
- activity kinds
- lifecycle fidelity
- content stream kinds
- disclosure strength
- correlation support
- typed task-list replacement support by activity kind
- exact child-work observation and provider collaboration-action fidelity
- unknown-event posture

Preparation requires no consumer enumeration of native provider event names.
The profile is evidence, not a request to make unsupported events appear.
An opt-in partial or preview stream is a different prepared behavior profile.
A route that does not select the required option cannot claim its lifecycle or
disclosure merely because the installed interface can emit it.

A consumer may require exact activity constraints before effects. Missing
constraints fail at preflight. A consumer may also accept a thinner profile
and render only the events actually supplied.

## Unknown And Newer Events

Unknown semantic provider activity must not silently become a coalescible
snapshot or empty generic progress event.

When identity and semantic classification are safe, the adapter emits one
bounded namespaced unknown activity with no raw payload. When safe identity,
ordering, or bounds cannot be established, the operation fails closed.

Contract 029 applies:

- guaranteed profiles bind qualified version milestones
- maintained ranges may contain several activity-schema segments
- unverified-newer execution remains allowed only where already authorized
- unverified-newer admission does not widen the guaranteed activity profile
- new fields may be ignored only when the exact decoder permits additive
  fields without semantic loss

## Direct And Realtime Boundaries

Direct inference may expose assistant, reasoning-summary, consumer-tool, and
provider-tool activity when the selected API supplies them. It must not invent
commands, files, subagents, plans, or harness work.

Attached local runtimes follow the same rule. Local compute does not create a
harness activity claim.

Realtime-media response events remain under Contract 026. Shared provider
observations and explicit tool activity may be correlated where qualified,
but audio, transcript, input commit, interruption, and rollover are not
flattened into ordinary text activity.

Catalogue and serving-only operations expose no agent-activity profile.

## Consumer Ownership

Consumers own:

- durable message and activity persistence
- transcript reconstruction
- grouping and collapsed presentation
- work-log labels and summaries
- disclosure UI and user preferences
- authorization and review policy
- retention, archive, and deletion of consumer records
- product analytics and telemetry

Swallowtail owns portable event identity, ordering, bounds, redacted
formatting, route fidelity, and adapter projection. It does not store a
consumer chat transcript.

## Conformance

Deterministic fixtures prove:

- monotonic event and per-activity lifecycle ordering
- stable activity identity and exact owner
- no synthetic lifecycle phases
- separate intermediate and final assistant content
- reasoning-summary disclosure without hidden reasoning
- tool, callback, request, and result correlation
- command, file, plan, task, hook, and subagent detail where claimed
- typed task-list replacement, clearing, bounds, status, and priority
- exact route activity profile and preflight requirements
- option-dependent partial and preview fidelity
- completion-only and unavailable fidelity
- unknown semantic event preservation or safe failure
- unverified-newer admission without profile widening
- bounded content and redacted formatting
- no raw provider payload in diagnostics or public evidence
- unchanged terminal, cancellation, deadline, and joined-cleanup truth

No live authentication, paid inference, installed harness, attached runtime,
or provider effect runs in default conformance.

Contract 045 defines the child-work graph, attribution, and operator-control
boundary carried by subagent or collaboration activity.
