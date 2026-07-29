# 066 Non-ACP Harness Activity Inventory And Corpus

Status: promoted
Owner: Tom
Date: 2026-07-29

## Question

What observable activity can Swallowtail honestly guarantee across every
production non-ACP harness route, and which exact version, transport, option,
ownership, and absence boundaries must implementation preserve?

## Method

Evidence was accessed on 2026-07-29.

- checked official or maintained transport and headless documentation
- inspected exact tagged schemas, event declarations, stream formatters, and
  package sources at every selected upper bound
- checked current stable releases beyond qualified upper bounds
- compared exact source digests where a current release remained unqualified
- reused existing compatibility, protocol, failure, and retention corpora
- froze bounded native-lifecycle, completion-only, partial, unknown,
  malformed, and failure fixtures

No executable, installation, authentication, provider request, model call,
account, paid operation, attached server, or consumer repository was used.

## Route Inventory

| Route | Qualified interface | Current stable | Native activity truth | Correlation | Exact absence |
| --- | --- | --- | --- | --- | --- |
| OpenCode HTTP/SSE | `1.14.48..=1.18.4`, published gaps excluded | `1.18.9`, unverified newer | message and reasoning part delta/replacement; tool state; step, retry, compaction, session close | session, message, part, call | `1.14.51` has no tool or reasoning part replacement |
| Pi RPC | exact `0.80.10` | `0.82.1`, unverified newer | agent, turn, message, tool, compaction, and retry lifecycle | content index and `toolCallId` | no qualified direct bash update or summarization-retry event |
| Kimi local server | `0.28.1`, `0.29.0..=0.29.2` | `0.30.0`, unverified newer | turn, step, message, thought, tool, shell, subagent, task, compaction, retry | session, turn, step, tool, command, subagent, cursor | assistant deltas have no provider item id beyond turn and offset |
| Anthropic Managed Agents | beta `managed-agents-2026-04-01` | same beta authority | authoritative persisted completion records and session/span milestones | event, session, thread, tool, parent tool | selected route excludes best-effort previews; thinking has no readable body |
| Claude Code headless | exact `2.1.220` | `2.1.220` | completed assistant and tool records | session, message, UUID, tool, parent tool | production argv omits `--include-partial-messages` |
| Gemini CLI headless | `0.51.0..=0.52.0` | `0.53.0`, unverified newer | assistant chunks; completion-only tool use/result | session and tool; assistant id is local | no tool progress or readable reasoning |
| Kimi Code headless | `0.29.0..=0.29.2` | `0.30.0`, unverified newer | completed assistant/tool records; retry progress | tool id; assistant id is local | no init, message lifecycle, or readable reasoning |
| Qwen Code headless | exact `0.19.11` | `0.21.1`, unverified newer | message/content-block start, delta, stop and completed records | session, message, UUID, content index, tool, parent tool | result is terminal truth, not another activity |

The eight route profiles are machine-frozen in
`swallowtail-testkit/tests/fixtures/non-acp-harness-activity.json`.

## Activity Depends On Selected Options

Executable capability alone is insufficient.

- Claude Code can emit partial message events only when
  `--include-partial-messages` is selected with streaming JSON output.
  Swallowtail's qualified production command does not select it.
- Qwen Code's qualified command does select `--include-partial-messages`.
  Its message and content-block lifecycle is therefore part of the route
  guarantee.
- Managed Agents preview events are best effort, not persisted, and not
  replayable. The selected production route consumes authoritative persisted
  events and deliberately excludes previews.

The route profile must bind these choices. Allowing a newer executable does
not enable richer activity implicitly. Contract 044 now makes
activity-affecting interface options part of immutable prepared evidence.

## OpenCode HTTP/SSE

The existing compatibility corpus covers all 45 published members of the
qualified range and 18 selected OpenAPI surface revisions.

Most selected releases expose typed message parts:

- text and reasoning part delta or replacement
- tool state `pending`, `running`, `completed`, or `error`
- stable session, message, part, and tool-call identity
- step, retry, compaction, patch, snapshot, todo, and session milestones

Provider-executed and built-in tool parts are harness-owned activity.
Permission and question requests remain callback exchanges.

`1.14.51` is a real compatibility segment, not a reason to reject the whole
range. Its selected event closure retains text delta, session status, and
session idle but lacks `message.part.updated`. Swallowtail may expose the
assistant delta and close it at session idle. It cannot claim reasoning or
tool lifecycle for that exact release.

Current `1.18.9`, commit
`4da7bb44c84e013fa53e9c5d02ac753d1435c81a`, has the same selected Event
closure digest as qualified `1.18.4`:

`636c2931cf30b6000a8b5a5c1ec70b8c6231cee18df7f5e1582384f45f1ad551`

It remains unverified newer. The source match does not widen the complete
harness guarantee.

## Pi RPC

