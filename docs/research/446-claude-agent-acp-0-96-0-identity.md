# Research 446: Claude Agent ACP 0.96.0 Identity and Currentness

Evidence checked 2026-10-26 for `claude-agent.acp` only. Claude Code and
`claude-agent.sdk` are separate families and are unchanged here.

## Official current point

Re-probed at the start of the run and again before this record. npm `latest`
is `0.96.0`. npm `preview` is `0.96.1-preview.7` and is ignored. GitHub release
`v0.96.0` is not a draft or prerelease, published 2026-10-26T09:50:23Z at
`f6958db1ee8d3e9ead6767a3a294f98b9a21adb9`. ACP registry id `claude-acp` reports
`0.96.0`. npm `gitHead` matches that tag. The exact npm integrity is
`sha512-C+rp+lFQQdIi8Zg/Q9+jJ0301lkgghe7Phe7XcUuXR8uTlO13Yv2j+9ooi6Jg6cLIfFnjAMGYg3JoT8KKmkM2A==`;
the downloaded tarball SHA-256 is
`cb70069d2538e481795ca1853888107fd53c0754dea78814467ba1e26f72b9d6`.
The package pins ACP SDK `2.0.2` and Claude Agent SDK `0.3.310`, requires Node
`>=22`, and exposes `dist/index.js` as its binary entry. Host
`claude-agent-acp` was absent from `PATH`.

The brief's `0.95.0` is the interior hop published 2026-10-23. The re-probe
target is `0.96.0`.

Sources: [official npm package](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.96.0),
[official GitHub tag](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.96.0),
and the [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json).

## Published stable hops

The complete npm version list has these stable points after the previous
qualified ceiling `0.87.0` and through current `0.96.0`:

| Version | Published | GitHub tag commit | ACP SDK | Claude Agent SDK |
| --- | --- | --- | --- | --- |
| `0.88.0` | 2026-10-08 12:19:49Z | `d43fec3fa3118e88ff8a8ed99b85d2a5f444e538` | `1.7.0` | `0.3.293` |
| `0.89.0` | 2026-10-09 22:06:29Z | `96b6b4f478c5276b727c0a1d3cc3717da2d9e9ae` | `1.8.0` | `0.3.293` |
| `0.89.1` | 2026-10-10 09:11:59Z | `4e1fc1e8df2b9af3b1b2d6880ba90c5862283ba8` | `1.8.0` | `0.3.293` |
| `0.90.0` | 2026-10-13 16:57:17Z | `7f7a1404a15024a45e62b699ced4de3fb7a814a2` | `1.8.0` | `0.3.296` |
| `0.91.0` | 2026-10-15 12:25:22Z | `c0af592d252e6bd072b53407f618a9dd4cd02797` | `1.8.0` | `0.3.300` |
| `0.92.0` | 2026-10-16 12:42:02Z | `af3d7f9335dfe8fc2ea5320649f72b34d3cbf410` | `1.9.0` | `0.3.300` |
| `0.93.0` | 2026-10-16 19:21:32Z | `dbf386d8d35d2bc30f17f11f15d1a4305c9d391f` | `1.9.0` | `0.3.300` |
| `0.94.0` | 2026-10-19 21:31:04Z | `7bcbfc7a5ca7ab9c7eece4f0749ccaf49c7ba7e1` | `2.0.0` | `0.3.303` |
| `0.95.0` | 2026-10-23 20:00:57Z | `ebfbfbdaa9c2a6a8bd9eb0a4965b521009152350` | `2.0.1` | `0.3.309` |
| `0.96.0` | 2026-10-26 09:53:45Z | `f6958db1ee8d3e9ead6767a3a294f98b9a21adb9` | `2.0.2` | `0.3.310` |

