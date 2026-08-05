# Input, Callback, And Provider-Tool Admission

Status: active
Owner: Tom
Created: 2026-07-28
Updated: 2026-07-31

## Purpose

Admit finite attachments, consumer tool exchange, provider approval or
question requests, and provider-owned search without merging their transport,
execution authority, operation shape, or lifecycle.

This contract does not create a generic content part, tool, callback, or
network API.

## Independent Features

The five portable features remain independent:

- `Attachments` transports finite consumer-approved operation input
- `ToolCalls` transports declared consumer tool calls and exact results
- typed harness user input transports losslessly representable questions
- namespaced provider extensions transport exact approval and richer
  provider-specific request semantics
- `ExternalSearch` enables one qualified provider-owned search tool

Support for one does not imply another. Provider branding, compatible JSON,
an agent loop, MCP configuration, a media-capable model, or provider network
access cannot supply a missing capability.

Each requested feature appears in the immutable plan with exact constraints.
Request, prepared operation, plan, dispatch, callback exchange, terminal
outcome, and cleanup agree before the feature can be claimed.

## Finite Attachment Admission

Contract 010's portable attachment descriptor remains safe metadata plus an
opaque host reference. It is not a path, URL, model artifact, working
resource, or realtime-media chunk.

An adapter keeps these states separate:

1. requested — the consumer supplied a descriptor and opaque reference
2. planned — preflight authorized count, media, size, representation, and
   model support
3. materialized — the host returned one operation-scoped lease
4. dispatched — the adapter encoded the exact provider or harness input
5. accepted — the selected surface did not reject the input
6. released — operation-scoped materialization or upload cleanup completed

Materialization does not prove provider acceptance. Provider acceptance does
not prove that the model used or understood the input.

Inline bytes, base64, data URLs, provider uploads, and temporary files are
different representations. The route corpus fixes the allowed conversion.
Drivers cannot borrow a representation from a sibling route or accept an
arbitrary client URL.

Media type, declared length, actual materialized length, count, digest when
present, model route, and version milestone are checked before dispatch.
Mismatch, unsupported media, overflow, partial materialization, provider
rejection, or failed release remains explicit.

Continuous image, audio, or video chunks stay under Contract 026. A realtime
media stream cannot be relabelled as a finite attachment.

## Consumer Tool Exchange

Three mechanisms remain distinct:

- native client tools — declarations cross the selected provider transport;
  the consumer executes calls and returns exact results
- provider-owned tools — the provider executes a named server tool
- MCP or harness tools — an agent discovers tools through separately
  configured protocol servers

Only native client tools satisfy `ToolCalls` directly. MCP may qualify through
a later operation-scoped bridge contract. Ambient or user configuration does
not become a portable callback path.

Contract 012 governs inline harness callbacks. Contract 030 governs direct
model continuation: each tool result is an explicit consumer action that
authorizes one further inference attempt. A tool call returned by a
one-attempt structured run is output evidence, not exchange support.

Tool name, declaration, input schema, call id, arguments, result, owning
operation, turn, attempt, count, size, deadline, and terminal state remain
separate. Swallowtail validates transport and correlation. It never validates
business meaning, executes the tool, chooses a result, or retries it.

Mixed native and provider-owned tools retain their execution source per call.
A provider-owned search result cannot be submitted through the consumer tool
result port. A client tool call cannot be treated as provider-executed merely
because both appear in one response.

## Approval And Question Requests

Approval and permission requests are namespaced provider extensions under
Contract 012. Losslessly representable questions use the common typed harness
user-input callback. Richer question schemas remain namespaced provider
extensions. Neither is a consumer tool.

Every exact extension declares one handling strength:

- reject
- observe and stop
- exchange a one-shot response

One-shot permission and persistent permission remain distinct provider
semantics. The common question model preserves ordered stable ids,
single-choice, multi-choice, free text, optional other input, secret text,
skipped answers, and optional auto-resolution timing. A driver exposes only
the qualified subset. It never widens one-shot permission into a persistent
rule, invents an option missing from the provider request, or flattens a
provider question that the common model cannot represent losslessly.

Opaque provider or consumer context is not inferred from header, prompt,
description, option label, tool id, or prose. A common context field requires
one exact end-to-end source and preservation proof. ACP `_meta` availability
alone is insufficient. The first Claude form route has no such proof:
claude-agent-acp does not forward arbitrary `AskUserQuestion` metadata or
question context. Option preview is separate presentation content and keeps
that form outside the qualified common subset.

The callback record preserves runtime operation, turn, provider request id,
its qualified scalar representation when supplied by the provider, callback
id, deadline, and either the exact provider namespace and bounded payload or
the typed question schema. Text and signed-integer forms with the same visible
value remain distinct correlations. The response repeats the exact correlation
and is accepted once. Typed answers are checked against the offered question
and option ids before translation.

