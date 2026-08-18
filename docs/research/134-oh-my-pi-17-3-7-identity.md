# 134 Oh My Pi 17.3.7 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 242

## Question

After g03.076 qualified Claude Code response-only through `2.1.234`, which
AllowUnverified family should move first, and is npm `@oh-my-pi/pi-coding-agent`
`17.3.7` a compatible extension of exact `oh-my-pi.package` `17.2.9`, a new
milestone, or a stop?

## Remaining AllowUnverified rank

Claude Code headless and response-only are done. Remaining host-drifted
families, Research 127 numbers unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Oh My Pi RPC | `omp/17.2.15` | exact `17.2.9` | named next after response-only; host and npm both above the pin |
| 2 | Cursor Agent | `2026.08.04-aaa8809` | July exact points | later family |
| 3 | OpenCode HTTP | `1.18.18` | through `1.18.10` | later family |
| 4 | Kimi ACP / headless / local-server | `0.34.0` | through `0.31.1` | later family |
| 5 | Ollama attached | `0.32.9` | `0.14.0..=0.32.1` excluding `0.32.2` | later family |

Host still on a qualified bound (registry newer only): Claude Agent ACP,
Pi, Qwen, Antigravity. Rank those after host-drifted families.

Gemini stays deferred. Do not flatten `oh-my-pi.package` onto `pi.package`.

Research 127 already classified Oh My Pi as visible unverified-newer:
host `omp/17.2.15`, npm then `17.3.5`, exact qualified `17.2.9`. This card
asks whether selected RPC evidence now justifies moving the qualified
boundary. Official npm `latest` moved to `17.3.7` on 2026-08-18. Leaving
that point UnverifiedNewer would skip useful-newer support.

## Method

Compared npm `@oh-my-pi/pi-coding-agent@17.3.7`, host `omp --version` /
`--help` at `17.2.15`, extracted `17.3.7` `dist/cli.js` selected-flag and
RPC command strings, GitHub `docs/rpc.md` at tag `v17.3.7`, the frozen
`17.2.9` RPC corpus, and the production `oh-my-pi.package` claim.

No provider prompt, host install, update, or claim edit in this research
card.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `17.3.7` (published 2026-08-18T08:51:09.945Z) |
| npm integrity | `sha512-z2W77ThFqtKP9P+wqISCtjMZFUZBNbR3jddZ0odgpBPRzNeORpcVVWbyhVGLsqXRWB3YQP2vYtxy+ohsnhG1+A==` |
| npm shasum | `023e762e42dca3b1f60aa3d03e690e8f228743ba` |
| GitHub tag | `v17.3.7` at `8500092296621a6826b7136e840f8a59ea338958` |
| host CLI | `omp/17.2.15` |
| host executable SHA-256 | `60a12d6c14d4877efeef9e6cb86de3ba84e39be59e2e43204b09dbdd75386020` |
| host size | 12394519 |
| extracted `17.3.7` `dist/cli.js` SHA-256 | `d00ede5c46eb34c1a20a892209cf35424bb0a7a4233605f5e7bfd8c582f70050` |
| extracted size | 12540811 |
| Research 127 npm | `17.3.5`; no npm `17.3.6` |

Version parse still requires exactly `omp/<semver>` plus one trailing
newline.

Extracted `17.3.7 --version` was not executed: the unpacked tarball fails
without `@oh-my-pi/pi-natives`. Package identity is npm metadata plus
`package.json` `17.3.7` and selected-flag strings inside `dist/cli.js`.
The host install stayed `17.2.15`.

## Protocol comparison

Selected Swallowtail argv flags are still documented on host `17.2.15`
help and present as strings in extracted `17.3.7` `dist/cli.js`:

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

Selected RPC commands remain documented in `v17.3.7` `docs/rpc.md` and
present in the extracted CLI:

- `negotiate_protocol` v2
- `set_model` / `get_state` / `get_available_models`
- `set_auto_retry` / `set_auto_compaction`
- `set_steering_mode` / `set_follow_up_mode` / `set_interrupt_mode`
- `set_thinking_level`
- `prompt` / `steer` / `follow_up` / `abort`

Help and RPC docs also name surfaces the selected command does not pass,
including `--advisor`, `--from-claude`, `--from-codex`, `omp acp`,
`set_host_tools`, `switch_session`, `branch`, and subagent commands.
Those are unused deltas, not selected-protocol changes. Swallowtail still
omits write tools, host-tool injection, session switching, and subagent
authority.

Ready-frame v1/v2 advertisement, 1 MiB physical cap, and 64 MiB
reassembly cap remain the frozen `17.2.9` decoder contract. This card did
not send a provider prompt.

## Segment decision for card 243

Compatible extension. Same axis `oh-my-pi.package`. Same behavior
`oh-my-pi.rpc-v2-v17.2.9`. Keep AllowUnverified.

Raise latest qualified from exact `17.2.9` to exact `17.3.7` on the
existing segment, so `17.2.9..=17.3.7` is Maintained. Intermediates
including host `17.2.15` and former npm `17.3.5` become qualified as a
compatible ceiling-raise, not a Grok-style gap. npm has no `17.3.6`.
After qualification, synthetic later-stable UnverifiedNewer is `17.3.8`.

Do not mix `pi.package` into this card. No new public operation. No
provider prompt. No host install or update.
