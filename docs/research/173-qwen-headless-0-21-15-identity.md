# 173 Qwen Headless 0.21.15 Identity

Status: promoted
Owner: Tom
Date: 2026-08-21
Card: g04 batch 072

## Question

Is official Qwen Code `0.21.15` a compatible extension of Maintained
`0.21.0..=0.21.14` `qwen-code.headless.v0.21.0-catalogue-filter`, a new
milestone, or a stop? Ignore dist-tags `preview` and `nightly`.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Qwen headless | not installed | `0.19.11..=0.20.1` and `0.21.0..=0.21.14` | operator-named family; official npm `latest` is `0.21.15` |

Gemini stays deferred. Do not flatten this family onto Model Studio or
ACP. Do not reopen unpublished `0.20.2`. Do not touch Codex.

## Method

Compared npm `@qwen-code/qwen-code@0.21.15`, GitHub tag `v0.21.15`, and
selected git blobs
`packages/cli/src/nonInteractive/types.ts`,
`packages/cli/src/nonInteractive/control/controllers/systemController.ts`,
and `packages/cli/src/config/config.ts` against the frozen `0.21.14`
corpus. Selected mapped flags were checked in `config.ts`.

No provider prompt. No live catalogue. No live headless session. Host
install was not present and was not replaced. Dist-tags `preview` and
`nightly` were not installed.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `qwen` absent from PATH; missing install is not a gap |
| Official npm/GitHub latest | `0.21.15` | published 2026-08-20T17:36:46.233Z; GitHub release 2026-08-20T17:38:51Z; integrity `sha512-f4ER/SRVLpwhcqzuytK3Qeq8bG9HnVhv7f7wsf3cpE/AkRfzKSvaeURnW7s7zI3nWkEqA7DM6njSLYS2s6DWDg==`; gitHead/tag `5dce2515a778f9cf2013168962b4fbc3454636e3`; tarball SHA-256 `8d405b065888b7000a6989d99c2d79257cd8f9f5b68e9078fb76484527351b9a` |

Published stables after previous ceiling `0.21.14`: `0.21.15` only.
npm still has no stable `0.20.2` and no `0.21.16`. Dist-tag `preview` is
still `0.21.14-preview.0` and is ignored. Nightly is ignored. GitHub tag
`v0.21.15` matches the npm gitHead.

## Selected protocol

`types.ts` and `systemController.ts` are byte-identical to `0.21.14`:

- `types.ts` `53801c8b14f50a7ece0ea7d649b7f228fcd7d85b`
- `systemController.ts` `69dd23662534ed827abb2871e58953027c3c5bf3`

`config.ts` changed (`8babd54e3fac09519d584f305a1d5e098c504867`, was
`bf538d25955bdacc664352d5beeba66d54c41c7a`). The delta is unmapped
`--session-id` occupancy: case-insensitive lookup plus fail-closed
read errors. Selected mapped flags remain:
`--safe-mode`, `--approval-mode`, `--core-tools`, `--exclude-tools`,
`--max-wall-time`, `--max-tool-calls`, `--max-session-turns`,
`--include-partial-messages`, `--input-format`, `--output-format`, and
exact `--resume`. Ambient `--continue` stays unselected. Catalogue still
filters `imageOnly`. Nested `goal_state` and initialize `effort_status`
stay unmapped.

Release notes add Web Shell Goal v3, `/review --resume`, standalone
conversation isolation, a hybrid-model Thinking toggle, authenticated
HTTPS Git extension installs, and PTY Agent View workers. Those stay
unmapped. Bundled `cli.js` digest changed; `cli-entry.js` is identical to
`0.21.14`. Decoder specimen remains `qwen-code-v0.19.11`.

No known breaking changes. No new mapped public operation.

## Segment decision for card 073

Compatible extension of existing
`qwen-code.headless.v0.21.0-catalogue-filter`. Same axis. Keep
AllowUnverified. Keep Deprecated `0.19.11..=0.20.1`. Keep unpublished
stable `0.20.2` incompatible.

Raise latest qualified from `0.21.14` to `0.21.0..=0.21.15`. After
qualification, synthetic unpublished later stable is `0.21.16`. Preview
`0.21.14-preview.0` stays a prerelease reject.

No new public operation. No provider prompt.

## Sources

- npm `@qwen-code/qwen-code@0.21.15`
- [GitHub `v0.21.15`](https://github.com/QwenLM/qwen-code/releases/tag/v0.21.15)
- frozen `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.14/`
