# 064 Codex Observable Activity Range Evidence

Status: promoted
Owner: Tom
Date: 2026-07-29

## Question

Which Codex app-server and exec activity shapes exist across Swallowtail's
qualified executable range, and which exact milestones must production
projection respect?

## Method

Evidence was accessed on 2026-07-29.

- checked the current official app-server and non-interactive documentation
- scanned official `openai/codex` stable tags from `0.80.0` through `0.145.0`
- compared app-server protocol types, request and notification maps, exec
  JSONL types, and the exec JSONL event processor
- checked npm publication history and the current official GitHub release
- reused Swallowtail's existing executable claims and publication gaps
- froze bounded offline fixtures without running Codex, authenticating, or
  invoking a model

No executable, credential, provider request, paid operation, live app-server,
or consumer repository was used.

## Currentness Delta

`0.146.0` became the latest stable Codex release on 2026-07-29 after the
existing `0.145.0` qualification work.

This does not make `0.146.0` guaranteed. Both Codex claims already permit
stable newer releases as explicit unverified-newer attempts. The guaranteed
upper bound remains `0.145.0`; `0.146.0` may run without widening its activity
profile.

The tagged `0.146.0` app-server command-action shape adds `pluginId` and
`scriptPath`. The corpus freezes those fields as an additive unverified-newer
case. It does not promote them into portable disclosure.

Prereleases remain rejected. The existing gaps remain:

- `0.82.0..=0.83.0`
- `0.108.0..=0.109.0`

## App-Server Baseline

The qualified app-server range is richer at its oldest point than the current
Swallowtail projection.

At `0.80.0`, tagged source already has:

- `item/started` and `item/completed` with stable item ids
- agent messages and message deltas
- readable reasoning-summary part boundaries and text deltas
- separate raw reasoning text deltas
- commands, output deltas, exit status, and duration
- file changes, legacy patch output, and turn-level diff replacement
- MCP tool calls and progress
- web search and image view
- entered and exited review items
- turn-level plan replacement
- the legacy `thread/compacted` completion notification
- command and file approval requests

Raw reasoning remains excluded. `reasoning.summary`,
`item/reasoning/summaryPartAdded`, and
`item/reasoning/summaryTextDelta` are provider-intended readable summaries.
`reasoning.content` and `item/reasoning/textDelta` are raw reasoning surfaces
and do not enter the portable activity stream.

## App-Server Milestones

The activity-relevant milestones are:

| First release | Tagged commit | Change |
| --- | --- | --- |
| `0.80.0` | `f0679a6ab2563135e01cc922698cb54921a1719f` | core item lifecycle, messages, summaries, commands, files, MCP, search, image view, review, turn plan and diff |
| `0.85.0` | `4607330eff53e3ae39126afef76882042e14a03c` | collaborative-agent tool-call item |
| `0.88.0` | `149625a4f983e1b3c8530abef1c863689d98a1cc` | user-input server request |
| `0.92.0` | `a09055074e082d70e8b92795b0cec1e969a6aaf9` | dynamic-tool request and structured web-search action |
| `0.93.0` | `d86cf538f5e7d210ebb7a3493718aaaff40146da` | plan item and delta; context-compaction item replaces deprecated notification |
| `0.105.0` | `a7eda6a29b3ee25549f385197ff109508dc49a90` | assistant `commentary` or `final_answer` phase |
| `0.106.0` | `ffd726a656403b69b75130025587d5e0a0d6b7d1` | dynamic-tool lifecycle item |
| `0.107.0` | `19f8797c0f62abecb347e817aac36d18c5fc554e` | `serverRequest/resolved` correlation |
| `0.111.0` | `8c75cd9afcd405d134530e53c78e5e0e4e5312a3` | image-generation item; no direct portable kind |
| `0.113.0` | `81c4928825d1e468447a17d6bc74b9abb48743f4` | permission approval and additive MCP plugin metadata |
| `0.114.0` | `b9904c0ae4ecb773549efd6ea3fb05229402fdb9` | hook started and completed notifications |
| `0.115.0` | `f028679abb30051cec2434e624cd99975986b41b` | unstable approval-review lifecycle |
| `0.116.0` | `38771c9082535aa16b4c4d0395d3532f32f656ff` | additive assistant memory citation |
| `0.117.0` | `4c70bff480af37b1bf1a9b352b8341060fe55755` | hook-prompt item |
| `0.123.0` | `0785b66228dff87f891e291cb5686631865b6922` | structured file-patch replacement snapshot |
| `0.129.0` | `2808a4deb181e5ca2b1293a1a5980938cb746861` | item timestamps; legacy file-output event no longer emitted |
| `0.140.0` | `6506579001c322927a3e4bd440563267a7ac6c1f` | subagent-activity item |
| `0.141.0` | `3fb81667d30d9d24297216ea61fbfcc4351b2aa9` | sleep item; no direct portable kind |
| `0.142.0` | `3a76f3ac68c8949d1cac6ea769b6ec7b8953a415` | additive MCP app context |
| `0.144.0` | `767822446c7a594caa19609ca435281a9ec67e0d` | extension-backed web-search wire shape |

