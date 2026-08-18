# 138 Ollama 0.32.14 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 250

## Question

After g03.080 qualified Kimi Code through `0.36.1`, which AllowUnverified
family should move first, and are host Ollama `0.32.9` and official GitHub
`v0.32.14` a compatible extension of `ollama.runtime` through `0.32.1`, a
new private milestone, or a stop?

## Remaining AllowUnverified rank

Kimi is done. Remaining host-drifted families, Research 127 numbers unless
noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Ollama attached | `0.32.9` | `0.14.0..=0.32.1` excluding `0.32.2` | named next after Kimi |

Host still on a qualified bound (registry newer only): Claude Agent ACP,
Pi, Qwen, Antigravity. Rank those after this host-drifted family.

Gemini stays deferred. Do not flatten this family onto Ollama Cloud, the
generate API, tools, or thinking as first-class selected operations.

Research 127 already classified Ollama as visible unverified-newer: host
`0.32.9`, GitHub `v0.32.14` (2026-08-15), qualified through `0.32.1` with
exclusion `0.32.2`. Official GitHub latest is still `v0.32.14` on
2026-08-18. Leaving that point UnverifiedNewer would skip useful-newer
support.

## Method

Compared host `ollama --version` / `--help`, GitHub `ollama/ollama` stable
releases from `v0.32.1` through `v0.32.14`, and selected tagged source
`api/types.go` structs plus `server/routes.go` registrations for
`GET /api/version`, `GET /api/tags`, `GET /api/ps`, `POST /api/show`, and
`POST /api/chat`.

No provider prompt. The attached Ollama server was not started. The host
install was not replaced. Official macOS app archives were not downloaded;
identity for the official point is the GitHub tag, commit, tree, and
selected source.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.32.9` | `/usr/local/bin/ollama` → app Resources binary; SHA-256 `ee63fd25df47b95b5ff762d28b40734699b6d61f88de6348946c9dd507c103d9`; size `67927040`; Mach-O universal x86_64/arm64; signer Infra Technologies, Inc (`3MU9H2V9Y9`); CLI identifier `ai.ollama.ollama`; app short version `0.32.9`; GitHub commit `1d5febee105f00c430e19214b7b7b620cf186f98` |
| Official GitHub latest | `0.32.14` | release `v0.32.14` published 2026-08-15T19:41:23Z, `prerelease: false`; commit `d67ad83426633195089509347ffd4fe795120198`; tree `5111fd6297e195cc4ba2abece091721a9c5737a8` |

Published non-prerelease stables after previous ceiling `0.32.1`:
`0.32.3`, `0.32.4`, `0.32.5`, `0.32.6`, `0.32.7`, `0.32.8`, `0.32.9`,
`0.32.11`, `0.32.12`, `0.32.13`, `0.32.14`. No unpublished patch in that
span.

GitHub still marks plain `v0.32.2` and `v0.32.10` as prereleases. Keep
`0.32.2` excluded. Add `0.32.10` as the same class of named hole inside
the raised window. Semantic prereleases such as `0.32.3-rc.0` stay
incompatible.

## Selected protocol

`ChatRequest`, `ChatResponse`, `Message`, `ListResponse`,
`ListModelResponse`, `ProcessResponse`, `ProcessModelResponse`,
`ShowRequest`, `ShowResponse`, and `Options` are byte-identical at
`v0.32.1`, host `v0.32.9`, and official `v0.32.14`.
`ChatRequest` SHA-256
`d7035a0da458f5ab354f771d2ee3eb9239f1ff40dae6700bdcd8e9806b18ae14`.

The five selected routes remain registered:

- `GET`/`HEAD` `/api/version`
- `GET`/`HEAD` `/api/tags`
- `GET` `/api/ps`
- `POST` `/api/show`
- `POST` `/api/chat`

Whole-file `api/types.go` and `server/routes.go` hashes moved. That is
unselected content. No new public selected operation.

## Decision

Compatible extension of `ollama.native-text-v1`. Raise
`OLLAMA_LATEST_QUALIFIED_VERSION` to `0.32.14`. Keep baseline `0.14.0`,
claim id `ollama.native-runtime-window-2`, AllowUnverified, and decoder
specimen `ollama-native-v0.14.0-v0.32.1`. Exclude `0.32.2` and `0.32.10`.
Synthetic later-stable UnverifiedNewer is `0.32.15`.

Card 251 owns the claim change.

## Sources

- Host `ollama --version` / `--help` and codesign on 2026-08-18
- [GitHub `v0.32.14`](https://github.com/ollama/ollama/releases/tag/v0.32.14)
- [GitHub `v0.32.10` still prerelease](https://github.com/ollama/ollama/releases/tag/v0.32.10)
- [GitHub `v0.32.2` still prerelease](https://github.com/ollama/ollama/releases/tag/v0.32.2)
- Tagged `api/types.go` and `server/routes.go` at `v0.32.1`, `v0.32.9`,
  and `v0.32.14`
