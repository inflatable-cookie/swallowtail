# 327 Oh My Pi 18.1.22 Major-Line Identity

Status: promoted
Owner: Tom
Date: 2026-09-15
Card: g05.079 (Research 308 campaign, nineteenth and final family)
Authority: Contracts 017, 023, and 029; Research 217 and 308; the Oh My Pi
selection, prepared profiles, drivers, protocol decoder, and frozen fixtures;
and the official npm, GitHub, and installed-host channels.

## Question

Is the Oh My Pi RPC package window a compatible extension of the existing
`17.2.9..=17.4.0` claim through published `17.4.1` / `17.4.2` and every
published `18.x` stable through official `18.1.22`, a private milestone that
needs a distinct adapter-private behavior segment, a new driver/facade, or a
stop? Installed `omp/18.1.16` sits inside the assigned family.

## Remaining AllowUnverified rank

Named family only. Standing currentness; not a generation rank rewrite.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Oh My Pi RPC (`oh-my-pi.package`) | installed `omp/18.1.16` | `17.2.9..=17.4.0` | operator-authorized final family; official npm and GitHub stable is `18.1.22` |

Gemini stays deferred. Pi stays a separate family:
`@earendil-works/pi-coding-agent` is `0.85.1` and `pi.package` and
`oh-my-pi.package` never substitute for each other.

## Method

Re-probed npm `@oh-my-pi/pi-coding-agent` `dist-tag latest`, the GitHub latest
release and tag, and the installed host `omp --version` on 2026-09-15. npm
`latest` is `18.1.22`, published `2026-09-14T19:41:55.446Z`. GitHub latest
release is `v18.1.22`, published `2026-09-14T19:29:59Z`, tag commit
`23a5b9ae38864d3f785dc6cbc96eb6d674a1d32d`. Installed `/Users/tom/.local/bin/omp`
prints `omp/18.1.16` with SHA-256
`99eed6d45d984d2f13d76832f78782b9aa07862921ab4e98e2d8e31ca8129795`; it is a
native executable, was not installed, updated, replaced, or executed beyond
`--version`.

Retrieved every published npm stable from the previous ceiling `17.4.0`
through `18.1.22` into `/tmp` and reproduced the registry `integrity` and
`shasum` for all 36 tarballs before extraction. Downloaded artifacts were
never executed. The registry record carries no source commit (`gitHead` is
null) for any point, so the shipped package tree, the GitHub tag commit, and
`docs/rpc.md` are the identity and discovery evidence. Extracted the complete
shipped `package/` tree for each point and built one deterministic
path-and-SHA-256 inventory per point plus the per-hop added / removed / changed
ledger.

Read the official release notes (`CHANGELOG.md`, shipped in every tarball) as
the behavioural discovery authority, then inspected only the shipped source
that contains or feeds the selected `--mode rpc` route: `src/modes/rpc/**`,
`src/jsonrpc/message-framing.ts`, `src/cli/flag-tables.ts`, and
`src/modes/index.ts`. Compared `docs/rpc.md` git blobs at each tag through the
GitHub contents API. No whole-package repetition and no binary archaeology.

Reproduced Research 217's retained identity exactly: npm `18.0.5` integrity
and shasum, extracted `dist/cli.js` `3edd2768e2ace4fdc034c8c2f8579d8e21c97954e2605ace51e86283a8be9651`
at size `19316793`, GitHub `v18.0.5` commit `eab72e88e447a4be45bea2bc302995844c0c51a2`,
and `docs/rpc.md` blob `310b44702de66b4eecd6da37660f9fc40075d973`.

No provider prompt, login, credential, catalogue call, live session,
installation, host update, downloaded-artifact execution, release, tag,
publication, or consumer mutation occurred.

## Identity

npm `latest` and the GitHub latest release/tag agree on `18.1.22`. The full
36-row published ledger (npm published time, integrity, shasum, tarball
SHA-256, `dist/cli.js` SHA-256 and size, shipped file count, package version,
GitHub tag commit) is frozen in
`tests/fixtures/oh-my-pi-18.1.22/identity.json`; the complete per-hop shipped
tree delta is in `dist-inventory.json`.

Published npm stables after the previous ceiling `17.4.0` are exactly the
36 points `17.4.1`, `17.4.2`, `18.0.0`, `18.0.1`, `18.0.3..=18.0.11`,
`18.1.0..=18.1.6`, and `18.1.8..=18.1.22`. GitHub tags `v17.4.3`, `v17.4.4`,
`v18.0.2`, and `v18.1.7` exist with no published npm package. They stay
incompatible and are never inferred from the tag, the neighbouring versions, or
the shared major line. No npm `17.4.3` / `17.4.4` and no later stable above
`18.1.22` exists; `18.1.23` is the synthetic later-stable point. Not a Pi
package point.

