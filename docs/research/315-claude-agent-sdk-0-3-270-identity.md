# 315 Claude Agent SDK 0.3.270 Tuple Identity

Status: promoted
Owner: Tom
Date: 2026-09-14
Card: g05.065 (Research 308 useful-newer campaign)
Authority: Contract 029; Research 278, 280, 287, 301, 308; official npm
registry and the frozen `@anthropic-ai/claude-agent-sdk` `0.3.259`
through `0.3.270` tarballs

## Question

Official npm stable moved from the qualified `0.3.259` to `0.3.270`. Can
the exact `claude-agent.sdk` package and native axes rebind from
`0.3.259`/`2.1.259` to `0.3.270`/`2.1.270` without changing the mapped
behavior, the behavior revision, or the credential boundary — and what
happens to Research 301's exact-tuple live registered-tool qualification?

Answer: yes to the rebind; no to transferring the live evidence. The
selected mapped subset is unchanged, every changed shipped file is
classified, and no delta touches the wire, lifecycle, permissions, usage,
capability, session, model, tool-admission, or failure behavior this route
maps. The rebind is an exact one-point move on two coupled axes. No stop
fired. Research 301 stays bound to `0.3.259`/`2.1.259`; the newer compiled
tuple projects `registered_tools` and `consumer_tool_exchange` unqualified
until a separately authorized live requalification runs on the new tuple.

## Method

All ten wrapper tarballs (`0.3.259` plus the nine published stables
`0.3.260`, `0.3.261`, `0.3.263`, `0.3.265`, `0.3.266`, `0.3.267`,
`0.3.268`, `0.3.269`, `0.3.270`) were downloaded to `/tmp`, hashed, and
extracted. Nothing was executed. No platform package was downloaded; the
native binaries are identified from the shipped `manifest.json` digests.
No host was changed, no provider session or login occurred, and no token
was read. Host observations only: Node `22.23.2` at
`/Users/tom/.local/bin/node`, installed Claude Code `2.1.258` (identifies
only itself, not the bundled native target).

A deterministic 15-file package-tree inventory was derived per version
with SHA-256 per relative path, then diffed per consecutive hop.
Declarations were diffed in full; the shipped implementation was compared
by targeted invariant probes because it is a minified bundle where symbol
renames dominate a textual diff.

Frozen evidence:
`../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/`.

## 1 Artifact identity

Official `0.3.270`: `latest` and `next`, published
`2026-09-12T18:53:13.002Z`, tarball SHA-256
`36e86fc13a1ddc8c026dcf5e5bcf72f4107bcbceaff1a45e7f81c5d44d703575`,
SHA-1 `2a1d5fd4e265320a64e62607ae87567bd7b955d8`, 15 files. The `0.3.259`
digests reproduce Research 280 exactly, so this is a clean chain from the
previously frozen point.

Published stables after `0.3.259`: `0.3.260`, `0.3.261`, `0.3.263`, and
`0.3.265..=0.3.270`. `0.3.262` and `0.3.264` are unpublished gaps and stay
gaps. First unpublished later stable: `0.3.271`. Every wrapper `0.3.X`
carries its coupled native `2.1.X`; `claudeCodeVersion` equality holds on
all ten points.

## 2 Package-tree inventory

15 files on every point, 0 added, 0 removed on every hop. Six files are
identical through all ten points: `LICENSE.md`, `README.md`,
`agentSdkTypes.d.ts`, `bridge.d.ts`, `extractFromBunfs.d.ts`,
`extractFromBunfs.js`. Per-hop changed sets are frozen in
`dist-inventory.json`. Three hops carry zero declaration changes (pure
native rotation and metadata): `0.3.261→0.3.263`, `0.3.265→0.3.266`,
`0.3.269→0.3.270`. The `browser-sdk.d.ts` declaration moved exactly once
(`0.3.266→0.3.267`, SSE resume cursors on the prohibited `/browser`
subpath); `bridge.d.ts` never moved.

