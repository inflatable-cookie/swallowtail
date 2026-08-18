# 139 Claude Agent ACP 0.69.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 252

## Question

After g03.081 qualified Ollama through `0.32.14`, which AllowUnverified
family should move first, and are host Claude Agent ACP `0.63.0` and
official npm/GitHub `0.69.0` a compatible extension of
`claude-agent.acp-adapter` through `0.64.0`, new private milestones, or a
stop?

## Remaining AllowUnverified rank

Ollama is done. Remaining families have host still on a qualified bound
(registry newer only), Research 127 numbers unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Agent ACP | `0.63.0` | `0.53.0..=0.64.0` excluding `0.58.0` | named next after Ollama |
| 2 | Pi RPC | `0.83.0` | published points through `0.83.0` | later family |
| 3 | Qwen headless | `0.21.2` | through `0.21.2` | later family |
| 4 | Antigravity | `1.1.9` | exact `1.1.9` | later family |

Gemini stays deferred. Do not flatten this family onto Claude Code
headless or response-only.

Research 127 already classified Claude Agent ACP as visible
unverified-newer: host `0.63.0`, npm/GitHub `0.69.0` (2026-08-16),
qualified through `0.64.0` excluding unpublished `0.58.0`. Official npm
`latest` is still `0.69.0` on 2026-08-18. Leaving that point
UnverifiedNewer would skip useful-newer support.

## Method

Compared host `claude-agent-acp --version`, npm
`@agentclientprotocol/claude-agent-acp`, GitHub tags, ACP registry
Claude Agent `0.69.0`, and selected npm-tarball `dist/acp-agent.js`,
`dist/elicitation.js`, and `dist/tools.js` plus `package.json` from
`0.64.0` through `0.69.0`.

No provider prompt. No live ACP initialize. The host install was not
replaced.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.63.0` | Homebrew npm global; `dist/index.js` SHA-256 `260aac90bf75f197b93640087c1de66441761d43c2784efa035fdcee60b5dacd`; gitHead `15979bba7907484ee22111cdc33b79b0bdcd452d`; ACP SDK `1.3.0`; Agent SDK `0.3.220` |
| Official npm/GitHub latest | `0.69.0` | published 2026-08-16T09:21:07.935Z; integrity `sha512-YH4k22QvewY4dlxR7IOEhPk4IyTSvvUMUYfTF1cfD/JasPWonjylCJjeUCp3bOyQhlh3LvxsCCAPKkTa+V/a/g==`; commit `59a7e9367b3931a50178de4783cf6074b20060cd`; ACP registry `claude-acp` `0.69.0`; tarball SHA-256 `73334255e17f5f48f08030fa4e0c54c118e820f9aaaf29f4629aa230e48c65c2` |

Published stables after previous ceiling `0.64.0`: `0.64.1`, `0.64.2`,
`0.65.0`, `0.66.0`, `0.67.0`, `0.68.0`, `0.69.0`. No unpublished patch in
that span. `0.58.0` remains unpublished.

## Selected protocol

`dist/elicitation.js` is byte-identical from `0.64.0` through `0.69.0`
(`d40be7d05a0ca9f65621a54fd61aa1a903875d5afcd95051c1f86b561abdd095`).

Initialize still returns protocol v1 with the same selected
`promptCapabilities`, `mcpCapabilities`, `loadSession`, and
`sessionCapabilities` `{close, delete, fork, list, resume}`. Permission
option kinds remain `allow_once` / `allow_always` / `reject_once`.
Swallowtail still skips `allow_always`.

`dist/acp-agent.js` changes every wrapper. Material selected-adjacent
deltas:

- `0.64.1`/`0.64.2`: permission `allow_always` `_meta` policy description;
  `0.64.2` restores `0.64.0` `tools.js`
- `0.65.0`: unmapped steering idle/cancel settlement internals;
  initialize `_meta` still only `steering.supported`
- `0.66.0`: initialize `_meta.goal` control-method advertisement
- `0.67.0`/`0.68.0`: Air/JetBrains session-failure `_meta`; Agent SDK
  `0.3.232`; `tools.js` task-list parse
- `0.69.0`: agent file-change report capability folded into initialize
  `_meta`

ACP SDK stays `1.3.0`. Nested transcripts and host-owned steering
fallback stay unmapped. Goal, Air, and file-change report stay unmapped.

## Decision

Compatible extension of the mapped v1 subset, with one new private
milestone for initialize `_meta` extras.

- Keep baseline `0.53.0`, unpublished `0.58.0` exclusion, and claim id
  `claude-agent.acp.window-2`.
- Extend `0.64.0..=0.65.0` on existing
  `claude-agent.acp.host-steering-form-marker-v6` and mark that segment
  Deprecated.
- Add Maintained `0.66.0..=0.69.0` on
  `claude-agent.acp.initialize-meta-extensions-v7`.
- Raise `CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION` to `0.69.0`.
- Synthetic later-stable UnverifiedNewer is `0.70.0`.
- Decoder specimens remain `claude-agent-acp-v0.53.0-v0.61.0` and
  `claude-agent-acp-v0.62.0-v0.64.0`.

Card 253 owns the claim change.

## Sources

- Host `claude-agent-acp --version` on 2026-08-18
- npm `@agentclientprotocol/claude-agent-acp@0.69.0`
- [GitHub `v0.69.0`](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.69.0)
- ACP registry `claude-acp` `0.69.0`
- npm tarballs `0.64.0` through `0.69.0`
