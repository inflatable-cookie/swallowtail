# Research 373: Claude Agent ACP 0.87.0 Identity and Currentness

Swallowtail#099; evidence checked 2026-10-07 for `claude-agent.acp` only.

## Official current point

The official npm package reports `latest=0.87.0` and `preview=0.86.1-preview.3`.
The ACP registry entry `claude-acp` also reports `0.87.0`. npm `gitHead` for
`0.87.0` matches GitHub tag `v0.87.0` at
`b2dbc8f5a1b84f48cc1512d06f50a9d85d6e757b`. The exact npm integrity is
`sha512-ylCn0Y9otQxNPcUJtiHTBwhceXqDziwBw6hsJuXqlHUlKTYjnwkMVFm3RF/BQwN1+TZi4xSTgLDVaH44Av1Jww==`;
the downloaded tarball SHA-256 is
`4c21b1168754fdb5343cf4a4f1661ece28a5d518d3d7738a6d92988855dd4705`.
The package pins ACP SDK `1.7.0` and Claude Agent SDK `0.3.287`, requires Node
`>=22`, and exposes `dist/index.js` as its binary entry.

Sources: [official npm package](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.87.0),
[official GitHub tag](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.87.0),
and the [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json).

## Published stable hops

The complete npm version list has these stable points after the previous
qualified ceiling `0.81.2` and through current `0.87.0`:

| Version | Published | GitHub tag commit | ACP SDK | Claude Agent SDK |
| --- | --- | --- | --- | --- |
| `0.82.0` | 2026-09-28 14:06:22Z | `18de37624071b48e95aed9ec5382823e2d72cd39` | `1.5.0` | `0.3.280` |
| `0.83.0` | 2026-09-28 16:01:42Z | `691328a9190d8729387149afad9bdb012450028b` | `1.5.1` | `0.3.283` |
| `0.84.0` | 2026-09-28 19:18:31Z | `bdb50ad984336e62dde1d41339f04071f6617085` | `1.5.1` | `0.3.284` |
| `0.85.0` | 2026-10-01 11:40:57Z | `c84845272fe3c55c1f97759f00ee48a1356fccae` | `1.5.1` | `0.3.286` |
| `0.85.1` | 2026-10-02 09:00:37Z | `686c0c99b3b89217b74d1f5de8272e7c9ef1aab4` | `1.6.0` | `0.3.286` |
| `0.86.0` | 2026-10-05 13:18:45Z | `7b5c61a4ed55c03028ac60e1768bc57d92df24a4` | `1.7.0` | `0.3.287` |
| `0.87.0` | 2026-10-07 13:30:51Z | `b2dbc8f5a1b84f48cc1512d06f50a9d85d6e757b` | `1.7.0` | `0.3.287` |

For each point, npm integrity metadata was checked against the downloaded
tarball, and npm `gitHead` matched its exact GitHub tag. The retained fixture
records every integrity value, tarball SHA-256, package dependency identity,
and complete file tree in
[`claude-agent-acp-0.87.0`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.87.0/README.md).
There are no removed files across the seven stable hops. Preview only patch
lines are `0.81.3`, `0.82.1`, `0.83.1`, `0.84.1`, `0.85.2`, and `0.86.1`;
none is a stable hop. The historical `0.52.0` and unpublished `0.58.0`
exclusions remain. Other prerelease and unpublished holes stay unqualified.

## Selected ACP v1 review

Swallowtail continues to send ACP `protocolVersion: 1`, file read only, and
form elicitation support. It does not advertise AIR, goal, async tasks, session
notices, context compaction, or native subagent sessions. The selected client
does not consume AIR-only initialize metadata or provider-specific config
controls beyond the existing `model`, `effort`, and `mode` options.

The largest wire change is the `0.82.0` tool field tracker. ACP fields present
on `tool_call_update` replace their prior value; upstream now omits unchanged
fields. Swallowtail's decoder already accepts optional update fields, preserves
an existing tool label when `title` is omitted, and emits no content update
when `content` is omitted. The existing
`tool_title_refines_and_survives_payload_only_completion` regression covers
that path. Raw input and output remain excluded from Swallowtail activity.

The `0.82.0` initialize change gates goal and AIR metadata on an AIR client.
Swallowtail neither advertises nor consumes those values. Permission and
elicitation additions are also AIR presentation metadata; the selected
one-shot permission subset, form field keys, and `Other` choice remain. The
provider adds a denied model filter and a custom agent config option in later
releases. The filter hides models the provider refuses while the CLI remains
the enforcer; the agent option is not interpreted or applied by Swallowtail.
Existing `model`, `effort`, and `mode` identifiers remain selected.

Versions `0.85.1` and `0.86.0` add cancellation and teardown fixes. The
`session/close`, `session/delete`, and cancel method surfaces and response
shapes remain ACP v1. The `0.86.0` load handler schedules replay through a new
helper but returns the same v1 session response and replay updates; Swallowtail
buffers updates that arrive before the response. The package also adds an
experimental ACP v2 router, but the binary keeps v1 as the default and
Swallowtail requests version 1. ACP v2 is not qualified here.

Version `0.87.0` improves background task routing and closes abandoned tool
calls with a failed tool update. Background task and subagent metadata remain
capability gated. The failed status is already in Swallowtail's ACP decoder and
activity mapping. Usage fields, cumulative total calculation, selected stop
reason domain, and unknown stop reason failure behavior remain unchanged.
These findings and the per file delta classifications are frozen in
`protocol.json` and `dist-inventory.json` beside the fixture.

## Decision and limits

This is a compatible extension of `claude-agent.acp.initialize-meta-extensions-v7`.
Extend the existing maintained segment `0.66.0..=0.81.2` through `0.87.0`.
Keep baseline `0.53.0`, claim ID `claude-agent.acp.window-2`, the seven
behavior milestones, exclusions `0.52.0` and `0.58.0`, and
`AllowUnverified`. The next synthetic stable point is `0.88.0`; it remains
`UnverifiedNewer`. Prerelease points remain incompatible without their own
exact segment.

No activity, route, permission, session, or consumer capability was added.
HTTP MCP live honouring remains exact to `0.79.0` and `0.81.2` only
(Research 361 and 364); no evidence transfers to `0.82.0`–`0.87.0`. The root
package's exact `0.81.2` development and probe pin stays unchanged. No tag,
publication, host install, package install, provider prompt, live initialize,
credential use, or host mutation occurred.
