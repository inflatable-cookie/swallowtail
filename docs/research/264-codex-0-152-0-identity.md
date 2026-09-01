# 264 Codex 0.152.0 Identity

Status: promoted
Owner: Tom
Date: 2026-09-01

## Question

Is official npm `@openai/codex` `0.152.0` (published 2026-09-01) a
compatible-extension on the current qualified `0.151.0` ceiling, or does
it require a new milestone or a stop?

## Remaining Rank

Named family only. This run does not rank other families.

| Surface | Local | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `codex.cli` (exec + app-server) | `0.150.1` | npm `@openai/codex` `0.152.0` | latest qualified `0.151.0` | host-qualified; official-newer |

The current host's signed darwin-arm64 `0.150.1` binary matches the official
platform package. `0.152.0` was already admitted as unverified-newer under
`AllowUnverified`. Gemini stays deferred.

## Method

Compared:
- Official npm `@openai/codex@0.152.0` wrapper and platform tarballs
  (published 2026-09-01T02:02:46.635Z)
- GitHub release `rust-v0.152.0` (published 2026-09-01T01:58:32Z;
  annotated tag `7f6bee13` peels to `316795b3`)
- Official extracted darwin-arm64 `0.152.0` binary versus the frozen
  `0.151.0` corpus, including exec help, app-server help, generated
  app-server schema, and selected method/param shapes
- linux-x64 binary digest and size from the official npm platform tarball
- Current host `codex-cli 0.150.1`, its binary digest and size, Developer ID
  signature, and the matching official darwin-arm64 platform package

No provider prompt sent. No live session. Host install not changed.
Official artifacts stayed in `/tmp`.

## Identity

### Official

- npm package: `@openai/codex@0.152.0`
- npm integrity: `sha512-Vx0tg/J5SbxYYGJazTtL/XySK9Dlqc5KW1MZM71NMwVci/4F1ap+FfSKPFTlrICEtOTuq3KNcWSdv9oMGdPuRw==`
- npm shasum: `a9f8948612ef63fd7441b0c551d8805e1213cbb9`
- tarball SHA256: `054963eb89072c77cfa7cdca71dc65c3834e0015c45a74d172a7983a1712c6fd`
- darwin-arm64 tarball SHA256: `cc26642531cb490dec79e8fc1aace175065e4a9e47410628c869965d8d98d20f`
- darwin-arm64 CLI SHA256: `166e0593c333c1c6412cc9cea72b6e1dfc4fc79b4813da02c511a3339c9b9593`
- darwin-arm64 CLI size: 217812496 bytes
- linux-x64 tarball SHA256: `9e4c4a25b88d9c93ce0ee4f64fba23fc55949f66d9ecb79e419c541b64b80997`
- linux-x64 CLI SHA256: `f541420d35d3ad757fe71c0a34a3de0ec80fd513e10e5c52596b39d8be6e445c`
- linux-x64 CLI size: 255513312 bytes
- Git tag: `rust-v0.152.0`
- Tag object: `7f6bee13af649d0da23ac0c2bf5c83f571fcd611`
- Peeled commit: `316795b3cf2a45e90d121d9f46499d4658b2645c`
- CLI version string: `codex-cli 0.152.0`

Published stables after previous ceiling `0.151.0`: `0.152.0` only.
Unpublished gaps already in the window: `0.149.2` and `0.150.2`.
Unpublished `0.151.1` sits between `0.151.0` and `0.152.0` and stays a
gap after qualification. First unpublished later stable is `0.152.1`.
Dist-tag `alpha` is `0.153.0-alpha.2` and is ignored.

Wrapper `0.152.0`, platform packages `0.152.0-darwin-arm64` /
`0.152.0-linux-x64`, extracted CLI `codex-cli 0.152.0`, and GitHub tag
`rust-v0.152.0` reconcile.

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

Byte-identical at `0.149.1`, `0.150.0`, `0.150.1`, `0.151.0`, and
`0.152.0`. Selected flags remain:
- `exec`
- `--json`
- `--ephemeral`
- `--ignore-user-config`
- `--ignore-rules`
- `--skip-git-repo-check`
- `--sandbox` with `read-only`

