# 357 Gemini CLI 0.61.0 Identity

Status: promoted
Owner: Tom
Date: 2026-09-26
Task: swallowtail#071; planning `4970ad02b8ea281a8ce600589756e25db42eefd9`
Authority: Contracts 023, 029 and 063; Research 324, 351 and 356; the Gemini
CLI selection, prepared ACP/headless guide, drivers, decoders and frozen
fixtures; the official npm and GitHub channels.

## Question

Do official npm `@google/gemini-cli` `0.60.0` and `0.61.0` extend the
`gemini-cli.acp` and `gemini-cli.headless` claims qualified through `0.59.0`,
and does the Research 351 ACP `mcpServers` HTTP mapping still hold at
`0.61.0`?

## Answer

Yes on both axes. Compatible extension to maintained `0.51.0..=0.61.0` on the
unchanged behavior revisions. The HTTP MCP mapping is byte-identical.

## Rank

The plan's `version-currentness` lane names Gemini next: the host
auto-updated from `0.53.0` to official `0.61.0` on 2026-09-26, above the
`0.59.0` ceiling, and the next task runs the `gemini-cli.acp` HTTP MCP live
gate on the host. No Gemini deferral is recorded.

## Method

Re-probed npm `latest`, GitHub latest release and host `gemini --version` on
2026-09-26. npm `latest` is `0.61.0`, published `2026-09-24T00:04:53.021Z`.
GitHub latest is `v0.61.0`, published `2026-09-23T23:59:15Z`, not a
prerelease. Published stables after `0.59.0` are exactly `0.60.0` and
`0.61.0`. `0.61.1` is the first unpublished later stable. `0.62.0-preview.0`
is an ignored preview.

Downloaded npm tarballs, GitHub tagged source archives and darwin-arm64
unsigned assets for `0.59.0`, `0.60.0` and `0.61.0` into `/tmp`. Every
tarball reproduces its registry `integrity` and `shasum`. Every asset
reproduces its GitHub-declared digest. Every `0.59.0` value reproduces
Research 324. Nothing downloaded was executed.

One deterministic inventory per tagged source tree: every regular file mapped
to its SHA-256; each hop's changed paths are the symmetric difference. The
method reproduces Research 324's 24-path `0.58.0..0.59.0` ledger. This window
changes 93 paths `0.59.0..0.60.0` and 89 paths `0.60.0..0.61.0`. Release
notes were discovery only.

No prompt, login, credential, catalogue call, ACP session, install or host
update.

## Identity

Host `gemini 0.61.0`: npm bin link to `bundle/gemini.js`, unsigned,
SHA-256 `0b6e283ae88682b0e27e8ef85a608ab74807a1513dc0c053e5aa80d5b80b29ab`,
byte-identical to the official `0.61.0` npm bin entry. Read for `--version`
and digest only.

| Version | npm published | npm tarball SHA-256 | GitHub commit / tree | darwin-arm64 asset / extracted |
| --- | --- | --- | --- | --- |
| `0.59.0` | 2026-09-08T21:19:17.301Z | `59dc2cdb098b3000d36e34a185fc873932df4fd9d00900e817f2b19cd349d98b` | `fb0d535af931b27c51e87e5e6ade72905b1e8390` / `a080c8f15d68e93f65a29c6b177090206038dc97` | `0c8938c68df7e46fd1de63583e4c0a1d7a452e1ecb1d33065f9ac12b8814876a` / `f78acf4241ae6b1c9b04c9a2cb201c6a876e9e79266a9d1078b88cf833edf36c` |
| `0.60.0` | 2026-09-15T20:36:40.030Z | `cecb24eabf2eb23f0f49bf131cddd640e65017336b6298da9297bdf9d213cc0b` | `733edcb597ce690ac2e2fe3b3b3690b60a4c8f27` / `d1a4a41217df8500c4d286019dea98af44419bbc` | `8c7a2554fc9ad2ddcbcbd24f9542b1e1b25f32bfb74a02935b74d4d93a88885d` / `40c222cb54817092efee8afdc8308a1df6b5e8e0c0a8485cf51d000c579c3761` |
| `0.61.0` | 2026-09-24T00:04:53.021Z | `bc4efa5c925c4430105b552820ed3164bbeffa9dc227990fc922f954733bcd7d` | `bb523741c7429a44d03e964bc124c7c92df59d5f` / `c4226f654bb0b35109c4f5ce34535c5addecda47` | `829738c00cab5a73b3ed01ce874c7ba79064fbe5869ad821517cba24f778dce2` / `93a9d76e77a4a7716eb64127b24907309fe2b0af1e2273fb97a8c44b793fc5d2` |

Source archive, npm `package.json` and bin-entry digests are frozen in
`identity.json`.

## Selected Surface

### ACP

