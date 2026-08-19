# 148 Goose ACP 1.46.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 266

## Question

Is official Goose CLI `v1.46.0` ACP a distinct stdio wire that can freeze
initialize plus one bounded `session/prompt` without flattening onto
`goose serve`, `--with-builtin`, desktop, recipes as routing, or Goose
ACP-providers?

## Method

Reconciled Research 144/145 with tagged GitHub sources at `v1.46.0`, the
ACP registry entry `goose` `1.46.0`, and tagged docs under
`documentation/docs/guides/acp-clients.md` plus CLI command docs.

Did not install Goose. Did not download a platform archive. Did not log
in, run `goose configure`, send `initialize`, or send a prompt. Host PATH
has no `goose`.

Observed versions are not qualified claims.

## Identity

| Surface | Value |
| --- | --- |
| Route | `goose.acp` |
| Axis (provisional) | `goose.release` |
| Package (provisional) | `swallowtail-adapter-goose` |
| GitHub tag | `v1.46.0` lightweight commit `98c11ce2ee7b9b302978aa64b1eab7d0895607c7` |
| GitHub commit date | 2026-08-11T22:32:02Z |
| GitHub release | published 2026-08-12T16:05:13Z |
| Workspace version | `1.46.0` |
| Canonical repo URL | `https://github.com/block/goose` |
| Current GitHub name | `aaif-goose/goose` (API redirect from `block/goose`) |
| ACP registry | `goose` `1.46.0`, binary args `["acp"]` |
| Host | absent |
| ACP crates | `agent-client-protocol` `1.0`, schema `1.1` with `unstable` |

Registry darwin-aarch64 archive SHA-256 (not downloaded):
`de263fb06839de31345dff08aeba999ba165b023cd3cec7ec3bef20f6f4f7e73`.

Swallowtail binds a host-approved `goose` executable plus `acp`. It does
not wrap `goose serve`, desktop, or the ACP-provider adapters that let
Goose call Claude/Codex/Pi as providers.

## Selected wire

Entrypoint: `goose acp`. Stdio JSON-RPC. Tagged CLI
`Command::Acp` calls `goose::acp::server::run(builtins, enable_scheduler)`
which listens on stdin/stdout. Zed custom-agent docs use
`"args": ["acp"]`. ACP registry distribution args are `["acp"]`.

`--with-builtin` is optional on `goose acp` and has no clap default. Empty
`builtins` plus empty `AcpBuiltinSelection.defaults` means stdio ACP does
not inject developer by argv. Host Goose config still owns enabled
extensions. `goose serve --with-builtin` defaults to `developer` when
omitted; that is a different command.

First useful op:

1. `initialize`
2. `session/new` with an absolute existing `cwd` and `mcpServers: []`
3. one bounded `session/prompt`
4. `session/cancel` if the turn is still live
5. join/kill the child

Source-derived `initialize` result (`GooseAcpAgent::on_initialize`):

- `protocolVersion`: echoed client request
- `agentCapabilities.loadSession`: `true`
- `sessionCapabilities`: `list` and `close` advertised
- `promptCapabilities`: `{ image: true, audio: false, embeddedContext: true }`
- `mcpCapabilities.http`: `true`
- `agentInfo.name`: `"goose"`
- `agentInfo.version`: workspace `1.46.0`
- `authMethods`: `goose-provider` / `"Configure Provider"` with description
  `Run goose configure to set up your AI provider and API key`

`session/new` requires an absolute existing directory `cwd`. Provider and
model come from host Goose config (`get_goose_provider` /
`get_goose_model`) unless `_meta` or a recipe supplies them. Missing
provider/model fails as JSON-RPC internal error. Swallowtail does not run
`goose configure` or ACP `authenticate`.

`session/prompt` converts text, image, embedded resource, and resource-link
blocks; audio is dropped. Stop mapping: cancelled → `cancelled`, output
token limit → `max_tokens`, otherwise `end_turn`. Unknown session fails.
`session/cancel` cancels the in-flight prompt token; unknown session is a
no-op success.

Mapped activity from source: `agent_message_chunk`,
`agent_thought_chunk`, `user_message_chunk`, tool-call lifecycle, plus
`usage_update`, `config_option_update`, and `session_info_update`. Custom
`_goose/unstable/session/update` stays unmapped.

Permission kinds include `allow_once`, `allow_always`, `reject_once`,
`reject_always`. Session modes include `auto`, `approve`, `smart_approve`,
and `chat`. `auto` is described as automatically approving tool calls.
Contract 015 still forbids Swallowtail choosing `allow_always` or `auto`.

## Authority

Auth method is provider-owned `goose configure`. Swallowtail does not log
in. Host-owned `~/.config/goose/` and optional `GOOSE_PROVIDER` /
`GOOSE_MODEL` stay outside the prepared plan until a later card proves
them as Swallowtail selection.

Working resource is `session/new` `cwd`. Isolation is one owned stdio
child. Cleanup is cancel in-flight prompt, then join or kill.

`--with-builtin` is not Swallowtail's first argv. `--enable-scheduler` is
off.

## Unmapped on this corpus

`goose serve`, HTTP/WebSocket/TLS, `--dangerously-unauthenticated`,
`--with-builtin`, `--enable-scheduler`, desktop, TUI, recipes as routing
policy, Goose ACP-providers (Amp/Claude/Codex/Pi), `session/load`,
`session/list`, `session/close`, fork, steer, set-model as a Swallowtail
operation, client MCP servers, image/embedded prompt blocks, custom
Goose notifications, usage as a portable usage-evidence claim.

`loadSession: true` plus list/close are advertised and stay unmapped until
a later card proves them.

## Decision

Admit `goose.acp` as a first-party ACP stdio route. Freeze identity and
named fixtures under
`crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.46.0/`.
Card 267 may create the package and decoder. No production claim in this
card.

## Non-goals

- installing Goose or extracting a platform archive
- live initialize, prompt, configure, or OAuth
- `goose serve`, `--with-builtin`, version-range claims
