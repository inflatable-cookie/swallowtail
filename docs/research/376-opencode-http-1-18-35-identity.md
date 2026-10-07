# 376 OpenCode HTTP 1.18.35 Identity

Status: identity complete; claim extension proposed
Owner: Tom
Date: 2026-10-07
Task: swallowtail#105; OpenCode HTTP only

## Question

Does official OpenCode `1.18.35` extend the qualified `opencode.server` claim
through the current stable, or does a published hop require a separate ruling or
adaptation?

## Rank

| Surface | Host | Official | Swallowtail boundary | Observation |
| --- | --- | --- | --- | --- |
| `opencode.http` / `opencode.server` | installed `1.18.32`, asset matched | npm and GitHub stable `1.18.35` | qualified through `1.18.31`; AllowUnverified | four published newer hops |

This record covers only `opencode.http`. ACP, web search, consumer HTTP MCP
honouring, and other route families stay separate.

## Method

Re-probed npm `opencode-ai@latest` and GitHub releases. Both reported
`1.18.35`. Downloaded the npm packages and standard GitHub tag archives for
`1.18.31` through `1.18.35` into a fresh temporary directory, hashed them, and
extracted the source archives. The npm records have no `gitHead`; npm identifies
the published package, and GitHub tags identify source used for implementation
comparison. Every hop was compared from complete tree inventories, not commit
range summaries. The exact path deltas, inventory digests, npm file hashes,
OpenAPI hash, and changed implementation file hashes are frozen in
`crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.35/`.

The tree inventory sorts relative paths and hashes each file or symlink target
using the recorded canonical format. Exact added, removed, and changed paths are
retained for all four repository hops. npm tarballs contain four files at every
point. `LICENSE`, `bin/opencode.exe`, and `postinstall.mjs` are byte-identical;
only `package.json` changes. Standard GitHub archive hashes are recorded
separately from GitHub API tarball hashes because the archive bytes differ; the
extracted inventories match.

The installed `opencode` is on `PATH` at `1.18.32`. A version-only observation
under an isolated `OPENCODE_HOME` returned `1.18.32`. Its binary size and
SHA-256 match the official `v1.18.32` Darwin arm64 release asset. No artifact
was run beyond that version query; no install or host change occurred.

No provider prompt, login, live HTTP server, live catalogue, session, credential,
or provider contact was used.

## Official identities

| Version | npm published | npm tarball SHA-256 | GitHub tag commit | GitHub release published | Source archive SHA-256 |
| --- | --- | --- | --- | --- | --- |
| `1.18.31` | `2026-09-14T17:47:43.078Z` | `b6baa53003cd2e096981474ba9461a33aba2e6948b2b4b08c4aa1a88e6dc1b59` | `014614d35b397775e5d397a490fc72368c894ec2` | `2026-09-14T17:47:30Z` | `76f69fe27ec2b44e23fa1749029e7c012eb7e975a0f0c7819e9458198dfd3896` |
| `1.18.32` | `2026-09-21T22:50:42.463Z` | `454fbb032ade95a21323891138d4b425573f67631e7e888e516da893dc4be8ba` | `545f51d26cc39a907d2867492d498d9607ea5fa4` | `2026-09-21T22:51:20Z` | `65e95c9a6666ca65bbd17de1e7cecddac1504e66eeebbcfaf5ac68f97e6f392b` |
| `1.18.33` | `2026-09-28T04:23:28.113Z` | `6fed32445d04df1d9d56aef0f15ae200618663ef532f7ec4241432024f84097c` | `51ef4be1d3c122f18fefb510dca8d778571f4f18` | `2026-09-28T04:22:46Z` | `34a4b810f4e839f2c4ac62206bbc64d736006f3c96712b903cda83ca60271301` |
| `1.18.34` | `2026-09-30T22:38:58.987Z` | `279923bb5754e810b173a8a7401cf17699fccad3c1159500c10c477887f0b53b` | `aec0b9a6d8898f68f923aaf08b7306d931fd9d76` | `2026-09-30T22:39:45Z` | `c2c60efde22639b64c7bfa740da39b9c8079391a7540e2f67bf91b36e5797f17` |
| `1.18.35` | `2026-10-06T20:21:30.784Z` | `4d3408d0950d70cf870efe3f86e8bd0271d8149bea0767008d42de14f0986a04` | `53d1eabb61e21162157817bf677da0a4ad3332e3` | `2026-10-06T20:18:39Z` | `3092a7b9f55d80c42a9c1bb2e2cc0a9961316673faf92b741577257a1576dd59` |

