# 176 OpenCode HTTP 1.18.20 Identity

Status: promoted
Owner: Tom
Date: 2026-08-21
Card: g04 batch 081

## Question

Is official npm `opencode-ai` `1.18.20` a compatible extension of the
current `opencode.server` ceiling `1.18.18`, a new private milestone, or
a stop?

## Remaining Rank

This run qualifies only OpenCode HTTP. At observation time the family
sat official-newer:

| Surface | Local | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `opencode.http` / `opencode.server` | not installed | npm `opencode-ai` `latest` = `1.18.20` | latest qualified `1.18.18` | official-newer |

`1.18.20` and published intermediate `1.18.19` were already admitted as
unverified-newer under `AllowUnverified`. Gemini stays deferred. Do not
flatten this HTTP/SSE claim onto OpenCode ACP.

## Method

Compared:
- npm `opencode-ai@latest` = `1.18.20` (published 2026-08-21T08:09:54.390Z)
- npm `opencode-ai@1.18.19` (published 2026-08-20T06:21:26.028Z)
- GitHub tags `v1.18.18`, `v1.18.19`, `v1.18.20` and each tag's
  `packages/sdk/openapi.json`
- frozen compatibility corpus through `1.18.18` and production
  `opencode.server` claim
- ACP registry OpenCode row as discovery metadata only

Hashed the full OpenAPI document. Confirmed the six selected execution
operations plus delete and import/continuity paths remain present.
Changelog extras were classified as unmapped.

No provider prompt. No attached server. Host install not changed.

## Identity

| Fact | Value |
| --- | --- |
| host CLI | not on `PATH` |
| npm package | `opencode-ai` |
| npm latest | `1.18.20` |
| npm integrity | `sha512-8c2yJ/Oe1qFi9KYE0KS9WCyy6O1QtI9odzBmBWGOeyOgXTn/hGOwCp/fgcHY2qVQ2TVgkQXze7jXjJ6AFyeU0Q==` |
| npm shasum | `79ace165fba034da1599fb3411691611228c409a` |
| npm tarball SHA-256 | `d7af626824cab417d9c5c12e5c0187e506f1c903ea93bd8e4b1615be16305d2a` |
| GitHub tag `v1.18.20` | `7248bc1964b13fa67e601733f89ee9dc6dfa0563` |
| GitHub release published | 2026-08-21T08:09:31Z |
| ACP registry OpenCode | `1.18.20` (discovery only) |

Published stables since `1.18.18`: `1.18.19` and `1.18.20`. Contiguous.
No unpublished patch. First unpublished later stable: `1.18.21`.

| Version | Tag commit | npm published | OpenAPI SHA-256 |
| --- | --- | --- | --- |
| `1.18.18` | `31406ccc51b4bd2a4e1e086b2bcaa5f7f804f26d` | 2026-08-13T01:13:43.814Z | `5bbd6493a1a488ef4294889341c896e420f814ecea95822100aaa9f3f95ab2d1` |
| `1.18.19` | `2b72179c663cadcb54f54d9f19221b3fb3d11fb6` | 2026-08-20T06:21:26.028Z | same as `1.18.18` |
| `1.18.20` | `7248bc1964b13fa67e601733f89ee9dc6dfa0563` | 2026-08-21T08:09:54.390Z | same as `1.18.18` |

`1.18.18` OpenAPI SHA-256 matches the frozen corpus row. Path count stays
162. Operation count stays 188. The three tagged documents are
byte-identical.

## Protocol comparison

Selected execution operations remain present and unchanged:

- `GET /global/health`
- `GET /provider`
- `POST /session`
- `POST /session/{sessionID}/prompt_async`
- `GET /event`
- `POST /session/{sessionID}/abort`

Delete (`DELETE /session/{sessionID}`), import (`session.list` / `status`
/ `get` / `messages` / `prompt_async`), and continuity closures are
identical because the OpenAPI bytes are identical.

Unmapped changelog extras stay unmapped: Cloudflare AI Gateway
passthroughs, ChatGPT/Codex rate limits, `/connect` display, model
pricing, websocket fallbacks, Console URL, OpenCode Go web search, v1
database compatibility, desktop server dialog, subagent `task_id`,
`network_error` retries, Cerebras token cap, `opencode run` subagent
permissions, xAI capacity retries.

Import, reconcile, history, and detach still require a qualified server
version. They do not inherit on unverified-newer.

## Decision

**Compatible-extension.**

Same axis `opencode.server`. Keep baseline `1.14.48`, AllowUnverified,
surfaces `01` through `19`, and unpublished gaps. Do not flatten onto
one closed `1.14.48..=1.18.20` interval. No new private surface: selected
mapped subset is unchanged.

- keep `1.18.11..=1.18.18` on `surface-19` and extend through `1.18.20`
- qualify published intermediates `1.18.19` and `1.18.20`
- extend delete-02, import-07, continuity-07, callback, runtime-02, and
  reconciliation through `1.18.20`
- after qualification, synthetic later-stable UnverifiedNewer is `1.18.21`

Decoder specimen remains `opencode-1.14.48`. Claim card:
[g04 batch 082](../roadmaps/g04/batch-cards/082-opencode-http-1-18-20-claim.md)

## Sources

- npm registry: `https://registry.npmjs.org/opencode-ai`
- GitHub tag: `https://github.com/anomalyco/opencode/releases/tag/v1.18.20`
- OpenAPI: `https://github.com/anomalyco/opencode/blob/v1.18.20/packages/sdk/openapi.json`
- Frozen corpus: `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.20/`