`package.json` changed only in `version`, `claudeCodeVersion`, and the
eight platform `optionalDependencies` pins on every hop. The `exports`
map is identical on all ten points, so the `.` entry point and the
separate `/bridge` and `/browser` subpaths are unchanged.
`engines.node >=18.0.0`, peer dependencies, and license are unchanged.

## 3 Classified declaration deltas

The net `sdk.d.ts` diff is 59 hunks; all 42 named surfaces are classified
in `protocol.json` and none is mapped. The load-bearing ones:

- **`Options.pluginDelivery?: 'argv' | 'initialize'`** — new optional
  input, default `'argv'`. The route passes `plugins: []` with the
  selector unset, so zero `--plugin-dir` flags flow either way. A
  mutation-sensitive test asserts the sidecar never names it.
- **Ask-dialog `defaultToNo?` / `suppressAlwaysAllowRule?`** — optional
  rendering hints on the permission-ask dialog record. The `CanUseTool`
  callback signature and the `PermissionResult` union are byte-identical,
  and the sidecar answers programmatically without rendering dialogs.
- **`systemPromptSnapshot` default prose** — omitted-or-true now records.
  The route passes a constant digest-pinned `systemPrompt` only for
  selected-skill bundles and never varies the text, so the rendered wire
  prompt is identical. No sidecar change.
- **`sdkMcpServerManifests?` / initialize `plugins[]`** — one-shot
  handshake cache and stdin plugin form, both loaded only under flows the
  sidecar never uses (`--await-initialize`). Never sent, nothing loads.
- **`SDKAssistantMessageError` gains `verification_required` and
  `cloud_credential_error`** — the sidecar assistant projection never
  reads `message.error` (content parts only) and the Rust wire decodes
  sidecar-projected events, so no new string reaches classification.
- **New controls** (`reload_output_styles`, `list_permission_rules`,
  `get_hooks_listing`, `+3` union members), **result/usage/thinking
  additive fields** (`resume_reason`, `result_index`, `limitScope`,
  `skipBehaviors`, `betas`, usage `kind`), **Settings-tier keys**
  (`maxEffortLevel`, plugin ordering, gateway, output tuning) — all on
  surfaces the route never calls, reads, or loads (`settingSources: []`).
- **`sdk-tools.d.ts`** — project-memory methods, artifact fields, and
  skill prose; skills and tool I/O shapes stay prohibited.
- **`Transport.markDelivered?`** — optional cursor hook; the sidecar
  implements no `Transport`.

`query()`, `streamInput`, `close()`, `SpawnOptions`/`SpawnedProcess`,
`initializationResult`, `supportedModels`, `accountInfo` — the mapped
signatures are unchanged; `query()` moved only in line number.

## 4 Implementation invariants

Probed directly in the shipped `sdk.mjs` of `0.3.270` against `0.3.259`:

- `canUseTool` still conflicts with `permissionPromptToolName` and still
  pushes `--permission-prompt-tool stdio` by default; `--permission-prompts`
  is pushed only when set.
- `spawnClaudeCodeProcess(options)` still takes one `SpawnOptions` object;
  the sidecar destructures exactly `{command, args, cwd, env, signal}`.
- The bounded `waitForExit()` race remains and the `SIGKILL` escalation is
  still `unref()`'d: the SDK still supplies no joined stop.
- The `sdk-exit-after-stderr-drained` remap remains a single internal event.
- The bundled MCP literal still offers `2025-11-25` first with the same
  five supported versions on every hop (minified names only).

## 5 Credential non-custody

Re-verified on `0.3.270`, identical to Research 278/280: the ten-pattern
search over the `.` entry declarations returns the same three prose hits
at shifted lines; 17 exported functions, identical set, no login, logout,
or OAuth export. `bridge.d.ts` is byte-identical across all ten hops. The
sidecar imports only the host-provided `.` module path, asserts
`sdk.query` is a function, and the Research 278 falsifier set
(`bridge`, `browser-sdk`, `fetchRemoteCredentials`, `worker_jwt`,
`OAuthCredential`, `apiKeyHelper`, refresh helpers, `ANTHROPIC_API_KEY`)
still returns zero hits on the sidecar source.

