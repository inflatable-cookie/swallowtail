# 367 Claude Agent SDK 0.3.284 Tuple Identity

Status: promoted
Owner: operator-authorized family qualification
Date: 2026-09-29
Task: swallowtail#089
Authority: Contract 029; Research 278, 280, 287, 315, 329, 366; official npm
registry and the frozen `@anthropic-ai/claude-agent-sdk` `0.3.270`
through `0.3.284` tarballs

## Question

Official npm stable moved from the qualified `0.3.270` to `0.3.284`. Can
the exact `claude-agent.sdk` package and native axes rebind from
`0.3.270`/`2.1.270` to `0.3.284`/`2.1.284` without changing the mapped
behavior, the behavior revision, or the credential boundary — and does
the consumer stdio MCP path (`Options.mcpServers`, `strictMcpConfig`,
`alwaysLoad`, `mcpServerStatus` projection, `canUseTool` of `mcp__`
tools, and `close()`) stay intact?

Answer: yes. The selected mapped subset is unchanged. The one Card 146
follow-through is that `McpServerStatus` gained optional `source` at
`0.3.274`; the sidecar allowlist admits that declared key and discards
it, so projected evidence stays name + status + optional failureCode.
Research 301 stays bound to `0.3.259`/`2.1.259`.

## Remaining Rank

