# Research 460: Claude Agent ACP 0.98.0 Identity and Currentness

Evidence checked 2026-10-28 for `claude-agent.acp` only. Claude Code and
`claude-agent.sdk` are separate families and are unchanged here.

## Official current point

Re-probed at the start of the run. npm `latest` is `0.98.0`. npm `preview` is
`0.98.1-preview.1` and is ignored. GitHub release `v0.98.0` is not a draft or
prerelease, published 2026-10-27T18:48:47Z at
`c90f856c11b959d40328f86accc48f3a537aae96`. ACP registry id `claude-acp` reports
`0.98.0`. npm `gitHead` matches that tag. The exact npm integrity is
`sha512-6UGEODegNXXgS6wKX9T1v72oSLVtSmM1N/9H780ljqkaR7B+OdlfMYFdscNurfodt9l3xjGy0Bga/NtnLs/Vlw==`;
the downloaded tarball SHA-256 is
`94cdee902d7d4ddb0360b330c0be9f728aae3370b3028b7d835856ab33e20c8b`.
The package pins ACP SDK `2.0.2` and Claude Agent SDK `0.3.310`, requires Node
`>=22`, and exposes `dist/index.js` as its binary entry. Host
`claude-agent-acp` was absent from `PATH`.

The previous qualified ceiling is `0.96.0` (Research 446). Its tarball SHA-256
reproduces `cb70069d2538e481795ca1853888107fd53c0754dea78814467ba1e26f72b9d6`.

Sources: [official npm package](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.98.0),
[official GitHub tag](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.98.0),
and the [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json).

## Published stable hops

The complete npm version list has these stable points after the previous
qualified ceiling `0.96.0` and through current `0.98.0`:

| Version | Published | GitHub tag commit | ACP SDK | Claude Agent SDK |
| --- | --- | --- | --- | --- |
| `0.97.0` | 2026-10-26 12:48:22Z | `d52c7c64d8e4b3e5bb988658fed4cd487b9f935c` | `2.0.2` | `0.3.310` |
| `0.98.0` | 2026-10-27 18:53:50Z | `c90f856c11b959d40328f86accc48f3a537aae96` | `2.0.2` | `0.3.310` |

GitHub `v0.97.0` is not a draft or prerelease, published 2026-10-26T12:46:30Z.
For each point, npm integrity metadata was checked against the downloaded
tarball, and npm `gitHead` matched its exact GitHub tag. The `0.97.0` tarball
SHA-256 is `e41a8e21333c5ab6e94bdd9b7c58536f10897a9d07e54058c9e9e73fde41e5b9`.
The retained fixture records every integrity value, tarball SHA-256, package
dependency identity, and complete file tree in
[`claude-agent-acp-0.98.0`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.98.0/README.md).
No file was removed across the two stable hops. Preview-only patch lines are
`0.96.1` and `0.97.1`. The historical `0.52.0` and unpublished `0.58.0`
exclusions remain.

`0.96.0` has 282 files. `0.97.0` and `0.98.0` have 285. `dist/index.js`,
`dist/serve.js`, `dist/tools.js`, and `dist/session-mode.js` are
byte-identical from `0.96.0` through `0.98.0`. `deleteSessionIfStored` is
byte-identical.

`0.96.0` to `0.97.0` adds 3 files, removes none, and changes 16. Twelve of
those added or changed files are mapped-adjacent and seven are
provider-internal. 266 files are unchanged. `0.97.0` to `0.98.0` adds none,
removes none, and changes 24. Twelve are mapped-adjacent and twelve are
provider-internal. 261 files are unchanged.

## Selected ACP v1 review

Swallowtail still sends ACP `protocolVersion: 1`, file read only, and form
elicitation. It does not advertise AIR, goal, async tasks, session notices,
context compaction, native subagent sessions, `sessionIndex`, or boolean
config options. Selected routes stay initialize, session new, prompt, cancel,
load, resume, close, delete, request permission, elicitation create, set
config option, and set mode.

`CLAUDE_AGENT_ACP_PROTOCOL` lives in `dist/index.js` and `dist/serve.js`.
Both stay byte-identical with `0.96.0`. Unset, empty, and `auto` still serve
a v1 client the v1 agent. Swallowtail does not set the env and does not
request v2.

`0.97.0` adds config id `ultracode`. Swallowtail selects `model`, `effort`,
and `mode` by id and ignores the extra option. The config decoder already
accepts that select or boolean shape. Duplicate model display names gain a
parenthetical model id; Swallowtail selects the model value. Advisor tool
results emit an existing tool result. Swallowtail already decodes `pending`,
`in_progress`, `completed`, and `failed`. A usage-limit message after
compaction still settles `end_turn`. Auth-status `--settings` order,
body-parser, foreground-fork, and live-steer fixes stay on unselected CLI or
provider-internal paths.

`0.98.0` adds AIR `_session/batch_get` (`sessionBatchGet`), advertised only
with `sessionIndex`. Swallowtail does not advertise `sessionIndex`. Selected
`session/delete` still uses `deleteSessionIfStored`. From `0.91.0` through
`0.98.0`, delete of an absent transcript returns `{}` for this non-AIR
client. That empty success is not a stronger deletion claim. Live close of a
session in memory and delete of a stored transcript still return `{}`.
SandboxNetworkAccess permission answers emit `tool_call_update` `completed`
or `failed`. A v1 permission subject still includes `status: pending`; the
omission is v2-only. Monitor async tasks publish only when async tasks are
enabled. Swallowtail does not advertise them.

Permission kinds stay `allow_once`, `allow_always`, and `reject_once`.
`set_mode` still returns `{}`. Usage fields stay `inputTokens`,
`outputTokens`, `cachedReadTokens`, `cachedWriteTokens`, and `totalTokens`.
Stop reasons stay `end_turn`, `max_tokens`, `max_turn_requests`, `refusal`,
and `cancelled`. Unknown session updates still decode as unknown and are
ignored. ACP v2 stays unqualified.

## Decision and limits

This is a compatible extension of `claude-agent.acp.initialize-meta-extensions-v7`.
Extend the maintained segment `0.66.0..=0.96.0` through `0.98.0`.
Keep baseline `0.53.0`, claim id `claude-agent.acp.window-2`, the seven
behavior milestones, exclusions `0.52.0` and `0.58.0`, and `AllowUnverified`.
`0.99.0` is the synthetic `UnverifiedNewer` point. Prerelease points,
including `0.98.1-preview.1`, remain incompatible without their own exact
segment.

No new public mapped operation was added. HTTP MCP live honouring remains
exact to `0.79.0` and `0.81.2` (Research 361 and 364). The root package pin
stays `0.81.2`. No tag, publication, host install, package install, provider
prompt, live initialize, credential use, or host mutation occurred.
