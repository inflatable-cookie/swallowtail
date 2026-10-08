# 414 Claude Agent SDK 0.3.293 Qualification and 0.3.294 Currentness Stop

Date: 2026-10-08  
Route: `claude-agent.sdk`  
Axis: coupled wrapper package and embedded native; Node, sidecar wire, and
sidecar source are separate axes.

## Decision

Extend the existing package and native claims, retaining their `0.3.284` and
`2.1.284` baselines, claim IDs, behavior revision `claude-agent.sdk-v1`,
`QualifiedOnly` posture, and exclusions. Qualify every published point in
`0.3.284..=0.3.293` paired with `2.1.284..=2.1.293`. Do not qualify an
arbitrary cross-pair: open validation requires the wrapper and native patch
numbers to match. The selected npm channel is `dist-tags.latest`, freshly
observed at `0.3.293`; `next` also points to `0.3.293`. The official current
point is bound to the exact wrapper tarball and its embedded native manifest,
not the installed Claude Code executable. No behavioral change in the selected
route mapping was found. The independent Node segment remains governed by
Research 387; wire and sidecar source remain exact. This record does not change
those axes. The exact live registered-tool point remains
`0.3.259`/`2.1.259` and is not transferred.

## In-run official movement

The immediately-before-push re-probe on Oct 8, 2026 found npm `latest` and
`next` moved to `0.3.294`, published at `2026-10-08T16:36:21.522Z`; the
embedded native identity is `2.1.294`. This new point is not qualified by the
`0.3.284..=0.3.293` claim. Its wrapper tarball was downloaded for offline
inspection only. npm reports SHA-1
`b21effd0e72beec3a0471c0cc24bebb8ae983d1a` and integrity
`sha512-CSZBecJ57adb3bRoKA5O0miiegm9to59bhGiyKfj2uMoqZCdIoTlux77VFL5TNDWJmmGQOQpLyBSmSSq6SwV/Q==`;
its tarball SHA-256 is
`2c0a2db22531e690cfac98da0c764828d43b4cde38fca941dce7403cd49400d5`, its
complete 19-file tree digest is
`c36f910d8041231c3e7e2687f109b765714be9e9426ece08b86a15353c71f802`, and
the tarball URL is
`https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk/-/claude-agent-sdk-0.3.294.tgz`.
The wrapper's `package.json` declares SDK `0.3.294` and `claudeCodeVersion`
`2.1.294`. The embedded native manifest (`manifest.json`) SHA-256 is
`269e0ed1cf53f95f580d088a0d14558057648dfe418128ab669e22b0f01969a7`; it
names native `2.1.294`, native commit
`8f033c6ebe3d82a87f502e199307f38f5d55ccca`, mods commit
`765f236fe1bfcc678e0ec59af9170fb7d6771a3a`, build date
`2026-10-08T03:11:23Z`, and harness schema 1. The companion
`manifest.zst.json` SHA-256 is
`6b901af03f7b9b1fe8f51668027bc60d2f6f36e83bc28c0245d7f86b7bf35306`. The
manifest binds eight platform binary checksums and sizes. No platform native
package or binary was downloaded or executed for this new point.

Compared with the frozen `0.3.293` inventory, the new wrapper adds
`core-9fynjcde.mjs` and `core-z7m2ffy5.mjs`, removes `core-ae32wa3s.mjs` and
`core-d0szsqzn.mjs`, and changes `bridge.mjs`, `browser-sdk.js`, `core.mjs`,
`manifest.json`, `manifest.zst.json`, `package.json`, and `sdk.mjs`; the other
ten paths are byte-identical. `sdk.mjs` and `core.mjs` are selected execution
surfaces. The accepted frozen `0.3.293` evidence contains their hashes but not
the source bytes needed for the new semantic diff, and this continuation
forbids repeating completed artifact proof. Therefore these changed selected
surfaces remain unclassified; package/native `0.3.294`/`2.1.294` is a
currentness stop pending selected-hop classification and same-contract
qualification. Do not infer compatibility from exact identity, version
number, declarations, metadata, or changelog. No `0.3.294` claim or
`UnverifiedNewer` posture is added.

Contract 037's approved additive preparation adaptation exposes a finite exact
package/native pair choice while preserving the existing
`0.3.293`/`2.1.293` default. The prepared plan binds both selected axes, the
driver carries those exact identities to the shipped sidecar, and provider-free
fixtures exercise all ten pairs through the prepared facade and sidecar. This
operability proof does not qualify registered-tool live acceptance or alter
the separate Node, wire, or sidecar-source axes.

## Channel and package identity

