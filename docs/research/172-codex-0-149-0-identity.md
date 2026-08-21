# 172 Codex 0.149.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-21

## Question

Is official npm `@openai/codex` `0.149.0` (published 2026-08-20) a
compatible-extension on the current qualified `0.148.0` ceiling, or does
it require a new milestone or a stop?

## Remaining Rank

At observation time, one AllowUnverified family remains official-newer:

| Surface | Local | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `codex.cli` (exec + app-server) | not installed | npm `@openai/codex` `0.149.0` | latest qualified `0.148.0` | official-newer |

Host install was not present. `0.149.0` was already admitted as
unverified-newer under `AllowUnverified`.

## Method

Compared:
- Official npm `@openai/codex@0.149.0` tarball (published 2026-08-20T21:09:05.284Z)
- GitHub release `rust-v0.149.0` (published 2026-08-20T21:04:55Z; annotated tag `a4e15bf3` peels to `758ef40f`)
- darwin-arm64 and linux-x64 platform binaries extracted from official npm platform tarballs
- Exec help output, selected mapped flags, app-server schema, and method list

No provider prompt sent. No live session. Host install not changed.

## Identity

### Official

- npm package: `@openai/codex@0.149.0`
- npm integrity: `sha512-i4dryj2Y1j+00Mb5n+0n71EYnTK9/KDc2cdFo/dXD0d1oTog2bhUssKDEIOnKmnEf51P0Z/HJTWvTKw/UHyOvQ==`
- npm shasum: `2e38d3859f52f288a86596d0c22366a10154437b`
- tarball SHA256: `e0e5953eef17b560c09c813ffb4172b1755775b8c5659304197494e39c239df7`
- darwin-arm64 tarball SHA256: `d1ebcae20ffd79f64db3ebc3141a90d269a67540927f4405dd4d1a752197a642`
- darwin-arm64 CLI SHA256: `f4a74117b8142cda581c95ff753abf4508b5636d89682c1ed77e4a9249af8963`
- darwin-arm64 CLI size: 220538240 bytes
- linux-x64 tarball SHA256: `e06f3d106fe8bb058a6bfd30075d89ea17deaee7c8425e0c5d23072df0fdd0e7`
- linux-x64 CLI SHA256: `bbc3341e44c9ead340ed9570c17be936e37870f570751a941699ffd04d672827`
- linux-x64 CLI size: 258322048 bytes
- Git tag: `rust-v0.149.0`
- Tag object: `a4e15bf371341b067c8278d3b70b1a8c7b3d793e`
- Peeled commit: `758ef40f50c1a458425c7cfbf1eb12cbc07af0b0`
- CLI version string: `codex-cli 0.149.0`

### Host

Not installed. Observed official binary only.

## Selected Protocol

### Exec

Exec help output SHA256: `23e8b383723998a8ae8427c449ebe77a88ac82b04b4f8a18aad62925b9d3d0ee`

**Identical to 0.148.0.** Selected flags remain:
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

Schema bundle hashes differ from `0.148.0`:
- v2 bundle: `9b3de71a5a2ffc980b792a18aa8f8dec3f85f48829560222a0264fe494b679a9` (was `e5a20eb7...`)
- experimental bundle: `02a4c63a638fdae4a5f6c3ad32a41a377b642c66f3abc84f6fc47c7f3d6074df` (was `819fe7b4...`)

But `ModelListParams.json` is unchanged:
- SHA256: `de29a536c00a5b8f46f34dba417dabd93365305571a8ed200e33bea85db68b5a`

All selected mapped methods remain:
- `initialize`, `model/list`, `thread/list`, `thread/read`, `thread/start`,
  `thread/resume`, `thread/archive`, `thread/delete`, `turn/start`,
  `turn/interrupt`, `item/started`, `item/completed`, `item/plan/delta`,
  `subAgentActivity`, `collabAgentToolCall`

Unmapped additions remain unused:
- `exec fork`, `top-level fork`, `thread/fork`, `amazon-bedrock-builtin-provider`,
  `tui /export`, `async-hooks`

## Decision

**Compatible-extension.**

Keep claim IDs: `codex.exec.cli-window-2`, `codex.app-server.cli-window-2`.

Raise `CODEX_LATEST_QUALIFIED_VERSION` from `0.148.0` to `0.149.0`.

Keep baseline `0.80.0`. Qualify intermediate `0.149.0`. Keep existing
behavior revisions:
- exec: `codex.exec.jsonl-v1` (maintained)
- app-server: `codex.app-server.v2.workspace-roots` (maintained)
- lifecycle: `codex.app-server.lifecycle.v1.strict-descendant-hard-delete`

Keep gaps: `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`.

After qualification, `0.149.1` (unpublished) remains `UnverifiedNewer`.

No new milestone. No decoder update. No provider work.

Claim card: [g04 batch 069](../roadmaps/g04/batch-cards/069-codex-0-149-0-claim.md)

## Sources

- npm registry: `https://registry.npmjs.org/@openai/codex`
- GitHub releases: `https://github.com/openai/codex/releases/tag/rust-v0.149.0`
- Frozen corpus: `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.149.0/`
