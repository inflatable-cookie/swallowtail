# 202 Claude Code 2.1.241 Identity

Status: promoted
Owner: Tom
Date: 2026-08-24
Card: g04 batch 153

## Question

Is official npm `@anthropic-ai/claude-code` `latest` = `2.1.241` a
compatible extension of headless `2.1.220..=2.1.238` and response-only
`2.1.227..=2.1.238`, a new milestone, or a stop? Headless and
response-only stay one family.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Code | not installed | headless `2.1.220..=2.1.238`; response-only `2.1.227..=2.1.238` | operator-named family; official npm `latest` is `2.1.241` |

Do not flatten this family onto Claude Agent ACP. Do not split headless and
response-only into separate currentness runs. Do not touch Codex, Qwen, or
Ollama.

## Method

Compared npm `@anthropic-ai/claude-code@2.1.241` and published
intermediates `2.1.239` and `2.1.240` to the frozen `2.1.238` identity
corpus, the `2.1.220` headless decoder specimen, the `2.1.227`/`2.1.228`
response-only specimens, extracted official wrapper tarballs, extracted
official linux-x64 binaries (`--version` and `--help`), and GitHub
`CHANGELOG.md` for `2.1.239`, `2.1.240`, and `2.1.241`.

No provider prompt, install, update, or live stream. Host `claude` was
not on PATH. Missing install is not a gap. Official binaries were
extracted to `/tmp` only.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `2.1.241` (published 2026-08-22T23:58:33.046Z) |
| npm integrity | `sha512-S7DWEmJJAsI5taAUjhKm6soXcFJYIVeTH6Lg9kmp3yntFllCP612hGwZ7thOGh8r7YaRUH9+1jCX5A9QGazsxg==` |
| npm shasum | `150077700180a6f915a486a34b4c34404e4aee59` |
| npm tarball SHA-256 | `752252ff9a65431c356ce1ae54b7ded74a138aaa7b93148573d97ff541a2e7e6` |
| linux-x64 tarball SHA-256 | `f96dcac778c84318f07beab37056948c076e00412afdc8a3b8d052312a5d8e34` |
| linux-x64 binary SHA-256 | `0771bd866cff82b76581fc0499f6529e1a36845078f144f8c81dccb3bc7037b8` |
| official `--version` | `2.1.241 (Claude Code)` |
| official `--help` SHA-256 | `71ad650f59e08ae40ede14c534db4f49d8590ee5a4f92f6da2882d3a5560fea6` |
| host CLI | not installed |
| Agent SDK latest | `@anthropic-ai/claude-agent-sdk@0.3.241` (published 2026-08-22T23:59:33.491Z) |
| frozen SDK pin | `@anthropic-ai/claude-agent-sdk@0.3.220` (the `2.1.220` corpus) |
| Research 175 ceiling | `2.1.238` |

Published stables after previous ceiling `2.1.238`: `2.1.239`
(2026-08-21T17:18:54.506Z), `2.1.240` (2026-08-22T13:03:23.566Z),
`2.1.241`. First unpublished later stable is `2.1.242`. Dist-tag
`stable` remains `2.1.231`; this family's documented channel is npm
`latest`.

The npm package remains an installer wrapper. `cli-wrapper.cjs`,
`install.cjs`, `bin/claude.exe`, `LICENSE.md`, and `README.md` are
byte-identical to `2.1.238`. `package.json` changes only the version pin
and optionalDependencies platform packages. `sdk-tools.d.ts` added
unmapped WebFetch `read` / `watch` / `unwatch` / `watches` result types
at `2.1.239`; that file is identical from `2.1.239` through `2.1.241`.
That is not selected stream-JSON.

Official extracted `--help` is byte-identical from `2.1.238` through
`2.1.241`.

## Protocol comparison

Selected Swallowtail argv flags are present and unchanged. Host `--help`
was not re-probed because the CLI was not installed. Official extracted
help matches the frozen `2.1.238` dump:

- `-p` / `--print`
- `--input-format text` (`text`, `stream-json`)
- `--output-format stream-json` (`text`, `json`, `stream-json`)
- `--verbose`
- `--no-session-persistence`
- `--model`
- `--effort` (`low`, `medium`, `high`, `xhigh`, `max`)
- `--permission-mode plan` (headless)
- `--tools Read,Glob,Grep` (headless) or `--tools ""` (response-only)
- `--setting-sources user,project,local` (headless)
- `--safe-mode`, `--disable-slash-commands`, `--no-chrome`,
  `--prompt-suggestions false` (response-only)
- `--mcp-config` and `--strict-mcp-config`

Unused help flags stay unused: `--bare`, `--brief`, `--cloud`,
`--include-hook-events`, `--include-partial-messages`,
`--forward-subagent-text`, `--json-schema`, `--max-budget-usd`.

Changelog extras stay unmapped: `--max-budget-usd` premium wording,
fullscreen renderer offer, `/claude-api upgrade`, cloud plugin
`name@synced`, musl clipboard add-ons, Bedrock proxy fixes, WebFetch
retention, MCP elicitation/reconnect, resume/directory/session-title
interactive fixes, `keybindingFlavor` readline word keys,
`CLAUDE_CODE_RETRY_WATCHDOG` spend-limit fail-fast, and
`ListAgents`/`SendMessage` teammate extras. None of those are selected
mapped stream-JSON.

`2.1.240` and `2.1.241` changelog rows are bug-fix only.

g03.068's fail-closed response-only validator still applies. This card
did not capture a live stream.

## Segment decision for card 154

Compatible extension on both axes. Same behaviors
`claude-code.headless.stream-json.v1` and
`claude-code.response-only.stream-json.v1`. Keep AllowUnverified and the
empty response-only deny-list.

Raise latest qualified from `2.1.238` to `2.1.241` on both existing
segments: headless `2.1.220..=2.1.241`, response-only
`2.1.227..=2.1.241`. Qualify published intermediates `2.1.239` and
`2.1.240`. After qualification, synthetic later-stable UnverifiedNewer
is `2.1.242`.

Do not flatten onto Claude Agent ACP. No new public operation. No
provider prompt.

## Sources

- npm `@anthropic-ai/claude-code@2.1.239`, `@2.1.240`, `@2.1.241`
- official `@anthropic-ai/claude-code-linux-x64@2.1.238` through `@2.1.241`
- [CHANGELOG.md](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md)
- frozen `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.238/`
