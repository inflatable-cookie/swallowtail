# 161 Claude Agent ACP 0.70.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 310

## Question

After g03.098 qualified Codex through `0.148.0`, is official Claude Agent
ACP `0.70.0` a compatible extension of `claude-agent.acp-adapter` through
`0.69.0`, a new private milestone, or a stop?

## Remaining AllowUnverified rank

Codex is done. Remaining Research 159 families, host still on a qualified
bound unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Agent ACP | `0.63.0` | `0.53.0..=0.69.0` excluding `0.58.0` | named next after Codex |
| 2 | Claude Code | `2.1.235` | headless `2.1.220..=2.1.234`; response-only `2.1.227..=2.1.234` | later family; host and official sit above the ceiling |
| 3 | Grok Build | `1.0.5` | exact `1.0.4` | later family |
| 4 | Qwen headless | registry `0.21.14` | through `0.21.13` | later family |
| 5 | Kimi Code | host `0.34.0` | through `0.36.1` | later family |
| 6 | Oh My Pi | registry `17.3.8` | through `17.3.7` | later family |
| 7 | Antigravity | registry `1.1.15` | `1.1.9..=1.1.14` | later family |

Gemini stays deferred. Do not flatten this family onto Claude Code
headless or response-only.

## Method

Compared host `claude-agent-acp --version`, npm
`@agentclientprotocol/claude-agent-acp@0.70.0`, GitHub tag `v0.70.0`,
ACP registry Claude Agent `0.70.0`, and selected npm-tarball
`dist/acp-agent.js`, `dist/elicitation.js`, and `dist/tools.js` against
the frozen `0.69.0` corpus.

No provider prompt. No live ACP initialize. The host install was not
replaced.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.63.0` | Homebrew npm global; `dist/index.js` SHA-256 `260aac90bf75f197b93640087c1de66441761d43c2784efa035fdcee60b5dacd`; gitHead `15979bba7907484ee22111cdc33b79b0bdcd452d`; ACP SDK `1.3.0`; Agent SDK `0.3.220` |
| Official npm/GitHub latest | `0.70.0` | published 2026-08-18T13:19:04.016Z; GitHub 2026-08-18T13:16:17Z; integrity `sha512-Psqj6fhV4pQ8IM480zpJ+xGiMMIqNLxlsTj5Mzn+T8KSURCVNJdl0ktcqLMjgHJC/QnOvDdDkFf3xTW9VIV9aQ==`; commit `d0aafb1ca26427285ffaeac8d8a4452fff28e9c3`; ACP registry `claude-acp` `0.70.0`; tarball SHA-256 `da2cf1b5f66981578313126a49002c4ae7e0c7e71d92b545f2b834835c6db465` |

Published stables after previous ceiling `0.69.0`: `0.70.0` only.
`0.69.1` unpublished. `0.58.0` remains unpublished. First unpublished
later stable is `0.70.1`.

## Selected protocol

`dist/elicitation.js` is byte-identical from `0.64.0` through `0.70.0`
(`d40be7d05a0ca9f65621a54fd61aa1a903875d5afcd95051c1f86b561abdd095`).
`dist/tools.js` matches `0.69.0`
(`b9f6e42e59047bfc0554f862c0d71354cb6af54299f8947963886171657ee230`).

Initialize still returns protocol v1 with the same selected
`promptCapabilities`, `mcpCapabilities`, `loadSession`, and
`sessionCapabilities` `{additionalDirectories, close, delete, fork, list,
resume}`. Permission option kinds remain `allow_once` / `allow_always` /
`reject_once`. Swallowtail still skips `allow_always`.

`dist/acp-agent.js` is not byte-identical (`0.69.0`
`02841efc1088e324aad292c60c72c07d47143c6f6c66e71e978d4eb7fc5eb8a3` vs
`0.70.0` `f0cbbe408bb758cc4bacdae9a244bcac6efbdb6413f680195d017648abc6d816`).
Changelog: recreate loaded Claude SDK queries when `providers/set` or
`providers/disable` changes routing. That Providers API stays unmapped.
Goal, Air, and file-change initialize `_meta` stay unmapped.

ACP SDK stays `1.3.0`. Agent SDK stays `0.3.232`.

## Decision

Compatible extension of the mapped v1 subset on existing
`claude-agent.acp.initialize-meta-extensions-v7`.

- Keep baseline `0.53.0`, unpublished `0.58.0` exclusion, and claim id
  `claude-agent.acp.window-2`.
- Extend Maintained `0.66.0..=0.70.0` on v7. Do not add a new milestone.
- Raise `CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION` to `0.70.0`.
- Synthetic later-stable UnverifiedNewer is `0.70.1`.
- Decoder specimens remain `claude-agent-acp-v0.53.0-v0.61.0` and
  `claude-agent-acp-v0.62.0-v0.64.0`.

Card 311 owns the claim change.

## Sources

- Host `claude-agent-acp --version` on 2026-08-19
- npm `@agentclientprotocol/claude-agent-acp@0.70.0`
- [GitHub `v0.70.0`](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.70.0)
- ACP registry `claude-acp` `0.70.0`
- npm tarball `0.70.0` versus frozen `0.69.0` dist hashes