Official source: [npm package metadata](https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk),
[0.3.293 GitHub release](https://github.com/anthropics/claude-agent-sdk-typescript/releases/tag/v0.3.293),
and [upstream changelog](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md).
The public GitHub repository is useful for discovery; the npm tarball and
embedded manifests establish shipped artifact identity. The selected package
is `@anthropic-ai/claude-agent-sdk`. At the qualification observation npm
`latest` and `next` both resolved to `0.3.293`; no later stable version or
published gap occurred in that selected segment. The old unpublished
`0.3.279` is below the retained baseline. The later `.294` publication is
recorded separately above as an unqualified in-run movement.

The previous exact point is Research 367: wrapper `0.3.284`, native `2.1.284`,
wrapper SHA-256
`4550e830246026133fc1802a2208dd0f3a785cae1eec83f261d114c33d797771`. Each
published wrapper tarball from `0.3.285` through `0.3.293` was downloaded
without execution, checked against npm `dist.integrity` and SHA-1 metadata, and
expanded into a deterministic complete file inventory. Every package contains
19 files. The exact tarball URLs, publish timestamps, SHA-256/SHA-1/SHA-512
integrity values, tree digests, and all per-file hashes are frozen in
[`dist-inventory.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.293/dist-inventory.json)
and [`identity.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.293/identity.json).

| npm point | Published (UTC) | wrapper SHA-256 | embedded native | native source commit |
| --- | --- | --- | --- | --- |
| 0.3.285 | 2026-09-29 17:35:20 | `5c68fdbf22cbe796d533dfb59e939aecdee7d3ce37d4853003b9d4b33863f40f` | 2.1.285 | `afb212976052ab038df25d5e871f6c049e094d3b` |
| 0.3.286 | 2026-09-30 17:17:08 | `c70947ab030c4b627a2f640fe1df91b02296026a59ade47155fa7ef806e8c07c` | 2.1.286 | `f344a08993bbba4fc1ea9b295245324610f02c58` |
| 0.3.287 | 2026-10-01 17:03:16 | `eff703b8cb9d9fc51e2dc80bf86257a063f2556f6d4bb5aa9814020a6bf48b55` | 2.1.287 | `3c446a1b98aceb99a6cdee0f84a8bea42f4a8937` |
| 0.3.288 | 2026-10-02 18:33:37 | `eb97c0f5a7d96189bceaf1986c2751a024b7e13844ba843531cc55bb0b4fac54` | 2.1.288 | `17fe1eb736e5b1433d6ca86a1db334cec8520450` |
| 0.3.289 | 2026-10-03 20:12:59 | `5ce0058cb62535a88705619ef0bd4293612d1b13ae95d620b2eb4dadbf83c898` | 2.1.289 | `736d26eef42d1e3017e07e6d5bd0970b8c3a068f` |
| 0.3.290 | 2026-10-05 18:15:52 | `086d6923a4232db6d2d9480dbc27a67555ee9031a12b75eb5ae5336fb8dd3108` | 2.1.290 | `3897a65075994f4efddcf34b4a32ea7c7c5ebc13` |
| 0.3.291 | 2026-10-06 03:33:35 | `9bd28f18be0f9651ae32ab7780099db93d9a2a4b785d722056a3be231f47571a` | 2.1.291 | `f7d32cfe47f735a7e70c5cce1ff6044b24692b4c` |
| 0.3.292 | 2026-10-06 17:14:32 | `967024d934865047062b5b5ed6e0d613d4454a846e9c71e1ac55ee3c6d57c168` | 2.1.292 | `37832d0b7cad7b40bac7c82dff58629313913edf` |
| 0.3.293 | 2026-10-07 17:21:04 | `395bfbda4294c6334fa18be3706a46ff31851b4f4a6f74ba11ac96c9d97ced9f` | 2.1.293 | `3abc54a9d60b4d12c627afad22d6e5f58a6199d2` |

The final wrapper's npm integrity is
`sha512-DVfn46SmpfB7xaKuUHsAC+qwY27hOLHNbvF7FoNOAXLZPRTeCr68SjhBjlFOtk+jz5Nf7pfUwnIUDFcdVih6Dg==`.
Its embedded manifest declares native build date `2026-10-07T06:56:40Z`,
mods commit `765f236fe1bfcc678e0ec59af9170fb7d6771a3a`, and harness schema 1.
It binds exact SHA-256 digests and sizes for all eight native platform
packages. The current Darwin arm64 package
[`claude-agent-sdk-darwin-arm64-0.3.293.tgz`](https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk-darwin-arm64/-/claude-agent-sdk-darwin-arm64-0.3.293.tgz)
was separately downloaded and inspected offline; its `claude` binary has
SHA-256 `4e21122a227857da1178aca3299700c1fd7f2b77c93f12e73c2c76db796a105e`
and size `236330608`, matching the wrapper manifest. The platform package
tarball SHA-256 is
`15a4b534a79a173bd33111399a14afede4214ffd17dfb1bddfc6bc620372be74`.
No downloaded artifact was executed.

## Complete hop inventory and selected-surface review

The fixture records all ten complete wrapper trees from the `0.3.284`
baseline through `0.3.293`, exact file SHA-256s, deterministic tree digests,
and exact added/removed/changed/identical path sets for each adjacent
published hop. All nine hops `0.3.285`–`0.3.293` are contiguous. The native
manifest version and commit rotate at each point; every one of its eight
platform checksums rotates at each hop. The wrapper file count stays at 19.
The published native harness schema remains 1. Each hop's path classifications
are recorded in `dist-inventory.json`; selected default-entry changes are
reviewed in [`protocol.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.293/protocol.json).

The sidecar imports only the package's `.` entry. It explicitly sets
`settingSources: []`, `skills: []`, `plugins: []`, `agents: {}`, `hooks: {}`,
`includePartialMessages: false`, `strictMcpConfig: true`, a selected permission
mode, an explicit native executable path, and bounded tool lists. It does not
load the credential-bearing `./bridge`, browser `./browser`, or `./core`
subpaths. The sidecar's selected query options, `Query` control methods, SDK
message union, per-turn usage fields, permission callback, bounded MCP status
projection, and close path retain the existing mapped shape. No mapped
behavior change was identified, so the existing behavior revision and public
route contract stay. Runtime capability observations continue to come from
first-turn `system/init`; declarations and npm metadata do not qualify runtime
behavior.

Per-hop findings, including exact unselected file/path classes, are:

- `0.3.285`: tool-server `alwaysLoad` now honors explicit server metadata
  opting out of prompt-time loading. The route sends `alwaysLoad: true` for
  required servers, keeps configured tools in its admitted set, and still
  mediates each call. This changes server-selected discovery timing, not route
  operations, authority, or grants. Added provider/settings keys are not
  supplied; settings sources are empty.
- `0.3.286`: in-process MCP initialization metadata is not used; only
  consumer-owned stdio servers are admitted. Agents remain empty, and
  Agent/Task tools stay excluded. Each open supplies an explicit permission
  mode, including the route's default when no host choice is present.
- `0.3.287`: partial-message and in-process MCP changes are outside the
  selected configuration. Large `structuredContent` is top-level SDK result
  metadata discarded by the sidecar; its user `tool_result` mapping remains
  bounded to tool-use id and error presence.
- `0.3.288`: added provider/task configuration and auto-compaction declaration
  members are not supplied; settings sources remain empty.
- `0.3.289`: no selected declaration delta. Default-entry byte changes are
  classified as build output/provider-internal parity after comparing the
  selected declarations and operations.
- `0.3.290`: WebFetch, WebSearch, Task, Chrome, tool aliases and settings
  changes are not selected. Sidecar inputs have no UUID, so same-UUID replay is
  disabled; partial-message handling remains off.
- `0.3.291`: agents and skills remain disabled, Agent/Task tools excluded, and
  no persistent permission-grant operation exists. Additional callback
  options do not enter the sidecar's one-shot allow/deny wire.
- `0.3.292`→`0.3.293`: subagent task members and new startup-failure reason
  variants are not mapped. No agents or Agent/Task tools are admitted; failure
  evidence remains bounded to the existing type/status facts.

The complete inventory also bounds unchanged or unselected subpaths, helper
bundles, tool declarations, and platform payloads. Files under selected
`sdk.d.ts` and `sdk.mjs` were reviewed at each hop for configuration, query,
permission, message, usage, failure, and lifecycle semantics. No credential,
authentication helper, or provider-facing artifact code was executed.

## Qualification and residual limits

The SDK package/native pair is qualified as a maintained segment from baseline
`0.3.284`/`2.1.284` through `0.3.293`/`2.1.293`, with the original claim IDs
and behavior revision. The open validator rejects nonmatching package/native
patch pairs. It leaves exact points `0.3.283`, `0.3.294`, `2.1.283`, and
`2.1.294` outside the range. The independent Node segment remains governed by
Research 387; private wire, sidecar source, configuration posture, and other
claims do not change. Existing `0.3.279`
absence remains below the segment; there are no published holes in this
segment. No `AllowUnverified` posture is added.

The newer compiled tuple does not gain the Research 301 live
registered-tool/consumer-tool-exchange evidence for `0.3.259`/`2.1.259`.
Both feature cells remain producer gaps pending their existing separate live
requalification gate. No live session, provider prompt, credential, host
installation/update, or workflow mutation occurred. No tag, release, or
publication is authorized.

## Local artifacts

- `tests/fixtures/claude-agent-sdk-0.3.293/identity.json` — selected-channel,
  exact wrapper and coupled native identities, every published package hop,
  and latest native platform manifest identity.
- `tests/fixtures/claude-agent-sdk-0.3.293/dist-inventory.json` — complete
  deterministic wrapper inventory and per-hop path/hash classifications.
- `tests/fixtures/claude-agent-sdk-0.3.293/protocol.json` — mapped surface,
  every selected declaration change, and per-hop classification rationale.
- `tests/claude_agent_sdk_identity.rs` — mutation-sensitive exact identity,
  hop, segment, and surface assertions.
