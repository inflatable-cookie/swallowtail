# Research 391: Deep Agents ACP 0.1.34 Identity and Qualification

Swallowtail#124 extends the existing `deepagents.acp` package window from
`0.1.30` through the official npm stable `0.1.34`. The approved axis is only
`deepagents-acp` npm package identity. npm `deepagents`, the ACP registry
discovery entry `0.1.7`, library embedding, and other Deep Agents routes are
not package points for this claim.

## Frozen identity

Official npm `latest` resolved to `0.1.34` on 2026-10-08. The release was
published at `2026-10-05T14:30:28.049Z`; its registry SHA-512 integrity is
`sha512-2c8wt/Na9AjiTBINRRfDXSR8sdsFEpd1ser+dJdIzZu7VvvLQdNQTLzjgTwVym7DsgDCkMjN/AEAfIJiqrr3uA==`
and tarball SHA-256 is
`7b5faf49d58b6afede606cb45d5da70569ee85b671bf7e336eac7e79ec60963f`. Exact
artifact metadata for `0.1.31`–`0.1.34`, the previous qualified `0.1.30`,
and runtime `deepagents` `1.13.4`–`1.14.2` is frozen in the [identity
fixture](../../crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.34/identity.json).
The identity freeze preceded the production claim edit.

The package pins four stable hops after the previous ceiling:

| `deepagents-acp` | Publication time (UTC) | Exact `deepagents` runtime |
| --- | --- | --- |
| `0.1.31` | `2026-09-16T22:18:46.048Z` | `1.13.5` |
| `0.1.32` | `2026-09-18T15:59:34.262Z` | `1.14.0` |
| `0.1.33` | `2026-09-24T13:56:39.118Z` | `1.14.1` |
| `0.1.34` | `2026-10-05T14:30:28.049Z` | `1.14.2` |

All five ACP package trees have eleven files. Every hop changes only
`package.json`, advancing the exact runtime pin. The selected `dist/cli.js`,
`dist/index.js`, `dist/index.cjs`, `dist/index.d.ts`, and `dist/index.d.cts`
are byte-identical across `0.1.30`–`0.1.34`. npm metadata has no `gitHead`;
the exact registry tarballs, complete extracted inventories, and shipped
source maps are the source identity. See the [protocol and hop
ledger](../../crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.34/protocol.json)
and [complete tree inventory](../../crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.34/dist-inventory.json).

## Selected-path classification

The route remains the host-approved `deepagents-acp` executable, with no
extra arguments, over ACP v1 stdio. It selects `initialize`, `session/new`,
`session/prompt`, and `session/cancel`; `session/load`, `session/set_mode`,
`session/close`, slash commands, and client MCP servers remain unselected.
The existing mapping still observes tool permissions and cancels without
selecting `allow_always`. It makes no usage claim and keeps one in-process
session with joined process, connection, and task cleanup.

The exact-pinned runtime chain has four hops. The source maps in each npm
artifact identify the changed TypeScript sources, and the inventory records
their full compiled package trees.

| Runtime hop | Selected change and boundary |
| --- | --- |
| `1.13.4` → `1.13.5` | `src/middleware/fs.ts` reformats `read_file` text with an `@@` range header; the CLI relays it as text in the existing ACP `tool_call_update` content slot. `src/middleware/skills.ts` changes skill-state caching, while the selected CLI has `skills: []`. |
| `1.13.5` → `1.14.0` | Skills state/reducer and agent generic types change; subagent skill state becomes readonly. The selected CLI has no skills or subagents, and ACP frames are unchanged. |
| `1.14.0` → `1.14.1` | `FilesystemBackend` real-path containment checks run only with `virtualMode: true`; the CLI uses the default `false`. The oversized-result fallback changes path IDs and text only for long IDs, within the same opaque ACP result slot. |
| `1.14.1` → `1.14.2` | `UnsupportedContentMiddleware` changes model-request messages without rewriting stored messages or ACP events. Binary offload remains disabled by default and unselected. The internal retry for provider HTTP 400 on multimodal `read_file` content may affect model outcomes, but not the tool result already emitted to ACP or the ACP event/lifecycle mapping. Subagent changes remain unselected. |

`deepagents-acp` retains `@agentclientprotocol/sdk: ^1.1.0` and peer floors
`@langchain/core: ^1.1.40`, `@langchain/langgraph: ^1.4.10`. At runtime
`deepagents@1.14.2` raises its `@langchain/core` peer floor from `^1.2.9` to
`^1.2.14`; dependency resolution is host-controlled and this qualification
does not claim a resolved peer tree. The source package has no registry
`gitHead`, and no source repository identity is inferred from changelog text.

## Claim decision

The selected CLI and ACP library entrypoints are byte-identical across all
five points. Package changes advance only the exact runtime pin; runtime
changes either affect opaque result text in an existing content slot, inactive
configuration, or model-internal handling. No published hop adds an ACP
operation, wire shape, capability, permission option, usage event, authority,
or lifecycle path. This is a compatible extension under the same public
behavior contract.

The production claim now retains baseline `0.1.30`, claim id
`deepagents.acp.package-window-1`, behavior revision
`deepagents.acp.stdio-v1`, `QualifiedOnly` posture, and no exclusions, while
qualifying maintained `0.1.30..=0.1.34`. Stable points below `0.1.30` and
above `0.1.34` remain incompatible. `0.1.35` was not published at identity
freeze, and no unverified-newer execution is introduced. The existing
Research 322 identity and prior qualification remain historical evidence.

No package was installed, no downloaded artifact was executed, and no
provider prompt, credential, login, live ACP initialize, or host update was
performed. The host has no `deepagents-acp` executable on `PATH`.
