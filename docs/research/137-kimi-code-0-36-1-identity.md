# 137 Kimi Code 0.36.1 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 248

## Question

After g03.079 qualified OpenCode HTTP through `1.18.18`, which
AllowUnverified family should move first, and are host Kimi Code `0.34.0`
and official `@moonshot-ai/kimi-code` `0.36.1` a compatible extension of
the three `kimi-code.executable` claims through `0.31.1`, new private
milestones, or a stop?

## Remaining AllowUnverified rank

OpenCode HTTP is done. Remaining host-drifted families, Research 127
numbers unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Kimi ACP / headless / local-server | `0.34.0` | through `0.31.1` | named next after OpenCode |
| 2 | Ollama attached | `0.32.9` | `0.14.0..=0.32.1` excluding `0.32.2` | later family |

Host still on a qualified bound (registry newer only): Claude Agent ACP,
Pi, Qwen, Antigravity. Rank those after host-drifted families.

Gemini stays deferred. Do not flatten this family onto Python `kimi-cli`.
Do not flatten ACP onto local-server.

Research 127 already classified Kimi as visible unverified-newer: host
`0.34.0`, npm `0.36.1`, qualified through `0.31.1`. Official npm `latest`
is still `0.36.1` on 2026-08-18. Leaving that point UnverifiedNewer would
skip useful-newer support.

## Method

Compared host `kimi --version` / `--help` / `acp --help`, prompt-free ACP
initialize on the host binary, npm `@moonshot-ai/kimi-code@0.36.1`, GitHub
tags `@moonshot-ai/kimi-code@0.31.1` through `@0.36.1`, selected ACP,
headless, and local-server source blobs, the downloaded darwin-arm64
`0.36.1` archive, extracted `--version` / `--help` / initialize, and the
production `kimi-code.executable` claims.

No provider prompt, local-server start, host install, update, or claim
edit in this research card.

## Identity

| Fact | Value |
| --- | --- |
| host CLI | `0.34.0` |
| host executable SHA-256 | `9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859` |
| host size | 176894272 |
| npm package | `@moonshot-ai/kimi-code` |
| npm latest | `0.36.1` (published 2026-08-14T12:53:34.177Z) |
| npm integrity | `sha512-dAYvA0qIZ/nPOtf+8X0axRP3Supa06oP9xK/JlY/DsrID5IVmDRc2fKTdASNBvSs1XPUbPFwD1cDNXMoEDQfEA==` |
| GitHub tag `0.36.1` | annotated `336fed3b5f265c986d4f43808da98f3c6b4bbd16` |
| commit | `13d86f8b7bb2443a3b8222e7d94deb0a66429f8e` |
| tree | `dea9380c7bcb299706869e7bf6a30aa777580eb0` |
| darwin-arm64 ZIP SHA-256 | `14a09fb898742be77eb2bf41fc7fe0d78fdbdc73a4aa8fd3c80b04ebf6bee193` |
| extracted CLI SHA-256 | `53b8a5d9380131a23c58937f28d64e93830c56aa92c41432f24ab9d8eccf0e50` |
| signer | Beijing Moonshot Technology Co., Ltd (`2J9472RW75`) |
| Research 127 host/npm | `0.34.0` / `0.36.1` |

Published stables `0.32.0`, `0.33.0`, `0.34.0`, `0.35.0`, `0.36.0`,
`0.36.1` are contiguous minors plus one patch. No unpublished patch in
that span. Not Python `kimi-cli`.

## Protocol comparison

Selected ACP source is byte-identical through `0.36.1`:
`packages/acp-adapter/src/events-map.ts` and `server.ts`. The adapter
package version only moves `0.3.6` → `0.3.9` from `0.35.0`. SDK stays
`@agentclientprotocol/sdk ^0.23.0`.

Selected default headless source keeps renderer
`apps/kimi-code/src/cli/prompt-render.ts` and `options.ts`. `run-prompt.ts`
only comments the unselected experimental v2 branch.

Prompt-free initialize on host `0.34.0` and extracted `0.36.1` both
returned protocol v1, auth method `login`, zero stderr, and the same
capability keys. Initialize advertises session close/delete/fork/list;
Swallowtail still does not map close or delete.

Local-server selected deltas:

| Span | Selected delta |
| --- | --- |
| `0.32.0..=0.34.0` | optional `experimental_flags` on GET meta; extra error codes and unknown events; rest-prompt import path; session/broadcaster internals |
| `0.35.0..=0.36.1` | application WebSocket `ping` with required `pong` (`nonce`); extra unknown events; unselected title-generate route at `0.36.1` |

Bearer middleware and model-catalogue protocol blobs stay identical.
Unknown events already preserve namespaced. Optional meta fields are
ignored. The `ping` frame has no `seq`/`session_id`; the current decoder
would treat it as a malformed event. Card 249 must answer `ping` with
`pong` to qualify `0.35.0..=0.36.1`.

## Segment decision for card 249

Compatible extension for all three routes through official `0.36.1`:

- ACP: keep `kimi.acp.reasoning.declared-effort-v2`, raise
  `0.29.0..=0.36.1`
- headless: keep `kimi.headless.stream-json.v1`, raise `0.29.0..=0.36.1`
- local-server: keep exact `0.31.1` refresh-stable; add
  `0.32.0..=0.34.0` `kimi.local-server.rest-ws-v2-optional-meta-flags`;
  add `0.35.0..=0.36.1` `kimi.local-server.rest-ws-v2-heartbeat-ping`
  with ping/pong in the driver

Keep baselines. Keep AllowUnverified. Synthetic later-stable is
`0.37.0`. No new public operation. Decoder specimens stay on the existing
corpora.

## Sources

- [Kimi Code `0.36.1` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.36.1)
- [Kimi Code repository](https://github.com/MoonshotAI/kimi-code)
- [Kimi Code npm package](https://www.npmjs.com/package/@moonshot-ai/kimi-code)