Qualified `0.80.10`, commit
`8dc78834cde4e329284cf505f9e3f99763df5529`, has explicit:

- agent start, end, and settled
- turn start and end
- message start, update, and end
- text, thinking, and tool-call deltas
- tool execution start, replacement update, and end keyed by `toolCallId`
- queue, compaction, retry, and extension-error events

Text and thinking chunks are typed client-facing deltas. Thinking remains a
readable reasoning summary, never hidden chain-of-thought.

Tool execution is Pi-owned. Extension UI requests and responses remain
consumer callbacks.

Current `0.82.1`, commit
`b4f293684bba718d59cc1157679bcf6157b3a7f5`, adds direct bash execution
updates and summarization-retry events. The maintained RPC documentation
digest changes from:

- `0.80.10`:
  `0078a7740e6c471b0f6e3d5ecb1692088b31bcd04ff0c3be6f1f4ed8f98a5440`
- `0.82.1`:
  `89b0d9c93838870e8401e75ab634c28cdeab2716f15010bf66bd776c27af24ec`

Those newer events are not part of the qualified profile.

## Kimi Local Server

The selected WebSocket schema exposes the richest non-ACP route:

- turn and turn-step lifecycle
- assistant and client-display thinking deltas
- tool-call delta, start, progress, and result
- shell start, output, and completion
- subagent spawn, start, suspension, completion, and failure
- compaction, background task, cron, goal, hook, and retry milestones
- cursor sequence, epoch, volatile offset, resync, and session identity

Tool, shell, and subagent events are harness-owned. Awaiting approval and
question records remain callbacks.

Qualified `0.29.2` source commit
`8a45f10eddbb35c317047e82e567cdb59a220b4f` and current `0.30.0` source
commit `16c7189bd54a42fae65b1bbafd0843420523f797` have byte-identical selected
event schema:

`57ff04a2ff5c256ced0e8e6a174818912648102af8bf55e32874606397fdda03`

`0.30.0` remains unverified newer. The local-server lifecycle cannot be
borrowed by the separate Kimi headless or ACP routes.

The guaranteed range has two exact schema segments:

- `0.28.1` digest
  `30f1e88d1dbd57e3312a0cf48ce04139b5d02de06215f1c4cfa7b206864b1689`
- `0.29.0..=0.29.2` digest
  `57ff04a2ff5c256ced0e8e6a174818912648102af8bf55e32874606397fdda03`

The only semantic event-set addition is `agent.created` and
`agent.disposed` from `0.29.0`. The common turn, step, message, thought, tool,
shell, subagent, task, compaction, and retry set exists at `0.28.1`.

## Anthropic Managed Agents

Persisted events are authoritative and replayable. The selected route may
observe:

- completed `agent.message`
- identity-only `agent.thinking`
- built-in `agent.tool_use` and `agent.tool_result`
- provider-executed `agent.mcp_tool_use` and `agent.mcp_tool_result`
- consumer-executed `agent.custom_tool_use`
- session status and span milestones
- thread and parent-tool correlation

Built-in and MCP tools are provider-owned. Custom tool use is a callback. The
consumer sends `user.custom_tool_result`; this must not become evidence that
Anthropic executed the custom tool.

Preview `event_start` and `event_delta` records are best effort and cannot be
replayed. `agent.message` previews may be replaced by the later authoritative
persisted event. Thinking previews expose start only. The existing selected
route excludes previews, so its assistant and tool profile is
completion-oriented.

## Headless Routes

### Claude Code

Exact `2.1.220`, tag
`7ef6eec9d9ba84ea6f233f26c45f1df5c5991843`, is frozen against the matching
Agent SDK output declarations.

Without `--include-partial-messages`, assistant messages and tool records are
complete when observed. Tool input and result bodies remain excluded from
portable display. A safe correlated post-init unknown record may become
namespaced activity; a pre-init unknown or malformed record fails closed.

### Gemini CLI

The stream emits init, message chunks, tool use, tool result, error, and
result records. Assistant chunks update one operation-local identity.
Tool use and result are completion records keyed by `tool_id`.

The selected event declarations and formatter are byte-identical from
qualified `0.52.0`, commit
`d14583b926769bd98f807cdc6b1ca50e91ae26ec`, through current `0.53.0`,
commit `decc0b46c6e11f8cad90710dcfb38fc3cdb24a96`:

- types:
  `23f7ea24497c88a703e0e4f8b6deb8bda969c2c2a32ca213beacfae46d798341`
- formatter:
  `f78377bb9cbb56cfe3509655ff6ebfaef8873641942139562dc4ab7a3347e721`

`0.53.0` remains unverified newer.

### Kimi Code

Headless stream JSON emits assistant, tool, metadata, and retry records.
Assistant and tool records have completion-only fidelity. Resume hints remain
session metadata. Tool arguments and result bodies remain excluded.