### App-server

App-server help SHA256:
`95d290035d274e91e6f85b9af63e9a3fd2cf70a2295d9eedbfc23a2ee82d4383`

Byte-identical to official extracted `0.151.0`. Selected flags remain:
- `app-server`
- `--listen`
- `stdio://`

`ModelListParams` is byte-identical to `0.151.0` and `0.149.1`:
`de29a536c00a5b8f46f34dba417dabd93365305571a8ed200e33bea85db68b5a`.

Selected param files are byte-identical to `0.151.0` for `thread/list`,
`thread/read`, `thread/start`, `thread/resume`, `thread/archive`,
`thread/delete`, `turn/start`, `turn/interrupt`, and `initialize`.
`ThreadResumeParams` SHA-256
`8ac68582a81d60940b10b330be8546123f56bfe246b56f8a4f121da00f347cf2`
is byte-identical to `0.151.0` and still includes already-selected mapped
optional `excludeTurns` (`threadId` required; properties unchanged).
Current production `session_role.rs` emits `excludeTurns: true` from
`0.129.0`. ClientRequest method titles are unchanged (98).

Generated bundles differ by unmapped extras:
- v2 bundle: `08fa1b1072c5d8a889f00fdd96d1c853084e288d89d246552c1c47c23142adbb`
- experimental bundle: `d8faa38d5f00aa7ddfe635a2d374ee5f871ffd217d4d175c72fbe7f009f4f669`

All selected mapped methods remain:
- `initialize`, `model/list`, `thread/list`, `thread/read`, `thread/start`,
  `thread/resume`, `thread/archive`, `thread/delete`, `turn/start`,
  `turn/interrupt`, `item/started`, `item/completed`, `item/plan/delta`,
  `subAgentActivity`, `collabAgentToolCall`

Selected required fields are unchanged. `thread/resume` `excludeTurns` is
already mapped and unchanged; it is not an unused addition.

Additive unmapped fields:
- `thread/shellCommand` optional `timeoutMs`
- ModelProvider `authRecoveryStarted` / `authRecoveryCompleted`
  notifications

Unmapped additions stay unused:
- `--thread-source`, `exec fork`, `top-level fork`, `thread/fork`,
  `thread/turns/list`, `thread/items/list`, `--code-mode-host`,
  `codex-code-mode-host`, plugin catalogs, optional MCP grace, Guardian,
  Bedrock, TUI extras, MCP package-style names, MCP `output_token_limit`,
  planning-tool opt-in

Feature-specific exact pins (model verbosity, fast mode, personality,
plan-mode effort) stay on the `0.147.0..=0.149.1` probed points.

## Decision

**Compatible-extension.**

Keep claim IDs: `codex.exec.cli-window-2`, `codex.app-server.cli-window-2`.

Raise `CODEX_LATEST_QUALIFIED_VERSION` from `0.151.0` to `0.152.0`.

Qualify published `0.152.0`. There is no published intermediate between
`0.151.0` and `0.152.0`.

Keep baseline `0.80.0`. Keep existing behavior revisions:
- exec: `codex.exec.jsonl-v1` (maintained)
- app-server: `codex.app-server.v2.workspace-roots` (maintained)
- lifecycle: `codex.app-server.lifecycle.v1.strict-descendant-hard-delete`

Keep gaps: `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`, unpublished `0.149.2`
and `0.150.2`, and unpublished `0.151.1` now inside the raised window.

After qualification, `0.152.1` (unpublished) remains `UnverifiedNewer`.

No new milestone. No decoder update. No provider work. Do not flatten
`codex-code-mode-host` onto `codex.cli`. Do not map new public
operations.

## Sources

- npm registry: `https://registry.npmjs.org/@openai/codex`
- GitHub releases: `https://github.com/openai/codex/releases/tag/rust-v0.152.0`
- Frozen corpus: `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.152.0/`
