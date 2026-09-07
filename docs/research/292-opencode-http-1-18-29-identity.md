# 292 OpenCode HTTP 1.18.29 Identity

Status: promoted
Owner: Tom
Date: 2026-09-07
Card: g05 batch 134

## Question

Is official npm/GitHub OpenCode `1.18.29` a compatible extension of the
qualified `opencode.server` ceiling `1.18.28`, a private milestone, a new
facade, or a stop?

## Remaining Rank

This g05.037 run covers only OpenCode HTTP. At observation time the
family was AllowUnverified official-newer after g05.028 / Research 285 left
`1.18.29` as `UnverifiedNewer`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `opencode.http` / `opencode.server` | not on `PATH` | npm and GitHub `1.18.29` | qualified through `1.18.28`; AllowUnverified | official-newer |

Gemini remains deferred. Kimi local server and Antigravity remain stopped.
OpenCode ACP and web search are separate surfaces and were not reopened.

## Method

Re-probed npm `opencode-ai@latest` and GitHub latest release, then downloaded
the npm tarball and GitHub tag source archive for `1.18.28` and `1.18.29`
into `/tmp/opencode-identity`. Each archive was SHA-256 hashed before
extraction. Downloaded executables were never run. Host `opencode` was not
on `PATH` and was not installed.

For the hop:

- hashed all four npm package files;
- built a deterministic SHA-256 inventory of `packages/opencode/src`;
- compared exact added, removed, and changed implementation-source paths;
- hashed `packages/sdk/openapi.json`;
- inspected the one changed source file; and
- bounded the Codex OAuth catalogue-filter change as unmapped.

The npm package contains four files at each hop. `LICENSE`, the platform
bootstrap `bin/opencode.exe`, and `postinstall.mjs` are byte-identical;
only `package.json` changes. The correlated GitHub tag source contains 407
`packages/opencode/src` files at both points. npm metadata provides no
`gitHead`, so the npm artifact remains release authority; GitHub tags are
independently frozen correlation and implementation evidence.

Tag `v1.18.29` is commit `16747470f976aca3d362ad730bcd3fe82ecc2c9a`
(`release: v1.18.29`). That commit's parent is
`02a167e048d3bd7299225068d79e4fce5c830d67`, the Codex GPT-version filter
fix named by the GitHub release `target_commitish`. The extracted tag
archive includes both. Channels agree on version `1.18.29`.

No prompt, login, live server, catalogue, session, installation, provider
contact, or host change occurred.

## Identity

npm and GitHub latest agree on `1.18.29`. npm published it at
2026-09-04T23:46:25.653Z; GitHub published `v1.18.29` at
2026-09-04T23:47:16Z.

| Version | npm tarball SHA-256 | GitHub tag commit | Source archive SHA-256 |
| --- | --- | --- | --- |
| `1.18.28` | `ae46e3653cb85edb4eab36127f289ba71833d70c0efb56992f99eca2940117c4` | `22006d97652839999596a34a48ff6be7dbb40c6e` | `8eea501a6a00cbebe524af7c3248c0bfc56290f444671903e32aa6b799ee6616` |
| `1.18.29` | `bec74d1c33582ac16489e524d947f4f1f02ee0eadc6053577d66b32f7034a5be` | `16747470f976aca3d362ad730bcd3fe82ecc2c9a` | `8fd2a4e179a6a001e68f1f0986e6687be524a9a28239b8997c6b7e9a72033231` |

All npm integrity values, SHA-1 sums, publication times, release times, and
archive identities are frozen in `identity.json`. Published stables after the
previous ceiling are exactly `1.18.29`, with no gap. First unpublished later
stable at observation: `1.18.30`.

## Selected Protocol

Selected route declaration and handler files for health, provider catalogue,
session create/list/status/get/messages/prompt/abort/delete, and global events
are byte-identical from `1.18.28` to `1.18.29`. Session identity,
import/history/reconciliation, callback, usage, and detachment closures retain
their existing public wire shapes. OpenAPI SHA-256 is unchanged
(`00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3`).

The only `packages/opencode/src` change is
`plugin/openai/codex.ts`: Codex OAuth model filtering now parses integer GPT
majors such as `gpt-6` instead of requiring a dotted minor. Changelog notes
match that discovery. The change is unmapped sibling-auth catalogue filtering
and does not alter selected HTTP/SSE operations, lifecycle, failure,
permissions, usage, capability advertisement, or session operations.

No new public operation or adapter control is required.

## Decision

**Compatible-extension.**

- keep axis `opencode.server`, baseline `1.14.48`, AllowUnverified, existing
  historical segments and gaps, claim IDs, and `surface-19` behavior revision;
- qualify published point `1.18.29`;
- raise all existing OpenCode HTTP closures on this axis through `1.18.29`;
- keep decoder specimen `opencode-1.14.48`; and
- use synthetic `1.18.30` as the later `UnverifiedNewer` point after claim.

This identity record changes no production claim. Serial card 136 applies
the admitted segment.

## Sources

- npm registry: `https://registry.npmjs.org/opencode-ai`
- npm tarballs: registry `dist.tarball` URLs for `1.18.28` and `1.18.29`
- GitHub releases: `https://github.com/anomalyco/opencode/releases`
- GitHub tag archives and refs: `v1.18.28` and `v1.18.29`
- Frozen corpus: `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.29/`