The consecutive published stables after the existing `1.18.31` ceiling are
`1.18.32`, `1.18.33`, `1.18.34`, and `1.18.35`. There is no unpublished or
withdrawn point in that sequence. `1.18.36` was not published at observation
time and stays outside the proposed claim.

## Hop inventories

| Hop | Added | Removed | Changed | Identical | Repository files | Symlinks |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `1.18.31` | — | — | — | — | 6,566 | 60 |
| `1.18.31` → `1.18.32` | 6 | 0 | 102 | 6,524 | 6,572 | 60 |
| `1.18.32` → `1.18.33` | 6 | 0 | 143 | 6,489 | 6,578 | 60 |
| `1.18.33` → `1.18.34` | 5 | 2 | 128 | 6,508 | 6,581 | 60 |
| `1.18.34` → `1.18.35` | 15 | 28 | 151 | 6,462 | 6,568 | 60 |

The complete added, removed, and changed path sets for each hop are in
`dist-inventory.json`. It also records all changed files under `packages/core`
and `packages/opencode` with before/after SHA-256 values, and the full hashes for
every mapped stable file. The source implementation file counts are 408 at
`1.18.31` and `1.18.32`, then 409 at `1.18.33`, `1.18.34`, and `1.18.35`.

## Selected behavior

The selected routes stay `global.health`, `provider.list`, `session.create`,
`session.prompt_async`, `event.subscribe`, `session.abort`, `session.delete`,
`session.list`, `session.status`, `session.get`, and `session.messages`.
Selected route declaration and handler files are byte-identical across all
five versions. `packages/sdk/openapi.json` remains byte-identical with SHA-256
`00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3`.

The changed files that feed selected provider output or requests were inspected
and classified:

- `1.18.31` → `1.18.32`: `session/message-v2.ts` narrows Bedrock tool-result
  image hoisting to Claude, Nova, and Llama 4 in provider output conversion.
  It does not change persisted message or HTTP response shape.
  `@ai-sdk/togetherai` moves from `2.0.41` to `2.0.68`; provider stream usage
  values may change, but the usage wire shape and Swallowtail decoder do not.
- `1.18.32` → `1.18.33`: `provider/provider.ts` changes Cloudflare AI Gateway
  timeout behavior. `provider/transform.ts` changes Gemini reasoning variant
  values returned by the dynamic catalogue, and `plugin/openai/codex.ts` adds
  provider model IDs. Existing provider and model schemas remain unchanged;
  Swallowtail uses only exact catalogue-reported variants and adds no aliases.
- `1.18.33` → `1.18.34`: `core/session/runner/llm.ts` and
  `session/llm/request.ts` add session and parent-session headers to outbound
  model requests. Those values already travel through existing headers. This
  does not change OpenCode HTTP/SSE payloads, session identity, or authority.
- `1.18.34` → `1.18.35`: `session/message-v2.ts` filters unsupported xAI
  tool-result image formats during provider output conversion. Persisted
  session messages and HTTP response shapes stay unchanged. The xAI SDK moves
  from `3.0.102` to `3.0.139` and `gitlab-ai-provider` from `6.18.0` to
  `6.19.0`; these are provider dependencies.

Other changed implementation paths add or adjust local debug redaction,
CLI/browser launch behavior, MCP client OAuth URL checks, provider login helpers,
filesystem search schema imports, and plugin module resolution. They do not
implement selected attached HTTP/SSE routes. Exact changes to those paths and
the rest of the repository are bounded by the frozen full-tree path inventories.
No selected operation, adapter control, permission authority, lifecycle rule,
failure decoder, usage schema, or public capability changes.

## Decision

**Compatible extension.** Extend the existing `opencode.http.server-window-1`
claim on axis `opencode.server` from `1.18.31` through `1.18.35`, retaining
baseline `1.14.48`, behavior revision `opencode.http-sse.surface-19`,
`AllowUnverified`, every existing supported segment and historical gap, and all
exclusions. Leave synthetic `1.18.36` as `UnverifiedNewer`. No private mapping
milestone is needed. The identity record itself changes no production claim.

## Sources

- npm metadata: <https://registry.npmjs.org/opencode-ai>
- npm tarballs: registry `dist.tarball` for each version in `identity.json`
- GitHub releases: <https://github.com/anomalyco/opencode/releases>
- GitHub tag archives: `https://github.com/anomalyco/opencode/archive/refs/tags/v1.18.N.tar.gz`
- Previous OpenCode HTTP identity: [Research 332](./332-opencode-http-1-18-31-identity.md)
- Frozen fixture: `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.35/`