The shipped-tree inventory shows 1773 of the 2908–3156 files byte-identical
across all 36 compared versions (digest
`2844f8eaab5fad6fe429e053705a072b3b57901a6eef1781812370bf4a046cbc`). Per-hop
`dist/cli.js` digests and sizes, tarball digests, and file counts are frozen in
the inventory.

## Selected protocol

Selected Swallowtail argv is unchanged: `--mode rpc`, `--no-session`,
`--provider`, `--model`, `--tools read,grep,glob,todo,ask`, `--no-extensions`,
`--no-skills`, `--no-rules`, `--no-prewalk`, `--approval-mode always-ask`, and
the catalogue path `--no-tools`. `--mode` still accepts `text`, `json`, `rpc`,
and `rpc-ui`.

Selected RPC commands are unchanged: `negotiate_protocol` v2, `set_model`,
`set_thinking_level`, `set_auto_retry`, `set_auto_compaction`,
`set_steering_mode`, `set_follow_up_mode`, `set_interrupt_mode`, `get_state`,
`get_available_models`, `prompt`, `steer`, `follow_up`, and `abort`.

Wire invariants are unchanged: the `ready` frame still advertises
`protocolVersion` 1 with `supportedProtocolVersions` `[1, 2]`,
`maxFrameBytes` 1048576, and `maxReassembledFrameBytes` 67108864; protocol
version 2 is negotiated; strict-LF JSONL framing, `rpc_chunk` reassembly
(`chunkId`, `index`, `count`, `byteLength`, `data`), `response`
(`id`, `command`, `success`, `data`), `agent_end` `isTerminal` terminal
semantics, assistant `message_end` `stopReason: "error"` provider failure,
usage `input` / `output` / `cacheRead` / `cacheWrite` fields, tool
`toolCallId` / `toolName` / `isError` fields, and the `select` / `confirm` /
`input` / `editor` plus `notify` / `setStatus` / `setWidget` / `setTitle` /
`set_editor_text` UI methods all hold.

`docs/rpc.md` is blob `310b44702de66b4eecd6da37660f9fc40075d973` at every tag
from `v17.4.2` through `v18.1.22`; it changed only once, from `v17.4.0` to
`v17.4.2`, to document optional `optionDetails` on `select` with the explicit
statement that hosts which do not render descriptions "can continue using
`options` alone".

## Selected-source ledger

`tests/fixtures/oh-my-pi-18.1.22/protocol.json` carries the ten mapped RPC
files, their per-version SHA-256 digest groups, and one classification per
changed mapped hop. Byte-identical across all 36 compared versions:
`src/jsonrpc/message-framing.ts`, `src/modes/rpc/rpc-input.ts`,
`src/modes/rpc/rpc-messages.ts`, and `src/modes/rpc/host-uris.ts`.
`src/modes/rpc/rpc-types.ts` is byte-identical from `17.4.2` through `18.1.22`.

Per-hop verdict for every hop that touches a mapped or selected-support file:

