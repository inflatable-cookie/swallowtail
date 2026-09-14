# 309 Claude Agent ACP 0.76.0 Identity

Status: promoted
Owner: Tom
Date: 2026-09-14
Task: g05.059

## Question

Is official npm/GitHub Claude Agent ACP `0.76.0` a compatible extension of
the qualified `claude-agent.acp.initialize-meta-extensions-v7` segment
through `0.73.0`, a private milestone, a new facade, or a stop?

This is the first family of the operator-authorized Research 308 currentness
campaign. Claude Code, the Claude Agent SDK sidecar, and the watcher stay
separate surfaces and were not reopened.

## Remaining Rank

This run covers only Claude Agent ACP. At observation time the family was
AllowUnverified official-newer: g05.018 left the qualified ceiling at
`0.73.0` while official stable had moved to `0.76.0`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-agent.acp-adapter` | `0.63.0` on `PATH` | npm, GitHub, and ACP registry `0.76.0` | qualified `0.53.0..=0.73.0` excluding `0.58.0`; AllowUnverified | official-newer |

## Method

Re-probed npm `@agentclientprotocol/claude-agent-acp@latest`, the GitHub
latest release, and the ACP registry, then retrieved the official npm
tarballs for the previous ceiling `0.73.0` and every published hop
`0.74.0`, `0.75.0`, `0.75.1`, and `0.76.0` into `/tmp/cacp`. Every tarball
was SHA-256 hashed; the `0.73.0` values reproduce Research 272. The GitHub
tag commits were resolved separately and each npm `gitHead` matches its
tag.

Extracted packages were never executed. For every compared version the
complete package file set (`LICENSE`, `README.md`, `package.json`,
`dist/**`) was inventoried with per-file SHA-256 digests and frozen in
`dist-inventory.json`. `dist/acp-agent.js` changes on every hop, so each
hop's changed file set was classified as mapped-adjacent, provider-internal,
or unmapped with a reason. The published TypeScript tree was read from the
GitHub tags as discovery to understand the compiled deltas; it is not
substituted for the shipped artifact evidence.

The host `claude-agent-acp` is exact `0.63.0`, SHA-256
`260aac90bf75f197b93640087c1de66441761d43c2784efa035fdcee60b5dacd`, which
equals the frozen `0.70.0` host digest and therefore the frozen `0.73.0`
host digest. It was observed once with `--version` and was not installed,
updated, replaced, or executed further.

No provider prompt, login, credential, live ACP initialize, install, host
update, or downloaded-artifact execution occurred. Changelog and release
bodies were read as discovery only.

## Identity

npm, GitHub, and the ACP registry agree on `0.76.0`. npm published it at
2026-09-09T21:16:32.982Z with integrity
`sha512-8Xh1ryud5b1fOGLfF+S7dW2hhIZ/h4WopKtD//JRcQWxu7dutZvzN/MhRSDjqRcxqf3UIZRx1K0OhnlY/hMaOg==`
and tarball SHA-256
`b836570348c9400062e39f4d5866734b5fe89613d80b5cf1c47c9446c1b4eafd`. GitHub
published `v0.76.0` at 2026-09-09T21:13:16Z with tag commit
`c2e4815029ef3962787ecaefe208b0b6f8b81302`. The ACP registry `claude-acp`
entry reports `0.76.0`.

| Version | npm published | GitHub tag commit | tarball SHA-256 | acp-agent.js SHA-256 |
| --- | --- | --- | --- | --- |
| `0.73.0` | 2026-09-01T20:27:53.428Z | `ea7076c0bc324603e65d8c124b7573f158749969` | `eb03d0c6c1934726535d5c6b8defd3b37c2c2d77f5d9d037d3a97d624ea733c3` | `e41014b49c5ac096b5e18a89f990ee0ec64452e440666b59dcf4e087f632e370` |
| `0.74.0` | 2026-09-04T11:11:03.220Z | `847a3b642929654b5bf32b143b3e1efd5884d331` | `c0aa74be9298153a6e919aeee6a3d2da035e0c6df34b15f16890db2eca2d6d99` | `0eeabe7242f82b2a27bd9a98c9812ff7fb29c4a67c1d289947b88bcacceb8548` |
| `0.75.0` | 2026-09-05T08:56:11.799Z | `a0122dae4c6f0d19a6716561683607dac4f50215` | `ec6c746939a1fde7f7fc899fff3615d5caf445e1ba89194f1e877741b7fa7350` | `731746d8ec2609201c92d31aca28e5b4f2149552c9a8fab078025d65805e897c` |
| `0.75.1` | 2026-09-05T14:37:57.183Z | `3e23c5b960b66a6d2c892e7524c952e731c076a7` | `77edc54c77e21792827bb4ca6d3929855424fcf73a2ebcfbc713e44c2a430148` | `c22424c297429378166524b59ee6bed239c999a420ad0dd06571b768efa7525b` |
| `0.76.0` | 2026-09-09T21:16:32.982Z | `c2e4815029ef3962787ecaefe208b0b6f8b81302` | `b836570348c9400062e39f4d5866734b5fe89613d80b5cf1c47c9446c1b4eafd` | `c1635f643ceff71c906011b85f33f1b135a4a07aeaa5546d44fb6c5fad11c96d` |

Published stables after the previous `0.73.0` ceiling are exactly `0.74.0`,
`0.75.0`, `0.75.1`, and `0.76.0`. The unpublished gaps `0.73.1`, `0.74.1`,
`0.75.2`, and `0.76.1` stay incompatible; `0.75.2` and `0.76.1` have preview
builds only, so neither is a stable. `0.58.0` stays unpublished and
incompatible. First unpublished later stable at observation: `0.77.0`.

Host `0.63.0` is qualified Deprecated and is observation input only; it is
not a qualification or install authority.

## Selected Protocol

Package file counts grow `0.73.0` (96) → `0.74.0` (99) → `0.75.0` (111) →
`0.75.1` (117) → `0.76.0` (123). No file was removed at any hop. The ACP SDK
pin stays `1.4.0` and the Agent SDK pin stays `0.3.257` at every hop, so the
ACP v1 wire schema and the SDK control surface are unchanged.

Byte-identical across every compared hop (76 files, including the complete
`dist/permissions/**` tree):

- `dist/index.js` (identical from `0.70.0`), `dist/elicitation.js`
  (identical from `0.64.0`), `dist/settings.js` and `dist/utils.js`
  (identical from `0.70.0`);
- `dist/tools.js`, `dist/session-mode.js`, and `dist/session-config-ids.js`
  (identical from `0.72.0` and `0.73.0` respectively); and
- every `dist/permissions/**` file.

Because the mode module, the config-id module, and the permissions tree are
untouched, the mapped mode option id `mode` and category `mode`, the
`plan`/`acceptEdits` advertisement, the effort config id `effort` with
category `thought_level`, and the permission option kinds `allow_once` /
`allow_always` / `reject_once` are unchanged without inference.

`dist/lib.js` changes only at `0.75.0`, where it re-exports the new
`authStatus` notification types; no selected driver path imports it.

Skipped-hop ledger:

- **`0.74.0`** adds `dist/hide-claude-auth.*` and changes
  `dist/acp-agent.js`, `dist/session-failure-extension.*`, and
  `package.json` (version bump only). The `--hide-claude-auth` guard is an
  `argv` flag that Swallowtail never passes, so it stays inactive; the
  session-failure extension is an unmapped `_meta` payload. The
  `acp-agent.js` deltas are provider-update error containment,
  allowlist/model-info plumbing, the session/new effort seed no longer being
  pushed to the SDK, and resumed context-window scheduling. Mapped route and
  request shapes, config-option ids/categories, permission kinds, the
  `stopReason` domain, and usage fields are unchanged.
- **`0.75.0`** adds `dist/auth-status.*`, `dist/context-compaction.*`,
  `dist/context-compaction-meta.*`, and `dist/usage-markdown.*`, and changes
  `dist/acp-agent.js`, `dist/hide-claude-auth.*`, `dist/lib.*`, and
  `package.json` (version bump only). The `authStatus` extension adds an
  additive `agentCapabilities._meta.authStatus` marker and pushes
  `_auth/status_update`; the adapter ignores every `_`-prefixed notification
  and does not reject unknown capability keys. Context compaction is
  emitted as an ordinary `tool_call` / `tool_call_update` with `kind: think`
  and mapped statuses plus an extra `_meta.contextCompaction` key the
  decoder ignores. Usage Markdown is display only for the `/usage` local
  command.
- **`0.75.1`** adds `dist/resumed-session.*` and `dist/session-timing.*`, and
  changes `dist/acp-agent.js`, `dist/fork-session.*`, and `package.json`
  (version bump only). `session/load` now reads the live model from the local
  transcript instead of a pre-turn `getContextUsage` control request, and a
  compaction `usage_update.used` now comes from the boundary `post_tokens`.
  Both affect reported values, not the mapped request/response or replay
  shapes. The fork change is AIR `_meta.jetbrains.air.fork` fidelity;
  Swallowtail maps no fork operation.
- **`0.76.0`** adds `dist/session-effort.*` and `dist/session-model.*`, and
  changes `dist/acp-agent.js`, `dist/air-extension.*`,
  `dist/clear-context-coordinator.*`, `README.md`, and `package.json`
  (version bump plus a `vitest` dev-dependency bump). Recommended config
  values are gated on a new client capability
  (`_meta.air.recommendedValue`) that the adapter does not advertise, so the
  legacy option shape survives: the model option keeps id `model`, category
  `model`, `type: select`, and its `currentValue`; the effort option keeps id
  `effort`, category `thought_level`, and its `default` entry. Clear-context
  coordination is unmapped.

Release bodies name no mapped change. They add a hidden-auth guard, an
`authStatus` extension, context compaction as a tool lifecycle, usage
Markdown, fork/load fixes, and recommended config values — each classified
above. No new `sessionUpdate` kind is emitted, and the shipped
`sessionUpdate` kind set is identical across every hop. `providers`,
`steering`, `loadSession`, and the required `sessionCapabilities` entries
(`close`, `delete`, `resume`) are still advertised.

The mapped compatibility argument is unchanged: Swallowtail confirms the
model and effort through explicit `session/set_config_option` plus exact
`currentValue` matching, ignores unknown session updates and `_`-prefixed
notifications, and requires no new public operation. Additive extensions the
adapter does not select stay unmapped with reasons.

## Decision

**Compatible-extension.**

- keep axis `claude-agent.acp-adapter`, claim id `claude-agent.acp.window-2`,
  baseline `0.53.0`, and exclusion `0.58.0`;
- keep behavior revision `claude-agent.acp.initialize-meta-extensions-v7` and
  extend its Maintained segment from `0.66.0..=0.73.0` to `0.66.0..=0.76.0`;
- qualify the published hops `0.74.0`, `0.75.0`, `0.75.1`, and `0.76.0` and
  raise `latest_qualified` to `0.76.0`;
- keep every historical segment and gap, keep `AllowUnverified`, and keep
  host `0.63.0` observation-only Qualified Deprecated;
- use synthetic `0.77.0` as the later `UnverifiedNewer` point after claim;
  and
- leave the `--hide-claude-auth` guard, `authStatus`, context-compaction
  `_meta`, usage Markdown, AIR fork metadata, clear-context coordination, and
  recommended config values unmapped.

This identity record changes no production claim. The g05.059 claim commit
applies the admitted segment.

## Sources

- npm registry: `https://registry.npmjs.org/@agentclientprotocol/claude-agent-acp`
- npm tarballs: registry `dist.tarball` URLs for `0.73.0` through `0.76.0`
- GitHub releases: `https://github.com/agentclientprotocol/claude-agent-acp/releases`
- GitHub tags: `v0.73.0` through `v0.76.0`
- ACP registry: `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
- Frozen corpus:
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.76.0/`
