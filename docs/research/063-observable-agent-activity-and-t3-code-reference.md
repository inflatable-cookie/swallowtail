# 063 Observable Agent Activity And T3 Code Reference

Status: promoted
Owner: Tom
Date: 2026-07-29

## Question

What must Swallowtail expose so consumer applications can render the
provider-visible steps of an agent run without owning provider parsing,
exposing hidden reasoning, or moving consumer presentation and persistence
into the library?

## Method

Evidence was accessed on 2026-07-29.

- inspected Swallowtail runtime events, handles, callback exchanges,
  capability profiles, conformance packs, prepared facades, and every
  production adapter event projection
- checked current Codex app-server documentation for turn, item, delta,
  approval, plan, hook, and tool lifecycle events
- inspected T3 Code at exact commit
  `694f8d1c6eaaabafbf5c2861ae524174919ef625`
- compared T3 Code's provider event contract, Codex and ACP mappings,
  server-side projections, and client-side collapsed work grouping
- retained existing ACP and route-version evidence from Research 038, 048,
  052, and 062

No executable, account, credential, provider request, paid operation, harness
server, or model server was used.

## Current Swallowtail Foundation

The execution seam already fits live activity delivery:

- run and turn handles expose one ordered event stream
- event sequences are monotonic within one operation
- semantic records cannot be silently dropped
- streams and operation content are bounded
- terminal outcome and cleanup remain separate from event delivery
- callback and direct-tool exchanges carry correlated opaque request and
  result data
- prepared facades return the same low-level handles rather than buffering
  provider work into one final response

This is enough for a consumer to render assistant deltas, final output, broad
progress, safe reasoning progress, callbacks, direct tool availability, and
typed provider observations.

It is not enough for a provider-neutral work log.

## Information Loss

`RuntimeEvent` currently carries only:

- one sequence
- one broad event kind
- optional bounded content

It has no general activity identity, lifecycle, category, status, content
stream owner, provider item reference, message phase, detail strength, or
completeness claim.

Current adapter mappings demonstrate the loss:

- Codex app-server preserves agent-message deltas and final text, but maps
  every other completed item to contentless `Progress`; unknown
  notifications become `ProgressSnapshot`
- Claude Agent and Gemini ACP preserve message and thought chunks, but map
  tool calls, tool updates, and plans to contentless `Progress`
- Kimi ACP has the same shared ACP projection limit
- Kimi headless recognizes tool-bearing messages and tool results but maps
  them to generic progress
- Codex exec preserves selected search and reasoning summaries, but maps
  other completed items to generic progress
- several direct routes correctly expose only inference output, reasoning,
  tool continuation, usage, or provider evidence because no harness work log
  exists on those routes

The `StreamingEvents` capability is binary. It cannot tell a prepared
consumer whether a route supplies item lifecycle, reasoning summaries, plans,
tool input, tool result, command output, file changes, tasks, or only assistant
text.

## Codex Evidence

Current Codex app-server exposes:

- `turn/started`, `turn/completed`, `turn/plan/updated`, and
  `turn/diff/updated`
- `item/started` and `item/completed` with stable item ids
- assistant-message, plan, readable reasoning-summary, and command-output
  deltas
- command execution, file change, MCP tool call, dynamic tool call,
  collaborative-agent tool call, web search, image view, review, and context
  compaction items
- correlated approvals and server-request resolution
- hook lifecycle and token-usage updates

The app-server docs identify completed items as authoritative. They also
record that some item types or deltas are version-sensitive. A Swallowtail
mapping therefore needs exact interface milestones rather than one
latest-version shape.

## T3 Code Reference

T3 Code uses a useful ownership split:

1. Provider adapters map native events into a canonical provider-runtime
   event contract.
2. Events retain event, thread, turn, item, request, and provider references.
3. Item lifecycle, content deltas, plans, tasks, hooks, and tool activity
   remain distinct.
4. Server projections store assistant messages and work activities
   separately.
5. The web client groups consecutive work entries and decides how many are
   visible before a collapsed toggle.

Its common item vocabulary includes assistant messages, reasoning, plans,
commands, file changes, MCP and dynamic tools, collaborative tools, web
search, image view, review, compaction, errors, and unknown items. Its content
stream vocabulary distinguishes assistant text, reasoning text, reasoning
summary text, plan text, command output, and file-change output.

Swallowtail should adopt the separation, not copy the product model:

- Swallowtail owns normalized portable activity records and exact route
  fidelity.
- Consumers own message storage, activity storage, grouping, collapse,
  labelling, review UX, and transcript policy.
