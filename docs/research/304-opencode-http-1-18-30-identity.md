# 304 OpenCode HTTP 1.18.30 Identity

Status: promoted
Owner: Tom
Date: 2026-09-11
Task: g05.051

## Question

Is official npm/GitHub OpenCode `1.18.30` a compatible extension of the
qualified `opencode.server` ceiling `1.18.29`, a private milestone, a new
facade, or a stop?

## Remaining Rank

This run covers only OpenCode HTTP. At observation time the family was
AllowUnverified official-newer because g05.037 left `1.18.30` as
`UnverifiedNewer`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `opencode.http` / `opencode.server` | `1.18.18` on `PATH` | npm and GitHub `1.18.30` | qualified through `1.18.29`; AllowUnverified | official-newer |

Gemini remains deferred. Kimi local server and Antigravity remain stopped.
OpenCode ACP and web search are separate surfaces and were not reopened.

## Method

Re-probed npm `opencode-ai@latest` and the GitHub latest release, then
downloaded the npm tarballs and GitHub tag source archives for `1.18.29` and
`1.18.30` into `/tmp/opencode-identity-11830`. Each archive was SHA-256 hashed
before extraction. Downloaded executables were never run. Host `opencode` was
observed once with `--version`; it was not installed, updated, or invoked
further.

The published tags diverge, so the hop was proved from deterministic complete
source-tree inventories rather than from a commit-range summary:

- hashed all four npm package files at both hops;
- built SHA-256 inventories of `packages/opencode/src` (407 → 408 files) and of
  the complete repository tree (6557 → 6561 files, 60 identical symlink
  targets) at both hops, excluding host paths;
- compared exact added, removed, and changed paths at both scopes;
- recorded per-version hashes for every changed or added shipped file feeding
  mapped behaviour;
- hashed `packages/sdk/openapi.json` and the symlinked
  `packages/docs/openapi.json`; and
- inspected every changed implementation file feeding mapped behaviour and
  bounded the remaining repository changes.

The npm package contains four files at each hop. `LICENSE`, the platform
bootstrap `bin/opencode.exe`, and `postinstall.mjs` are byte-identical; only
`package.json` changes (version and optional dependency versions). npm
metadata provides no `gitHead`, so the npm artifact remains release authority;
GitHub tags are independently frozen correlation and implementation evidence.

No prompt, login, live server, catalogue, session, installation, provider
contact, or host change occurred.

## Identity

npm and GitHub latest agree on `1.18.30`. npm published it at
2026-09-09T03:33:55.588Z; GitHub published `v1.18.30` at
2026-09-09T03:34:27Z.

| Version | npm tarball SHA-256 | GitHub tag commit | Source archive SHA-256 |
| --- | --- | --- | --- |
| `1.18.29` | `bec74d1c33582ac16489e524d947f4f1f02ee0eadc6053577d66b32f7034a5be` | `16747470f976aca3d362ad730bcd3fe82ecc2c9a` | `8fd2a4e179a6a001e68f1f0986e6687be524a9a28239b8997c6b7e9a72033231` |
| `1.18.30` | `3a97a99230d07fcbe6fd1ea2a001556297225d57f55d7880d3c9e6e29960ae90` | `3104c1428ec91f809e5ab86631300de41eb6952e` | `d54574de6a2b02d58fe4d403035103a08bdca0f4eafac63d3681cda774e85cd9` |

The GitHub compare between the two tags reports `diverged` with merge base
`02a167e048d3bd7299225068d79e4fce5c830d67`, `ahead_by` 30, and `behind_by` 1.
Tag `v1.18.30` is `release: v1.18.30` over release target commit
`5cd8e68fdd72b27818d26d168b9c7a06b359567e`; tag `v1.18.29` is
`release: v1.18.29` over `02a167e048d3bd7299225068d79e4fce5c830d67`. Linear
ancestry was not assumed.

All npm integrity values, SHA-1 sums, publication times, release times,
target commits, and archive identities are frozen in `identity.json`.
Published stables after the previous ceiling are exactly `1.18.30`, with no
gap. First unpublished later stable at observation: `1.18.31`.

Host `opencode` is `1.18.18`, SHA-256
`4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422`, 143182562
bytes. That is observation input only and is below the qualified ceiling, so it
is not a qualification or install authority.

## Selected Protocol

Selected route declaration and handler files for health, provider catalogue,
session create/list/status/get/messages/prompt/abort/delete, and global events
are byte-identical from `1.18.29` to `1.18.30`. Session identity,
import/history/reconciliation, callback, usage, and detachment closures retain
their existing public wire shapes. OpenAPI SHA-256 is unchanged
(`00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3`) at both
the SDK copy and the symlinked docs copy.

Only three `packages/opencode/src` files change, and one file is added:

- `provider/provider.ts` adds an `arn:` pass-through and narrows the `us`
  region prefix list from `deepseek` to `deepseek.r1` inside the Bedrock
  provider model-resolution hook. This shapes outgoing provider requests. The
  `Info`/`Model` schemas and `toPublicInfo` in the selected `provider.list`
  route are unchanged.
- `provider/transform.ts` adds a `gitlab-ai-provider` branch that maps
  reasoning effort to `reasoningEffort` for GPT families and adaptive thinking
  for Claude families. This is provider-facing request option shaping.
- `session/system.ts` selects a new `gpt-6` system prompt ahead of the Codex
  and generic GPT branches, and `session/prompt/gpt-astra.txt` is that prompt.
  This is internal agent prompt content.
- `packages/core/src/plugin/provider/amazon-bedrock.ts` repeats the same
  Bedrock `arn:`/`deepseek.r1` change in the bundled core package.

The remaining repository changes are bounded: `packages/opencode/package.json`
and `packages/core/package.json` carry version bumps plus provider SDK bumps
(`@ai-sdk/openai` 3.0.84 → 3.0.88, `@ai-sdk/azure` 3.0.88 → 3.0.93,
`gitlab-ai-provider` 6.13.0 → 6.15.0); the repository root registers a new
`patches/@ai-sdk%2Fopenai@3.0.88.patch` that preserves explicit OpenAI service
tiers; the rest are workspace version bumps, the hosted console/web/docs
applications, translations, lockfile and nix hashes, and the VS Code
extension. None ships an OpenCode HTTP/SSE route, catalogue schema, lifecycle,
callback, failure, usage, or capability shape. The GitHub release notes name
exactly the Astra prompt, Bedrock model IDs, Azure/OpenAI SDK compatibility,
and GitLab reasoning variants; changelog text is subordinate to the tree
inventory.

No new public operation or adapter control is required.

## Decision

**Compatible-extension.**

- keep axis `opencode.server`, baseline `1.14.48`, AllowUnverified, existing
  historical segments and gaps, claim IDs, and `surface-19` behavior revision;
- qualify published point `1.18.30`;
- raise all existing OpenCode HTTP closures on this axis through `1.18.30`;
- keep decoder specimen `opencode-1.14.48`; and
- use synthetic `1.18.31` as the later `UnverifiedNewer` point after claim.

This identity record changes no production claim. The g05.051 claim commit
applies the admitted segment.

## Sources

- npm registry: `https://registry.npmjs.org/opencode-ai`
- npm tarballs: registry `dist.tarball` URLs for `1.18.29` and `1.18.30`
- GitHub releases: `https://github.com/anomalyco/opencode/releases`
- GitHub tag archives and refs: `v1.18.29` and `v1.18.30`
- Frozen corpus: `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.30/`