The headless renderer is byte-identical between qualified `0.29.2` and
current `0.30.0`:

`d413a0678dcebb5a0a6f9dda8ce51d53e4b70d55b162afe6635a7d8c1a1536c4`

`0.30.0` remains unverified newer.

### Qwen Code

The selected command enables partial messages. The stream exposes message and
content-block start, delta, and stop records before the completed assistant
message and terminal result. Tool blocks retain tool identity through partial
input and completion.

The selected non-interactive declarations are byte-identical between
qualified `0.19.11`, commit
`f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad`, and current `0.21.1`, commit
`41b4ee8373fb4aa324925e69e0515ca72959ec5b`:

`53be8cff0542711c75744c138d7b14adc4bbe4a20aecf10583526f43d7b837e0`

`0.21.1` remains unverified newer.

## Unknown And Disclosure Posture

The common rule remains exact:

- correlated, bounded, machine-readable semantic records may become
  namespaced unknown activity
- raw provider envelopes never become public evidence or diagnostics
- records without safe identity, ordering, or bounds fail closed
- foreign-session records are filtered where the transport supplies session
  identity
- unverified-newer records cannot widen a qualified profile

Readable reasoning is available only from provider-intended display channels:
Pi thinking deltas, Kimi thinking deltas, OpenCode reasoning parts, and Qwen
thinking deltas when present. Managed `agent.thinking` has no readable body.
Claude, Gemini, and Kimi headless have no qualified reasoning channel.

## Corpus

The deterministic corpus adds:

- one eight-route profile inventory
- OpenCode rich lifecycle and exact `1.14.51` gap streams
- Pi message, thought, tool, compaction, and retry lifecycle
- Kimi turn, step, thought, tool, and subagent lifecycle
- Managed Agents provider-tool, MCP-tool, custom-tool, thought, and status
  persisted events
- Qwen partial tool lifecycle
- headless unknown and malformed cases missing from prior corpora

Existing provider failure, disconnect, schema-drift, and retention fixtures
remain authoritative. Focused tests parse all new raw fixtures and
machine-check route count, ownership, lifecycle, exact absences, malformed
posture, and OpenCode's complete 45-release window.

Card 128 changes no production decoder or route profile.

## Implementation Order

Card 129 should proceed:

1. Pi RPC — clearest complete native lifecycle and tool correlation.
2. Kimi local server — WebSocket cursor, step, tool, subagent, and callback
   coexistence.
3. OpenCode HTTP/SSE — widest qualified range and exact `1.14.51` gap.
4. Managed Agents — authoritative completion records plus provider,
   MCP, and consumer tool ownership.

Card 130 should proceed:

1. Qwen — partial-message lifecycle selected explicitly.
2. Gemini — message chunks with completion-only tools.
3. Claude Code — entirely completion-oriented under current argv.
4. Kimi Code — thinnest completion-only stream plus retry metadata.

This order maximizes transport and fidelity information before repeating
similar headless projections.

## Contract Fit

Contract 044 remains sufficient after one clarification:

- selected interface options that affect activity are immutable prepared
  profile inputs
- an available but unselected partial or preview mode cannot widen the route
  profile

No new activity kind, transport abstraction, callback authority, lifecycle
phase, or provider fallback is required.

## Sources

- [OpenCode server documentation](https://opencode.ai/docs/server/)
- [OpenCode `1.18.4` generated event types](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/js/src/gen/types.gen.ts)
- [OpenCode releases](https://github.com/anomalyco/opencode/releases)
- [Pi RPC documentation](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md)
- [Pi releases](https://github.com/earendil-works/pi/releases)
- [Kimi Code `0.29.2` local-server event schema](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/protocol/events-zod.ts)
- [Kimi command reference](https://moonshotai.github.io/kimi-cli/en/reference/kimi-command.html)
- [Kimi Code releases](https://github.com/MoonshotAI/kimi-code/releases)
- [Managed Agents events and streaming](https://platform.claude.com/docs/en/managed-agents/events-and-streaming)
- [Managed Agents event API reference](https://platform.claude.com/docs/en/api/beta/sessions/events)
- [Managed Agents permission policies](https://platform.claude.com/docs/en/managed-agents/permission-policies)
- [Claude Code headless mode](https://code.claude.com/docs/en/headless)
- [Gemini CLI headless mode](https://geminicli.com/docs/cli/headless/)
- [Gemini CLI releases](https://github.com/google-gemini/gemini-cli/releases)
- [Qwen Code headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Qwen Code releases](https://github.com/QwenLM/qwen-code/releases)

## Promotion

- accounted for all eight production non-ACP harness routes
- froze exact lifecycle, completion-only, partial, unknown, malformed, and
  failure evidence
- retained current releases above qualified upper bounds as unverified newer
- clarified option-dependent activity profiles in Contract 044
- made card 129 ready without changing production behavior
