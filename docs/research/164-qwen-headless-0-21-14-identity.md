# 164 Qwen Headless 0.21.14 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 316

## Question

After g03.101 qualified Grok Build through `1.0.5`, is official Qwen Code
`0.21.14` a compatible extension of Maintained `0.21.0..=0.21.13`
`qwen-code.headless.v0.21.0-catalogue-filter`, a new milestone, or a stop?
Ignore dist-tag `preview` `0.21.14-preview.0`.

## Remaining AllowUnverified rank

Grok is done. Remaining Research 159 families:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Qwen headless | `0.21.2` | `0.19.11..=0.20.1` and `0.21.0..=0.21.13` | named next; host sits on a qualified bound; official latest is `0.21.14` |
| 2 | Kimi Code | host `0.34.0` | through `0.36.1` | later family |
| 3 | Oh My Pi | registry `17.3.8` | through `17.3.7` | later family |
| 4 | Antigravity | registry `1.1.15` | `1.1.9..=1.1.14` | later family |

Gemini stays deferred. Do not flatten this family onto Model Studio or
ACP. Do not reopen unpublished `0.20.2`.

## Method

Compared host `qwen --version`, npm `@qwen-code/qwen-code@0.21.14`, GitHub
tag `v0.21.14`, and selected git blobs
`packages/cli/src/nonInteractive/types.ts`,
`packages/cli/src/nonInteractive/control/controllers/systemController.ts`,
and `packages/cli/src/config/config.ts` against the frozen `0.21.13`
corpus.

No provider prompt. No live catalogue. No live headless session. The host
install was not replaced. Dist-tag `preview` was not installed.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.21.2` | PATH `qwen` reports `0.21.2`; launcher SHA-256 `08d28a806f88eb00351fd32f32f891ae5c17d39d28c2538a57a8f8b931684a13`; size 71; shebang `#!/usr/bin/env sh`; npm gitHead for this point `456fc9b02d7ed69357dd87db8fe4bcd7e2e55ac1` |
| Official npm/GitHub latest | `0.21.14` | published 2026-08-19T02:45:12.309Z; GitHub release 2026-08-19T02:46:42Z; integrity `sha512-+sheZkLj6K34SKN5r6lZ0yQBmJrLNWyzflUmG5UNk3Ycdha643Dr1T3tv5PI3HANNoUiBVMEjTqQzU0hHCe5kw==`; gitHead/tag `6e20a58923b0a00baafa5a7221ff63054ad1af63`; tarball SHA-256 `ea865c120c3d73474f44e27fdc0bffc3ed21eb39403918def1c1a917fc1bc737` |

Published stables after previous ceiling `0.21.13`: `0.21.14` only.
npm still has no stable `0.20.2` and no `0.21.15`. Dist-tag `preview` is
`0.21.14-preview.0` and is ignored. GitHub tag `v0.21.14` matches the npm
gitHead.

## Selected protocol

The three selected source blobs are byte-identical to `0.21.13`:

- `types.ts` `53801c8b14f50a7ece0ea7d649b7f228fcd7d85b`
- `systemController.ts` `69dd23662534ed827abb2871e58953027c3c5bf3`
- `config.ts` `bf538d25955bdacc664352d5beeba66d54c41c7a`

Selected flags remain: `--safe-mode`, `--approval-mode`, `--core-tools`,
`--exclude-tools`, `--max-wall-time`, `--max-tool-calls`,
`--max-session-turns`, `--include-partial-messages`, `--input-format`,
`--output-format`, and exact `--resume`. Ambient `--continue` stays
unselected. Catalogue still filters `imageOnly`. Nested `goal_state` and
initialize `effort_status` stay unmapped.

Release notes add `qwen sessions ps`, `/advisor`, and a live-session
registry. Those stay unmapped. Bundled `cli.js` digest changed; selected
TypeScript source did not. Decoder specimen remains `qwen-code-v0.19.11`.

## Segment decision for card 317

Compatible extension of existing
`qwen-code.headless.v0.21.0-catalogue-filter`. Same axis. Keep
AllowUnverified. Keep Deprecated `0.19.11..=0.20.1`. Keep unpublished
stable `0.20.2` incompatible.

Raise latest qualified from `0.21.13` to `0.21.0..=0.21.14`. After
qualification, synthetic unpublished later stable is `0.21.15`. Preview
`0.21.14-preview.0` stays a prerelease reject.

No new public operation. No provider prompt.

## Sources

- host `qwen --version` on 2026-08-19
- npm `@qwen-code/qwen-code@0.21.14`
- [GitHub `v0.21.14`](https://github.com/QwenLM/qwen-code/releases/tag/v0.21.14)
- frozen `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.13/`
