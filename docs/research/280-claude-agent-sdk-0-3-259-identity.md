# 280 Claude Agent SDK 0.3.259 Identity

Status: promoted; corrected 2026-09-05 (spawn hook call shape)
Owner: Tom
Date: 2026-09-03
Card: g05 batch 055 (selected refresh exit)
Authority: Contract 029; Research 278; official npm registry and the frozen
`@anthropic-ai/claude-agent-sdk` `0.3.258` and `0.3.259` tarballs

## Question

Card 055 stopped when official npm stable moved off the qualified point. The
operator selected the refresh exit. Can the exact `claude-agent.sdk` package and
native axes rebind from `0.3.258` to `0.3.259` without changing the mapped
behavior, the behavior revision, or the credential boundary?

Answer: yes. The selected mapped subset is unchanged, every changed shipped file
is classified, and no delta touches the wire, lifecycle, permissions, usage,
capability, session, model, tool-admission, or failure behavior this route maps.
The rebind is an exact one-point move on two coupled axes. No stop fired.

## Method

Both tarballs were downloaded to `/tmp`, hashed, and extracted. Nothing was
executed. No platform package was downloaded; the native binaries are identified
from the shipped `manifest.json` digests. No host was changed, no provider
session or login occurred, and no token was read.

A deterministic 15-file package-tree inventory was derived per version with
SHA-256 per relative path, then diffed. Declarations were diffed in full;
the shipped implementation was compared by targeted invariant probes because it
is a minified bundle where symbol renames dominate a textual diff.

Frozen evidence:
`../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.259/`.

## 1 Artifact identity

| Surface | `0.3.258` | `0.3.259` |
| --- | --- | --- |
| dist-tags | (superseded) | `latest` and `next` |
| Published | `2026-09-01T22:24:14.573Z` | `2026-09-02T21:22:40.857Z` |
| Tarball SHA-256 | `656cf237…b398` | `0c5740e4…3f7e` |
| Tarball SHA-1 | `7d44d24c…b88a` | `daf465f8…a012` |
| Files / unpacked | 15 / 5 019 497 | 15 / 5 043 385 |
| `claudeCodeVersion` | `2.1.258` | `2.1.259` |

The `0.3.258` digests reproduce Research 278 exactly, so this is a clean hop
from the previously frozen point. No published stable exists between them.

## 2 Package-tree inventory

15 files in both versions: **7 identical, 8 changed, 0 added, 0 removed.**

Identical: `LICENSE.md`, `README.md`, `agentSdkTypes.d.ts`, `bridge.d.ts`,
`browser-sdk.d.ts`, `extractFromBunfs.d.ts`, `extractFromBunfs.js`.

Changed: `bridge.mjs`, `browser-sdk.js`, `manifest.json`, `manifest.zst.json`,
`package.json`, `sdk-tools.d.ts`, `sdk.d.ts`, `sdk.mjs`.

`package.json` changed only in `version`, `claudeCodeVersion`, and the eight
platform `optionalDependencies` pins. The `exports` map is identical, so the
`.` entry point and the separate `/bridge` and `/browser` subpaths are
unchanged. `engines.node >=18.0.0`, the three peer dependencies, and the license
field are unchanged.

## 3 Classified declaration deltas

The complete `sdk.d.ts` diff is nine hunks. None is mapped.

- **`Options.permissionPrompts?: 'host' | 'none'`** — new optional input. The
  implementation forwards `--permission-prompts` only when it is set, so an
  unset option keeps the default host surface and `canUseTool` still governs
  admission. `'none'` would deny anything that would otherwise prompt and never
  call `canUseTool`, so the route must not set it. It does not, and a
  mutation-sensitive test asserts the shipped asset never names it.
- **`user_message_uuids?: string[]`** — added on four record shapes as an
  additive sibling of `user_message_uuid`, for binding a reply to a prompt
  batch. The route reads neither field. Unknown record *fields* are ignored
  while unknown record and part *types* still fail closed, so the addition
  cannot reach behavior.
- **task `summary`** — documentation only; the field already existed. Background
  tasks are outside this route.
- **`Settings.managedMcpServers`** plus the `allowedMcpServers` and
  `managedSourcesBehavior` prose rewrites — managed-settings tier. The route
  passes `settingSources: []`, no MCP servers, and `strictMcpConfig`, so no
  settings source is loaded at all.
- **`sdk-tools.d.ts`** — two optional fields on a skill-publishing tool and a
  reworded skill-description comment. Skills are prohibited on this route.