## 6 Native artifact rotation

`manifest.json` moves `2.1.259` → `2.1.270`, commit
`9b549c8d1c72e407ea9d3af3b9d5e50da794ec4d` →
`97ecbf7abeb4170dcfd26c4d4b397afd9015030e`, build date
`2026-09-02T20:40:41Z` → `2026-09-12T18:17:55Z`. All eight platform
binaries rotate digest on every hop. `sdkCompat.harnessSchema` stays `1`
on all ten points. `testedWrapperVersions` still excludes the shipping
wrapper (topping at `0.3.269` on the `0.3.270` point), so the
declaration-versus-artifact discrepancy persists, unchanged in kind.

## 7 Decision

Rebind two coupled axes, keep everything else:

- `claude-agent.sdk.package`: exact `0.3.259` → exact `0.3.270`
- `claude-agent.sdk.native`: exact `2.1.259` → exact `2.1.270`
- unchanged: `claude-agent.sdk.node`, `claude-agent.sdk.wire`,
  `claude-agent.sdk.sidecar`, every claim id, the `claude-agent.sdk-v1`
  behavior revision, and the QualifiedOnly posture with no unverified-newer

The route continues to qualify exactly one point per axis. `0.3.259`
becomes unqualified rather than a second supported point, and `0.3.271`
does not exist.

The registered-tool feature does not follow the rebind. Research 301's
acceptance ran only on `0.3.259`/`2.1.259`; identical bundled MCP
constants on `0.3.270` are provider-free equality, not live evidence. The
claim names the consequence: the compiled tuple projects
`registered_tools` and `consumer_tool_exchange` unqualified, the
qualification function additionally requires the compiled tuple to equal
the frozen live tuple, and the feature matrix returns both cells to
producer-gap with this research as the cross-reference.

Follow-up: a separately authorized live registered-tool requalification
on exact SDK `0.3.270` / native `2.1.270` (same Desktop Card 318 shape as
Research 301). It updates the frozen live tuple and re-opens the two
cells; nothing in this task presumes its outcome.

## 8 Falsification

| Claim | Falsifier | Result |
| --- | --- | --- |
| `0.3.270` is official latest | npm dist-tags | held; also `next` |
| The chain is clean from the frozen point | re-hash `0.3.259` | held; matches Research 280 |
| 15 files, 6 identical through all hops | full tree inventory | held; frozen in `dist-inventory.json` |
| Entry points unchanged | compare `exports` on all ten | held; identical map |
| Credential subpath declaration unchanged | compare digests | held; `bridge.d.ts` byte-identical |
| No token material in the `.` entry | ten-pattern search | held; same three prose hits |
| The SDK now offers a joined stop | probe `waitForExit` and `SIGKILL` | refuted; still a discarded race, still `unref()`'d |
| `permissionPrompts` changes admission when unset | read the forwarding branch | refuted; flag pushed only when set |
| `pluginDelivery` changes the route's launch | read the default branch | refuted; unset defaults to `argv` |
| Native rotation implies a protocol change | compare `harnessSchema` | refuted; stays `1` on all ten |
| MCP constants moved | literal search per hop | refuted; identical version list |

## 9 Withheld

No new mapped surface. `pluginDelivery`, ask-dialog hints,
`systemPromptSnapshot` default prose, `sdkMcpServerManifests`,
initialize `plugins`, interrupt latch prose, new error variants, new
controls, result/usage/thinking additive fields, Settings-tier keys,
`Transport.markDelivered`, skill/tool shapes, `/bridge`, and `/browser`
all stay unmapped for the reasons above. No claim, guide, or matrix
statement widens beyond the one-point rebind plus the honest live-feature
gating named in §7.
