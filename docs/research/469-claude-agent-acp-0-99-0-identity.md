# Research 469: Claude Agent ACP 0.99.0 Identity and Currentness

Evidence checked 2026-11-03 for `claude-agent.acp` only. Claude Code and
`claude-agent.sdk` are separate families and are unchanged here. This record
covers the first hop from the `0.98.0` ceiling (Research 460) toward official
`0.100.0` (Research 471).

## Official current point

Official npm `latest` is `0.100.0`, re-probed at the start of the run. This
hop's own official point was current when published: `0.99.0` reached npm at
2026-10-28T14:27:15Z, after Research 460's identity commit
(`c90f856c11b959d40328f86accc48f3a537aae96`, committed 09:37:26Z) had landed,
which is why the `0.98.0` record named `0.99.0` as its synthetic
`UnverifiedNewer` point. GitHub release `v0.99.0` is not a draft or prerelease,
published 2026-10-28T14:25:02Z at
`106e3031e393daebc7170c3fe9ed46d0e0ee6c40`. The ACP registry id `claude-acp`
now reports `0.100.0`. npm `gitHead` matches the tag. The exact npm integrity
is
`sha512-xT6Ajub9fa9LeMpQU6a+/nHEqI5lefvgWAP8liiD1EZBv3vTXKEnKq5mRpKCu2089CHpFXu7Y/PEHCH5iPWfPw==`;
the downloaded tarball SHA-256 is
`03e7997feee9548bd5a2e180ec9e0a7e462fe03ca278b29b2b1050b8d7f02790`.
The package pins ACP SDK `2.1.0` and Claude Agent SDK `0.3.312`, requires Node
`>=22`, and exposes `dist/index.js` as its binary entry. Host
`claude-agent-acp` was absent from `PATH` in the observation environment; that
observation belongs to that environment and is not a claim of universal
absence.

The previous qualified ceiling is `0.98.0` (Research 460). Its tarball SHA-256
reproduces
`94cdee902d7d4ddb0360b330c0be9f728aae3370b3028b7d835856ab33e20c8b`.

## Published stable hops

The complete npm version list has these stable points after the previous
qualified ceiling `0.98.0` and through current `0.100.0`:

| Version | Published | GitHub tag commit | ACP SDK | Claude Agent SDK |
| --- | --- | --- | --- | --- |
| `0.99.0` | 2026-10-28 14:27:15Z | `106e3031e393daebc7170c3fe9ed46d0e0ee6c40` | `2.1.0` | `0.3.312` |
| `0.99.1` | 2026-10-29 19:37:04Z | `0f78dbbdd089e5c64fc908173e8656aa9cc757fb` | `2.1.0` | `0.3.312` |
| `0.100.0` | 2026-11-02 14:38:37Z | `981bf9a28ac3afef26e7877a2a638814c34f1679` | `2.4.0` | `0.3.316` |

The dispatch named hops `0.99.0` and `0.100.0`; `0.99.1` is also a published
stable between them, so the run walks all three and each gets its own record
(Research 470, Research 471). For each point, npm integrity metadata was
checked against the downloaded tarball, and npm `gitHead` matched its exact
GitHub tag. Preview-only patch lines in the window are `0.98.1` and `0.99.2`.
The `0.100.1-preview.1` tag is ignored. The historical `0.52.0` and
unpublished `0.58.0` exclusions remain.

The retained fixture records every integrity value, tarball SHA-256, package
dependency identity, and complete file tree in
[`claude-agent-acp-0.100.0`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.100.0/README.md).
No file was removed across the three stable hops.

## Selected ACP v1 review

Swallowtail still sends ACP `protocolVersion: 1`, file read only, and form
elicitation. It does not advertise AIR, goal, async tasks, session notices,
context compaction, native subagent sessions, `sessionIndex`, boolean config
options, or plan operations. Selected routes stay initialize, session new,
prompt, cancel, load, resume, close, delete, request permission, elicitation
create, set config option, and set mode.

`0.98.0` to `0.99.0` adds 3 files (`dist/blob-resources.*`), removes none, and
changes 37. `dist/index.js`, `dist/serve.js`, and `dist/tools.js` stay
byte-identical. 266 files are unchanged.

- Failure codes move to protocol errors: session lookups and invalid config,
  mode, and ultracode values now throw `RequestError.sessionNotFound` /
  `invalidParams` instead of generic errors. Swallowtail's connection maps
  every JSON-RPC error to one `request_rejected` failure class and discards
  the code, so no mapped failure class changes and every selected success
  path stays.
- `dist/session-model.js` makes the model config option always list
  `currentValue` (paired with removing the `set_config_option`
  currentValue-not-listed fallback). A client selecting values from the
  option list — Swallowtail's selection path — behaves as before.
- Client-provided MCP servers reported `failed` or `needs-auth` now produce a
  notice; clients without session notices receive the existing
  `agent_message_chunk` fallback text, once per server per session. Swallowtail
  passes client-supplied HTTP MCP entries on `session/new`, so this content is
  reachable, but it is text on an already-decoded update kind with unchanged
  shape.
- `dist/session-ultracode.js` reaches the `configOptions` payload of selected
  sessions; the delta is the invalid-value error code only. It is classified
  mapped-adjacent here, correcting the provider-internal classification the
  `0.98.0` fixture gave the same files.
- `EnterPlanMode` joins the tool-call reporters with the existing
  `switch_mode` shape already decoded for `ExitPlanMode`; Read locations clamp
  an offset of 0 to line 1.
- Blob resource prompt chunks now map to image or document blocks. Swallowtail
  sends text prompt blocks only, so the new branch is never exercised.
- `publishCurrent` session titles and the cancelled-permission denial are
  reached only from the v2 agent or from permission outcomes Swallowtail never
  sends (its answers always carry a selected option id; abandonment writes no
  response).

## Decision and limits

This is a compatible extension of `claude-agent.acp.initialize-meta-extensions-v7`,
recorded per hop and applied together with Research 470 and 471 in one claim
revision that extends the maintained segment `0.66.0..=0.96.0` →
`0.66.0..=0.100.0` through `0.100.0`, keeping baseline `0.53.0`, claim id
`claude-agent.acp.window-2`, the seven behavior milestones, exclusions `0.52.0`
and `0.58.0`, and `AllowUnverified`. Until that claim revision lands, `0.98.0`
stays the qualified ceiling and `0.99.0` stays `UnverifiedNewer` under
`AllowUnverified`. `0.100.1` is the synthetic `UnverifiedNewer` point after the
claim revision. Prerelease points, including `0.98.1-preview.1` and
`0.99.2-preview.*`, remain incompatible without their own exact segment.

No new public mapped operation was added. HTTP MCP live honouring remains
exact to `0.79.0` and `0.81.2` (Research 361 and 364). The root package pin
stays `0.81.2`. The pinned ACP SDK `2.1.0` and Agent SDK `0.3.312` are
recorded dependency identity on separate axes. No tag, publication, host
install, package install, provider prompt, live initialize, credential use, or
host mutation occurred.

Sources: [official npm package 0.99.0](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.99.0),
[official GitHub tag v0.99.0](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.99.0),
and the [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json).
