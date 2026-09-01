# 179 Kimi Code 0.38.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-21
Card: g04 batch 087

## Question

Is official npm `@moonshot-ai/kimi-code` `0.38.0` (published 2026-08-20)
a compatible extension of the three `kimi-code.executable` claims through
`0.37.2`, a private milestone, or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Kimi ACP / headless / local-server | not installed | through `0.37.2` | operator-named family; official npm `latest` is `0.38.0` |

Gemini stays deferred. Do not flatten this family onto Python `kimi-cli`
`1.49.0` or Kimi Platform Chat. Do not flatten ACP onto local-server.

## Method

Compared official npm `@moonshot-ai/kimi-code@0.38.0`, GitHub tag
`@moonshot-ai/kimi-code@0.38.0`, selected ACP, headless, and local-server
source blobs against the frozen `0.37.2` corpus, the downloaded linux-x64
and darwin-arm64 archives, extracted `--version` / `--help` / prompt-free
ACP initialize on linux-x64, and the production `kimi-code.executable`
claims.

No provider prompt, local-server start, host install, update, or claim
edit in this research card. Official latest stayed `0.38.0` through the
probe.

## Identity

| Fact | Value |
| --- | --- |
| host CLI | not installed |
| npm package | `@moonshot-ai/kimi-code` |
| npm latest | `0.38.0` (published 2026-08-20T13:13:41.488Z) |
| npm integrity | `sha512-O/z6sfjFdoDPPeTnoXzdsJ2U8IqP6K2gD3LsT+Nu8BAlHwdhCjdCQFkFTjIbLBun+aZT6x81ha5FiFt7trEilg==` |
| GitHub annotated tag | `488fe6bb311959227c8c2602e12486e48f8b5446` |
| commit | `0999454bdcb5ddd98f39bffee434dcf0a810f394` |
| tree | `b0f988c19b396db2d7127d1f6482743f8c8d4a26` |
| linux-x64 ZIP SHA-256 | `2278e0c90283985c4df46b775bf0f163d07684a7b1bfc83ee3b42844f6fccdfb` |
| extracted linux-x64 SHA-256 | `7f18b701ea751d14bf051776747ca0339f8d59693aec9051276933067a914b00` |
| darwin-arm64 ZIP SHA-256 | `48f534fcbf2d42c0cf80334c1c89e8253d4c198a149980e234b6e927c2759fda` |
| extracted darwin-arm64 SHA-256 | `92bf3b4b6643e7c4cc12c82e5680cc5b54a5a6768a301de815e5e9a02d2184bb` |
| PyPI `kimi-cli` | `1.49.0` (separate axis; not flattened) |

Published stables after previous ceiling `0.37.2`: `0.38.0` only. npm has
no `0.37.3` and no `0.38.1`. Not a major-line reset.

## Selected protocol

Selected ACP source is byte-identical through `0.38.0`:
`packages/acp-adapter/src/events-map.ts`, `server.ts`, `auth-methods.ts`,
`session.ts`, and `convert.ts`. Adapter package stays `0.3.10`. SDK stays
`@agentclientprotocol/sdk ^0.23.0`.

Selected default headless source keeps renderer, options, and
`run-prompt.ts` byte-identical to `0.37.2`. Experimental v2 stays
unselected.

Prompt-free initialize on extracted linux-x64 `0.38.0` returned protocol
v1, auth method `login`, zero stderr, and the same capability keys as
`0.37.2`. Initialize still advertises session close/delete/fork/list.
Nested `terminal-auth` metadata stays unmapped. The initialize command
path was discarded.

Local-server selected protocol blobs stay identical: bearer middleware,
REST model-catalogue protocol, and WebSocket control. Application
`ping`/`pong` remains. `routes/modelCatalog.ts` only refactors the
existing collection actions into a dispatch table.

Changelog extras stay unmapped: `acp --region` / `login --region` on the
already-unmapped login flow, and the WaitFor agent tool (TUI renderer
plus agent-core-v2; not in the selected ACP event map).

## Segment decision for card 088

Compatible extension for all three routes through official `0.38.0`:

- ACP: keep `kimi.acp.reasoning.declared-effort-v2`, raise
  `0.29.0..=0.38.0`
- headless: keep `kimi.headless.stream-json.v1`, raise `0.29.0..=0.38.0`
- local-server: keep `0.32.0..=0.34.0` optional-meta-flags; extend
  `0.35.0..=0.38.0` `kimi.local-server.rest-ws-v2-heartbeat-ping`

Keep baselines. Keep AllowUnverified. No published intermediates.
Synthetic later-stable is `0.38.1`. No new public operation. Decoder
specimens stay on the existing corpora.

## Errata (2026-08-25, Research 210)

Card 088 raised headless qualified ceiling to `0.38.0` alongside ACP and
local-server because selected v1 headless source blobs (renderer, options,
`run-prompt.ts` v1 branch) remained byte-identical to `0.37.2`. Research 210
proved that naked npm `0.38.0` `kimi -p` defaults to agent-core-v2
`runV2Print`, not the legacy v1 print body, unless `KIMI_CODE_LEGACY_FLAG` is
truthy. Swallowtail's headless adapter does not set that flag.

Headless qualified ceiling is therefore retracted to `0.37.2` under existing
`kimi.headless.stream-json.v1`. Exact `0.38.0` headless remains visible
`UnverifiedNewer` until agent-core-v2 stream-json is independently qualified
under an exact revision. ACP and local-server `0.38.0` qualifications stand.

## Errata (2026-09-01, Research 270)

Two named surfaces in this record are wrong, though its digests and its
`0.38.0` identity stand.

The selected ACP source is `packages/acp-server`, not
`packages/acp-adapter`. `sub/acp.ts` has registered the native `acp-server`
command unless `KIMI_CODE_LEGACY_FLAG` is truthy since `0.33.0`, and
Swallowtail never sets that flag. The blob list above describes the legacy
path. No qualified ACP point moves: the mapped `acp-server` blobs are
byte-identical from `0.33.0` through `0.39.1` apart from `server.ts`, which
changes once at `0.37.0`, and the prompt-free initialize observation recorded
here was already taken on the native path.

The 2026-08-25 errata below dates the headless v1→v2 default flip at
`0.38.0`. The flip is `0.33.0`:
`KIMI_CODE_EXPERIMENTAL_FLAG` stopped selecting the engine there, and
`isKimiV2Enabled()` became `!isLegacyEnabled()`. `0.38.0` was simply the only
point sampled. Consequently `0.33.0..=0.37.2` was never a working
`kimi.headless.stream-json.v1` span. See
`crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.33.0-headless-routing/`.

## Sources

- npm `@moonshot-ai/kimi-code@0.38.0`
- [GitHub `0.38.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.38.0)
- frozen `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.37.2/`
