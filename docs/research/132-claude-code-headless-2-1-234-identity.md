# 132 Claude Code Headless 2.1.234 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 237

## Question

After Research 127 and the Muse pin move, which AllowUnverified family should
move first, and is installed Claude Code `2.1.234` a compatible extension of
exact headless `2.1.220`, a new milestone, or a stop?

## AllowUnverified ranking

Host already UnverifiedNewer (useful-newer for this machine). Research 127
numbers unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Code headless | `2.1.234` (was `2.1.233` on 2026-08-17) | exact `2.1.220` | named by Next Task; host now matches npm `latest`; one-shot plan-mode route |
| 2 | Claude Code response-only | same CLI `2.1.234` | `2.1.227..=2.1.228` | same binary, different axis; do not mix into this milestone |
| 3 | Oh My Pi RPC | `17.2.15` | exact `17.2.9` | host drifted; later family |
| 4 | Cursor Agent | `2026.08.04-aaa8809` | July exact points | opaque dates; later family |
| 5 | OpenCode HTTP | `1.18.18` | through `1.18.10` | later family |
| 6 | Kimi ACP / headless / local-server | `0.34.0` | through `0.31.1` | later family |
| 7 | Ollama attached | `0.32.9` | `0.14.0..=0.32.1` excluding `0.32.2` | later family |

Host still on a qualified bound (registry newer only): Claude Agent ACP
`0.63.0` (window `0.53.0..=0.64.0` excluding `0.58.0`), Pi `0.83.0`, Qwen
`0.21.2`, Antigravity `1.1.9`. Rank those after host-drifted families.

Gemini stays deferred. Research 127 labeled headless
`record only; future range work deferred` because raising the qualified
bound needed a family card, not because `2.1.233` was rejected.
AllowUnverified already classifies later stables as UnverifiedNewer.

## Method

Compared npm `@anthropic-ai/claude-code@2.1.234`, local `claude --version`,
root `--help` selected flags, the frozen `2.1.220` stream-JSON corpus, and
the production `claude-code.headless-stream-json` claim.

No provider prompt, install, update, or claim edit.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `2.1.234` (published 2026-08-17T18:19:13.187Z) |
| npm integrity | `sha512-Q53mRcFLqPAWfkvqn7vOzTtMHprzwKdKGRW4OS/Kgr/Tsa+2pyVwVetLb7DRZxhBkYsYld2l8Eo4SX76YoNOOA==` |
| npm shasum | `eea30699ec57eb975d3b11e29d6f180c25555665` |
| local CLI | `2.1.234 (Claude Code)` |
| local executable SHA-256 | `08d8700313697cbe730a25420c908a299ce52d56f0eb2cf4fac94cab5109bc57` |
| local size | 310740672 |
| Agent SDK latest | `@anthropic-ai/claude-agent-sdk@0.3.234` (published 2026-08-17T18:20:12.142Z) |
| frozen SDK pin | `@anthropic-ai/claude-agent-sdk@0.3.220` (the `2.1.220` corpus) |
| Research 127 host | `2.1.233`; npm was already `2.1.234` |

Version parse still requires exactly `<semver> (Claude Code)` plus one
trailing newline.

## Protocol comparison

Selected Swallowtail argv flags are still documented:

- `-p` / `--print`
- `--input-format text` (`text`, `stream-json`)
- `--output-format stream-json` (`text`, `json`, `stream-json`)
- `--verbose`
- `--no-session-persistence`
- `--model`
- `--effort` (`low`, `medium`, `high`, `xhigh`, `max`)
- `--permission-mode plan` (`acceptEdits`, `auto`, `bypassPermissions`,
  `manual`, `dontAsk`, `plan`)
- `--tools Read,Glob,Grep`
- `--setting-sources user,project,local`
- `--mcp-config` and `--strict-mcp-config`

Help also documents flags the selected command does not pass, including
`--bare`, `--brief`, `--cloud`, `--include-hook-events`,
`--include-partial-messages`, and `--forward-subagent-text`. Those are
unused deltas, not selected-protocol changes.

`--effort` now lists `xhigh` and `max`. Swallowtail forwards caller-selected
effort and omits `default`. Extra effort names are not a new adapter-private
mapping.

Production still omits `--include-partial-messages`. The frozen
`2.1.220` JSONL remains completion-only activity evidence. This card did not
capture a live stream.

Agent SDK `0.3.234` is lockstep with CLI `2.1.234`. The decoder contract is
the selected stream-JSON types already frozen in `claude-code-2.1.220`, not
a byte-identical SDK republish.

## Segment decision for card 238

Compatible extension. Same axis `claude-code.headless-stream-json`. Same
behavior `claude-code.headless.stream-json.v1`. Keep AllowUnverified.

Raise latest qualified from exact `2.1.220` to exact `2.1.234` on the
existing segment, so `2.1.220..=2.1.234` is Maintained. Intermediates
`2.1.221..=2.1.233` become qualified as compatible ceiling-raise, not a
Grok-style gap. After qualification, synthetic later-stable UnverifiedNewer
is `2.1.235`.

Do not mix `claude-code.response-only-stream-json` into this card. No new
public operation. No provider prompt.
