# 209 Anthropic Messages Adaptive-Thinking Evidence

Status: promoted
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Card: g04.062 / 173

## Question

Which exact Anthropic Messages model and operation profiles can dispatch
adaptive thinking with omitted display and, when consumer tools are used,
preserve every required signed private block under Contracts 030, 040, and 044?

## Method

Official Anthropic public documentation only, retrieved 2026-08-25T14:36:21Z.
No account, credential, catalogue call, prompt, or provider request was used.
The existing route identity is `anthropic.messages`, driver
`swallowtail.anthropic.direct`, facade `anthropic-2023-06-01`, hosted API-key
access. Prepared profiles remain the resource-free structured one-attempt
route and the fixed direct-continuation session from Research 004, 067, 169,
and 185.

## Finding

The smallest exact subset that survives Contracts 030, 040, and 044 is:

| Model | Thinking | Display | Profiles | Disposition |
| --- | --- | --- | --- | --- |
| `claude-opus-4-7` | adapter-local `AnthropicThinkingMode::adaptive()` only | explicit `"omitted"` | one-attempt structured, including optional qualified web search; direct continuation with the mode fixed at preparation and repeated on every attempt and fresh restoration | **deliver-now** |

Official troubleshooting lists Claude Opus 4.7 as adaptive-only, default off,
rejecting `thinking.type=enabled`. Thinking is off until the request sets
`thinking: {type: "adaptive"}`. The thinking overview names `"omitted"` as the
Opus 4.7 display default and documents that omitted blocks still carry a
`signature` for multi-turn continuity. The Messages create schema separately
says `display` defaults to `"summarized"`. This lane does not rely on either
default. The exact qualified wire is:

```json
{"thinking":{"display":"omitted","type":"adaptive"}}
```

Canonical object-key order puts `display` before `type`. Omission of the
adapter-local mode keeps current request bytes: no `thinking` object.

`adaptive` is not a `ReasoningMode` value and is not an `output_config.effort`
value. Effort remains the portable `low|medium|high|xhigh|max` control from
Research 185. Official thinking and effort pages treat the two as independent:
thinking controls whether Claude thinks in thinking blocks; effort controls
how much work the whole response receives, including how often and how deeply
it thinks in adaptive mode. This lane composes them without defaults, clamps,
shared confirmation, or rewriting omission of either control.

Manual `thinking.type=enabled` with `budget_tokens` is rejected on Claude 4.7
and later. Summarized display, hidden-reasoning disclosure, `ReasoningSummary`
activity, other models, other facades, Managed Agents, Claude Code, UltraCode,
Fast mode, and newer web-search types remain **not applicable**. Fixture ids
(`claude-fixture-primary`, `claude-fixture-search-capable`) remain **withheld**.
Other Anthropic model ids remain **evidence-gated**.

## Official Sources

Retrieved 2026-08-25. Complete-body SHA-256 is the HTTP response body. No
`Last-Modified` or `ETag` was present. `adaptive-thinking` redirected (307);
the thinking overview remains the primary configuration corpus.
`extended-thinking-models` also redirected (307) onto the thinking overview.