## 4 Implementation invariants

Probed directly in the shipped `sdk.mjs` of both versions:

- `canUseTool` still pushes `--permission-prompt-tool stdio` and still conflicts
  with `permissionPromptToolName`; the option destructuring and call sites are
  structurally identical.
- `spawnClaudeCodeProcess` is present with the same call sites, so the
  independently joinable native handle this route depends on still exists.
  **Correction 2026-09-05 (card 100):** the hook is present, but its call
  shape was not verified here and the fake-SDK fixture mirrored the sidecar's
  own assumption. `0.3.259` calls `spawnClaudeCodeProcess(options)` with one
  `SpawnOptions` object `{command, args, cwd?, env, signal}` and expects a
  `SpawnedProcess` with piped stdin/stdout plus `kill`/`on`/`once`/`off`;
  the `v0.4.1` sidecar took positional arguments and threw at construction
  on the first live open (Bovine Desktop, 2026-09-05). The declarations are
  now frozen in the adapter's `tests/fixtures/claude-agent-sdk-0.3.259/`
  `sdk-declarations.d.ts`, together with `Query.initializationResult()`,
  `supportedModels()`, and `accountInfo()`. "Same call sites" must not be
  read as "same call shape".
- The bounded `waitForExit()` race remains and the `SIGKILL` escalation is still
  `unref()`'d: the SDK still supplies no joined stop, which is the premise of
  the route's own close design.
- The `sdk-exit-after-stderr-drained` remap remains a single internal event.

## 5 Credential non-custody

Re-verified on `0.3.259`, identical to Research 278: the ten-pattern search over
the `.` entry declarations returns three prose hits and no declared value; there
are 17 exported functions and no login, logout, or OAuth export.

`bridge.d.ts` and `browser-sdk.d.ts` are **byte-identical** across the hop, so
the credential-bearing subpath declarations did not move. Their implementations
changed, but the route never imports them and a mechanical grep enforces that.

## 6 Native artifact rotation

`manifest.json` moves `2.1.258` → `2.1.259`, commit
`b3cd543a1f6fcdf4d8fabc0f5e5538d2ee7f38e1` → `9b549c8d1c72e407ea9d3af3b9d5e50da794ec4d`,
build date `2026-09-01T22:02:56Z` → `2026-09-02T20:40:41Z`. All eight platform
binaries rotate digest and grow slightly.

`sdkCompat.harnessSchema` stays `1`. That is the surface that would force a new
behavior revision; a rotation alone does not.
`sdkCompat.testedWrapperVersions` still tops out at `0.3.227`, so the shipped
declaration-versus-artifact discrepancy Research 278 recorded persists.

## 7 Decision

Rebind two coupled axes, keep everything else:

- `claude-agent.sdk.package`: exact `0.3.258` → exact `0.3.259`
- `claude-agent.sdk.native`: exact `2.1.258` → exact `2.1.259`
- unchanged: `claude-agent.sdk.node`, `claude-agent.sdk.wire`,
  `claude-agent.sdk.sidecar`, every claim id, the `claude-agent.sdk-v1`
  behavior revision, and the QualifiedOnly posture with no unverified-newer

The route continues to qualify exactly one point per axis. `0.3.258` becomes
unqualified rather than a second supported point, and `0.3.260` does not exist.

## 8 Falsification

| Claim | Falsifier | Result |
| --- | --- | --- |
| `0.3.259` is official latest | npm dist-tags | held; also `next` |
| The hop is clean from the frozen point | re-hash `0.3.258` | held; matches Research 278 |
| 15 files, 8 changed | full tree inventory | held; frozen in `dist-inventory.json` |
| Entry points unchanged | compare `exports` | held; byte-identical map |
| Credential subpath declarations unchanged | compare digests | held; byte-identical |
| No token material in the `.` entry | ten-pattern search | held; three prose hits |
| The SDK now offers a joined stop | probe `waitForExit` and `SIGKILL` | refuted; still a discarded race, still `unref()`'d |
| `permissionPrompts` changes admission when unset | read the forwarding branch | refuted; flag pushed only when set |
| Native rotation implies a protocol change | compare `harnessSchema` | refuted; stays `1` |

## 9 Withheld

No new mapped surface. `permissionPrompts`, `user_message_uuids`,
`managedMcpServers`, task summaries, skill publishing, `/bridge`, and
`/browser` all stay unmapped for the reasons above. No claim, guide, or matrix
statement widens beyond the one-point rebind.
