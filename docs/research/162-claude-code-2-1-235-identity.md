# 162 Claude Code 2.1.235 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 312

## Question

After g03.099 qualified Claude Agent ACP through `0.70.0`, is official
Claude Code `2.1.235` a compatible extension of the headless
`2.1.220..=2.1.234` and response-only `2.1.227..=2.1.234` claims, a new
milestone, or a stop? Headless and response-only stay one family.

## Remaining AllowUnverified rank

Claude Agent ACP is done. Remaining Research 159 families, host still on a
qualified bound unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Code | `2.1.235` | headless `2.1.220..=2.1.234`; response-only `2.1.227..=2.1.234` | named next; host and official sit above the ceiling; one family |
| 2 | Grok Build | `1.0.5` | exact `1.0.4` | later family; ignore alpha `1.0.6` |
| 3 | Qwen headless | registry `0.21.14` | through `0.21.13` | later family |
| 4 | Kimi Code | host `0.34.0` | through `0.36.1` | later family |
| 5 | Oh My Pi | registry `17.3.8` | through `17.3.7` | later family |
| 6 | Antigravity | registry `1.1.15` | `1.1.9..=1.1.14` | later family |

Gemini stays deferred. Do not flatten this family onto Claude Agent ACP.
Do not split headless and response-only into separate currentness runs.

## Method

Compared npm `@anthropic-ai/claude-code@2.1.235`, local `claude --version`,
root `--help` selected flags, the frozen `2.1.234` identity corpus, the
`2.1.220` headless decoder specimen, the `2.1.227`/`2.1.228` response-only
specimens, and GitHub `CHANGELOG.md` for `2.1.235`.

No provider prompt, install, update, or live stream.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `2.1.235` (published 2026-08-18T18:24:10.210Z) |
| npm integrity | `sha512-poJ4l/nro9NEZEoLU1txUGMMw92m5P3o6Nh86GfaQuPryvOdKIz/ChlPaq3FDetiaXmoNit3ZkEgnQ1PN7z/dQ==` |
| npm shasum | `c82eb033efe6148a49edad03948249ad9299ed57` |
| npm tarball SHA-256 | `a048adbd153529ef4137ffb945580080e716e6a8b320baa73b6a5777e31f6e34` |
| local CLI | `2.1.235 (Claude Code)` |
| local executable SHA-256 | `83b8f806f6f2eea316cfe246628e6c23374711d868f1fd0409db551b877b7748` |
| local size | 313334608 |
| Agent SDK latest | `@anthropic-ai/claude-agent-sdk@0.3.235` (published 2026-08-18T18:25:11.534Z) |
| frozen SDK pin | `@anthropic-ai/claude-agent-sdk@0.3.220` (the `2.1.220` corpus) |
| Research 159 host | already `2.1.235`; npm already `2.1.235` |

Published stables after previous ceiling `2.1.234`: `2.1.235` only.
First unpublished later stable is `2.1.236`. Version parse still requires
exactly `<semver> (Claude Code)` plus one trailing newline.

The npm package remains an installer wrapper. `cli-wrapper.cjs` and
`install.cjs` match `2.1.234` aside from the version pin.
`sdk-tools.d.ts` added unmapped artifact `capabilities` and `contract`
fields. That is not selected stream-JSON.

## Protocol comparison

Selected Swallowtail argv flags remain documented with the same choices:

- `-p` / `--print`
- `--input-format text` (`text`, `stream-json`)
- `--output-format stream-json` (`text`, `json`, `stream-json`)
- `--verbose`
- `--no-session-persistence`
- `--model`
- `--effort` (`low`, `medium`, `high`, `xhigh`, `max`)
- `--permission-mode plan` (`acceptEdits`, `auto`, `bypassPermissions`,
  `manual`, `dontAsk`, `plan`)
- `--tools Read,Glob,Grep` (headless) or `--tools ""` (response-only)
- `--setting-sources user,project,local` (headless)
- `--safe-mode`, `--disable-slash-commands`, `--no-chrome`,
  `--prompt-suggestions false` (response-only)
- `--mcp-config` and `--strict-mcp-config`

`--prompt-suggestions` still documents a print/SDK `prompt_suggestion`
message when enabled. Selected argv still sets `false`.

Help still documents unused flags including `--bare`, `--brief`,
`--cloud`, `--include-hook-events`, `--include-partial-messages`, and
`--forward-subagent-text`. Those stay unused.

Changelog `2.1.235`: optional interactive `spellcheck` setting, terminal
UI and permission-dialog fixes, cloud-session CPU, embedded grep, and
Agent-tool default-agent errors. None of those are selected mapped
stream-JSON. Spellcheck is a setting, not a selected flag.

g03.068's fail-closed response-only validator still applies. This card
did not capture a live stream.

## Segment decision for card 313

Compatible extension on both axes. Same behaviors
`claude-code.headless.stream-json.v1` and
`claude-code.response-only.stream-json.v1`. Keep AllowUnverified and the
empty response-only deny-list.

Raise latest qualified from `2.1.234` to `2.1.235` on both existing
segments: headless `2.1.220..=2.1.235`, response-only
`2.1.227..=2.1.235`. After qualification, synthetic later-stable
UnverifiedNewer is `2.1.236`.

Do not flatten onto Claude Agent ACP. No new public operation. No
provider prompt.

## Sources

- host `/Users/tom/.local/bin/claude` → `.../versions/2.1.235`
- npm `@anthropic-ai/claude-code@2.1.235`
- [CHANGELOG.md](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md)
- frozen `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.234/`
