# 156 Kiro ACP 2.18.1 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 291

## Question

Is official Kiro CLI ACP a distinct stdio wire that can freeze initialize
plus one bounded `session/prompt` without flattening onto
`kiro-cli chat --no-interactive`, `--cloud`, TUI, `_kiro.dev/*`
extensions, or advertised `session/load` continuation?

## Method

Reconciled Research 153 with official ACP docs (page updated 2026-08-18),
authentication docs, CLI changelog, installer script
`https://cli.kiro.dev/install`, and
`https://prod.download.cli.kiro.dev/stable/latest/manifest.json`.

Checked ACP registry
`https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
version `1.0.0` (38 agents). No Kiro row. Registry absence is not a stop.

Did not run the installer. Did not download a platform archive or DMG.
Did not log in, send `initialize`, or send a prompt. Host PATH has no
`kiro-cli` or `kiro`.

Observed versions are not qualified claims.

## Identity

| Surface | Value |
| --- | --- |
| Route | `kiro.acp` |
| Axis (provisional) | `kiro-cli.release` |
| Package (provisional) | `swallowtail-adapter-kiro` (create in card 292) |
| Official channel | `https://cli.kiro.dev/install` → `prod.download.cli.kiro.dev/stable/latest` |
| Manifest version | `2.18.1` |
| Manifest SHA-256 | `a79ea0db26bc43848ff83e80a1873fd32649a485c16cc6df9b8fd15f8b884e5f` |
| Installer script SHA-256 | `91a21bfa05cd7b58601cb83e0f1f187a9d0084726e5b824d4a4cf60306250908` |
| ACP docs HTML SHA-256 | `7ff8826ab0f7bbb1cbca636d3fd27ba4c640f6604298ec47dbe8242e6e6135c5` |
| Changelog headline | `2.18.0` dated 2026-08-12; no named `2.18.1` notes page |
| Binary | `kiro-cli`; macOS DMG `cliPath` `Contents/MacOS/kiro-cli` |
| ACP registry | absent |
| Host | absent |

Platform artifacts recorded from the manifest, not extracted:

| Artifact | SHA-256 |
| --- | --- |
| `2.18.1/Kiro CLI.dmg` | `07893e9477c8d296ebc653192648772fd305718da4bf3a97583718e122c47061` |
| `2.18.1/kirocli-aarch64-linux.tar.xz` | `c1f860b63f6656501dbcf8995e2687c3e31142b95755c76ce99f8d18000017df` |

Swallowtail binds a host-approved `kiro-cli` executable plus `acp`. It
does not wrap `kiro-cli-chat`, npm `kiro-cli@0.0.1`, PyPI `kiro-cli`,
`kirodotdev/Kiro` (issue tracker, not CLI source), or
`aws-samples/sample-kiro-acp-ui`.

Linux manifest `variant: headless` names installer archives. That is not
the deferred Swallowtail sibling `kiro.headless`
(`kiro-cli chat --no-interactive`).

## Selected wire

Entrypoint: `kiro-cli acp`. Stdio JSON-RPC 2.0. Official JetBrains and
Zed examples spawn `["acp"]`. `--agent <name>` is optional and is not
first argv.

First useful op:

1. `initialize` with `protocolVersion: 1`
2. `session/new` with `{cwd, mcpServers: []}`
3. one bounded `session/prompt` of text blocks under field `prompt`
4. `session/cancel` if the turn is still live
5. join/kill the child

Official docs still show `session/prompt` params field `content`. That
example is not the selected payload. `kirodotdev/Kiro#7144` records the
server requiring `prompt` (`missing field prompt`) and hanging instead of
returning JSON-RPC invalid-params. Docs page still showed `content` on
2026-08-18. First driver sends `prompt`. Sending `content` is a named
negative.

Docs initialize **result** example:

- `protocolVersion`: 1
- `agentCapabilities.loadSession`: `true`
- `promptCapabilities.image`: `true`
- `agentInfo.name`: `"kiro-cli"`
- `agentInfo.version`: `"1.5.0"` (stale example; not the frozen CLI)

Initialize result is otherwise unrecovered from public source. The first
driver must fail closed on unexpected initialize results rather than
inventing them. Swallowtail initialize request advertises
`fs.readTextFile` / `writeTextFile` / `terminal` false.

Session updates: docs name `session/notification` types
`AgentMessageChunk`, `ToolCall`, `ToolCallUpdate`, `TurnEnd`. ACP v1
decode in Swallowtail is `session/update` with `sessionUpdate`
discriminators. First driver uses that decoder and fail-closes unknown
kinds. `TurnEnd` is not a substitute for the `session/prompt` RPC result.

## Authority

Auth is host-owned `kiro-cli login` (browser / device flow) or
`KIRO_API_KEY` for non-interactive CLI. Precedence: active browser
session, then `KIRO_API_KEY`, then prompt to sign in. Swallowtail does
not log in, run `kiro-cli login` / `whoami`, mint API keys, or bind
`KIRO_API_KEY` as a credential lease.

Working resource is `session/new` `cwd`. Isolation is one owned stdio
child. Cleanup is cancel in-flight prompt, then join or kill.

ACP sessions persist under `~/.kiro/sessions/cli/` (`.json` plus
`.jsonl`). That is provider-owned local retention. Swallowtail does not
claim archive, restore, delete, or continuation recovery.
`loadSession: true` stays unmapped.

`--trust-all-tools` / `--trust-tools` are headless flags, not ACP argv.
Permission kinds are unrecovered from public source. Swallowtail does not
auto-select `allow_always`.

## Unmapped on this corpus

`kiro.headless`, `--cloud`, `--resume-id`, `--agent`, TUI
`kiro-cli-chat`, `kiro-cli login`, `session/load`, `session/set_mode`,
`session/set_model`, `_kiro.dev/*` extensions, `_session/terminate`,
image-only prompts, client MCP servers, usage as portable usage-evidence,
and wrapping npm/PyPI `kiro-cli`.

## Decision

Admit `kiro.acp` as a first-party ACP stdio route. Freeze identity and
named fixtures under
`crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.18.1/`.
Card 292 may create the package and decoder. No production claim in this
card. Counts stay 38 packages / 45 production routes.

## Non-goals

- installing Kiro or extracting a platform archive
- live initialize, prompt, login, or API-key use
- `kiro.headless`, `--cloud`, version-range claims
