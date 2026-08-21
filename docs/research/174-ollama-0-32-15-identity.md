# 174 Ollama 0.32.15 Identity

Status: promoted
Owner: Tom
Date: 2026-08-21
Card: g04 batch 074

## Question

Is official GitHub `ollama/ollama` `v0.32.15` a compatible extension of
Maintained `ollama.runtime` `0.14.0..=0.32.14` excluding `0.32.2` and
`0.32.10`, a new private milestone, or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Ollama attached | not installed | `0.14.0..=0.32.14` excluding `0.32.2` and `0.32.10` | operator-named family; official GitHub latest is `v0.32.15` |

Gemini stays deferred. Do not flatten this family onto Ollama Cloud, the
generate API, tools, thinking, or llama.cpp attached/owned. Do not touch
Codex or Qwen.

## Method

Compared GitHub `ollama/ollama` latest stable `v0.32.15` to the frozen
`0.32.14` corpus. Selected tagged source `api/types.go` structs and
`server/routes.go` registrations for `GET /api/version`, `GET /api/tags`,
`GET /api/ps`, `POST /api/show`, and `POST /api/chat`.

No provider prompt. The attached Ollama server was not started. Host
install was not present and was not replaced. Official app archives were
not downloaded. Identity for the official point is the GitHub tag, commit,
tree, and selected source.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `ollama` absent from PATH; missing install is not a gap |
| Official GitHub latest | `0.32.15` | release `v0.32.15` published 2026-08-19T17:25:16Z, `prerelease: false`; commit `b7871fc0d1d82fe109536efa3e0e8e411c766c75`; tree `3af3821938feefb82726c5cf6efbd51d0eacc433` |

Published non-prerelease stables after previous ceiling `0.32.14`:
`0.32.15` only. No `0.32.16`. GitHub still marks plain `v0.32.2` and
`v0.32.10` as prereleases. Keep those named holes. Semantic prereleases
stay incompatible.

## Selected protocol

`api/types.go` is byte-identical at `v0.32.14` and `v0.32.15` (SHA-256
`032fe8c044429afd42fd9f898c6bbd6efc5977ffeeec4dd3c5a04035e9c3d0b1`).
`ChatRequest`, `ChatResponse`, `Message`, `ListResponse`,
`ProcessResponse`, `ShowRequest`, `ShowResponse`, and `Options` stay
byte-identical. `ChatRequest` SHA-256
`d7035a0da458f5ab354f771d2ee3eb9239f1ff40dae6700bdcd8e9806b18ae14`.

The five selected routes remain registered:

- `GET`/`HEAD` `/api/version`
- `GET`/`HEAD` `/api/tags`
- `GET` `/api/ps`
- `POST` `/api/show`
- `POST` `/api/chat`

`server/routes.go` hash moved. The delta is unselected: `scheduleRunner`
now takes a cached model instead of a name, and chat/generate cancel the
completion context on a mid-stream parser error instead of wedging. That
is the published wedge fix. No new public selected operation.

Release notes also add desktop onboarding, resolved-model metadata cache,
Qwen 3.8 system-message normalize, and MLX/llama.cpp dependency updates.
Those stay unmapped. Do not flatten llama.cpp onto this family.

Decoder specimen remains `ollama-native-v0.14.0-v0.32.1`.

## Decision

Compatible extension of `ollama.native-text-v1`. Raise
`OLLAMA_LATEST_QUALIFIED_VERSION` to `0.32.15`. Keep baseline `0.14.0`,
claim id `ollama.native-runtime-window-2`, AllowUnverified, and decoder
specimen `ollama-native-v0.14.0-v0.32.1`. Keep exclusions `0.32.2` and
`0.32.10`. Synthetic later-stable UnverifiedNewer is `0.32.16`.

Card 073 owns the claim change.

## Sources

- [GitHub `v0.32.15`](https://github.com/ollama/ollama/releases/tag/v0.32.15)
- [GitHub `v0.32.10` still prerelease](https://github.com/ollama/ollama/releases/tag/v0.32.10)
- [GitHub `v0.32.2` still prerelease](https://github.com/ollama/ollama/releases/tag/v0.32.2)
- Tagged `api/types.go` and `server/routes.go` at `v0.32.14` and `v0.32.15`
- frozen `crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.32.14/`
