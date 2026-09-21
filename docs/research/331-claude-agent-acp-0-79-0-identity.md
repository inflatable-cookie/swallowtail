# 331 Claude Agent ACP 0.79.0 Identity

Status: promoted
Owner: operator-authorized family qualification
Date: 2026-09-21

## Question

Is official npm/GitHub Claude Agent ACP `0.79.0` a compatible extension of
the qualified `claude-agent.acp.initialize-meta-extensions-v7` segment
through `0.76.0`, a private milestone, a new facade, or a stop?

This run covers only Claude Agent ACP. Claude Code stream-JSON
(`claude-code.headless` / `claude-code.response-only`) and the Claude Agent
SDK sidecar stay separate families and were not reopened.

## Remaining Rank

This run covers only Claude Agent ACP. At observation time the family was
AllowUnverified official-newer: Research 309 left the qualified ceiling at
`0.76.0` while official stable had moved to `0.79.0`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-agent.acp-adapter` | not on `PATH` | npm, GitHub, and ACP registry `0.79.0` | qualified `0.53.0..=0.76.0` excluding `0.58.0`; AllowUnverified | official-newer |

## Method

Re-probed npm `@agentclientprotocol/claude-agent-acp@latest`, the GitHub
latest release, and the ACP registry, then retrieved the official npm
tarballs for the previous ceiling `0.76.0` and every published hop
`0.77.0`, `0.78.0`, and `0.79.0` into `/tmp/cacp`. Every tarball was
SHA-256 hashed; the `0.76.0` values reproduce Research 309. The GitHub tag
commits were resolved separately and each npm `gitHead` matches its tag.

Extracted packages were never executed. For every compared version the
complete package file set (`LICENSE`, `README.md`, `package.json`,
`dist/**`) was inventoried with per-file SHA-256 digests and frozen in
`dist-inventory.json`. `dist/acp-agent.js` and `dist/elicitation.js` change
on hops, so each hop's changed file set was classified as mapped-adjacent,
provider-internal, or unmapped with a reason.

Host `claude-agent-acp` was not on `PATH`. Missing install is not a gap. It
was not installed, updated, or executed.

No provider prompt, login, credential, live ACP initialize, install, host
update, or downloaded-artifact execution occurred. Changelog and release
bodies were read as discovery only. Official latest was re-probed before
identity freeze: still `0.79.0`.

## Identity

npm, GitHub, and the ACP registry agree on `0.79.0`. npm published it at
2026-09-17T14:55:56.043Z with integrity
`sha512-/liYDBHElfzgbeijv8EZzvDUUAC8wUi1WSCZ+bw/DIKHQ3t3DLid+LS+6lszuQamHAWijjaZl1i5xZVRTnNPoA==`
and tarball SHA-256
`8c7a692b0266389eb7d81d4cb836c1f21293fb6a0ed11bff2e15db42c36177cb`. GitHub
published `v0.79.0` at 2026-09-17T14:51:12Z with tag commit
`d421f56a6c43cde16d9a7531d08a750a5ef2f04a`. The ACP registry `claude-acp`
entry reports `0.79.0`.

| Version | npm published | GitHub tag commit | tarball SHA-256 | acp-agent.js SHA-256 |
| --- | --- | --- | --- | --- |
| `0.76.0` | 2026-09-09T21:16:32.982Z | `c2e4815029ef3962787ecaefe208b0b6f8b81302` | `b836570348c9400062e39f4d5866734b5fe89613d80b5cf1c47c9446c1b4eafd` | `c1635f643ceff71c906011b85f33f1b135a4a07aeaa5546d44fb6c5fad11c96d` |
| `0.77.0` | 2026-09-14T13:14:18.405Z | `dfe823b9581979cd22db40272d4469cc7e42b77e` | `040871730a084d5da591fff441d51628fb14e6842021aea53d5b5d6923eac508` | `0f232770adcf279ce6e185c60ea005a3528c02aefb0b87af8053d75f7f622e71` |
| `0.78.0` | 2026-09-15T21:01:04.061Z | `becd854dd813af769475cf66e081d48c3f25f081` | `6ee5e95b94fd9a3523f94d84b476737e6583f5a35394cfa8423194092dd501c3` | `db1db1b10be8c39b47b0684a57fd3a82cf59e40b46be6bc8bd93d1d743dcfcdf` |
| `0.79.0` | 2026-09-17T14:55:56.043Z | `d421f56a6c43cde16d9a7531d08a750a5ef2f04a` | `8c7a692b0266389eb7d81d4cb836c1f21293fb6a0ed11bff2e15db42c36177cb` | `e9711af5c5dd150718c4a22b1760f913ce8a871b7355d1f0f63aa845d282e37c` |

Published stables after the previous `0.76.0` ceiling are exactly `0.77.0`,
`0.78.0`, and `0.79.0`. Unpublished gaps `0.73.1`, `0.74.1`, `0.75.2`,
`0.76.1`, `0.77.1`, and `0.78.1` stay incompatible; the last four have
preview builds only. `0.58.0` stays unpublished and incompatible. First
unpublished later stable at observation: `0.80.0`.

## Selected Protocol

Package file counts go `0.76.0` (123) → `0.77.0` (123) → `0.78.0` (126) →
`0.79.0` (126). No file was removed at any hop. The ACP SDK pin stays
`1.4.0` at every hop, so the ACP v1 wire schema is unchanged. The Agent SDK
pin moves `0.3.257` → `0.3.270` → `0.3.274` and stays unmapped; this family
is not flattened onto Claude Agent SDK or Claude Code stream-JSON.

Byte-identical across every compared hop (81 files):

- `dist/index.js` (identical from `0.70.0`);
- `dist/settings.js` and `dist/utils.js` (identical from `0.70.0`); and
- `dist/lib.js` (identical from `0.75.0`).

Mode option id `mode` and category `mode`, the `plan`/`acceptEdits`
advertisement, the effort config id `effort` with category `thought_level`,
and the permission option kinds `allow_once` / `allow_always` /
`reject_once` stay by classified source. Config-option ids and categories
are unchanged. Mapped usage fields, `stopReason` domain/catch-all, and
`session/cancel` stay selected-compatible. Wire `protocolVersion` stays `1`.

Skipped-hop ledger:

- **`0.77.0`** changes `dist/acp-agent.js`, `dist/elicitation.js`,
  `dist/session-config-ids.js`, `dist/session-mode.js`, permissions
  presentation/options, tools, and `package.json`. The `agent` config option
  is removed; Swallowtail never mapped it. Multi-select Other description
  changes to "Type your own answer to add to your selection above
  (optional)." Form keys and the `Other` title stay. Permission option
  order/`defaultToNo`, TaskList parser, native-subagent failure names, and
  AIR access-denied stay unmapped.
- **`0.78.0`** adds `dist/diff.*` and changes `dist/acp-agent.js`,
  `dist/context-compaction.js`, `dist/elicitation.js`, AIR/file-change
  audit, tools, and `package.json`. `compaction_update` is gated on client
  `session.compaction`, which Swallowtail does not advertise, so the
  selected route keeps the existing `tool_call` compaction lifecycle.
  Single-select Other description changes to "Type your own answer, or add
  a note to the option you chose above (optional)." Custom-answer notes
  after accept, AIR diffStats, and the extracted diff helper stay unmapped.
- **`0.79.0`** changes `dist/acp-agent.js`, permissions presentation, tools,
  and `package.json`. The Agent SDK pin moves to `0.3.274`. Shell permission
  titles keep command text; PowerShell is presented like bash. Both stay
  unmapped.

Release bodies name no new public mapped operation. No new `sessionUpdate`
kind is emitted on the selected route. Required `sessionCapabilities`
entries (`close`, `delete`, `resume`) stay advertised.

The mapped compatibility argument is unchanged: Swallowtail confirms model
and effort through explicit `session/set_config_option` plus exact
`currentValue` matching, ignores unknown session updates and `_`-prefixed
notifications, and requires no new public operation. Already-mapped form
elicitation keeps the same field keys and `Other` title; the adapter accepts
the two new Other description strings so those forms still map. Additive
extensions the adapter does not select stay unmapped with reasons.

This is not a major-line reset, not a new public operation, and does not
flatten families.

## Decision

**Compatible-extension.**

- keep axis `claude-agent.acp-adapter`, claim id `claude-agent.acp.window-2`,
  baseline `0.53.0`, and exclusion `0.58.0`;
- keep behavior revision `claude-agent.acp.initialize-meta-extensions-v7` and
  extend its Maintained segment from `0.66.0..=0.76.0` to `0.66.0..=0.79.0`;
- qualify the published hops `0.77.0`, `0.78.0`, and `0.79.0` and raise
  `latest_qualified` to `0.79.0`;
- widen already-mapped elicitation Other-description matching;
- keep every historical segment and gap, keep `AllowUnverified`;
- use synthetic `0.80.0` as the later `UnverifiedNewer` point after claim;
  and
- leave the removed `agent` config option, capability-gated
  `compaction_update`, Agent SDK pin, AIR file-change / diffStats,
  defaultToNo option order, and shell permission titles unmapped.

This identity record names the admitted segment. The same batch applies the
claim.

## Sources

- npm registry: `https://registry.npmjs.org/@agentclientprotocol/claude-agent-acp`
- npm tarballs: registry `dist.tarball` URLs for `0.76.0` through `0.79.0`
- GitHub releases: `https://github.com/agentclientprotocol/claude-agent-acp/releases`
- GitHub tags: `v0.76.0` through `v0.79.0`
- ACP registry: `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
- Frozen corpus:
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.79.0/`
