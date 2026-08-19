# 165 Kimi Code 0.37.2 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 318

## Question

After g03.102 qualified Qwen headless through `0.21.14`, is official Kimi
Code `0.37.2` a compatible extension of the three `kimi-code.executable`
claims through `0.36.1`, a new milestone, or a stop?

## Remaining AllowUnverified rank

Qwen is done. Remaining Research 159 families:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Kimi ACP / headless / local-server | `0.34.0` | through `0.36.1` | named next; host sits on a qualified bound; official latest is `0.37.2` |
| 2 | Oh My Pi | registry `17.3.8` | through `17.3.7` | later family |
| 3 | Antigravity | registry `1.1.15` | `1.1.9..=1.1.14` | later family |

Gemini stays deferred. Do not flatten this family onto Python `kimi-cli`.
Do not flatten ACP onto local-server.

## Method

Compared host `kimi --version`, npm `@moonshot-ai/kimi-code@0.37.2`, GitHub
tags `@moonshot-ai/kimi-code@0.36.1` through `@0.37.2`, selected ACP,
headless, and local-server source blobs, the downloaded darwin-arm64
`0.37.2` archive, extracted `--version` / `--help` / prompt-free ACP
initialize, and the production `kimi-code.executable` claims.

No provider prompt, local-server start, host install, update, or claim
edit in this research card.

## Identity

| Fact | Value |
| --- | --- |
| host CLI | `0.34.0` |
| host executable SHA-256 | `9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859` |
| host size | 176894272 |
| npm package | `@moonshot-ai/kimi-code` |
| npm latest | `0.37.2` (published 2026-08-18T17:40:35.134Z) |
| npm integrity | `sha512-TAteYb84mV44MEzCaAlfz5f3TiN2yMHuwj9Kd0ePEIMBUqgjlqV1w7PvMT9TN0t87LYfv7BhIYz+ZCHDOM5aJw==` |
| GitHub tag `0.37.2` | annotated `d81accc5aeef55c1c5b395565af560f3a9d17ac7` |
| commit | `c41fadf0f78b35ecaf3d613ca26580a9a093de80` |
| tree | `8bf72746a5e3947a694f0eb6f3cf3eeb7436775b` |
| darwin-arm64 ZIP SHA-256 | `d5256d7dc5f43bda1cddbdccd810d247becbc4884d6c971e465044e3a6999c7a` |
| extracted CLI SHA-256 | `47180f84b94278ffd1903ead3087f6a7065d1768ce8ce9fdba0a3a6347418b67` |
| signer | Beijing Moonshot Technology Co., Ltd (`2J9472RW75`) |

Published stables after previous ceiling `0.36.1`: `0.37.0`, `0.37.1`,
`0.37.2`. npm has no `0.36.2` and no `0.37.3`. Not Python `kimi-cli`.

## Selected protocol

Selected ACP source is byte-identical through `0.37.2`:
`packages/acp-adapter/src/events-map.ts` and `server.ts`. The adapter
package version only moves `0.3.9` → `0.3.10`. SDK stays
`@agentclientprotocol/sdk ^0.23.0`.

Selected default headless source keeps renderer, options, and
`run-prompt.ts` byte-identical to `0.36.1`. Experimental v2 stays
unselected.

Prompt-free initialize on extracted `0.37.2` returned protocol v1, auth
method `login`, zero stderr, and the same capability keys as `0.36.1`.
Initialize still advertises session close/delete/fork/list. Nested
`terminal-auth` metadata and `acp --login` stay unmapped. The initialize
command path was discarded.

Local-server selected deltas at `0.37.0` through `0.37.2` are comment
stripping plus unused optional `runtime_id` on watch-fs payloads.
Application WebSocket `ping`/`pong` remains. Bearer middleware comments
were stripped; bypass policy is unchanged.

## Segment decision for card 319

Compatible extension for all three routes through official `0.37.2`:

- ACP: keep `kimi.acp.reasoning.declared-effort-v2`, raise
  `0.29.0..=0.37.2`
- headless: keep `kimi.headless.stream-json.v1`, raise `0.29.0..=0.37.2`
- local-server: keep `0.32.0..=0.34.0` optional-meta-flags; extend
  `0.35.0..=0.37.2` `kimi.local-server.rest-ws-v2-heartbeat-ping`

Keep baselines. Keep AllowUnverified. Qualify published intermediates
`0.37.0` and `0.37.1`. Synthetic later-stable is `0.37.3`. No new public
operation. Decoder specimens stay on the existing corpora.

## Sources

- host `kimi --version` on 2026-08-19
- npm `@moonshot-ai/kimi-code@0.37.2`
- [GitHub `0.37.2`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.37.2)
- frozen `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.36.1/`
