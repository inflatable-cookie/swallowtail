# 146 Cline ACP 3.0.55 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 262

## Question

Is official Cline CLI `3.0.55` ACP a distinct stdio wire that can freeze
initialize plus one bounded `session/prompt` without flattening onto
`--json`, TUI, hub, `--id` resume, or `--auto-approve true`?

## Method

Reconciled Research 144/145 with official docs, npm `cline@3.0.55`
metadata, the ACP registry entry, GitHub annotated tag `cli-v3.0.55`, and
the tagged ACP TypeScript sources under `apps/cli/src/acp/`.

Downloaded the 6-file npm wrapper tarball. Did not install Cline. Did not
download a platform binary. Did not log in, send `initialize`, or send a
prompt. Host PATH has no `cline`.

Observed versions are not qualified claims.

## Identity

| Surface | Value |
| --- | --- |
| Route | `cline.acp` |
| Axis (provisional) | `cline.package` |
| Package (provisional) | `swallowtail-adapter-cline` |
| npm | `cline@3.0.55`, published 2026-08-14T07:55:21.353Z, `latest` on 2026-08-18 |
| Wrapper tarball SHA-256 | `7eec2ad80d8dfa27b9baaa22c7340ebe861850f6057b9e2e80a5dd9d2ef2f5ef` |
| Wrapper integrity | `sha512-3JQ5vPl8/BIyTShTpn0VDX59lbzFhGxRwYqZmAVLOPcX83ETaHbgH2fc9iJYv1pB2ChIpXQEXmFHowgz8F1GgQ==` |
| Wrapper files | 6; unpacked 46371 bytes |
| GitHub tag | `cli-v3.0.55` annotated `c238103e631d492b97bf9e63b060390f1bb8a8a6` |
| GitHub commit | `ad442cbb6a81d21773ceabc1398ea5eb58170718` |
| ACP registry | `cline` `3.0.55`, npx args `--acp` (discovery only) |
| Host | absent |

`cline` on npm is a Node wrapper that resolves an optional platform
package (`@cline/cli-darwin-arm64@3.0.55` and siblings). The ACP child is
that compiled binary with `--acp`. Swallowtail binds the host-approved
`cline` executable plus `--acp`; it does not wrap Node, Bun, or
`@cline/core`.

Platform package metadata (not extracted):

| Package | Integrity |
| --- | --- |
| `@cline/cli-darwin-arm64@3.0.55` | `sha512-9qSF6jH5hOYsWC8s3YgraNcJkOBZ8Aa+ia4uZ0iuPY8pOEzcWZEhtOITtfGzitk1aPJxNKegnTASOkQunJnHJw==` |

Tagged CLI depends on `@agentclientprotocol/sdk` `^0.16.1`.

## Selected wire

Entrypoint: `cline --acp`. Stdio NDJSON JSON-RPC via
`AgentSideConnection` / `ndJsonStream`. Console diagnostics go to stderr;
stdout is ACP only.

`--acp` is mutually exclusive with interactive/piped CLI modes in
`apps/cli/src/main.ts`. `--json` is the headless NDJSON `ask`/`say`
surface owned by g03.087; it is not this route.

First useful op:

1. `initialize`
2. `session/new`
3. one bounded `session/prompt`
4. `session/cancel` if the turn is still live
5. join/kill the child

Source-derived `initialize` result (`AcpAgent.initialize`):

- `protocolVersion`: SDK `PROTOCOL_VERSION` (ACP v1)
- `agentCapabilities.loadSession`: `true`
- `promptCapabilities`: `{ image: true, audio: false, embeddedContext: false }`
- no `mcpCapabilities` advertisement
- no `sessionCapabilities.close|delete|list|resume` advertisement
- `agentInfo.name`: CLI `displayName` `"cline"`
- `agentInfo.version`: `"3.0.55"`
- `authMethods`: `cline`, `cline-pass`, `openai-codex`

`session/new` requires a restored or env credential (`CLINE_API_KEY`) or
ACP `authenticate`. Swallowtail does not run OAuth `authenticate`. Host
supplies a credential reference / env; missing authority fails closed.

`session/prompt` extracts text blocks only. Empty text returns
`stopReason: "end_turn"`. Finish mapping: `completed` → `end_turn`,
`aborted` → `cancelled`, `max_iterations` → `max_turn_requests`. Fatal
agent errors become JSON-RPC `auth_required` or `internalError`.

`session/cancel` aborts the in-flight prompt AbortController. Unknown
session is a no-op.

Mapped activity from source `session-updates.ts`:
`agent_message_chunk`, `agent_thought_chunk`, `tool_call`,
`tool_call_update`, plus `current_mode_update`, `config_option_update`,
and `session_info_update`. Usage/iteration/error events are not forwarded
as session updates.

Permission options: `allow_once`, `allow_always`, `reject_once`. Contract
015 still forbids Swallowtail choosing `allow_always`.

## Authority

CLI `--auto-approve` help-default is `true`. ACP launch ignores that
default and passes `autoApproveTools` only when
`args.autoApproveOverride === true`. `AcpAgent` then defaults
`autoApproveTools` to `false`. Swallowtail does not pass
`--auto-approve true`.

Auth methods are provider-owned OAuth. `CLINE_API_KEY`, `CLINE_PROVIDER`,
and `CLINE_MODEL` are documented ACP env. Swallowtail does not log in.

Working resource is `session/new` `cwd`. Isolation is one owned stdio
child. Cleanup is abort in-flight work, `shutdown`/dispose, then join or
kill.

## Unmapped on this corpus

`--json`, `-i/--tui`, `--id`, `-z/--zen`, hub, kanban, schedule, teams,
`--worktree`, `--yolo`, `session/load`, `setSessionMode`,
`unstable_setSessionModel`, `setSessionConfigOption` (provider / model /
mode / `auto_approve` / organization), ACP `authenticate`, image prompt
blocks, MCP servers, plan/act as a Swallowtail operation.

`loadSession: true` is advertised and stays unmapped until a later card
proves it.

## Decision

Admit `cline.acp` as a first-party ACP stdio route. Freeze identity and
named fixtures under
`crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.55/`.
Card 263 may create the package and decoder. No production claim in this
card.

## Non-goals

- installing Cline or extracting the platform binary
- live initialize, prompt, catalogue, or OAuth
- `cline.headless`, version-range claims, README crate-count repair
