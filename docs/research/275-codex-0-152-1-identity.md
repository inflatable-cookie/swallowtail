# 275 Codex 0.152.1 Identity

Status: promoted
Owner: Tom
Date: 2026-09-02

## Question

Is official npm `@openai/codex` `0.152.1` (published 2026-09-01T22:36:50.784Z)
a compatible-extension on the current qualified `0.152.0` ceiling, or does it
require a new milestone or a stop?

## Remaining Rank

Named family only. This run does not rank other families.

| Surface | Local | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `codex.cli` (exec + app-server) | `0.150.1` | npm `@openai/codex` `0.152.1`; GitHub `rust-v0.152.1` | latest qualified `0.152.0` | host-qualified; official-newer |

The current host's `0.150.1` binary keeps the digest and size recorded in
Research 264 and 262, which matched the official platform package. `0.152.1`
was already admitted as unverified-newer under `AllowUnverified`. Gemini stays
deferred. Research 274 selected this family alone.

## Method

Compared, with downloaded binaries hashed and never executed:

- Official npm `@openai/codex@0.152.1` wrapper tarball and the
  `0.152.1-darwin-arm64` / `0.152.1-linux-x64` platform tarballs; every
  shipped file hashed and inventoried against the extracted `0.152.0`
  packages
- GitHub release `rust-v0.152.1` (published 2026-09-01T22:33:02Z; annotated
  tag `3c6cfbab` peels to `5adb68a4`), with the complete recursive source
  tree compared against `rust-v0.152.0` (`316795b3`)
- Upstream-published checked-in app-server schema files under
  `codex-rs/app-server-protocol/schema/json/` hashed at both tags
- Exec and app-server help-generating sources (`codex-rs/cli`,
  `codex-rs/exec`, `codex-rs/app-server`) compared at both tags
- Current host `codex-cli 0.150.1`, its binary digest and size, and
  TeamIdentifier `2DC432GLL2`

No provider prompt sent. No live session. Host install not changed. No
downloaded official binary was executed; version presence inside the
extracted binaries was corroborated from bytes (`0.152.1` literal, no
`0.152.0` literal). Official artifacts stayed in `/tmp`.

## Identity

### Official

- npm package: `@openai/codex@0.152.1`
- npm integrity:
  `sha512-dSwQzl6JgsFe8L9i8xUnwRz9Vy8gn4UvXFU9xq2IJ1eC7zsSttqQ2SGq49ZZIjEyZQ0LZjCs6Bvtxort2Iyebg==`
- npm shasum: `9e51ebd177c5523b299636a2d5f788922fe6eb03`
- wrapper tarball SHA256:
  `3db7aab0e08454c908a874c561f75a93d3b304f2da21957272cd7b73ff45195b`
- darwin-arm64 tarball SHA256:
  `a780ff1a424724778f85c1ccb4de3b908ad1804ef09260cd8140a2ceb7e2ab12`
- darwin-arm64 CLI SHA256:
  `8194ea3181f330e63023b234b0b231855e5874e0331c5ef7cbc490591497a7bf`
- darwin-arm64 CLI size: 217778592 bytes
- linux-x64 tarball SHA256:
  `0ed4978e80117a5e203a436026c37276029a3642d633b6916ab45143d10565cd`
- linux-x64 CLI SHA256:
  `b82018241214a4a7c6b97b198585192d1dbc3aab1ddcdc640f04d8dee8c606f9`
- linux-x64 CLI size: 255505120 bytes
- Git tag: `rust-v0.152.1`
- Tag object: `3c6cfbab81e44218c729dc8c6b304cb760d1b8a1`
- Peeled commit: `5adb68a49933ae446bf11935662c83dba55a0804`
- CLI version string: `codex-cli 0.152.1` (expected from package, tag, and
  in-binary literal agreement; binary not executed)

Published stables after previous ceiling `0.152.0`: `0.152.1` only.
Unpublished gaps inside the window remain `0.149.2`, `0.150.2`, and
`0.151.1`. First unpublished later stable is `0.152.2`. Dist-tag `alpha` is
`0.153.0-alpha.4` and is ignored.

The `0.152.0` tarball digests recomputed this run match Research 264 exactly,
and the host digest and size match the prior records.

Wrapper `0.152.1`, platform packages `0.152.1-darwin-arm64` /
`0.152.1-linux-x64`, extracted CLI version literal, and GitHub tag
`rust-v0.152.1` reconcile.

### Host

- version: `codex-cli 0.150.1`
- target: `aarch64-apple-darwin`
- CLI SHA256: `a14f9a907c12c8812878b70e6b7d65f81c39ed795513e46a55817d7428c0ca6b`
- CLI size: 228986048 bytes
- codesign TeamIdentifier: `2DC432GLL2`
- matches the frozen prior host identity record exactly

The host install was observed only and not changed.

## Selected Protocol

### Tree inventory

The complete shipped-tree delta `0.152.0` → `0.152.1` is:

- wrapper: only `package.json` (version plus platform pins); `README.md` and
  `bin/codex.js` are byte-identical
