# 262 Codex 0.151.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-31

## Question

Is official npm `@openai/codex` `0.151.0` (published 2026-08-29) a
compatible-extension on the current qualified `0.149.1` ceiling, or does
it require a new milestone or a stop?

## Remaining Rank

Named family only. This run does not rank other families.

| Surface | Local | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `codex.cli` (exec + app-server) | `0.150.1` | npm `@openai/codex` `0.151.0` | latest qualified `0.149.1` | host-and-official-newer |

The current host's signed darwin-arm64 `0.150.1` binary matches the official
platform package. `0.151.0` was already admitted as unverified-newer under
`AllowUnverified`. Gemini stays deferred.

## Method

Compared:
- Official npm `@openai/codex@0.151.0` wrapper and platform tarballs
  (published 2026-08-29T09:59:26.300Z)
- GitHub release `rust-v0.151.0` (published 2026-08-29T09:55:39Z;
  annotated tag `d8673cb6` peels to `78c29080`)
- Published intermediates `@openai/codex@0.150.0` and `@openai/codex@0.150.1`
- linux-x64 binaries extracted from official npm platform tarballs for
  `0.150.0`, `0.150.1`, and `0.151.0`; darwin-arm64 binary digest for
  `0.151.0` only
- Exec help output, selected mapped flags, generated app-server schema,
  and selected method/param shapes against the frozen `0.149.1` corpus
- Public GitHub schema files for selected `thread/*` and `turn/start`
  params at `rust-v0.149.1`, `rust-v0.150.0`, `rust-v0.150.1`, and
  `rust-v0.151.0`
- Current host `codex-cli 0.150.1`, its binary digest and size, Developer ID
  signature, and the matching official darwin-arm64 platform package

No provider prompt sent. No live session. Host install not changed.
Official artifacts stayed in `/tmp`.

## Identity

### Official

- npm package: `@openai/codex@0.151.0`
- npm integrity: `sha512-mhtWmOZRdmWD1jPbLDnQb59BsaVP/V+lXe/OFNR9ZcLZU0UCiBwn98Fcav1ss7sDIlHkuqj6nWd44IPeXoOhJA==`
- npm shasum: `515ca678dd9eec6afd4a7dc34f571c6536b3f282`
- tarball SHA256: `c332ca76a7b913682b40669f20beefee9ce5c026dd6fdd80f05e282385bf6f68`
- darwin-arm64 tarball SHA256: `93659b8bd69c4ecd28ae08e2960c668f64d4760480bebe98841904d447026740`
- darwin-arm64 CLI SHA256: `98491713ffb196061003ee148636e743997cc31d76144ba7c53462269896891d`
- darwin-arm64 CLI size: 231563824 bytes
- linux-x64 tarball SHA256: `b3bcf2c11693d7c8155de637dd6562ba19d916ba13471a7c8737de55e5328fc6`
- linux-x64 CLI SHA256: `9739cbc928b9c573be83256acd46668f5dd4f119d2d09e05246895ca2aaf0c9a`
- linux-x64 CLI size: 270815680 bytes
- Git tag: `rust-v0.151.0`
- Tag object: `d8673cb68e349c208659b986697773d3145dbb14`
- Peeled commit: `78c290807ce710180111df227df3b7a4fe845452`
- CLI version string: `codex-cli 0.151.0`

Published stables after previous ceiling `0.149.1`: `0.150.0`
(2026-08-26T19:43:31.702Z, tag `3b3b4f8f`), `0.150.1`
(2026-08-27T02:01:46.851Z, tag `90854393`), and `0.151.0`. Unpublished
gaps: `0.149.2` and `0.150.2`. First unpublished later stable is
`0.151.1`. Dist-tag `alpha` is `0.152.0-alpha.6` and is ignored.

### Host

- version: `codex-cli 0.150.1`
- target: `aarch64-apple-darwin`
- CLI SHA256: `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- CLI size: 228986048 bytes
- signature: `Developer ID Application: OpenAI OpCo, LLC (2DC432GLL2)`
- official `0.150.1-darwin-arm64` package match: exact binary digest and size

The host install was observed only and not changed.

## Selected Protocol

### Exec

Exec help output SHA256:
`e504bac5a6364566fbe408132dec7993639def9258ece34e8352f51f8d43687c`

Byte-identical at `0.149.1`, `0.150.0`, `0.150.1`, and `0.151.0`.
Selected flags remain:
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

`ModelListParams` is byte-identical to `0.149.1`:
`de29a536c00a5b8f46f34dba417dabd93365305571a8ed200e33bea85db68b5a`.

Generated bundles differ by unmapped extras:
- v2 bundle: `2442b15801bc019ad55987ad03e0f0ae60c51417825b9b6d708db640e6c2651c`
- experimental bundle: `424b204943b18e5ffa52667a2aa397c9950730ec1e49ad767e2a016743990541`

All selected mapped methods remain:
- `initialize`, `model/list`, `thread/list`, `thread/read`, `thread/start`,
  `thread/resume`, `thread/archive`, `thread/delete`, `turn/start`,
  `turn/interrupt`, `item/started`, `item/completed`, `item/plan/delta`,
  `subAgentActivity`, `collabAgentToolCall`

Selected required fields are unchanged. Additive unmapped fields:
- `thread/read` description now prefers `thread/turns/list` and
  `thread/items/list`; `includeTurns` remains
- `thread/resume` optional `excludeTurns`
- `turn/start` optional `serviceTierForTurn`, `toolOutput`, `turnTrigger`

Unmapped additions stay unused:
- `--thread-source`, `exec fork`, `top-level fork`, `thread/fork`,
  `thread/turns/list`, `thread/items/list`, `--code-mode-host`,
  `codex-code-mode-host`, plugin catalogs, optional MCP grace, Guardian,
  Bedrock, TUI extras

Feature-specific exact pins (model verbosity, fast mode, personality,
plan-mode effort) stay on the `0.147.0..=0.149.1` probed points.

## Decision

**Compatible-extension.**

Keep claim IDs: `codex.exec.cli-window-2`, `codex.app-server.cli-window-2`.

Raise `CODEX_LATEST_QUALIFIED_VERSION` from `0.149.1` to `0.151.0`.

Qualify published intermediates `0.150.0` and `0.150.1`.

Keep baseline `0.80.0`. Keep existing behavior revisions:
- exec: `codex.exec.jsonl-v1` (maintained)
- app-server: `codex.app-server.v2.workspace-roots` (maintained)
- lifecycle: `codex.app-server.lifecycle.v1.strict-descendant-hard-delete`

Keep gaps: `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`, plus unpublished
`0.149.2` and `0.150.2`.

After qualification, `0.151.1` (unpublished) remains `UnverifiedNewer`.

No new milestone. No decoder update. No provider work. Do not flatten
`codex-code-mode-host` onto `codex.cli`. Do not map new public
operations.

## Sources

- npm registry: `https://registry.npmjs.org/@openai/codex`
- GitHub releases: `https://github.com/openai/codex/releases/tag/rust-v0.151.0`
- Frozen corpus: `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.151.0/`