For each point, npm integrity metadata was checked against the downloaded
tarball, and npm `gitHead` matched its exact GitHub tag. The `0.87.0` tarball
SHA-256 reproduces the Research 373 fixture
`4c21b1168754fdb5343cf4a4f1661ece28a5d518d3d7738a6d92988855dd4705`. The retained
fixture records every integrity value, tarball SHA-256, package dependency
identity, and complete file tree in
[`claude-agent-acp-0.96.0`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.96.0/README.md).
No file was removed across the ten stable hops. Preview-only patch lines are
`0.87.1`, `0.88.1`, `0.89.2`, `0.90.1`, `0.91.1`, `0.92.1`, `0.93.1`, `0.94.1`,
and `0.95.1`. The historical `0.52.0` and unpublished `0.58.0` exclusions
remain.

## Selected ACP v1 review

Swallowtail still sends ACP `protocolVersion: 1`, file read only, and form
elicitation. It does not advertise AIR, goal, async tasks, session notices,
context compaction, native subagent sessions, or `sessionIndex`. Selected
routes stay initialize, session new, prompt, cancel, load, resume, close,
delete, request permission, elicitation create, set config option, and set
mode.

`0.89.0` adds the AIR `sessionIndex` surface and custom instructions that
travel only in AIR meta. Swallowtail does not advertise either. The same hop
makes `closeSession` of a session that is not in memory return `{}`. Through
`0.88.0` that call threw. Delete of an absent transcript still calls Agent SDK
`0.3.293` `deleteSession` and throws, on `0.88.0` and `0.89.0` alike, for this
non-AIR client. Live close of a session in memory and delete of a stored
transcript still return `{}`. Swallowtail does not expose missing-close
rejection as a consumer operation, and the empty close success is not a
stronger deletion claim.

`0.90.0` closes open tool calls before a cancel reports `cancelled`. The stop
reason stays `cancelled`. Swallowtail already decodes `pending`,
`in_progress`, `completed`, and `failed`. Dropping late streamed output after
cancel stays inside the existing cancel contract.

`0.91.0` narrows models to a `providers/set` gateway list only when that
gateway lists models. Swallowtail does not use `providers/set`. The model
config id stays `model`.

`0.92.0` publishes predicted follow-up prompts as prompt-suggestion meta.
Those suggestions are not prompt-response usage JSON. `0.94.0` moves the ACP
SDK import to `@agentclientprotocol/sdk` `2.0.0` (`unstable` entry). The v1
wire is unchanged. `0.95.0` removes `CLAUDE_AGENT_ACP_EXPERIMENTAL_V2` and
adds `CLAUDE_AGENT_ACP_PROTOCOL`. Unset, empty, and `auto` serve both: a v1
client gets the v1 agent, and a v2 client gets experimental v2. Swallowtail
sends `protocolVersion` 1 and does not set the env, so it still gets v1.
Autonomous turns in `0.95.0` return immediately unless the client advertises
`asyncTasks`. Swallowtail does not. `0.96.0` adds model display lookup. The
model config id stays `model`.

Permission kinds stay `allow_once`, `allow_always`, and `reject_once`.
`set_mode` still returns `{}`. Usage fields stay `inputTokens`,
`outputTokens`, `cachedReadTokens`, `cachedWriteTokens`, and `totalTokens`.
Stop reasons stay `end_turn`, `max_tokens`, `max_turn_requests`, `refusal`,
and `cancelled`. Unknown session updates still decode as unknown and are
ignored. ACP v2 stays unqualified.

## Decision and limits

This is a compatible extension of `claude-agent.acp.initialize-meta-extensions-v7`.
Extend the maintained segment `0.66.0..=0.87.0` through `0.96.0`.
Keep baseline `0.53.0`, claim id `claude-agent.acp.window-2`, the seven
behavior milestones, exclusions `0.52.0` and `0.58.0`, and `AllowUnverified`.
The next synthetic stable point is `0.97.0`; it remains `UnverifiedNewer`.
Prerelease points remain incompatible without their own exact segment.

No new public mapped operation was added. HTTP MCP live honouring remains
exact to `0.79.0` and `0.81.2` (Research 361 and 364). The root package pin
stays `0.81.2`. No tag, publication, host install, package install, provider
prompt, live initialize, credential use, or host mutation occurred.
