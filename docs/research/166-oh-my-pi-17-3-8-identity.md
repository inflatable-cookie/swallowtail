# 166 Oh My Pi 17.3.8 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 320

## Question

After g03.103 qualified Kimi Code through `0.37.2`, is official npm
`@oh-my-pi/pi-coding-agent` `17.3.8` a compatible extension of
`oh-my-pi.package` through `17.3.7`, a new milestone, or a stop?

## Remaining AllowUnverified rank

Kimi is done. Remaining Research 159 families:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Oh My Pi RPC | `omp/17.2.15` | through `17.3.7` | named next; host sits on a qualified bound; official latest is `17.3.8` |
| 2 | Antigravity | registry `1.1.15` | `1.1.9..=1.1.14` | later family |

Gemini stays deferred. Do not flatten `oh-my-pi.package` onto `pi.package`.

## Method

Compared npm `@oh-my-pi/pi-coding-agent@17.3.8`, host `omp --version` /
`--help` at `17.2.15`, extracted `17.3.8` `dist/cli.js` selected-flag and
RPC command strings, GitHub `docs/rpc.md` at tags `v17.3.7` and
`v17.3.8`, the frozen `17.2.9` RPC corpus, the frozen `17.3.7` identity
corpus, and the production `oh-my-pi.package` claim.

No provider prompt, host install, update, or claim edit in this research
card.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `17.3.8` (published 2026-08-19T11:23:31.769Z) |
| npm integrity | `sha512-0Qc25+97SREzKJcSYMw434/kZKFFKxWW9WZV5i9S3m+SNtQ6K1tigHBMLM9PhUM7fr2grpyGXS3asnd7owTq6Q==` |
| npm shasum | `137eb21be3ec4c1e606899802c363be4c7eecab6` |
| GitHub tag | `v17.3.8` at `858f7dd91fff9b84cf8a2c6a6bb85aa0e6d03a55` |
| host CLI | `omp/17.2.15` |
| host executable SHA-256 | `60a12d6c14d4877efeef9e6cb86de3ba84e39be59e2e43204b09dbdd75386020` |
| host size | 12394519 |
| extracted `17.3.8` `dist/cli.js` SHA-256 | `5fef4bf7186d34a42e0ab694e830f178b7c54a089201da075f8a96db660d9c29` |
| extracted size | 12573701 |
| `docs/rpc.md` blob | identical at `v17.3.7` and `v17.3.8` (`500caf0b4876868c19c5f9ca400d02650ef358e6`) |

Published stables after previous ceiling `17.3.7`: `17.3.8`. npm still
has no `17.3.6` and no `17.3.9`. Not `@earendil-works/pi-coding-agent`.

Extracted `--version` was not executed: the unpacked tarball fails
without `@oh-my-pi/pi-natives`. Package identity is npm metadata plus
`package.json` `17.3.8` and selected-flag strings inside `dist/cli.js`.
The host install stayed `17.2.15`.

## Selected protocol

Selected Swallowtail argv flags are still documented on host `17.2.15`
help and present as strings in extracted `17.3.8` `dist/cli.js`:

- `--mode rpc` (`text`, `json`, `rpc`, `rpc-ui`)
- `--no-session`
- `--provider`
- `--model`
- `--tools read,grep,glob,todo,ask`
- `--no-extensions`
- `--no-skills`
- `--no-rules`
- `--no-prewalk`
- `--approval-mode always-ask`
- catalogue path `--no-tools`

Selected RPC commands remain documented in unchanged `docs/rpc.md` and
present in the extracted CLI: `negotiate_protocol` v2, `set_model` /
`get_state` / `get_available_models`, `set_auto_retry` /
`set_auto_compaction`, `set_steering_mode` / `set_follow_up_mode` /
`set_interrupt_mode`, `set_thinking_level`, `prompt` / `steer` /
`follow_up` / `abort`.

Changelog extras at `17.3.8` stay unmapped: `providers.cacheRetention`,
read-tool single-pass materialize, `bash.patterns` eval note, advisor,
`omp acp`, `--from-claude` / `--from-codex`, host-tool injection, session
switching, and subagent authority.

Ready-frame v1/v2 advertisement, 1 MiB physical cap, and 64 MiB
reassembly cap remain the frozen `17.2.9` decoder contract. This card did
not send a provider prompt.

## Segment decision for card 321

Compatible extension. Same axis `oh-my-pi.package`. Same behavior
`oh-my-pi.rpc-v2-v17.2.9`. Keep AllowUnverified.

Raise latest qualified from `17.3.7` to exact `17.3.8` on the existing
segment, so `17.2.9..=17.3.8` is Maintained. Keep unpublished `17.3.6`.
After qualification, synthetic later-stable UnverifiedNewer is `17.3.9`.

Do not mix `pi.package` into this card. No new public operation. No
provider prompt. No host install or update.

## Sources

- host `omp --version` on 2026-08-19
- npm `@oh-my-pi/pi-coding-agent@17.3.8`
- [GitHub `v17.3.8`](https://github.com/can1357/oh-my-pi/releases/tag/v17.3.8)
- frozen `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-17.3.7/`
- frozen `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-rpc-17.2.9/`