| Hop | Verdict |
| --- | --- |
| `17.4.0→17.4.1` | compatible: `rpc-mode.ts` adds the unmapped host `runCommandInBackground` callback and reports the consumed-builtin `prompt` response field `agentInvoked` accurately; the adapter reads only `prompt` `success` |
| `17.4.1→17.4.2` | compatible: optional select `optionDetails` is added and `requestRpcSelect` extracted; `options` stays the positionally aligned label array and the adapter ignores the extra field |
| `17.4.2→18.0.0` | unmapped: only `src/modes/index.ts` adds a `./composer` barrel export |
| `18.0.6→18.0.7` | unmapped: `loadSlashCommands` receives `session.effectiveExtensionRoots` for the `available_commands_update` lifecycle record; the route runs `--no-extensions` |
| `18.0.7→18.0.8` | unmapped: the embedded `RpcClient` gains an optional `command` launcher override; Swallowtail owns the child process and never uses it |
| `18.0.8→18.0.9` | unmapped: `RpcClient` gains an optional `spawn` transport and nullable stdin flush; `eventBus` parameters are renamed to `subagentEventBus` / `observabilityBus`; the adapter does not use `RpcClient` or subscribe subagents |
| `18.0.9→18.0.10` | unmapped: `src/cli/flag-tables.ts` appends `SESSION_SOURCE_FLAGS` and `restartArgv` for in-place `/restart`; the selected flag tables are unchanged |
| `18.0.11→18.1.0` | unmapped: `rpc-frame.ts`, `rpc-mode.ts`, and `host-tools.ts` receive only `oxlint-disable-next-line` comments and line-wrap reflows |
| `18.1.2→18.1.3` | unmapped: the `/skill:*` prompt branch acknowledges before the expensive dispatch; `--no-skills` makes `enableSkillCommands` falsy, so the changed branch is not selected and the plain-prompt path is untouched |
| `18.1.5→18.1.6` | unmapped: the embedded `RpcClient` adds `tool_stream_update` to its forwarded-event name set; the server already forwarded session events and the adapter bounds unknown namespaced events and ignores them in the turn state machine |
| `18.1.16→18.1.17` | unmapped: only `src/modes/index.ts` preserves a `planSaveFileName` barrel re-export |
| `18.1.17→18.1.18` | resolved: `set_steering_mode`, `set_follow_up_mode`, and `set_interrupt_mode` now call the session setters with `persist: false`; the session-local mode is still applied and `get_state` still reports it, so `state_matches` is unchanged and only the machine-global `config.yml` write is removed |

Every other published hop changes no mapped or selected-support file. In
particular `rpc-mode.ts` is byte-identical `17.4.2..=18.0.6` and
`18.1.18..=18.1.22`.

## Unmapped extras

Subagent subscription commands and events, `get_subagents`,
`get_subagent_messages`, and nested subagent visibility; the shipped
`RpcClient` launcher, custom `spawn` transports, SSH/remote launcher builders,
and its forwarded-event name set; slash-command and `/skill:*` prompt branches;
host-tool and host-URI injection; imported and legacy session usage repair
(`--no-session`); provider-internal agent-core, compaction, telemetry,
autocomplete, TUI, MCP, browser, SSH, speech, and background-job internals;
and all later event names the server forwards. The adapter maps none of them,
and unknown namespaced events cannot complete or fail a turn.

Research 217's tag-based statement that `rpc-types.ts` and `rpc-mode.ts`
"change at `18.0.0`" is corrected: from the published artifacts both land in
`17.4.2`, and `18.0.0` adds only unmapped barrel or internal code. The
historical `oh-my-pi-18.0.5` fixture is retained unchanged as the record of
that earlier stopped run.

## Decision

**Compatible extension plus private major-line milestone.** Extend the
retained `17.x` segment from `17.2.9..=17.4.0` to `17.2.9..=17.4.2` on the
unchanged `oh-my-pi.rpc-v2-v17.2.9` behavior revision, and admit a distinct
adapter-private `18.x` segment `18.0.0..=18.1.22` on the new
`oh-my-pi.rpc-v2-v18.0.0` behavior revision. `18.0.2` and `18.1.7` are
explicit exclusions. `17.4.3` and `17.4.4` sit between the segments and
therefore stay incompatible. `18.1.23` stays permitted `UnverifiedNewer` on
the `18.x` revision.

Contract 029 gives a claim its own revision: adding a milestone changes
qualified membership and support authority, so the claim id moves from
`oh-my-pi.rpc.package-window-1` to `oh-my-pi.rpc.package-window-2`. Because the
`18.x` segment carries the newest behavior revision, the retained `17.x`
segment is labeled `Deprecated` by Contract 029's derived support status while
remaining executable. Baseline `17.2.9`, the `AllowUnverified` posture, the
frozen `oh-my-pi-rpc-17.2.9` decoder corpus, and every historical specimen
stay. No new public operation, shared type, or shared behavior revision. The
adapter's dispatch is unchanged and accepts either segment behavior revision.

Card g05.079 owns the claim change.

This record edits no production claim.

## Sources

- npm `@oh-my-pi/pi-coding-agent` `17.4.0` through `18.1.22`
- [GitHub `v18.1.22`](https://github.com/can1357/oh-my-pi/releases/tag/v18.1.22)
- `CHANGELOG.md` and `docs/rpc.md` at every tag `v17.4.0` through `v18.1.22`
- frozen `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.1.22/`
  and the historical `oh-my-pi-17.4.0/` and `oh-my-pi-18.0.5/` corpora
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md),
  [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md),
  and [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Research 217](./217-oh-my-pi-18-identity.md) and
  [Research 308](./308-all-route-version-currentness-checkpoint.md)