This run covers only `claude-agent.sdk`. At observation the family was
exact QualifiedOnly at `0.3.270`/`2.1.270` while official `latest` was
`0.3.284`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-agent.sdk` | Node `22.23.2`; host `claude` `2.1.283` (not the bundled native) | npm `@anthropic-ai/claude-agent-sdk` `0.3.284`; `next` `0.3.285` ignored | exact package `0.3.270`, native `2.1.270`; QualifiedOnly | official-newer |

## Method

Re-probed npm `@anthropic-ai/claude-agent-sdk@latest` (`0.3.284`) and
`next` (`0.3.285`). Downloaded the official tarballs for previous
ceiling `0.3.270` and every published hop `0.3.271`–`0.3.278`,
`0.3.280`–`0.3.284` into `/tmp`. Every tarball was SHA-256 and SHA-1
hashed; the `0.3.270` values reproduce Research 315 exactly.

Extracted packages were never executed. No platform package was
downloaded. Native identity is the shipped `manifest.json` version,
commit, build date, `sdkCompat.harnessSchema`, and per-platform
checksums. A deterministic package-tree inventory with SHA-256 per
relative path was derived and frozen in `dist-inventory.json`.

Host observations only: Node `22.23.2` on `PATH`, installed Claude Code
`2.1.283` (identifies only itself). No host was installed, updated, or
replaced. No provider prompt, login, credential, or live session.

Changelog is discovery only. Official latest was re-probed after
evidence extraction: still `0.3.284`.

Frozen evidence:
`../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.284/`.

## 1 Artifact identity

Official `0.3.284`: `latest` only, published `2026-09-28T17:15:09.221Z`,
tarball SHA-256
`4550e830246026133fc1802a2208dd0f3a785cae1eec83f261d114c33d797771`,
SHA-1 `28ab0fde5207c162d8dd1dbaaac5e0f5d5ec46c7`, 19 files. The
`0.3.270` digests reproduce Research 315, so this is a clean chain from
the previously frozen point.

Published stables after `0.3.270`: `0.3.271`–`0.3.278` and
`0.3.280`–`0.3.283`. `0.3.279` is unpublished and stays a gap. `next` is
`0.3.285` and is ignored. First unpublished later stable: `0.3.286`.
Every wrapper `0.3.X` carries its coupled native `2.1.X`;
`claudeCodeVersion` equality holds on all fourteen points.

## 2 Package-tree inventory

15 files through `0.3.281`. At `0.3.282` the tree gains `./core`
(`core.mjs`, `core.d.ts`, two hashed `core-*.mjs` chunks): 19 files.
Hashed core chunks rotate on `0.3.283` and `0.3.284` (add two, remove
two). Four files are identical through all fourteen points:
`LICENSE.md`, `agentSdkTypes.d.ts`, `extractFromBunfs.d.ts`,
`extractFromBunfs.js`. `README.md` moves once (`0.3.281→0.3.282`).
`bridge.d.ts` moves twice (`0.3.276→0.3.277`, `0.3.280→0.3.281`).

`package.json` `exports['.']` stays `{types: ./sdk.d.ts, default:
./sdk.mjs}`. `./core` is additive. `engines.node >=18.0.0` is unchanged.
`optionalDependencies` rotate the eight platform pins with the wrapper.

## 3 Classified declaration deltas

Net `sdk.d.ts` moves on ten of thirteen hops. Load-bearing mapped
surfaces that stayed equal `0.3.270` to `0.3.284`: `query()`,
`Options.mcpServers`, `Options.strictMcpConfig`,
`McpStdioServerConfig` except the new unmapped
`bareElicitationCapability`, `alwaysLoad`, `PermissionResult`,
`spawnClaudeCodeProcess`, `SpawnOptions`, `SpawnedProcess`,
`AccountInfo`, `canUseTool` as an `Options` field,
`permissionPromptToolName`, `interrupt()`, `Query.close()`.

Classified unmapped additions are frozen in `protocol.json`. The
consumer-path ones:

- **`McpServerStatus.source?`** (`0.3.274`) — optional provenance.
  Card 146 admits every declared top-level key and discards metadata.
  The sidecar allowlist adds `source`. Projected evidence does not
  change.
- **`McpServerStatus.tools[]._meta?`** (`0.3.280`) — nested under
  already-allowed `tools`.
- **`CanUseTool.options.mcpServer?`** — sidecar callback is
  `(toolName, input)` only.
- **`Query.readMcpResource`**, **`prewarm`**, **`./core`** — unused.

## 4 Implementation invariants

Probed in shipped `sdk.mjs` of `0.3.284` against `0.3.270`:

- `canUseTool` still conflicts with `permissionPromptToolName` and
  still pushes `--permission-prompt-tool stdio` by default;
  `--permission-prompts` is pushed only when set.
- Non-empty `mcpServers` still become `--mcp-config` JSON
  `{mcpServers: map}`; `strictMcpConfig` still pushes
  `--strict-mcp-config`.
- `mcpServerStatus()` is still a passthrough of native `mcp_status`
  rows.
- `spawnClaudeCodeProcess` still receives one `SpawnOptions` object.
  The sidecar still destructures `{command, args, cwd, env, signal}`.
- `Query.close()` is still `close(): void`.
- The bounded `waitForExit()` 2000 ms race remains. ProcessTransport
  still SIGTERM then unref'd SIGKILL after 5s. The in-process bash
  `process.kill(-pid)` left `sdk.mjs` at `0.3.281`; that tool is
  unmapped.
- Bundled MCP versions still offer `2025-11-25` first with the same
  five supported versions.

## 5 Credential non-custody

Re-verified on `0.3.284`: the ten-pattern search over the `.` entry
declarations returns the same three prose hits. Exported functions stay
the Research 315 set plus `prewarm`. No login, logout, or OAuth export.
`bridge.d.ts` changed only on the prohibited `/bridge` thinking-display
hook. The sidecar still imports only the host-provided `.` module path.

## 6 Native artifact rotation

`manifest.json` moves `2.1.270` → `2.1.284`, commit
`97ecbf7abeb4170dcfd26c4d4b397afd9015030e` →
`2b8ce618c24de26410e4bdfc4e1d592accd61f61`. All eight platform binaries
rotate digest on every hop. `sdkCompat.harnessSchema` stays `1` on all
fourteen points. `testedWrapperVersions` still excludes the shipping
wrapper (topping at `0.3.282` on the `0.3.284` point).

## 7 Decision

Rebind two coupled axes, keep everything else, update the Card 146
allowlist:

- `claude-agent.sdk.package`: exact `0.3.270` → exact `0.3.284`
- `claude-agent.sdk.native`: exact `2.1.270` → exact `2.1.284`
- sidecar source admits `source` as a discarded declared key; the
  sidecar axis tag stays the crate version
- unchanged: `claude-agent.sdk.node`, `claude-agent.sdk.wire`, claim
  ids, the `claude-agent.sdk-v1` behavior revision, and QualifiedOnly
  with no unverified-newer

The route continues to qualify exactly one point per axis. `0.3.270`
becomes unqualified rather than a second supported point. `0.3.285`
(`next`) and unpublished `0.3.279`/`0.3.286` stay out.

The registered-tool courier stays non-production. Research 301 stays
bound to `0.3.259`/`2.1.259`.

## 8 Falsification

| Claim | Falsifier | Result |
| --- | --- | --- |
| `0.3.284` is official latest | npm dist-tags | held; `next` is `0.3.285` |
| The chain is clean from the frozen point | re-hash `0.3.270` | held; matches Research 315 |
| 15 files then 19 after `./core` | full tree inventory | held; frozen in `dist-inventory.json` |
| `.` entry unchanged | compare `exports['.']` | held |
| MCP stdio path unchanged | `--mcp-config` / `strict-mcp-config` / `alwaysLoad` literals | held |
| `source` is discarded, not projected | sidecar allowlist + projection | held by construction; fake rows carry a fixture-only `source` |
| Credential subpath still unused | sidecar falsifier; `/bridge` never imported | held |
| The SDK now offers a joined stop | probe `waitForExit` and SIGKILL | refuted; still a discarded race, still unref'd |
| Native rotation implies a protocol change | compare `harnessSchema` | refuted; stays `1` |
| MCP constants moved | literal search | refuted; identical version list |

## 9 Withheld

No new mapped surface. `source` is admitted only so Card 146 stays
honest; it is not projected. `prewarm`, `./core`, `readMcpResource`,
`mcpServer` on `CanUseTool`, `bareElicitationCapability`, `/bridge`,
`/browser`, settings-tier keys, and skill/tool shapes stay unmapped.
The registered-tool courier is not widened.