- Raw provider envelopes remain adapter-private.

## Disclosure Boundary

"All available output" cannot mean every provider byte.

The portable surface may expose:

- provider-intended assistant messages
- provider-intended readable reasoning summaries or thought updates
- documented tool, command, file, plan, task, and hook display fields
- bounded command output, diffs, and provider-returned tool display content
- adapter-normalized summaries where the provider supplies lifecycle but no
  portable display body

It must not expose:

- hidden chain-of-thought
- provider-private reasoning continuation
- undocumented internal traces
- credentials, authorization headers, endpoint secrets, or raw provider
  envelopes
- raw callback bodies duplicated outside their existing bounded exchange
- raw provider payloads through diagnostics, `Debug`, or stable public
  formatting

Provider-visible thought chunks may become reasoning-summary activity only
when the selected interface intends them for client display. The portable name
must not imply complete reasoning.

## Required Portable Shape

One provider-neutral observable-activity layer should add:

- stable operation-local activity identity plus optional opaque provider item
  reference
- started, updated, and completed observations without inventing missing
  lifecycle phases
- explicit activity categories
- explicit assistant-message phase
- activity status and safe terminal detail
- typed content deltas owned by one activity
- callback, request, or direct-tool correlation where applicable
- bounded display content with redacted formatting
- disclosure strength: provider display content, normalized summary,
  identity-only, or unavailable
- lifecycle fidelity: complete lifecycle, update-and-completion,
  completion-only, or unavailable
- an exact prepared route activity profile

Unknown semantic provider activity must remain visible as a bounded
namespaced unknown activity without raw payload. It must not silently become a
coalescible snapshot.

## Route Posture

| Route family | Current portable truth | Required work |
| --- | --- | --- |
| Codex app-server | rich native source, thin projection | first full lifecycle proof |
| Codex exec | completion-oriented JSONL source | exact completion-only projection |
| ACP harnesses | messages and thoughts preserved; plans and tools flattened | shared ACP currentness and projection |
| OpenCode, Pi, local-server, and managed harnesses | route-specific partial events | exact corpus and mapping tranche |
| headless JSON/JSONL harnesses | output plus selected progress | completion and tool-detail audit |
| hosted direct inference | output, reasoning, tools, and evidence vary | exact non-harness activity truth |
| attached direct runtimes | output and usage dominate | no fabricated harness activity |
| realtime media | separate media lifecycle already exists | explicit applicability boundary |
| catalogue and serving-only routes | no inference activity operation | not applicable |

## Version Posture

Contract 029 applies unchanged.

- guaranteed activity fidelity binds exact qualified interface milestones
- a maintained range may contain several activity-schema segments
- unverified-newer execution remains allowed where the route already permits
  it
- unverified-newer admission does not extend the guaranteed activity profile
- unknown newer semantic events remain visible and bounded
- unsafe, malformed, contradictory, or uncorrelatable events still fail
  closed

## Sources

- [Codex app-server documentation](https://developers.openai.com/codex/app-server)
- [T3 Code provider-runtime contract](https://github.com/pingdotgg/t3code/blob/694f8d1c6eaaabafbf5c2861ae524174919ef625/packages/contracts/src/providerRuntime.ts)
- [T3 Code Codex adapter](https://github.com/pingdotgg/t3code/blob/694f8d1c6eaaabafbf5c2861ae524174919ef625/apps/server/src/provider/Layers/CodexAdapter.ts)
- [T3 Code ACP event projection](https://github.com/pingdotgg/t3code/blob/694f8d1c6eaaabafbf5c2861ae524174919ef625/apps/server/src/provider/acp/AcpCoreRuntimeEvents.ts)
- [T3 Code runtime ingestion](https://github.com/pingdotgg/t3code/blob/694f8d1c6eaaabafbf5c2861ae524174919ef625/apps/server/src/orchestration/Layers/ProviderRuntimeIngestion.ts)
- [T3 Code timeline grouping](https://github.com/pingdotgg/t3code/blob/694f8d1c6eaaabafbf5c2861ae524174919ef625/apps/web/src/components/chat/MessagesTimeline.logic.ts)
- [Contract 009](../contracts/009-async-operation-lifecycle.md)
- [Contract 012](../contracts/012-interactive-session-options-and-callback-exchange.md)
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037](../contracts/037-prepared-consumer-integration.md)

## Promotion

- Added the observable-activity boundary to system architecture.
- Promoted Contract 044.
- Compiled roadmaps g02.035-g02.040 and cards 119-137.
- Selected card 119 as the sole ready task.