The consumer chooses the response. Swallowtail transports it. Transport
acceptance does not prove provider action, tool execution, turn completion, or
durable configuration mutation.

Unknown, foreign, duplicate, late, oversized, malformed, or abandoned
responses never reach provider wire. Cancellation, deadline, terminal
failure, or close abandons pending requests before cleanup completes.

## Provider-Owned External Search

External search is a provider-owned tool selected by
`ExternalSearchPolicy::Enabled`. It is not arbitrary host networking, web
fetch, browser control, consumer tool execution, or a sibling product's search
API.

The route plan binds:

- exact provider tool name and revision
- selected model support
- provider-side network policy
- organization or account enablement when required
- access and billing authority
- positive query or use bounds
- returned usage, citation, and failure evidence when available

Provider endpoint access alone does not enable search. Search remains disabled
by default and cannot be silently added because a prompt appears current or
the model may benefit from it.

Dispatch, provider acceptance, tool invocation, result delivery, citation
delivery, usage, and billing remain separate evidence. A model may decline to
search. That is not adapter failure. Provider rejection, disabled organization
policy, unsupported model, quota, or tool revision drift remains explicit and
does not trigger fallback.

## Model, Version, Access, And Topology

Attachment media, client tools, provider requests, and search can change at
independent version milestones. Contract 029 applies without splitting the
integration family or hard-denying allowed unverified-newer attempts.

Guaranteed support follows the exact qualified baseline, milestones,
exclusions, and latest point. Unverified-newer use retains the latest
qualified mapping as best-effort behavior and cannot extend the guarantee.

Model catalogue presence does not prove input, tool, or search support. The
selected route requires exact model capability evidence when upstream behavior
is model-conditioned.

Endpoint audience, credential mechanism, entitlement, billing, provider-side
network, execution host, and support authority remain independent. A hosted
provider tool does not grant network authority to a local harness. An attached
harness cannot mutate shared configuration to gain a feature unless a
separate host-scoped configuration contract permits it.

## Selected First Tranche

The first implementation covers:

- Pi RPC `0.80.10` image attachment input
- OpenCode HTTP `1.14.48..=1.18.10` file-part input plus one-shot permission
  and question exchange
- Anthropic Messages `2023-06-01` image input, client-tool continuation under
  Contract 030, and provider-owned web search

The separate Oh My Pi `17.2.9` route carries one bounded PNG and exact typed
extension-UI question exchange. Its selected read-only tool set includes
`ask`; the question remains an exactly-once consumer callback. Write-capable
tools are absent, so OMP permission prompts are not admitted in this tranche.
`--approval-mode always-ask` is a deny-by-absence safety posture, not ambient
approval or a permission-exchange claim.

Anthropic tool continuation adds a separately prepared interactive role. It
does not widen the existing one-attempt structured role.

OpenCode exposes only `once` or `reject` permission responses in this tranche.
Persistent `always` permission remains unsupported. Question answers preserve
the provider's ordered question and selection structure.

## Failure And Cleanup

Stable diagnostics distinguish:

- unsupported capability, model, media, tool, extension, or search revision
- request, plan, lease, dispatch, callback, or result mismatch
- size, count, schema, or deadline overflow
- provider rejection, disabled policy, quota, or protocol drift
- cancellation, timeout, local transport failure, and uncertain provider work
- attachment, connection, task, or credential cleanup failure

Diagnostics expose no attachment bytes, filename, path, prompt, tool
arguments, result, question answer, search query, search result, raw provider
payload, credential, endpoint, or provider-private continuation.

Cleanup order remains route exact. Pending callbacks close before transport
and task join. Operation-scoped attachment material releases after provider
work. Hosted connection work joins before credential release. No reader,
timer, callback, upload, credential, or cleanup task detaches.

## Conformance

Deterministic fixtures prove:

- exact feature and constraint agreement before effects
- attachment media, representation, count, size, dispatch, and release
- native client versus provider-owned tool execution source
- explicit further-attempt authorization for direct tool results
- provider extension namespace, typed question ids and options, correlation,
  and one-shot response strength
- duplicate, late, foreign, malformed, cancelled, and timed-out responses
- explicit provider search policy, tool revision, model and account rejection,
  citations, usage, and billing separation
- version milestones, unverified-newer posture, safe diagnostics, and joined
  cleanup in local and remote-authoritative host topologies

Live credentials and provider requests remain separately gated and are not
required by default QA.

## Acceptance

- attachments remain finite host-mediated inputs
- realtime media, working resources, model artifacts, and attachments remain
  separate
- Swallowtail never executes consumer tools or chooses provider responses
- provider-owned tools never masquerade as consumer callbacks
- one-attempt structured inference never claims a tool-result loop
- persistent permission is never inferred from one-shot exchange
- search requires explicit provider network, access, model, and tool evidence
- version and model capability cannot be inferred or silently flattened
- every terminal path abandons callbacks and joins cleanup
