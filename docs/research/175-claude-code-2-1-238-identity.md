# 175 Claude Code 2.1.238 Identity

Status: promoted
Owner: Tom
Date: 2026-08-21
Card: g04 batch 079

## Question

Is official npm `@anthropic-ai/claude-code` `latest` = `2.1.238` a
compatible extension of headless `2.1.220..=2.1.235` and response-only
`2.1.227..=2.1.235`, a new milestone, or a stop? Headless and
response-only stay one family.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Code | not installed | headless `2.1.220..=2.1.235`; response-only `2.1.227..=2.1.235` | operator-named family; official npm `latest` is `2.1.238` |

Gemini stays deferred. Do not flatten this family onto Claude Agent ACP.
Do not split headless and response-only into separate currentness runs.
Do not touch Codex, Qwen, or Ollama.

## Method

Compared npm `@anthropic-ai/claude-code@2.1.238` and published
intermediates `2.1.236` and `2.1.237` to the frozen `2.1.235` identity
corpus, the `2.1.220` headless decoder specimen, the `2.1.227`/`2.1.228`
response-only specimens, extracted official tarball files, and GitHub
`CHANGELOG.md` for `2.1.236`, `2.1.237`, and `2.1.238`.

No provider prompt, install, update, or live stream. Host `claude` was
not on PATH. Missing install is not a gap.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `2.1.238` (published 2026-08-20T18:01:54.712Z) |
| npm integrity | `sha512-8AgGrM8qxsA5B8KU/MvVND/fMUsF3vZQxeYjz+1Z/rGx/ZmNr0iqjfmUVKVASKN7P9OzkAUHoXgKEpyvgRfUkA==` |
| npm shasum | `a8ba2539a61441b7a268a07dc2bf5623534fd127` |
| npm tarball SHA-256 | `6a7b0ef9b12feea02d7c166b16d2674edca7658daeb137efb4c85d9e5371b6ea` |
| host CLI | not installed |
| Agent SDK latest | `@anthropic-ai/claude-agent-sdk@0.3.238` (published 2026-08-20T18:02:54.893Z) |
| frozen SDK pin | `@anthropic-ai/claude-agent-sdk@0.3.220` (the `2.1.220` corpus) |
| Research 162 ceiling | `2.1.235` |

Published stables after previous ceiling `2.1.235`: `2.1.236`
(2026-08-19T18:45:14.539Z), `2.1.237` (2026-08-19T23:57:54.833Z),
`2.1.238`. First unpublished later stable is `2.1.239`.

The npm package remains an installer wrapper. `cli-wrapper.cjs`,
`install.cjs`, `bin/claude.exe`, `LICENSE.md`, and `README.md` are
byte-identical to `2.1.235`. `package.json` changes only the version pin
and optionalDependencies platform packages. `sdk-tools.d.ts` added an
unmapped optional `artifact_id` and reordered `read_asset` /
`delete_asset` field comments at `2.1.236`; that file is identical from
`2.1.236` through `2.1.238`. That is not selected stream-JSON.

## Protocol comparison

Selected Swallowtail argv flags are unchanged in changelog
`2.1.236..=2.1.238`. Host `--help` was not re-probed because the CLI
was not installed. The mapped subset stays the frozen `2.1.235` flags:

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

Unused help flags from the `2.1.235` corpus stay unused: `--bare`,
`--brief`, `--cloud`, `--include-hook-events`,
`--include-partial-messages`, `--forward-subagent-text`.

Changelog extras stay unmapped: `ANTHROPIC_DEFAULT_MODEL`,
`notify_when_idle`, Concise output style, `keybindingFlavor`, plugin
`headersHelper`, self-hosted-runner flags, MCP `headersHelper` trust
dialog (also mentioned under `claude -p`; Swallowtail still sends empty
`--mcp-config` plus `--strict-mcp-config`), and SIGTERM print/SDK
session-recording changes. None of those are selected mapped
stream-JSON.

g03.068's fail-closed response-only validator still applies. This card
did not capture a live stream.

## Segment decision for card 080

Compatible extension on both axes. Same behaviors
`claude-code.headless.stream-json.v1` and
`claude-code.response-only.stream-json.v1`. Keep AllowUnverified and the
empty response-only deny-list.

Raise latest qualified from `2.1.235` to `2.1.238` on both existing
segments: headless `2.1.220..=2.1.238`, response-only
`2.1.227..=2.1.238`. Qualify published intermediates `2.1.236` and
`2.1.237`. After qualification, synthetic later-stable UnverifiedNewer
is `2.1.239`.

Do not flatten onto Claude Agent ACP. No new public operation. No
provider prompt.

## Sources

- npm `@anthropic-ai/claude-code@2.1.236`, `@2.1.237`, `@2.1.238`
- [CHANGELOG.md](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md)
- frozen `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.235/`
