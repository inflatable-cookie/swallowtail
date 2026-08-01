# 092 Provider Session Discovery And Import

Status: promoted
Owner: Tom
Date: 2026-08-01

## Question

Can a Swallowtail consumer discover harness-origin sessions, import one into
its own thread database, replay its history, and continue it without turning
Swallowtail into a session database or granting authority from a raw provider
id?

## Method

The audit compared current official Codex app-server, ACP, OpenCode, and Kimi
surfaces with Swallowtail's qualified route corpora, Contracts 015, 017, 029,
037, and 038, and the provider feature matrix.

No provider executable, session, prompt, authentication flow, filesystem
mutation, consumer database, installation, or publication was used.

## Product Boundary

A proxy and a harness have separate histories. The consumer owns its thread
database, imported messages, local title, selection, merge policy, and UI.
The harness owns its persisted provider session. Swallowtail may expose only
the mechanism that discovers, validates, loads, and resumes that provider
state.

The useful first product flow is not continuous synchronization:

1. list provider sessions under one explicit configured instance and discovery
   scope
2. display bounded provider metadata as non-authoritative candidates
3. let the consumer select one candidate explicitly
4. revalidate the candidate against the same route, host, access, version, and
   working resource
5. issue the ordinary durable resume binding
6. use the existing load operation for ordered historical replay
7. let the consumer create and persist its local thread projection

Provider list refresh remains a read-only snapshot. It does not create local
threads, watch provider storage, merge concurrent edits, or grant attachment
authority.

## Operation Separation

| Operation | Result | Authority |
| --- | --- | --- |
| session catalogue | bounded candidates and cursor | read-only observation |
| explicit import | validated route-bound resume binding | attachment authority |
| load | ordered historical replay plus ready handle | replaying attachment |
| resume | ready handle without replay | non-replaying attachment |
| provider management | archive, restore, or delete one bound inactive target | separate lifecycle authority |
| consumer persistence | local messages, mapping, deduplication, and UI | downstream only |

A catalogue candidate is never a `SessionResumeBinding`. A raw session id,
provider title, cwd, list response, diagnostic, or copied local file remains
insufficient.

## Current Route Evidence

| Route | Discovery | History | Continuation | Decision |
| --- | --- | --- | --- | --- |
| Codex app-server | `thread/list` plus `thread/read` | `thread/read` or load replay | `thread/resume` | first complete proof |
| ACP v1 | stable optional `session/list` | separate `session/load` | agent-specific load/resume | shared protocol foundation only |
| Kimi Code ACP | qualified list, load, and resume evidence | ordered ACP replay | load and resume | first ACP adapter proof after the common kernel |
| OpenCode attached HTTP | `GET /session` and exact lookup | session message inventory and existing load replay | existing load and resume | first attached HTTP proof |
| Kimi local server | REST list and lookup plus transcript/event surfaces | route-specific transcript and cursor evidence | qualified resume | later exact import assessment |
| Claude Agent ACP | qualified load and resume | ordered ACP replay | load and resume | no qualified list capability |
| Cursor ACP | initialize advertises list and load | unqualified | unqualified | exact source work required before a claim |
| Pi RPC | persisted-session inventory and reads exist | ordered reads | provider switching exists | unchanged cwd-binding gate blocks import |
| Qwen, Antigravity, Gemini, Grok | no complete qualified list, replay, and public resume combination | route-dependent | route-dependent | classify unsupported until evidence changes |

Direct inference, realtime, model catalogue, and serving-only routes are not
harness-history import routes. A hosted provider resource needs its own exact
durable-conversation evidence rather than inheriting this capability.

## Codex Finding

Current Codex app-server documentation exposes paginated `thread/list` with
working-directory and source filters, metadata-only or history-bearing
`thread/read`, paginated turn and item reads, and `thread/resume`. This is the
strongest first proof because one provider-owned interface exposes discovery,
history, and continuation without storage scraping.

Current documentation is feasibility evidence, not a historical range claim.
The Codex roadmap must freeze method presence and behavior at every maintained
milestone before changing production capabilities.

## ACP Finding

ACP stabilized `session/list` on 2026-03-09. The method is independently
capability-gated and returns paginated session ids, cwd, optional title,
optional updated time, and bounded agent metadata. `session/load` remains the
separate history-replaying operation.

The shared protocol can therefore carry catalogue records, but no adapter
gains support from wire stability alone. Kimi is selected because its exact
qualified evidence already covers list, load, and resume. Claude lacks the
list capability. Cursor advertises optional operations that its current
production route deliberately does not qualify.

## OpenCode Finding

The maintained OpenCode server documents session list, exact session lookup,
status, children, message inventory, and prompt continuation. Swallowtail
already qualifies attached-server load and resume across its maintained
range. A new exact operation-closure corpus must prove list, lookup, history,
and resource scoping at every supported point before the facade exposes
import.

## Discovery Scope And Content

The first portable scope is one explicit configured instance plus one
host-approved working resource. A consumer may aggregate several such
catalogues. Account-wide or state-root-wide browsing is a separate scope and
cannot expose ambient filesystem paths or silently create resource authority.

Candidate title, preview, update time, provider state, and activity status are
optional provider content. They are bounded, source-labelled, and unavailable
to stable diagnostics. Raw provider payloads and arbitrary metadata remain
adapter-private.

An import revalidates the candidate after selection. It binds the same
configured instance, driver, transport, host, target, access profile,
interface version, model route, model, working resource, and session access
policy required by Contract 017. Candidate staleness, disappearance, mismatch,
or insufficient history fails before a usable binding is returned.

## Synchronization Boundary

The first tranche excludes background polling and bidirectional history
merge. Provider and consumer histories may change independently, and not all
routes expose stable message ids or authoritative active-writer state.

The consumer may explicitly refresh the catalogue or perform another load.
It owns deduplication and whether imported replay becomes durable local
messages. Swallowtail does not infer that two provider sessions, transports,
accounts, working resources, or local threads are equivalent.

## Decision

Add two provider-neutral roles:

- provider-session catalogue: bounded read-only discovery
- provider-session import: explicit candidate revalidation and binding issue

Keep import separate from load, resume, provider-session management, and
consumer persistence. Use the existing `SessionResumeBinding` and replay
contracts after import rather than creating another prompt/session API.

Sequence the production tranche as shared kernel, Codex, ACP/Kimi, OpenCode,
then provider-wide classification and a Nucleus handoff. Kimi local server,
Claude, Cursor, Pi, and the remaining harnesses stay evidence-gated.

## Risks

- provider titles and previews may contain prompt or path material
- a catalogue snapshot can become stale before import
- external clients may continue the same provider session concurrently
- provider history may be incomplete or lack stable item identity
- identical provider ids across routes or accounts are not the same session
- global browsing can exceed the authority of one approved working resource
- version milestones for listing may differ from existing load/resume ranges

## Promotion

- ACP optional listing: Contract 015
- portable catalogue/import boundary: Contract 046
- dependency and consumer boundary: system architecture
- delivery: g03.019-g03.023 and cards 049-063

## Primary Sources

- [Codex app-server](https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md)
- [ACP session-list stabilization](https://agentclientprotocol.com/announcements/session-list-stabilized)
- [ACP session-list specification](https://agentclientprotocol.com/rfds/session-list)
- [OpenCode server API](https://opencode.ai/docs/server/)
- [Kimi Code `0.31.1` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.31.1)
- Swallowtail Research 006, 040-041, 053, 076, and 086
- Swallowtail qualified fixtures and provider feature matrix
