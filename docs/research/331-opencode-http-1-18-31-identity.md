# 331 OpenCode HTTP 1.18.31 Identity

Status: promoted
Owner: Tom
Date: 2026-09-21
Task: standing-lane currentness; OpenCode HTTP only

## Question

Is official npm/GitHub OpenCode `1.18.31` a compatible extension of the
qualified `opencode.server` ceiling `1.18.30`, a private milestone, a new
facade, or a stop?

## Remaining Rank

This run covers only OpenCode HTTP. At observation time the family was
AllowUnverified official-newer because g05.051 left `1.18.31` as
`UnverifiedNewer`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `opencode.http` / `opencode.server` | not on `PATH` | npm and GitHub `1.18.31` | qualified through `1.18.30`; AllowUnverified | official-newer |

Gemini remains deferred. Claude Code stream-JSON and Claude Agent ACP stay
separate. OpenCode ACP and web search are separate surfaces and were not
reopened.

## Method

Re-probed npm `opencode-ai@latest` and the GitHub latest release, then
downloaded the npm tarballs and GitHub tag source archives for `1.18.30` and
`1.18.31` into `/tmp/opencode-identity-11831`. Each archive was SHA-256 hashed
before extraction. Downloaded executables were never run. Host `opencode` is
absent from `PATH` and was not installed.

The published tags diverge, so the hop was proved from deterministic complete
source-tree inventories rather than from a commit-range summary:

- hashed all four npm package files at both hops;
- built SHA-256 inventories of `packages/opencode/src` (408 files at both
  hops) and of the complete repository tree (6561 → 6566 files, 60 identical
  symlink targets) at both hops, excluding host paths;
- compared exact added, removed, and changed paths at both scopes;
- recorded per-version hashes for every changed shipped file feeding mapped
  behaviour;
- hashed `packages/sdk/openapi.json` and confirmed the docs OpenAPI symlink
  target; and
- inspected every changed implementation file feeding mapped behaviour and
  bounded the remaining repository changes.

The npm package contains four files at each hop. `LICENSE`, the platform
bootstrap `bin/opencode.exe`, and `postinstall.mjs` are byte-identical; only
`package.json` changes (version and optional dependency versions). npm
metadata provides no `gitHead`, so the npm artifact remains release authority;
GitHub tags are independently frozen correlation and implementation evidence.

The reproduced `1.18.30` tarball SHA-256
`3a97a99230d07fcbe6fd1ea2a001556297225d57f55d7880d3c9e6e29960ae90` and source
archive SHA-256
`d54574de6a2b02d58fe4d403035103a08bdca0f4eafac63d3681cda774e85cd9` match
Research 304.

No prompt, login, live server, catalogue, session, installation, provider
contact, or host change occurred.

## Identity

npm and GitHub latest agree on `1.18.31`. npm published it at
2026-09-14T17:47:43.078Z; GitHub published `v1.18.31` at
2026-09-14T17:47:30Z.

| Version | npm tarball SHA-256 | GitHub tag commit | Source archive SHA-256 |
| --- | --- | --- | --- |
| `1.18.30` | `3a97a99230d07fcbe6fd1ea2a001556297225d57f55d7880d3c9e6e29960ae90` | `3104c1428ec91f809e5ab86631300de41eb6952e` | `d54574de6a2b02d58fe4d403035103a08bdca0f4eafac63d3681cda774e85cd9` |
| `1.18.31` | `b6baa53003cd2e096981474ba9461a33aba2e6948b2b4b08c4aa1a88e6dc1b59` | `014614d35b397775e5d397a490fc72368c894ec2` | `76f69fe27ec2b44e23fa1749029e7c012eb7e975a0f0c7819e9458198dfd3896` |

The GitHub compare between the two tags reports `diverged` with merge base
`5cd8e68fdd72b27818d26d168b9c7a06b359567e`, `ahead_by` 25, and `behind_by` 1.
Tag `v1.18.31` is `release: v1.18.31` over release target commit
`a97622c801f4ca571530ddc51076af659a9c32cd`; tag `v1.18.30` is
`release: v1.18.30` over `5cd8e68fdd72b27818d26d168b9c7a06b359567e`. Linear
ancestry was not assumed.

All npm integrity values, SHA-1 sums, publication times, release times,
target commits, and archive identities are frozen in `identity.json`.
Published stables after the previous ceiling are exactly `1.18.31`, with no
gap. First unpublished later stable at observation: `1.18.32`.

Host `opencode` is not on `PATH`. Missing install is not a gap and is not
install authority.

## Selected Protocol

Selected route declaration and handler files for health, provider catalogue,
session create/list/status/get/messages/prompt/abort/delete, and global events
are byte-identical from `1.18.30` to `1.18.31`. Session identity,
import/history/reconciliation, callback, usage, and detachment closures retain
their existing public wire shapes. OpenAPI SHA-256 is unchanged
(`00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3`) at both
the SDK copy and the docs symlink.

`packages/opencode/src` adds and removes nothing. Six files change:

- `server/routes/instance/httpapi/middleware/error.ts` adds
  `ConfigErrorV1.RemoteAuthError` to the existing defect-to-400 list. That
  error is thrown only when remote config fetch returns an HTML login page.
  `config.ts` and the `RemoteAuthError` type are byte-identical. Selected
  handlers and OpenAPI stay unchanged. Previously this defect fell through to
  the generic 500 Unknown envelope.
- `acp/config-option.ts`, `acp/event.ts`, and `acp/service.ts` restore ACP
  session model, effort, and mode and retarget reasoning `messageId`. OpenCode
  ACP is a separate family and is not selected on `opencode.http`.
- `cli/cmd/tui.ts` changes TUI shutdown from `process.exit(0)` to
  `process.exit()` so remote-config auth failures can surface a non-zero
  status. TUI process lifecycle is not the attached HTTP/SSE route.
- `plugin/github-copilot/models.ts` always requests summarized adaptive
  thinking for Copilot models. This is outgoing provider request shaping.

`packages/core/src` is byte-identical. The remaining repository changes are
bounded: workspace version bumps, `@ai-sdk/gateway` 3.0.104 → 3.0.191 and
`@ai-sdk/provider` 3.0.8 → 3.0.16, hosted console/web/docs/UI, translations,
lockfile and nix hashes, TUI error display, and the VS Code extension. None
ships a selected OpenCode HTTP/SSE route, catalogue schema, lifecycle,
callback, failure, usage, or capability shape. The GitHub release notes name
the ACP restore, TUI remote-auth exit, and Copilot summarized thinking;
changelog text is subordinate to the tree inventory.

No new public operation or adapter control is required.

## Decision

**Compatible-extension.**

- keep axis `opencode.server`, baseline `1.14.48`, AllowUnverified, existing
  historical segments and gaps, claim IDs, and `surface-19` behavior revision;
- qualify published point `1.18.31`;
- raise all existing OpenCode HTTP closures on this axis through `1.18.31`;
- keep decoder specimen `opencode-1.14.48`; and
- use synthetic `1.18.32` as the later `UnverifiedNewer` point after claim.

This identity record changes no production claim. The claim commit applies the
admitted segment.

## Sources

- npm registry: `https://registry.npmjs.org/opencode-ai`
- npm tarballs: registry `dist.tarball` URLs for `1.18.30` and `1.18.31`
- GitHub releases: `https://github.com/anomalyco/opencode/releases`
- GitHub tag archives and refs: `v1.18.30` and `v1.18.31`
- Frozen corpus: `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.31/`
