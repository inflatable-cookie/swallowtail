# 362 Claude Agent ACP 0.81.2 Identity

Status: promoted
Owner: operator-authorized family qualification
Date: 2026-09-26

## Question

Is official npm/GitHub Claude Agent ACP `0.81.2` a compatible extension of
the qualified `claude-agent.acp.initialize-meta-extensions-v7` segment
through `0.79.0`, a private milestone, a new facade, or a stop?

This run covers only Claude Agent ACP. Claude Code stream-JSON
(`claude-code.headless` / `claude-code.response-only`) and the Claude Agent
SDK sidecar stay separate families and were not reopened. HTTP MCP honouring
from Research 361 stays bound to exact `0.79.0` and is not extended.

## Remaining Rank

This run covers only Claude Agent ACP. At observation time the family was
AllowUnverified official-newer: Research 335 left the qualified ceiling at
`0.79.0` while official stable had moved to `0.81.2`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-agent.acp-adapter` | Homebrew `0.63.0` on `PATH` | npm, GitHub, and ACP registry `0.81.2` | qualified `0.53.0..=0.79.0` excluding `0.58.0`; AllowUnverified | official-newer |

## Method

Re-probed npm `@agentclientprotocol/claude-agent-acp@latest`, the GitHub
latest release, and the ACP registry, then retrieved the official npm
tarballs for the previous ceiling `0.79.0` and every published hop
`0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2` into `/tmp/cacp`. Every tarball
was SHA-256 hashed; the `0.79.0` values reproduce Research 335. The GitHub
tag commits were resolved separately and each npm `gitHead` matches its tag.

Extracted packages were never executed. For every compared version the
complete package file set (`LICENSE`, `README.md`, `package.json`,
`dist/**`) was inventoried with per-file SHA-256 digests and frozen in
`dist-inventory.json`. `dist/acp-agent.js` changes on every hop, so each
hop's changed file set was classified as mapped-adjacent, provider-internal,
or unmapped with a reason.

Host `claude-agent-acp` was on `PATH` at Homebrew `0.63.0`. `--version` was
recorded. The host was not installed, updated, or executed beyond that flag.

No provider prompt, login, credential, live ACP initialize, install, host
update, or downloaded-artifact execution occurred. Changelog and release
bodies were read as discovery only. Official latest was re-probed before
identity freeze: still `0.81.2`.

## Identity

npm, GitHub, and the ACP registry agree on `0.81.2`. npm published it at
2026-09-24T10:18:05.741Z with integrity
`sha512-/yvpesq8e6Jv8kl5PHEhwbKRVvzVudXD+ujC0q6PZFmrZK0+xPtiVQYtZIdlUtdhgNKBgBTsGwZkbb2Iw+rbjA==`
and tarball SHA-256
`15368e30e06789df93c057d910247e15a3b38f59bddd039c2e44a0e946341b0c`. GitHub
published `v0.81.2` at 2026-09-24T10:15:45Z with tag commit
`5dbb453c63a89746627799b2b06b31ba01a1b674`. The ACP registry `claude-acp`
entry reports `0.81.2`.

| Version | npm published | GitHub tag commit | tarball SHA-256 | acp-agent.js SHA-256 |
| --- | --- | --- | --- | --- |
| `0.79.0` | 2026-09-17T14:55:56.043Z | `d421f56a6c43cde16d9a7531d08a750a5ef2f04a` | `8c7a692b0266389eb7d81d4cb836c1f21293fb6a0ed11bff2e15db42c36177cb` | `e9711af5c5dd150718c4a22b1760f913ce8a871b7355d1f0f63aa845d282e37c` |
| `0.80.0` | 2026-09-22T13:19:56.111Z | `16c9d1a3af6bd81e5a8b07420b4bb17bfda22ffb` | `e812682f7a3c06e2e30cdf71edb4bbd25e72e5b1162164e7cf7f2ae5228a42f3` | `16cd14aa02584dcbeb4b1b141eab5977c4c1a2d7521680c6601cf51879ba3dad` |
| `0.81.0` | 2026-09-22T17:08:09.967Z | `d571358e267ed21ed0ec9ec2622fda8935535d57` | `8549a4b03a158bcb636db73ae9b1721393fbbb625e4296b9439095413cef8f72` | `a17444ae89c5dd8f6cccaf278107438100521cf5cb367fb8991e48ce0f46bbf1` |
| `0.81.1` | 2026-09-23T10:39:18.490Z | `b264b52bee80e49f20caf1941f7d7cb89edb80c4` | `60326dd11884e968259bd9e3650ec60980d6507a18d1c58d3d9b447d3b8069e6` | `fdc6fdd16d47eac5ae1998b40aaccf1e675431fbe66c0ab064f4592dcdd1d8e3` |
| `0.81.2` | 2026-09-24T10:18:05.741Z | `5dbb453c63a89746627799b2b06b31ba01a1b674` | `15368e30e06789df93c057d910247e15a3b38f59bddd039c2e44a0e946341b0c` | `36a33a984624541ea013c085f5bd1d982294cfe70beb42826752f62bdc538d64` |

Published stables after the previous `0.79.0` ceiling are exactly `0.80.0`,
`0.81.0`, `0.81.1`, and `0.81.2`. Unpublished gaps `0.73.1`, `0.74.1`,
`0.75.2`, `0.76.1`, `0.77.1`, `0.78.1`, `0.79.1`, `0.80.1`, and `0.81.3`
stay incompatible; the last three have preview builds only. `0.58.0` stays
unpublished and incompatible. First unpublished later stable at observation:
`0.82.0`.

Host observation: Homebrew npm-global `0.63.0`,
`dist/index.js` SHA-256
`260aac90bf75f197b93640087c1de66441761d43c2784efa035fdcee60b5dacd`. Not
updated.

## Selected Protocol

Package file counts go `0.79.0` (126) → `0.80.0` (126) → `0.81.0` (129) →
`0.81.1` (132) → `0.81.2` (132). No file was removed at any hop. The ACP
SDK pin moves `1.4.0` → `1.5.0` at `0.80.0`. The ACP SDK schema
`PROTOCOL_VERSION` stays `1`, and the sidecar still returns
`protocolVersion: 1`. The Agent SDK pin moves `0.3.274` → `0.3.278` →
`0.3.280` and stays unmapped; this family is not flattened onto Claude Agent
SDK or Claude Code stream-JSON.

Byte-identical across every compared hop (99 files), including:

- `dist/elicitation.js` (identical from `0.78.0`);
- `dist/settings.js` and `dist/utils.js` (identical from `0.70.0`);
- `dist/lib.js` (identical from `0.75.0`); and
- `dist/session-config-ids.js` (identical from `0.77.0`).

`dist/index.js` is byte-identical `0.79.0` through `0.81.0` and changes only
at `0.81.1` to extract managed-policy env apply. Mode option id `mode` and
category `mode`, the `plan`/`acceptEdits` advertisement, the effort config
id `effort` with category `thought_level`, and the permission option kinds
`allow_once` / `allow_always` / `reject_once` stay by classified source.
Mapped usage fields, `stopReason` domain/catch-all, and `session/cancel`
stay selected-compatible. Wire `protocolVersion` stays `1`.
`mcpCapabilities` stays `{ http: true, sse: true }`. Required
`sessionCapabilities` entries (`close`, `delete`, `resume`) stay advertised.

`session/close` → `teardownSession` → `closeQueryStream` are byte-identical
on every hop. `cancel()` at `0.80.0` adds
`await session.contextCompaction?.interrupt()` before the existing unbounded
`query.interrupt()`. Swallowtail does not advertise `session.compaction`, so
that extra await is a no-op on the selected route. The unbounded
`interrupt()` await named in Research 355 remains. The `mcpServers` HTTP
mapping (`type === "http" || "sse"` → `{ type, url, headers }`) is
byte-identical on every hop. HTTP MCP honouring stays exact `0.79.0`
(Research 361) and is not extended.

Skipped-hop ledger:

- **`0.80.0`** changes `dist/acp-agent.js`, `dist/context-compaction.js`,
  tools, and `package.json`. ACP SDK pin moves to `1.5.0`; Agent SDK pin
  moves to `0.3.278`. The only selected-adjacent cancel change is the
  optional compaction interrupt. Terminal output deltas stay unmapped.
- **`0.81.0`** adds `dist/session-notices.*` and changes `dist/acp-agent.js`,
  `dist/async-tasks.*`, `dist/session-mode.*`, and `package.json`. `notice`
  updates are gated on client `session.notices`, which Swallowtail does not
  advertise; fallback is already-mapped `agent_message_chunk`. Agent SDK pin
  moves to `0.3.280`.
- **`0.81.1`** adds `dist/managed-policy.*` and changes `dist/index.js`,
  `dist/acp-agent.js`, session-mode, session-notices, tools, permissions
  tools options, and `package.json`. Managed-policy env apply is extracted
  from `index.js` and stays unmapped. Usage model rides in `_meta`; the
  adapter ignores unknown `_`-prefixed notifications. Plan-approval bypass
  presentation stays unmapped.
- **`0.81.2`** changes `dist/acp-agent.js`, `dist/exit-plan.*`,
  `dist/native-subagents.*`, and `package.json`. Native-subagent resume
  announcement and clear-context plan follow-up stay unmapped.

Release bodies name no new public mapped operation. No new `sessionUpdate`
kind is emitted on the selected route.

The mapped compatibility argument is unchanged: Swallowtail confirms model
and effort through explicit `session/set_config_option` plus exact
`currentValue` matching, ignores unknown session updates and `_`-prefixed
notifications, and requires no new public operation. Already-mapped form
elicitation is byte-identical from `0.78.0`. Additive extensions the adapter
does not select stay unmapped with reasons.

This is not a major-line reset, not a new public operation, and does not
flatten families.

## Decision

**Compatible-extension.**

- keep axis `claude-agent.acp-adapter`, claim id `claude-agent.acp.window-2`,
  baseline `0.53.0`, and exclusion `0.58.0`;
- keep behavior revision `claude-agent.acp.initialize-meta-extensions-v7` and
  extend its Maintained segment from `0.66.0..=0.79.0` to `0.66.0..=0.81.2`;
- qualify the published hops `0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2` and
  raise `latest_qualified` to `0.81.2`;
- keep HTTP MCP honouring bound to exact `0.79.0`;
- keep every historical segment and gap, keep `AllowUnverified`;
- use synthetic `0.82.0` as the later `UnverifiedNewer` point after claim;
  and
- leave the ACP SDK pin, Agent SDK pin, capability-gated notices,
  managed-policy env, optional compaction interrupt, native subagents, and
  clear-context follow-up unmapped.

This identity record names the admitted segment. The same batch applies the
claim.

## Sources

- npm registry: `https://registry.npmjs.org/@agentclientprotocol/claude-agent-acp`
- npm tarballs: registry `dist.tarball` URLs for `0.79.0` through `0.81.2`
- GitHub releases: `https://github.com/agentclientprotocol/claude-agent-acp/releases`
- GitHub tags: `v0.79.0` through `v0.81.2`
- ACP registry: `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
- ACP SDK tarballs: `@agentclientprotocol/sdk` `1.4.0` and `1.5.0` (schema
  `PROTOCOL_VERSION` stays `1`)
- Frozen corpus:
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.81.2/`