Image generation and sleep remain bounded namespaced unknown activities until
a durable portable classification is selected. Their semantic items must not
collapse into generic progress.

Memory citations, MCP plugin identity, and MCP app context are additive
provider display metadata. Card 123 must either map them through an exact
provider-specific detail or leave the route profile at its thinner guaranteed
disclosure. It must not expose raw provider envelopes to preserve them.

## Replacement Truth

Three replacement boundaries matter:

| Release | Older surface | Replacement |
| --- | --- | --- |
| `0.93.0` | `thread/compacted` | `contextCompaction` item lifecycle |
| `0.123.0` | `item/fileChange/outputDelta` | `item/fileChange/patchUpdated` |
| `0.129.0` | retained protocol entry no longer emitted | patch snapshots plus `turn/diff/updated` |

`turn/plan/updated` is an authoritative replacement snapshot across the whole
range.

`item/plan/delta` is proposed plan text. Its concatenation is not
authoritative; the completed plan item is.

`turn/diff/updated` is the latest aggregate turn diff. It replaces prior
snapshots and is not a text delta.

## Exec Truth

Exec and app-server remain separate transports and schemas.

The exec JSONL type has `item.started`, `item.updated`, and `item.completed`
from `0.80.0`, but its item kinds have different emission fidelity:

| Exec item | Qualified lifecycle truth |
| --- | --- |
| agent message | completion only |
| reasoning summary | completion only |
| file change | completion only |
| warning or non-fatal error | completion only |
| command execution | start and completion |
| MCP tool call | start and completion |
| web search | start and completion |
| todo list | start, replacement updates, and completion |
| collaborative tool call | start and completion from `0.92.0` |

The exec event processor deliberately omits app-server-only dynamic-tool,
image, review, compaction, and hook items. Card 124 must not claim them.

Exec milestones are smaller:

- `0.80.0` — core JSONL items and lifecycle
- `0.92.0` — collaborative tool calls and structured search action
- `0.125.0` — additive reasoning-token usage, not activity
- `0.132.0` — additive MCP result metadata
- `0.145.0` — additive cache-write usage, not activity

## Corpus

The deterministic corpus lives under:

`crates/swallowtail-adapter-codex/tests/fixtures/activity/`

It freezes:

- exact activity and disclosure milestones
- app-server rich lifecycle cases
- exec's per-kind lifecycle differences
- assistant commentary, final answer, and legacy unknown phase
- plan and diff replacement
- readable reasoning summaries and excluded raw reasoning
- command, file, MCP, dynamic, collaboration, search, image, review,
  compaction, task, and hook cases
- approval and request-resolution correlation
- deprecated and replacement events
- bounded namespaced unknown, malformed, foreign, and additive-newer cases

Validation is offline. No production decoder uses the corpus in card 122.

## Contract Fit

Contract 044 is sufficient.

- exact interface milestones bind route profiles
- lifecycle phases are not synthesized
- unknown semantic items remain visible or fail closed
- raw reasoning remains excluded
- unverified-newer admission does not widen guaranteed fidelity
- provider-specific additive fields do not require raw envelope exposure

No contract change is required before card 123.

## Sources

- [Official Codex app-server documentation](https://developers.openai.com/codex/app-server/)
- [Official Codex non-interactive documentation](https://developers.openai.com/codex/noninteractive/)
- [Official Codex releases](https://github.com/openai/codex/releases)
- [Codex `0.80.0` app-server protocol](https://github.com/openai/codex/blob/rust-v0.80.0/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.80.0` exec events](https://github.com/openai/codex/blob/rust-v0.80.0/codex-rs/exec/src/exec_events.rs)
- [Codex `0.145.0` app-server item types](https://github.com/openai/codex/blob/rust-v0.145.0/codex-rs/app-server-protocol/src/protocol/v2/item.rs)
- [Codex `0.145.0` exec JSONL processor](https://github.com/openai/codex/blob/rust-v0.145.0/codex-rs/exec/src/event_processor_with_jsonl_output.rs)
- [Codex `0.146.0` app-server item types](https://github.com/openai/codex/blob/rust-v0.146.0/codex-rs/app-server-protocol/src/protocol/v2/item.rs)
- [Official Codex npm package](https://www.npmjs.com/package/@openai/codex)

## Promotion

- froze the exact app-server and exec activity corpus
- retained the existing qualified upper bound
- classified current `0.146.0` as permitted unverified newer
- confirmed Contract 044 is sufficient
- made card 123 ready for production app-server projection

## Later Qualification

Roadmap g02.044 rechecked the exact stable `0.146.0` distribution on
2026-07-30 and promoted it into both guaranteed Codex ranges. The additive
command-action fields and deferred exec search query now form the exact
`0.146.0` activity revision. Operation, access, lifecycle, disclosure,
prerelease rejection, and existing range gaps did not change. Stable
`0.147.0` is retained as a synthetic unverified-newer classification point.
