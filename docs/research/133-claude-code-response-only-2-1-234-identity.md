# 133 Claude Code Response-Only 2.1.234 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 239
Correction: 2026-08-18 operator rejected keep-provisional; currentness
lane raises the qualified ceiling. g03.068 provisional-newer is the
policy for stables *above* the latest qualified point, not a reason to
leave the current official/host stable unqualified.

## Question

After g03.075 qualified Claude Code headless through `2.1.234`, should the
separate response-only axis raise its qualified ceiling from `2.1.227..=2.1.228`,
keep host `2.1.234` as UnverifiedNewer, or deny it?

## Remaining AllowUnverified rank

Headless is done. Remaining host-drifted families, Research 127 numbers
unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Code response-only | same CLI `2.1.234` | `2.1.227..=2.1.228` | named next after headless; same binary, different axis |
| 2 | Oh My Pi RPC | `17.2.15` | exact `17.2.9` | later family |
| 3 | Cursor Agent | `2026.08.04-aaa8809` | July exact points | later family |
| 4 | OpenCode HTTP | `1.18.18` | through `1.18.10` | later family |
| 5 | Kimi ACP / headless / local-server | `0.34.0` | through `0.31.1` | later family |
| 6 | Ollama attached | `0.32.9` | `0.14.0..=0.32.1` excluding `0.32.2` | later family |

Gemini stays deferred. Do not flatten this axis onto headless
`2.1.220..=2.1.234`.

Research 127 already classified response-only as visible unverified-newer:
AllowUnverified plus g03.068's provisional-newer policy. This card asks
whether protocol evidence now justifies moving the qualified boundary.

## Method

Reused Research 132 package identity. Compared selected response-only argv
to local `claude --help` on `2.1.234`, the frozen `2.1.227`/`2.1.228`
stream-JSON specimens, and g03.068.

No provider prompt, install, update, or claim edit.

## Identity

Same package as Research 132:

| Fact | Value |
| --- | --- |
| npm latest | `@anthropic-ai/claude-code@2.1.234` |
| local CLI | `2.1.234 (Claude Code)` |
| local executable SHA-256 | `08d8700313697cbe730a25420c908a299ce52d56f0eb2cf4fac94cab5109bc57` |
| response-only axis | `claude-code.response-only-stream-json` |
| qualified window | `2.1.227..=2.1.228` |
| deny-list | empty |
| posture | AllowUnverified |
| behavior | `claude-code.response-only.stream-json.v1` |

`2.1.234` already classifies as UnverifiedNewer on this axis.

## Protocol comparison

Selected response-only flags are still documented: `-p`, `--input-format`,
`--output-format stream-json`, `--verbose`, `--no-session-persistence`,
`--model`, `--effort`, `--tools ""`, `--safe-mode`,
`--disable-slash-commands`, `--no-chrome`, `--prompt-suggestions false`,
`--mcp-config`, `--strict-mcp-config`.

`--prompt-suggestions` help now says print/SDK mode may emit a
`prompt_suggestion` message after each turn when enabled. Selected argv
still sets `false`. Without a live transcript, that disabled path is not
proven silent.

g03.068 made later stables provisional on purpose: init must echo the
preflight version with empty `tools` and `mcp_servers`, then exactly one
text result (`num_turns: 1`). Qualified evidence is the frozen `2.1.227` /
`2.1.228` JSONL plus the gated `2.1.228` Max/OAuth probe. Help-flag presence
is not that transcript. Headless ceiling-raise is not this axis.

## Segment decision

Wrong first call (card 240): keep-provisional, leave latest qualified at
`2.1.228`. Operator rejected that. The currentness lane is supposed to
bring qualified support up to the current official/host stable.

Corrected decision for card 241: compatible extension. Same axis
`claude-code.response-only-stream-json`. Same behavior
`claude-code.response-only.stream-json.v1`. Keep AllowUnverified and the
empty deny-list. Raise latest qualified from `2.1.228` to `2.1.234` on
the existing segment, so `2.1.227..=2.1.234` is Maintained. After
qualification, synthetic later-stable UnverifiedNewer is `2.1.235`.
g03.068's fail-closed protocol validator still applies to every run,
including qualified points. Do not mix with headless.

No new public operation. No provider prompt.