- darwin-arm64 platform package: `package.json`, `bin/codex`,
  `bin/codex-code-mode-host`, `codex-package.json`, vendored `rg`, and
  vendored `zsh/bin/zsh` changed; `README.md` is byte-identical
- linux-x64 platform package: `package.json`, `bin/codex`,
  `bin/codex-code-mode-host`, and `codex-package.json` changed; `README.md`,
  vendored `rg`, `bwrap`, and `zsh/bin/zsh` are byte-identical
- `codex-package.json` is a version-only change in both packages

The public GitHub source tree at the two tag commits differs in exactly 12
files, none of which feeds a selected surface: the workspace version bump,
Guardian auto-review/node-REPL policy sources and tests, session
step-activation Guardian policy checks, models-manager tests, and one
optional serde-default `AutoReviewMessages.node_repl_policy` field in the
Guardian auto-review messages. No file under `codex-rs/exec`, `codex-rs/cli`,
`codex-rs/app-server`, `codex-rs/app-server-protocol`, `codex-rs/core`'s
rollout listers, `codex-rs/tui`, or the MCP servers changed. The full
inventory is frozen as `dist-inventory.json` with the delta-ledger test.

### Exec

Exec help-generating sources (`codex-rs/cli/src/main.rs`,
`codex-rs/exec/src/cli.rs`, and every other file in both crates) are
byte-identical at `rust-v0.152.0` and `rust-v0.152.1`, so the frozen help
observation carries over unchanged. The frozen exec help SHA256
`e504bac5a6364566fbe408132dec7993639def9258ece34e8352f51f8d43687c` was
byte-identical at `0.149.1` through `0.152.0`. Selected flags remain:

- `exec`
- `--json`
- `--ephemeral`
- `--ignore-user-config`
- `--ignore-rules`
- `--skip-git-repo-check`
- `--sandbox` with `read-only`

The exec JSONL wire sources (`codex-rs/exec/src/exec_events.rs`,
`event_processor_with_jsonl_output.rs`) are byte-identical.

### App-server

App-server help-generating sources (`codex-rs/app-server`) are byte-identical
at both tags, so the frozen help observation carries over. Selected flags
remain:

- `app-server`
- `--listen`
- `stdio://`

Upstream-published schema digests are byte-identical to the frozen `0.152.0`
corpus values:

- v2 bundle: `08fa1b1072c5d8a889f00fdd96d1c853084e288d89d246552c1c47c23142adbb`
- experimental bundle:
  `d8faa38d5f00aa7ddfe635a2d374ee5f871ffd217d4d175c72fbe7f009f4f669`
- `ModelListParams`:
  `de29a536c00a5b8f46f34dba417dabd93365305571a8ed200e33bea85db68b5a`
- `ThreadResumeParams` (including already-selected mapped `excludeTurns`):
  `8ac68582a81d60940b10b330be8546123f56bfe246b56f8a4f121da00f347cf2`
- `TurnStartParams`:
  `a3835e8c1e942e4b358e1a670939b89918b16c4d13105a579899892b7ade6dea`

All selected mapped methods remain `initialize`, `model/list`,
`thread/list`, `thread/read`, `thread/start`, `thread/resume`,
`thread/archive`, `thread/delete`, `turn/start`, `turn/interrupt`,
`item/started`, `item/completed`, `item/plan/delta`, `subAgentActivity`, and
`collabAgentToolCall`. Selected required fields are unchanged.

Unmapped additions stay unused: the inherited `0.152.0` set plus the new
optional Guardian `AutoReviewMessages.node_repl_policy`. The darwin-only
vendored ripgrep and zsh refreshes are not selected surfaces; their linux
counterparts are byte-identical.

Feature-specific exact pins (model verbosity, fast mode, personality,
plan-mode effort) stay on the `0.147.0..=0.149.1` probed points.

## Decision

**Compatible-extension.**

Keep claim IDs: `codex.exec.cli-window-2`, `codex.app-server.cli-window-2`.

Raise `CODEX_LATEST_QUALIFIED_VERSION` from `0.152.0` to `0.152.1`.

Qualify published `0.152.1`. There is no published intermediate between
`0.152.0` and `0.152.1`.

Keep baseline `0.80.0`. Keep existing behavior revisions:

- exec: `codex.exec.jsonl-v1` (maintained)
- app-server: `codex.app-server.v2.workspace-roots` (maintained)
- lifecycle: `codex.app-server.lifecycle.v1.strict-descendant-hard-delete`

Keep gaps: `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`, and unpublished `0.149.2`,
`0.150.2`, and `0.151.1`.

After qualification, unpublished `0.152.2` remains `UnverifiedNewer`.

No new milestone. No decoder update. No provider work. Do not flatten
`codex-code-mode-host` onto `codex.cli`. Do not map new public operations.
The claim card is g05 card 049.

## Sources

- npm registry: `https://registry.npmjs.org/@openai/codex`
- GitHub releases:
  `https://github.com/openai/codex/releases/tag/rust-v0.152.1`
- Frozen corpus: `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.152.1/`
