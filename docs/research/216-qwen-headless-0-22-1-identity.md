# 216 Qwen Headless 0.22.1 Identity

Status: promoted
Owner: Tom
Date: 2026-08-26
Card: g04 batch 192

## Question

Is official npm `@qwen-code/qwen-code` `latest` = `0.22.1` (via published
`0.22.0`) a compatible extension of exact Maintained `0.21.15`
`qwen-code.headless.v0.21.15-reasoning-control`, a private milestone,
a new driver/facade, or a stop? This is a new 0.22 line. Ignore
dist-tags `preview` and `nightly`.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Qwen headless | not installed | `0.19.11..=0.20.1`, `0.21.0..=0.21.14`, exact `0.21.15` | operator-named family; official npm `latest` is `0.22.1` |

Gemini stays deferred. Do not flatten this family onto Model Studio or
ACP. Do not reopen unpublished `0.20.2` or `0.21.16`. Do not extend
reasoning or budgets past exact `0.21.15`.

## Method

Compared npm `@qwen-code/qwen-code@0.22.0` and `@0.22.1`, GitHub tag
`v0.22.0`, public commit `2755dbe` (`chore(release): v0.22.1`; no
`v0.22.1` tag), and selected git blobs
`packages/cli/src/nonInteractive/types.ts`,
`packages/cli/src/nonInteractive/control/controllers/systemController.ts`,
`packages/cli/src/config/config.ts`,
`packages/cli/src/nonInteractive/session.ts`,
`packages/core/src/core/reasoning-effort.ts`, and
`packages/core/src/core/openaiContentGenerator/provider/dashscope.ts`
against the frozen `0.21.15` corpus. Selected mapped flags were checked
in `config.ts`. Official artifacts stayed in `/tmp`.

No provider prompt. No live catalogue. No live headless session. Host
install was not present and was not replaced. Dist-tags `preview` and
`nightly` were not installed.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `qwen` absent from PATH; missing install is not a gap |
| Official npm latest | `0.22.1` | published 2026-08-25T17:45:44.326Z; integrity `sha512-sDki8GaxUA7eEbo1SQNd15TXiP22CMmOpUmfKeDvl+vmyw5sMwX5XJunQ8R4zReRV8z+HIaqqK5u28UX807lhw==`; gitHead `2755dbe1399f94e53e24377d2e21fa86ce923529`; tarball SHA-256 `1108f84ad96f9582c7513f4d83fde2e015b54d0b32239943b1c4ce4044a0f998` |
| Published intermediate | `0.22.0` | published 2026-08-22T14:56:59.352Z; GitHub `v0.22.0` 2026-08-22T14:58:36Z; integrity `sha512-y66e3+gVso86miKbp1vc81cJ/RGx/OKvVlFGpMX09tFS3jvQyEmqa4VPYAMx/++04glRGIYMyv98pipoMMN1Qg==`; gitHead/tag `1c3a385d9bc83e0b2a1ce5a24454ce1d090595fb` |

Published stables after previous ceiling `0.21.15`: `0.22.0` then
`0.22.1`. npm still has no `0.21.16` and no `0.22.2`. Dist-tag `preview`
is `0.22.2-preview.1` and is ignored. Nightly is ignored. `cli-entry.js`
digest `68cb29eb7ccc936d78ece5564ef55cae41a55b630e6657dc417c1f2e561cf4c9`
is identical from `0.21.15` through `0.22.1`.

## Selected protocol

`0.22.0` stream types, catalogue controller, session, dashscope, and
`reasoning-effort.ts` are byte-identical to `0.21.15`. `0.22.0`
`config.ts` adds unmapped `--restore-ask-user-question` and
`lsToolEnabled`.

`0.22.1` mapped-subset deltas:

- `types.ts` (`9cf8d998…`, was `53801c8b…`): `PermissionMode` now aliases
  `ApprovalModeValue`; unmapped MCP `versionNegotiation`
- `systemController.ts` (`737d581b…`, was `69dd2366…`): extra unmapped
  MCP constructor arguments
- `session.ts`: `chat-recording-failure` import path only
- `config.ts`: `--approval-mode` parser uses shared `APPROVAL_MODES`;
  values stay `plan|default|auto-edit|auto|yolo`. Swallowtail still
  dispatches `default`
- `dashscope.ts`: clamps configured `max` to `xhigh` for the
  qwen3.8-max family. Not a mapped public operation. Reasoning stays
  exact `0.21.15`

Selected mapped flags remain:
`--safe-mode`, `--approval-mode`, `--core-tools`, `--exclude-tools`,
`--max-wall-time`, `--max-tool-calls`, `--max-session-turns`,
`--include-partial-messages`, `--input-format`, `--output-format`, and
exact `--resume`. Ambient `--continue` stays unselected. Catalogue still
filters `imageOnly`. Nested `goal_state` stays unmapped.

`0.22.0` release notes: no known breaking changes. Web Shell, review,
autofix, slash-command, and `list_directory` settings-opt-in extras stay
unmapped. `--core-tools` still names `list_directory`.

Decoder specimen remains `qwen-code-v0.19.11`.

No new mapped public operation. No live session required.

## Segment decision for card 193

Compatible extension of existing
`qwen-code.headless.v0.21.15-reasoning-control`. Same axis. Keep
AllowUnverified. Keep Deprecated `0.19.11..=0.20.1`. Keep unpublished
stables `0.20.2` and `0.21.16` incompatible.

Keep exact `0.21.15`. Add same-revision `0.22.0..=0.22.1`. Raise latest
qualified to `0.22.1`. After qualification, synthetic unpublished later
stable is `0.22.2`. Preview `0.22.2-preview.1` stays a prerelease reject.

Do not extend reasoning or budgets past exact `0.21.15`. No new public
operation. No provider prompt.

## Sources

- npm `@qwen-code/qwen-code@0.22.1` and `@0.22.0`
- [GitHub `v0.22.0`](https://github.com/QwenLM/qwen-code/releases/tag/v0.22.0)
- commit [`2755dbe`](https://github.com/QwenLM/qwen-code/commit/2755dbe1399f94e53e24377d2e21fa86ce923529)
- frozen `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.15/`
