# 141 Qwen Headless 0.21.13 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 256

## Question

After g03.083 qualified Pi RPC through `0.84.2`, which AllowUnverified
family should move first, and are host Qwen Code `0.21.2` and official npm
`@qwen-code/qwen-code` `0.21.13` a compatible extension of
`qwen-code.package` through `0.21.2`, a new private milestone, or a stop?

## Remaining AllowUnverified rank

Pi RPC is done. Remaining families have host still on a qualified bound
(registry newer only), Research 127 numbers unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Qwen headless | `0.21.2` | `0.19.11..=0.20.1` and `0.21.0..=0.21.2` | named next after Pi RPC |
| 2 | Antigravity | `1.1.9` | exact `1.1.9` | later family |

Gemini stays deferred.

Research 127 already classified Qwen headless as visible unverified-newer:
host `0.21.2`, npm then `0.21.13` (2026-08-17). Official npm `latest` is
still `0.21.13` on 2026-08-18. Leaving that point UnverifiedNewer would
skip useful-newer support.

## Method

Compared host `qwen --version`, npm `@qwen-code/qwen-code`, GitHub tags
`v0.21.2` through `v0.21.13`, and selected git blobs
`packages/cli/src/nonInteractive/types.ts`,
`packages/cli/src/nonInteractive/control/controllers/systemController.ts`,
and `packages/cli/src/config/config.ts`.

No provider prompt. No live catalogue. No live headless session. The host
install was not replaced.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.21.2` | PATH `qwen` reports `0.21.2`; launcher SHA-256 `08d28a806f88eb00351fd32f32f891ae5c17d39d28c2538a57a8f8b931684a13`; size 71; shebang `#!/usr/bin/env sh`; npm gitHead for this point `456fc9b02d7ed69357dd87db8fe4bcd7e2e55ac1` |
| Official npm/GitHub latest | `0.21.13` | published 2026-08-17T02:10:18.724Z; GitHub release 2026-08-17T02:11:15Z; integrity `sha512-xXyOK166EEeTjHUh9BEdH4h7Afhz53k+jJAv5mgFxQYJbHf25oxif6WRk6jvYGwMxpEdL3vaoURP/QQiplN9lQ==`; gitHead/tag `d959015974302fb60ebd99adb81a68c2f482eaa3`; tarball SHA-256 `b0bfd51d89c21ddbe214c568a7afb93ebc14b1dcf79967ac84bffdc01cb1ec53` |

Published stables after previous ceiling `0.21.2`: `0.21.3` through
`0.21.13`. npm has no stable `0.20.2` and no `0.21.14`. Every GitHub tag
in that span matches the npm gitHead.

## Selected protocol

Selected flags remain in `config.ts` at `0.21.13`: `--safe-mode`,
`--approval-mode`, `--core-tools`, `--exclude-tools`, `--max-wall-time`,
`--max-tool-calls`, `--max-session-turns`, `--include-partial-messages`,
`--input-format`, `--output-format`, and exact `--resume`. Swallowtail
still supplies those arguments. Ambient `--continue` stays unselected.

`types.ts` is byte-identical through `0.21.3`
(`6c7eb0d366f36ba0965fcf6b2fe7c840691f7e71`). At `0.21.4` it adds
unmapped nested `goal_state` on the existing `stream_event` union
(`53801c8b14f50a7ece0ea7d649b7f228fcd7d85b` through `0.21.13`). Top-level
mapped types remain `system`, `stream_event`, `assistant`, and `result`.
Swallowtail already treats unknown nested stream-event types as bounded
unknown activity. No decoder change.

`systemController.ts` still filters `imageOnly` models from
`get_available_models` and still advertises `can_get_available_models`.
`0.21.10` and `0.21.11` change reasoning-effort apply logging and add
unused initialize `effort_status`. Extra initialize fields are ignored.

Exact `--resume <session-id>` still loads that session id. ACP/daemon
session-restore projection is unused.

## Decision

Compatible extension of the mapped headless subset. Reuse
`qwen-code.headless.v0.21.0-catalogue-filter`. Do not add a private
milestone for unmapped `goal_state`.

- Keep baseline `0.19.11` and claim id `qwen-code.headless.package-window-2`.
- Keep Deprecated `0.19.11..=0.20.1`. Keep unpublished stable `0.20.2`
  incompatible.
- Extend Maintained `0.21.0..=0.21.13` on the existing catalogue-filter
  revision. Qualify published intermediates `0.21.3` through `0.21.12`.
- Raise `QWEN_CODE_LATEST_QUALIFIED_VERSION` to `0.21.13`.
- Synthetic later-stable UnverifiedNewer is `0.21.14`.
- Decoder specimen remains `qwen-code-v0.19.11`. Frozen
  `qwen-code-v0.19.11-v0.21.2` compatibility rows stay unchanged.

Card 257 owns the claim change.

## Sources

- Host `qwen --version` on 2026-08-18
- npm `@qwen-code/qwen-code@0.21.13`
- [GitHub `v0.21.13`](https://github.com/QwenLM/qwen-code/releases/tag/v0.21.13)
- git tags `v0.21.2` through `v0.21.13`
