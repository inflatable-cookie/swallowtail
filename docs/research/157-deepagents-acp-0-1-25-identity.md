# 157 Deep Agents ACP 0.1.25 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 299

## Question

Does first-party LangChain npm `deepagents-acp` expose a distinct ACP stdio
CLI that can freeze initialize plus one bounded `session/prompt`, or is the
surface still only a library embed / custom `tsx` script?

## Method

Reconciled Research 153 with official ACP docs
`https://docs.langchain.com/oss/javascript/deepagents/acp`, npm
`deepagents-acp@0.1.25`, and ACP registry
`https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
version `1.0.0` (38 agents).

Downloaded the 11-file `deepagents-acp@0.1.25` tarball. Inspected
`package.json`, `dist/cli.js`, and `dist/index.d.ts`. Did not install the
package. Did not run `npx deepagents-acp`, `deepagents-acp`, `--help`, or
`--version`. Did not send `initialize` or a prompt. Did not set
`ANTHROPIC_API_KEY` / `OPENAI_API_KEY`. Host PATH has no `deepagents-acp`.

Observed versions are not qualified claims. No production matrix, package,
or README count changed.

## Identity

| Surface | Value |
| --- | --- |
| Route | `deepagents.acp` |
| Axis (provisional) | `deepagents-acp.package` |
| Package (provisional) | `swallowtail-adapter-deepagents` (create in card 300) |
| Official channel | npm `deepagents-acp` |
| npm version | `0.1.25` published 2026-08-14T16:43:46.024Z |
| Integrity | `sha512-5S6Rpd74vV3YKVxAEqQkXKek+y1ChTpL0D2xf+WLaAYneJQZ9haZ4lPgjPy2VvszqErVsSr+T5tq8vdjuAWShQ==` |
| Tarball SHA-256 | `6a56fa60e985a0681217cd20b1e21c0f7782fb10ebed6728f2865346ba137141` |
| `package.json` SHA-256 | `cddb5563aafc9fc22e67760c2ac906187c69e83d1ed73c36ae13db04c35cdb5e` |
| `dist/cli.js` SHA-256 | `68b7d6cb31d181a399f623a4c6486892bf7d408aec61cac0a3ea9e033baa2319` |
| Docs HTML SHA-256 | `1fb8f2821788368e4b1667f2dfae6edfa3071ce943480ea5f3ff0c3f28809663` |
| Registry JSON SHA-256 | `2365b837490cdb35f828d8bc5d25ea1bdb8aaf1c0dd5b5fbea239c1b1c2b8ba4` |
| Binary | `deepagents-acp` → `dist/cli.js` |
| Repository | `github.com/langchain-ai/deepagentsjs` |
| Runtime deps | `@agentclientprotocol/sdk` `^1.1.0`; `deepagents` `1.12.4` |
| ACP registry | `deepagents` `0.1.7` / `npx deepagents-acp@0.1.7` (stale vs npm latest) |
| Host | absent |

Swallowtail binds a host-approved `deepagents-acp` executable with no extra
argv. It does not wrap `npx`, a custom `tsx` server, `startServer` /
`DeepAgentsServer` library embed, or Python Deep Agents.

Registry membership is discovery only. The frozen package is npm `0.1.25`,
not registry `0.1.7`.

## Selected wire

Entrypoint: `deepagents-acp`. Stdio NDJSON ACP via
`@agentclientprotocol/sdk` `AgentSideConnection`. Official Zed examples
spawn `npx` `["deepagents-acp"]`; Swallowtail does not select `npx`.

CLI default agent name is `"deepagents"`. CLI does not pass
`serverVersion`, so initialize `agentInfo` is `{name: "deepagents-acp",
version: "0.0.1"}`. That constructor default is not the npm package
version. First driver must not treat `agentInfo.version` as
`deepagents-acp.package`.

Working resource is the child's cwd (`--workspace` / `WORKSPACE_ROOT` /
`process.cwd()` at spawn). `session/new` does not read `params.cwd`. First
driver sets the owned child's cwd to the working resource and does not
pass `--workspace`, `--model`, `--name`, `--skills`, `--memory`,
`--debug`, or `--log-file`.

First useful op:

1. spawn `deepagents-acp` with no extra argv; child cwd is the working resource
2. `initialize` with `protocolVersion: 1`; host `fs` and `terminal` advertised false
3. `session/new` with `{cwd, mcpServers: []}`
4. one bounded `session/prompt` of text blocks under field `prompt`
5. `session/cancel` if the turn is still live
6. join/kill the child

`session/prompt` params field is `prompt` (SDK handler `prompt`). Docs do
not show a `content` example. Sending `content` is still a named negative.

Initialize result recovered from `dist/cli.js` `handleInitialize`:

- `protocolVersion`: echo of request, else `1`
- `agentInfo.name`: `"deepagents-acp"`
- `agentInfo.version`: `"0.0.1"` on the CLI path
- `agentCapabilities.loadSession`: `true`
- `promptCapabilities.image`: `true`; `audio`: `false`; `embeddedContext`: `true`
- `mcpCapabilities.http` / `sse`: `false`
- `sessionCapabilities.modes` / `commands`: `true`
- `authMethods`: env-var Anthropic/OpenAI plus generic agent setup

A present `agentInfo.name` must be `deepagents-acp`. Do not require
`agentInfo.version` to equal npm `0.1.25`.

## Authority

Auth is host-owned provider API keys in the isolated environment.
Docs mark `ANTHROPIC_API_KEY` required for the default Claude model.
`authenticate` is a no-op. Swallowtail does not log in, mint keys, or bind
`ANTHROPIC_API_KEY` / `OPENAI_API_KEY` as a credential lease.

Isolation is one owned stdio child. Cleanup is cancel in-flight prompt,
then join or kill.

Sessions live in an in-process `Map` plus LangGraph `MemorySaver`. That is
not durable across process restart. `loadSession: true` and
`session/load` stay unmapped. Default skill/memory files under
`.deepagents/` stay host-owned.

Permission options recovered from source: `allow-once`, `allow-always`,
`reject-once`, `reject-always`. Swallowtail observes and cancels; never
selects `allow_always`. A source fallback that allows on permission-request
failure is not Swallowtail policy.

When host `fs` is advertised false, the ACP filesystem backend falls back
to local `FilesystemBackend` writes in the child cwd. That is not a
Swallowtail bounded-write claim.

## Unmapped on this corpus

`npx`, library `startServer` / `DeepAgentsServer`, custom `tsx` servers,
`--name` / `--model` / `--workspace` / `--skills` / `--memory` / `--debug`
/ `--log-file` as Swallowtail argv, `session/load`, `session/set_mode`,
slash commands (`/plan`, `/agent`, `/ask`, `/clear`, `/status`), HITL
`interruptOn`, image/audio/resource prompt blocks, client MCP servers,
Python Deep Agents, and usage as portable usage-evidence.

## Decision

Admit `deepagents.acp` as a first-party ACP stdio route. Freeze identity
and named fixtures under
`crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.25/`.
Card 300 may create the package and decoder. No production claim in this
card. Counts stay 39 packages / 46 production routes.

## Non-goals

- installing the npm package or running the CLI
- live initialize, prompt, or API-key use
- wrapping `npx` or the programmatic server
- version-range claims
