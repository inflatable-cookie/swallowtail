# 201 Codex 0.149.1 Identity

Status: promoted
Owner: Tom
Date: 2026-08-24

## Question

Is official npm `@openai/codex` `0.149.1` (published 2026-08-24) a
compatible-extension on the current qualified `0.149.0` ceiling, or does
it require a new milestone or a stop?

## Remaining Rank

At observation time, one AllowUnverified family remains official-newer
on this axis:

| Surface | Local | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `codex.cli` (exec + app-server) | not installed | npm `@openai/codex` `0.149.1` | latest qualified `0.149.0` | official-newer |

Host install was not present. `0.149.1` was already admitted as
unverified-newer under `AllowUnverified`.

## Method

Compared:
- Official npm `@openai/codex@0.149.1` tarball (published 2026-08-24T00:32:45.066Z)
- GitHub release `rust-v0.149.1` (published 2026-08-24T00:28:28Z; annotated tag `980a6d12` peels to `ff29a443`)
- darwin-arm64 and linux-x64 platform binaries extracted from official npm platform tarballs
- Exec help output, selected mapped flags, app-server schema, and method list
- Public git compare `rust-v0.149.0...rust-v0.149.1` (5 commits ahead, 1 behind)

No provider prompt sent. No live session. Host install not changed.

## Identity

### Official

- npm package: `@openai/codex@0.149.1`
- npm integrity: `sha512-6q5pbcpFbJbqOpkubSDBwXmktQ55aD8eUzGzBF1zASob2DjwhBKDSNGtdZKalfrNJUdTDTPDMmzCXEXs5tMBYA==`
- npm shasum: `37bc183ebd129e01e404e932bcec4ea861c70933`
- tarball SHA256: `1616304fd7883b46d8887cf336496e2ae0cdf9a637b7bdf8824baa98c22c5b7b`
- darwin-arm64 tarball SHA256: `151f8b96af0529c1267e7438d2cbc6d26213922fa017b96540abaf5f07d792d2`
- darwin-arm64 CLI SHA256: `f0d8762236594359b60cfbe17f4c7e945a3ce8d1c91e74778838c968d250fb6c`
- darwin-arm64 CLI size: 220552944 bytes
- linux-x64 tarball SHA256: `734f865ed62d8be68796e7913651bbc69ad7c63a8c01ee28524ad69b4c9ab401`
- linux-x64 CLI SHA256: `73dc5888888f411c1f0fa7b81d866e721dcc86b527ce8e3b2cf4708661e823ba`
- linux-x64 CLI size: 258227840 bytes
- Git tag: `rust-v0.149.1`
- Tag object: `980a6d12110b110d29ec13bdcbe14011100b3566`
- Peeled commit: `ff29a44391deccde0aba0f8390337d7f3c319ea4`
- CLI version string: `codex-cli 0.149.1`

### Host

Not installed. Observed official binary only.

## Selected Protocol

### Exec

Exec help output SHA256: `e504bac5a6364566fbe408132dec7993639def9258ece34e8352f51f8d43687c`

Differs from `0.149.0` only by unmapped `--thread-source` (commit
`2b66d2ede5`, "Allow exec callers to classify new threads"). Selected
flags remain:
- `exec`
- `--json`
- `--ephemeral`
- `--ignore-user-config`
- `--ignore-rules`
- `--skip-git-repo-check`
- `--sandbox` with `read-only`

### App-server

Selected flags remain:
- `app-server`
- `--listen`
- `stdio://`

Schema bundles are byte-identical to `0.149.0`:
- v2 bundle: `9b3de71a5a2ffc980b792a18aa8f8dec3f85f48829560222a0264fe494b679a9`
- experimental bundle: `02a4c63a638fdae4a5f6c3ad32a41a377b642c66f3abc84f6fc47c7f3d6074df`
- `ModelListParams.json`: `de29a536c00a5b8f46f34dba417dabd93365305571a8ed200e33bea85db68b5a`

All selected mapped methods remain:
- `initialize`, `model/list`, `thread/list`, `thread/read`, `thread/start`,
  `thread/resume`, `thread/archive`, `thread/delete`, `turn/start`,
  `turn/interrupt`, `item/started`, `item/completed`, `item/plan/delta`,
  `subAgentActivity`, `collabAgentToolCall`

Unmapped additions remain unused:
- `--thread-source`, `exec fork`, `top-level fork`, `thread/fork`,
  `amazon-bedrock-builtin-provider`, `tui /export`, `async-hooks`,
  remote-compaction image budget, memory-consolidation identity

No `app-server-protocol` source files changed between the tags.

## Decision

**Compatible-extension.**

Keep claim IDs: `codex.exec.cli-window-2`, `codex.app-server.cli-window-2`.

Raise `CODEX_LATEST_QUALIFIED_VERSION` from `0.149.0` to `0.149.1`.

Keep baseline `0.80.0`. Qualify intermediate `0.149.1`. Keep existing
behavior revisions:
- exec: `codex.exec.jsonl-v1` (maintained)
- app-server: `codex.app-server.v2.workspace-roots` (maintained)
- lifecycle: `codex.app-server.lifecycle.v1.strict-descendant-hard-delete`

Keep gaps: `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`.

After qualification, `0.149.2` (unpublished) remains `UnverifiedNewer`.

No new milestone. No decoder update. No provider work.

Claim card: [g04 batch 152](../roadmaps/g04/batch-cards/152-codex-0-149-1-claim.md)

## Sources

- npm registry: `https://registry.npmjs.org/@openai/codex`
- GitHub releases: `https://github.com/openai/codex/releases/tag/rust-v0.149.1`
- Frozen corpus: `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.149.1/`
