# Kimi Code 0.43.0 installed currentness corpus

Secret-free identity corpus for official npm and GitHub
`@moonshot-ai/kimi-code@0.43.0`, frozen before Swallowtail moved any
`kimi-code.executable` claim. It covers the two installed-harness axes only:
`kimi-code.acp` and `kimi-code.headless`. `kimi-code.local-server` is a
separate currentness family (Research 282) and is untouched here even though it
ships in the same package.

`identity.json` holds npm and GitHub identity for every stable hop from the
previous ceilings through `0.43.0`: `0.39.1`, `0.40.0`, `0.40.1`, `0.41.0`,
`0.42.0`, and `0.43.0`. Each point carries the npm published time, registry
`integrity` and `shasum`, tarball SHA-256, file count, unpacked size, `bin`
entry, Node engine floor, and the `dist/main.mjs` digest and size, plus the
GitHub annotated-tag object, commit, tree, tagged-source archive SHA-256, and
the release-asset digests GitHub publishes for the darwin-arm64, linux-x64, and
manifest assets. Every npm `integrity`, `shasum`, and tarball digest was
recomputed from the downloaded tarball; every tagged source archive was
downloaded and hashed; every tag object was resolved to its commit and tree.

Host `kimi 0.34.0` stays at
`sha256:9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`,
size `176894272`, observed only through `--version` and its digest. Nothing was
installed, updated, replaced, or executed, and no host path appears in a
fixture.

Downloaded artifacts stayed in disposable scratch space and were never
executed. Identity comes from digests, git blobs, and static extraction only.

## Selected-file ledger

`protocol.json` freezes one deterministic `path -> git blob sha1` map per
extracted GitHub tagged source tree for every selected ACP and headless surface
and every selected-adjacent file that moves inside the window, at all six
points. A digest is the SHA-1 of the git blob
(`sha1("blob <len>\0" + bytes)`); `null` means the path does not exist at that
point. A silent edit to any selected surface has to survive this map, the
per-hop changed-path lists, the bundle oracle, and the live claim tests at once.

## Per-hop classification

Each of the five stable hops carries an independent verdict for each claim.

| Hop | ACP | Headless |
| --- | --- | --- |
| `0.39.1→0.40.0` | compatible selected-adjacent: the legacy `acp-adapter` package and `acp-native.ts` are removed, `sub/acp.ts` imports `@moonshot-ai/acp-server` directly, and `server.ts` requires a successful `restore()` after `session/fork`; `acpTerminalRunner.ts`, `acp-fs/`, `start.ts`, `modes.ts`, `approval.ts`, `config-options.ts`, and `model-catalog.ts` are byte-identical | compatible: `experimental-v2.ts` doc comment, `run-v2-print.ts` agent-scoped goal/cron lookup and `nonInteractive` request header; JSONL dispatch and writers unchanged |
| `0.40.0→0.40.1` | identical: no ACP source change | identical: only `apps/kimi-code/package.json` moves |
| `0.40.1→0.41.0` | identical: no ACP source change | compatible: the print-mode shutdown quiesce/flush and telemetry work; emitted grammar unchanged |
| `0.41.0→0.42.0` | compatible selected-adjacent: image format gating moves to the provider-bound engine; tool-event type imports move to agent-core-v2 re-exports; no authority, callback, or permission surface moves | compatible: the v1 print body and the `KIMI_CODE_LEGACY_FLAG` gate are deleted, `run-v2-print.ts` submits through `IAgentLoopService`, and a trust-gated MCP warning is written to stderr; `prompt-render.ts`, `options.ts`, and the dispatch switch are byte-identical |
| `0.42.0→0.43.0` | compatible selected-adjacent: `replay.ts` suppresses hidden thinking deltas; no authority surface moves | compatible: prompt submission moves to `IAgentLoopService.submit`/`promptHandle` and `kimi upgrade --yes` lands; dispatch, writers, meta types, and retry payload unchanged |

## A2 containment result

The g05.017 A2 ruling stays binding. `packages/acp-server/src/acp-terminal/acpTerminalRunner.ts`
is git blob `9016d48b643f35b263449d98dee25597a9a24d30` at all six points, and
the bundled `AcpProcessService` class in the npm `dist/main.mjs` is
`7c58e045273d9dbcea96e38d0792d6122193810cd3b9a7c93bb02d40d21284e4` at all six
points — the exact digest the frozen `kimi-code-0.39.0-acp-authority` corpus
records for `0.39.0` and `0.39.1`. The `terminal: false` branch still delegates
to a local host-process spawn. No containment or removal is present, so
`kimi-code.acp` stays capped at `0.38.0`; `0.40.0..=0.43.0` is the
posture-rejected published gap and the exact `0.39.0`/`0.39.1` exclusions do
not grow.

## Not in scope

The auto-permission-mode drop of the dangerous-command deny guard at `0.41.0`,
the stdin/stderr trust-gated MCP warning at `0.42.0`, the `kimi web` local
server, Remote Control, Tower, dynamic MCP tool loading, session UI, the model
service, and Python `kimi-cli` stay unmapped or with their own family. The
g05.009 provider-operation observation gate and card 034 are untouched.

No fixture in this directory contains a credential, bearer token, host path,
account identity, device id, provider payload, model observation, or session
id.
