# 163 Grok 1.0.5 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 314

## Question

After g03.100 qualified Claude Code through `2.1.235`, is official Grok
CLI `1.0.5` a compatible extension of exact `1.0.4`
`grok-build.acp-v1.cached-token-model-4-6-v3`, a new milestone, or a stop?
Ignore dist-tag `alpha` `1.0.6`.

## Remaining AllowUnverified rank

Claude Code is done. Remaining Research 159 families:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Grok Build | `1.0.5` | exact `1.0.4` | named next; host and official sit above the ceiling; ignore alpha `1.0.6` |
| 2 | Qwen headless | registry `0.21.14` | through `0.21.13` | later family |
| 3 | Kimi Code | host `0.34.0` | through `0.36.1` | later family |
| 4 | Oh My Pi | registry `17.3.8` | through `17.3.7` | later family |
| 5 | Antigravity | registry `1.1.15` | `1.1.9..=1.1.14` | later family |

Gemini stays deferred. Do not flatten this family onto hosted xAI
Responses WebSocket. Do not reopen `1.0.0..=1.0.3` or `0.2.118..=0.2.121`.

## Method

Compared npm `@xai-official/grok@1.0.5`, local
`grok --no-auto-update --version`, `grok agent stdio --help`, the frozen
`1.0.4` identity and handshake corpus, and one ACP handshake on the host
`1.0.5` executable: initialize, `authenticate(cached_token, headless=true)`,
`session/new` in an empty temporary cwd.

No provider prompt. Account metadata discarded. Session id not retained.
Host install was not replaced. Dist-tag `alpha` `1.0.6` was not installed.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `1.0.5` (published 2026-08-16T00:25:35.078Z) |
| npm alpha | `1.0.6` (published 2026-08-18T19:25:15.447Z; ignored) |
| npm integrity | `sha512-kk5hez+Oz5CvWonDGkMNmL483CWRIGRF2ki8jQzpIXH56P0fhCgaX9lrr0IUoFCKh/rYAm5vfCPgQsdIIYLu8Q==` |
| gitHead | `5115b46bc909ae5c7f5fc064455197440e796b6b` |
| platform package | `@xai-official/grok-darwin-arm64@1.0.5` |
| local CLI | `grok 1.0.5 (5115b46bc909) [stable]` |
| local executable SHA-256 | `3dfa7f04fbb5427a8fbead286591543aaecb478b3a0ab222c4329eca1a3b2f86` |
| local size | 134349648 |

Published stables after previous ceiling `1.0.4`: `1.0.5` only on
`latest`. First unpublished later stable is `1.0.7`. Registry `1.0.6` is
the alpha channel, not official latest.

Invocation still advertised: `grok agent stdio`. Discovery still uses
`--no-auto-update --version`. Version text still matches
`grok <semver> (<hex>) [stable]`.

## Selected protocol

Mapped handshake subset matches `1.0.4`:

- ACP protocol version `1`
- agent version `1.0.5`
- `loadSession` and embedded context present
- auth methods `cached_token`, `grok.com`; default `cached_token`
- authenticate succeeded without interactive login
- `session/new` succeeded; stderr empty
- model `grok-4.6` only
- efforts `xhigh`, `high`, `medium`, `low`
- session capabilities still advertise `list`, `resume`, and `close`

Help adds unused `--leader-socket`. Initialize now also advertises `auth`
and `mcpCapabilities`. Vendor notifications `_x.ai/mcp/servers_updated`,
`_x.ai/models/update`, `_x.ai/settings/update`,
`_x.ai/announcements/update`, `_x.ai/mcp_initialized`, and
`_x.ai/session_notification` appear during `session/new`. Those stay
unmapped. Resume and continuation recovery stay unqualified.

## Segment decision for card 315

Compatible extension of existing
`grok-build.acp-v1.cached-token-model-4-6-v3`. Same axis. Keep
AllowUnverified. Keep deprecated `0.2.114..=0.2.117`. Keep gaps
`0.2.118..=0.2.121` and unprobed `1.0.0..=1.0.3`.

Raise latest qualified from exact `1.0.4` to `1.0.4..=1.0.5`. After
qualification, published alpha `1.0.6` remains permitted UnverifiedNewer
and is not official latest. Synthetic unpublished later stable is
`1.0.7`.

No new public operation. No provider prompt.

## Sources

- host `grok 1.0.5 (5115b46bc909) [stable]`
- npm `@xai-official/grok@1.0.5`
- frozen `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-4/`