The six selected ACP sources (`acpCommandHandler.ts`, `acpRpcDispatcher.ts`,
`acpSessionManager.ts`, `acpSession.ts`, `acpFileSystemService.ts`,
`acpStdioTransport.ts`) are byte-identical `0.59.0..=0.61.0`. The ACP SDK pin
stays `@agentclientprotocol/sdk@0.16.1`, wire version `1`. Read-only Plan and
bounded-write Auto Edit keep their mode ids, filesystem capabilities and
callbacks.

The only ACP-directory change is `acpUtils.ts` (`0.60.0..0.61.0`):
`buildAvailableModels` reads the renamed latest-Flash and new Flash-Lite
access checks. `availableModels`/`currentModelId` keep their shape; values
stay provider-private negotiated observations.

Build-file protection (`0.61.0`) and out-of-workspace shell `dir_path`
(`0.60.0`) force `ASK_USER` in the policy engine. In ACP that surfaces as the
existing `session/request_permission`, built in byte-identical
`acpSession.ts`. Swallowtail reads only `sessionId`, `options` and
`toolCall.toolCallId`, then rejects and cancels, unchanged. The new
confirmation-detail fields (`isBuildFile`, `untrustedFlags`,
`modifiedBuildFiles`) are internal and are not put on the ACP wire.

### ACP HTTP MCP (Research 351 shape)

Unchanged at `0.61.0`:

- `acpRpcDispatcher.ts:98-101` advertises `mcpCapabilities: { http: true,
  sse: true }`.
- `acpSessionManager.ts:296-325` maps `http` → `httpUrl`, `sse` → `url`,
  headers via `Object.fromEntries`; stdio stays accepted.
- `acpSessionManager.ts:244` and `:262` throw `authRequired` with no
  selected auth.
- `MCPServerConfig` is byte-identical.
- `mcp-client.ts` changes once (`0.59.0..0.60.0`): stdio env and extension
  settings drop dangerous execution variables (`NODE_OPTIONS`, `LD_PRELOAD`,
  `DYLD_*` and similar). `createTransportRequestInit`, the `httpUrl`/`url`
  transport choice and SSE `requestInit.headers` (`:1686`) are unchanged.

Live honouring stays unproven. Research 356's isolated `0.59.0` gate is
unaffected.

### Headless

Every selected source is byte-identical `0.59.0..=0.61.0`: `config.ts`
(options), `nonInteractiveCli.ts`, `geminiChat.ts`, `turn.ts`,
`output/types.ts`, `stream-json-formatter.ts`, `exitCodes.ts`, and retention
`sessions.ts`, `sessionOperations.ts`, `gemini.tsx`, `sessionCleanup.ts`.
Every selected option literal keeps its multiplicity.

## Unmapped Surfaces

- **Flash rollout model routing.** On the Gemini API key path
  `hasLatestFlashGAAccess()` is unconditionally true. `0.61.0` retargets
  promotion to `gemini-3.8-flash`, adds `gemini-3.1-flash-lite` →
  `gemini-3.5-flash-lite`, and wraps the API-key generator in
  `ModelMappingContentGenerator`. Explicit `--model gemini-3.5-flash` or
  `gemini-3-flash` is served as `gemini-3.8-flash`. The `init` event still
  echoes `config.getModel()`, the requested id. This is not new at `0.61.0`:
  through `0.59.0` and `0.60.0`, `resolveModel` already promoted any `*flash`
  id to `gemini-3.5-flash` on the same path. It stays provider-internal and
  unmapped here. Served-model fidelity for explicit Flash ids is a separate
  question from this claim.
- Build-file protection and shell `dir_path` checks: provider tool policy
  behind the existing permission stop.
- `storage.ts`: runtime state moves to `~/.cache/.gemini` only under
  `SANDBOX=sandbox-exec`; selected routes run unsandboxed.
- Insecure system settings files skipped (`settings.ts`), sandbox managers
  and Seatbelt profiles, MCP OAuth issuer check and token-storage directory
  creation: ambient or separate surfaces.
- Browser login, individual-account service, Vertex, gateway, Gemini Live and
  the Gemini Models catalogue stay untouched.

## Decision

- ACP: compatible extension. Keep `gemini-cli.acp.v0.51.0`, baseline `0.51.0`,
  `AllowUnverified`; maintained `0.51.0..=0.61.0`.
- Headless: compatible extension. Keep `gemini-cli.headless.stream-json.v1`,
  baseline `0.51.0`, `AllowUnverified`; maintained `0.51.0..=0.61.0`.
- Qualify `0.60.0` and `0.61.0`. `0.61.1` stays the visible `UnverifiedNewer`
  point.
- No new public operation, flag, driver, facade or behavior revision. ACP
  activity and headless decoder corpora stay authoritative.

## Sources

- [npm `@google/gemini-cli@0.61.0`](https://registry.npmjs.org/@google%2Fgemini-cli/0.61.0)
- [GitHub `v0.61.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.61.0)
- [GitHub `v0.60.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.60.0)
- [GitHub `v0.61.0` source tree](https://github.com/google-gemini/gemini-cli/tree/v0.61.0)
- frozen corpus: `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.61.0/`
- prior corpus: `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.59.0/`