| Source | Use | SHA-256 of retrieved source body |
| --- | --- | --- |
| [Thinking](https://platform.claude.com/docs/en/build-with-claude/thinking) | adaptive/disabled/enabled distinction, omitted vs summarized display, omitted stream sequence, tool-loop preservation, interleaved thinking, encryption, redacted blocks, effort independence, Opus 4.7 compatibility | `1464ad466492773b47a854a7ca499103e0040d765c97327ca14a2414eed8f7b5` |
| [Troubleshooting thinking](https://platform.claude.com/docs/en/build-with-claude/thinking-troubleshooting) | per-model type table; Opus 4.7 adaptive-only default off; 400 for modified thinking/redacted blocks; adaptive skip; `max_tokens` interaction | `27b7c9b526841d07b24c51cdd527190089cab81cfa22637cb1ec7580bd6c32b3` |
| [Streaming](https://platform.claude.com/docs/en/build-with-claude/streaming) | omitted display emits no `thinking_delta`; thinking block opens, one `signature_delta`, then stop | `6e979faa58c625d5f9188fbc2702ebe3bf52330aade0509ef438dd62368f6aa2` |
| [Thinking tool workflows](https://platform.claude.com/docs/en/build-with-claude/thinking-tool-workflows) | echo the assistant content array verbatim, including thinking/redacted blocks, with the tool result | `3fb0a931c758785eb920a51b67e04800c59758c8f9b1607b2c2bceba3a76f210` |
| [Messages create API](https://platform.claude.com/docs/en/api/messages/create) | request `thinking` object; `ThinkingBlock`/`RedactedThinkingBlock`; `thinking_delta`/`signature_delta`; replay params | `57de56266199a4d09fd60934b983f6a39889321a94460bbc75aa0c80005f4b3a` |
| [Effort](https://platform.claude.com/docs/en/build-with-claude/effort) | effort is independent of thinking; Opus 4.7 five values; `adaptive` is not an effort value | `a030a5d45a847cba2321f18a2a52f17caa5bc04cc7fa4c0e9ee0be99447d8e33` |
| [Model IDs and versions](https://platform.claude.com/docs/en/about-claude/models/model-ids-and-versions) | exact `claude-opus-4-7` identity form | `5a785b2fa4812fca02f08e084aba18b9c8bf7ab6c85492f40af6935f7536e3c0` |
| [Adaptive thinking](https://platform.claude.com/docs/en/build-with-claude/adaptive-thinking) | retrieved; redirected. Billing/display table agrees with the thinking overview; not a second model-qualification source | `6faec4e6d95a0de69d204719762c4ca05dc84d8fd0c3f2d26c8eb0b085cbd729` |
| [Steering thinking](https://platform.claude.com/docs/en/build-with-claude/thinking-steering-and-cost) | effort steers thinking volume in adaptive mode; not a new portable value | `5afedac2c19ce2731252fe5f212edee99f35106c2d710db64f617997622215fd` |

Converted documentation text extracted from those pages was the digestable
corpus. HTML bodies are Next.js shells without cache validators.

## Exact Request Forms

| Form | Wire | This lane |
| --- | --- | --- |
| Omitted adapter-local mode | no `thinking` object | preserve byte-identical existing requests |
| Adaptive omitted display | `thinking: {"display":"omitted","type":"adaptive"}` | only admitted public selection |
| Adaptive without display | `thinking: {"type":"adaptive"}` | withhold; depends on a contradicted default |
| Adaptive summarized | `thinking: {"type":"adaptive","display":"summarized"}` | withhold; readable thought is out of scope |
| Disabled | `thinking: {"type":"disabled"}` | withhold |
| Manual enabled/budget | `thinking: {"type":"enabled","budget_tokens":N}` | withhold; 400 on Opus 4.7 |

`tool_choice` stays `{"type":"auto"}` on continuation. Adaptive thinking
supports forced tool use; this route does not select it. Sampling fields stay
absent: Opus 4.7 rejects non-default `temperature`, `top_p`, or `top_k` on
every request.

Thinking tokens count toward `max_tokens`. That does not change prepared
limits. Structured attempts keep the caller `maximum_output_tokens`. Direct
continuation keeps 8,192 tokens per attempt. A `stop_reason: "max_tokens"`
finish remains the existing Length outcome. Official 64k guidance for
`xhigh`/`max` effort is advisory, not a route clamp.

## Private Response Grammar

Omitted-display adaptive thinking can produce:

1. **Absence.** Adaptive mode may emit no thinking block. Valid. Replay
   reconstructs only the consumer-visible assistant envelope.
2. **Ordinary thinking block, complete.** Non-streamed shape:
   `{"type":"thinking","thinking":"","signature":"<opaque>"}`. Stream:
   `content_block_start` with empty `thinking` and `signature`, one
   `signature_delta`, then `content_block_stop`. No `thinking_delta`.
3. **Multiple consecutive thinking blocks.** One or more thinking blocks may
   precede text or tool use. Replay preserves exact order.
4. **Redacted thinking.** Distinct type
   `{"type":"redacted_thinking","data":"<opaque>"}`. Not omitted display.
   Official stream examples do not show a redacted delta type. Admit a
   start-complete `data` field and no deltas; any delta, missing `data`, or
   empty `data` fails closed.
5. **Interleaved later thinking.** After a tool result, the next attempt may
   again start with thinking/redacted blocks, then text or another tool. That
   later sequence is a new attempt's private state, not a reconstruction of
   consumer transcript text.

Fail closed:

- `thinking_delta` under this omitted-display selection
- non-empty `thinking` text on a thinking start
- missing, empty, duplicate, or extra `signature_delta`
- unknown semantic block or delta
- thinking/redacted blocks when the adapter-local mode was omitted
- reordered, partial, or reconstructed private blocks

`usage.output_tokens_details.thinking_tokens` may appear on the final
`message_delta`. Ignore it. Do not extend portable `TokenUsage`. Do not claim
effective thinking depth from tokens, blocks, or prose.

## Tool-Loop Replay

Required: when returning the correlated tool result, pass every `thinking` and
`redacted_thinking` block from that in-progress assistant message complete,
unmodified, and in original order immediately before the `tool_use` block.
The 400 message is:

```text
`thinking` or `redacted_thinking` blocks in the latest assistant message cannot be modified
```

The one documented exception is text placed in the empty `thinking` field of
an omitted block, which is ignored. This route never writes that field; it
replays the empty string and the signature as received.

Recommended across later user turns; allowed to omit thinking outside an
in-progress tool loop. Opus 4.7 is in the keep-all preservation class (Opus
4.5 and later). Smallest safe production delta: store the first-assistant
private sequence and emit it on both the continuation attempt and later-turn
history so the stored assistant message is not reconstructed as `tool_use`
only. Final-attempt thinking is not required for the tool-result 400 rule;
discard it at that attempt's terminal.

A consumer tool result remains the only authority for a continuation attempt.
Thinking blocks never authorize tool execution or network work. Adaptive mode
does not require a thinking block at the start of an assistant turn.

## Route Audit And Smallest Production Delta

Current `protocol.rs` emits effort through `output_config.effort` and never
emits `thinking`. `protocol/events.rs` accepts text, client tool, and
qualified search; thinking starts and `signature_delta` are protocol
failures. `driver/pump.rs` and `driver/session/attempt.rs` assume one active
text or tool/search block. `driver/session/history.rs` rebuilds the assistant
continuation as `tool_use` only. Activity is assistant, optional search, and
consumer-tool only. Continuation bounds already include 256 KiB private
continuation and 1 MiB private history. Contract 030 already requires
bounded zeroizing private continuation. Contract 044 already excludes hidden
reasoning and private continuation from activity.

Smallest safe production delta, all inside
`crates/swallowtail-adapter-anthropic` plus the Anthropic guide/matrix:

- opaque `AnthropicThinkingMode::adaptive()` on prepared structured and
  session input; not a portable capability
- immutable prepared evidence carries the selection; the shared plan does not
  grow a thinking capability
- driver-local field, copied onto the session handle, restored only as the
  same prepared request with no private-state recovery
- exact omitted-display object; independent effort field; omission
  byte-identical
- SSE grammar for omitted thinking and start-complete redacted blocks
- structured attempts validate and discard private blocks; no thinking
  activity
- continuation captures the first-assistant private sequence in zeroizing
  memory, bounds it, and replays it unmodified before `tool_use`
- thought text, signatures, redacted data, and raw blocks stay out of
  events, activity, output, callbacks, evidence, `Debug`, `Display`, and
  stable diagnostics
- malformed, missing, duplicate, reordered, oversized, foreign, or
  post-terminal private state fails closed without retry or fallback

No shared contract, shared runtime, generic provider JSON, durable private
state, or live provider work is required.

## Claim Bounds

Deterministic request fixtures prove planned/dispatched adaptive mode. A
successful response fixture proves parser acceptance only. This evidence does
not prove provider-effective thinking depth, skipped-thinking frequency, or
paid inference behavior.

## Compatibility Verdict

The current `anthropic-2023-06-01` facade and existing continuation bounds
are sufficient. Cards 174-175 may bind and accept only the one
`claude-opus-4-7` adaptive/omitted-display row above. Keep g04 open.

## Binding Status

Cards 174-175 bound the deliver-now row on the worker branch. Deterministic
fixtures prove planned/dispatched adaptive mode, omitted SSE grammar, private
replay before `tool_use`, effort composition, and fail-closed `thinking_delta`.
They do not prove provider-effective thinking depth.
