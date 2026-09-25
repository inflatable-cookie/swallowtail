# 352 Claude Agent ACP 0.81.2 Identity

Status: promoted
Owner: operator-authorized family qualification
Date: 2026-09-25

## Question

Is official npm/GitHub/ACP-registry Claude Agent ACP `0.81.2` a compatible
extension of the qualified `claude-agent.acp.initialize-meta-extensions-v7`
segment through `0.79.0`, a private milestone, a new facade, or a stop?

This run covers only Claude Agent ACP. Claude Code stream-JSON
(`claude-code.headless` / `claude-code.response-only`) and the Claude Agent
SDK sidecar stay separate families and were not reopened.

## Remaining Rank

This run covers only Claude Agent ACP. At observation time the family was
AllowUnverified official-newer: Research 335 left the qualified ceiling at
`0.79.0` while official stable had moved to `0.81.2`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-agent.acp-adapter` | not on `PATH` | npm, GitHub, and ACP registry `0.81.2` | qualified `0.53.0..=0.79.0` excluding `0.58.0`; AllowUnverified | official-newer |

## Method

Re-probed npm `@agentclientprotocol/claude-agent-acp@latest`, the GitHub
latest release, and the ACP registry `claude-acp`, then retrieved the
official npm tarballs for the previous ceiling `0.79.0` and every published
hop `0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2` into `/tmp/cacp`. Every
tarball was SHA-256 hashed; the `0.79.0` tarball reproduces Research 335.
The GitHub tag commits were resolved separately and each npm `gitHead`
matches its tag. The published ACP SDK tarballs `1.4.0` and `1.5.0` were
retrieved the same way and compared; their `dist/acp.js` files are
byte-identical.

Extracted packages were never executed. For every compared version the
complete package file set (`LICENSE`, `README.md`, `package.json`,
`dist/**`) was inventoried with per-file SHA-256 digests and frozen in
`dist-inventory.json`. `dist/acp-agent.js` changes on every hop, so each
hop's changed file set was classified as mapped-adjacent,
provider-internal, or unmapped with a reason.

Host `claude-agent-acp` was not on `PATH`. Missing install is not a gap. It
was not installed, updated, or executed.

No provider prompt, login, credential, live ACP initialize, install, host
update, or downloaded-artifact execution occurred. Changelog and release
bodies were read as discovery only. Official latest at identity freeze was
`0.81.2` on npm, GitHub, and the ACP registry.

## Identity

npm, GitHub, and the ACP registry agree on `0.81.2`. npm published it at
2026-09-24T10:18:05.741Z with integrity
`sha512-/yvpesq8e6Jv8kl5PHEhwbKRVvzVudXD+ujC0q6PZFmrZK0+xPtiVQYtZIdlUtdhgNKBgBTsGwZkbb2Iw+rbjA==`
and tarball SHA-256
`15368e30e06789df93c057d910247e15a3b38f59bddd039c2e44a0e946341b0c`. GitHub
published `v0.81.2` at 2026-09-24T10:15:45Z with tag commit
`5dbb453c63a89746627799b2b06b31ba01a1b674`. The ACP registry `claude-acp`
entry reports `0.81.2`.

| Version | npm published | GitHub tag commit | tarball SHA-256 | acp-agent.js SHA-256 | ACP SDK | Agent SDK |
| --- | --- | --- | --- | --- | --- | --- |
| `0.79.0` | 2026-09-17T14:55:56.043Z | `d421f56a6c43cde16d9a7531d08a750a5ef2f04a` | `8c7a692b0266389eb7d81d4cb836c1f21293fb6a0ed11bff2e15db42c36177cb` | `e9711af5c5dd150718c4a22b1760f913ce8a871b7355d1f0f63aa845d282e37c` | `1.4.0` | `0.3.274` |
| `0.80.0` | 2026-09-22T13:19:56.111Z | `16c9d1a3af6bd81e5a8b07420b4bb17bfda22ffb` | `e812682f7a3c06e2e30cdf71edb4bbd25e72e5b1162164e7cf7f2ae5228a42f3` | `16cd14aa02584dcbeb4b1b141eab5977c4c1a2d7521680c6601cf51879ba3dad` | `1.5.0` | `0.3.278` |
| `0.81.0` | 2026-09-22T17:08:09.967Z | `d571358e267ed21ed0ec9ec2622fda8935535d57` | `8549a4b03a158bcb636db73ae9b1721393fbbb625e4296b9439095413cef8f72` | `a17444ae89c5dd8f6cccaf278107438100521cf5cb367fb8991e48ce0f46bbf1` | `1.5.0` | `0.3.280` |
| `0.81.1` | 2026-09-23T10:39:18.490Z | `b264b52bee80e49f20caf1941f7d7cb89edb80c4` | `60326dd11884e968259bd9e3650ec60980d6507a18d1c58d3d9b447d3b8069e6` | `fdc6fdd16d47eac5ae1998b40aaccf1e675431fbe66c0ab064f4592dcdd1d8e3` | `1.5.0` | `0.3.280` |
| `0.81.2` | 2026-09-24T10:18:05.741Z | `5dbb453c63a89746627799b2b06b31ba01a1b674` | `15368e30e06789df93c057d910247e15a3b38f59bddd039c2e44a0e946341b0c` | `36a33a984624541ea013c085f5bd1d982294cfe70beb42826752f62bdc538d64` | `1.5.0` | `0.3.280` |

Published stables after the previous `0.79.0` ceiling are exactly `0.80.0`,
`0.81.0`, `0.81.1`, and `0.81.2`. `0.58.0` stays unpublished and is the
standing exclusion. `0.79.1` and `0.80.1` have preview builds only and are
not new exclusions; the maintained segment already admits interior version
strings the way `0.76.0` and `0.79.0` did. First unpublished later stable
at observation: `0.81.3`. No `0.81.3` preview is published.

## Selected Protocol

Package file counts go `0.79.0` (126) → `0.80.0` (126) → `0.81.0` (129) →
`0.81.1` (132) → `0.81.2` (132). No file was removed at any hop. Ninety-nine
files are byte-identical across all five packages, including
`dist/elicitation.js`, `dist/lib.js`, `dist/settings.js`, `dist/utils.js`,
`dist/session-config-ids.js`, `dist/permissions/options.js`, and
`dist/permissions/presentation.js`. `dist/index.js` is identical from
`0.79.0` through `0.81.0` and changes only at `0.81.1` (then stays identical
through `0.81.2`).

The ACP SDK pin moves `1.4.0` → `1.5.0` at `0.80.0`. Published
`@agentclientprotocol/sdk` `dist/acp.js` and `dist/acp.d.ts` are
byte-identical across that pin, `PROTOCOL_VERSION` stays `1`, and the schema
adds optional `Notice` definitions plus an unstable `notice` session update
that agents send only when the client advertised `session.notices`.
Swallowtail initialize sends protocol version `1` and does not advertise
`session.notices` or `session.compaction`. The Agent SDK pin moves
`0.3.274` → `0.3.278` → `0.3.280` and stays unmapped; this family is not
flattened onto Claude Agent SDK or Claude Code stream-JSON.

Initialize `sessionCapabilities` stay
`additionalDirectories`, `close`, `delete`, `fork`, `list`, `resume`,
`subagents`, with `loadSession` true. The adapter still requires `close`,
`delete`, and `resume`. Client methods stay `completeElicitation`,
`createElicitation`, `extNotification`, `readTextFile`,
`requestPermission`, `sessionUpdate`, and `writeTextFile`. Mode option id
`mode`, `plan` / `acceptEdits`, the effort config id `effort`, and
permission kinds `allow_once` / `allow_always` / `reject_once` stay.
Prompt usage fields and the sum invariant stay. `stopReason` domain stays
`end_turn`, `cancelled`, `max_tokens`, `max_turn_requests`, `refusal`.
Unknown session updates and `_meta` stay ignored.

Skipped-hop ledger:

- **`0.80.0`** changes `dist/acp-agent.js`, context-compaction, tools, and
  `package.json`. Compaction interrupt sets `interrupted` only when the
  presentation is `compaction_update`, which Swallowtail does not advertise.
  Hand-back frame unwrap is tool-text cleanup. `preferTerminalOutputDelta`
  defaults false and is never passed. The ACP SDK pin move is a runtime
  byte-identical pin, not a wire reset.
- **`0.81.0`** adds `dist/session-notices.js` and changes `dist/acp-agent.js`,
  async-tasks, and session-mode. `notice` is gated on
  `clientCapabilities.session.notices`. Without that advertisement the
  fallback is `agent_message_chunk`. The selected route does not emit
  `notice`. Auto-mode fallback and task-stopped acknowledgements keep the
  previous transcript text when notices are unsupported.
- **`0.81.1`** adds `dist/managed-policy.js` and changes `dist/index.js`,
  `dist/acp-agent.js`, session-notices, session-mode, and write-tool
  aliases. Managed-policy environment is applied before ACP traffic and is
  provider-internal. Usage model rides in `_meta["_claude/model"]`, not a
  usage field. Informational chunks may carry `_meta.claudeCode.kind` and
  stay ignored. Exit-plan extra options remain kind `allow_always`, which
  the adapter already skips. Write-tool aliases normalize onto
  `file_path` / `content` before projection. `disableBypassPermissionsMode`
  is a settings gate. `prePlanMode` is an optional type the runtime
  `shared.js` and `modes.js` do not change.
- **`0.81.2`** changes `dist/acp-agent.js`, exit-plan, native-subagents, and
  `package.json`. The exit-plan interruption diagnostic and the resumed
  subagent parent stay provider-internal. Subagent update kinds are
  unchanged.

Release bodies name no new public mapped operation. No new `sessionUpdate`
kind is emitted on the selected route.

The mapped compatibility argument is unchanged: Swallowtail confirms model
and effort through explicit `session/set_config_option` plus exact
`currentValue` matching, ignores unknown session updates and `_`-prefixed
notifications, and requires no new public operation. Additive extensions the
adapter does not select stay unmapped with reasons.

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
- keep every historical segment and gap, keep `AllowUnverified`;
- do not add exclusions for preview-only `0.79.1` or `0.80.1`;
- use synthetic `0.81.3` as the later `UnverifiedNewer` point after claim;
- leave the ACP SDK pin move, Agent SDK pin, capability-gated `notice` and
  `compaction_update`, managed policy, usage-model `_meta`, informational
  chunk `_meta`, exit-plan `allow_always` extras, and subagent parent
  diagnostics unmapped; and
- leave Claude Code headless and response-only at their current ceilings.

This identity record names the admitted segment. g06.038 applies the claim.

## Sources

- npm registry: `https://registry.npmjs.org/@agentclientprotocol/claude-agent-acp`
- npm tarballs: registry `dist.tarball` URLs for `0.79.0` through `0.81.2`
- ACP SDK tarballs: `@agentclientprotocol/sdk` `1.4.0` and `1.5.0`
- GitHub releases: `https://github.com/agentclientprotocol/claude-agent-acp/releases`
- GitHub tags: `v0.79.0` through `v0.81.2`
- ACP registry: `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
- Frozen corpus:
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.81.2/`
