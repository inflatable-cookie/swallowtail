# 178 Oh My Pi 17.4.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-21
Card: g04 batch 082

## Question

Is official npm `@oh-my-pi/pi-coding-agent` `17.4.0` a compatible
extension of `oh-my-pi.package` through `17.3.8`, a private-milestone, or
a stop? This is a published minor-line step, not a patch.

## Remaining AllowUnverified rank

Assigned family only. Extracurricular currentness; not a generation
rank rewrite.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Oh My Pi RPC | not installed | through `17.3.8` | named family; official latest is `17.4.0` |

Gemini stays deferred. Do not flatten `oh-my-pi.package` onto
`pi.package` (`@earendil-works/pi-coding-agent` remains `0.84.2`).

## Method

Compared npm `@oh-my-pi/pi-coding-agent@17.4.0`, extracted `17.4.0`
`dist/cli.js` selected-flag and RPC command strings, GitHub `docs/rpc.md`
and mapped RPC sources at tags `v17.3.8` and `v17.4.0`, the frozen
`17.2.9` RPC corpus, the frozen `17.3.8` identity corpus, and the
production `oh-my-pi.package` claim.

Private-milestone was checked before compatible-extension: adapter
mapping would change only if selected argv, RPC commands, framing, or
decoded response fields changed. They did not.

No provider prompt, host install, update, or claim edit in this research
card.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `17.4.0` (published 2026-08-20T06:42:13.785Z) |
| npm integrity | `sha512-RMLu7DrF/W2lEPNgQECGR1Uw6jbhAKnDUVGGhhRXvVPp3ntx8CCwW48aC2kfp5QV/lDFYg0Rw6/CXMo/85jIBw==` |
| npm shasum | `557a9343748e8720b0600f95779c11ad7f447575` |
| GitHub tag | `v17.4.0` at `72000acfeb902e21816252699482887f34d1a5a4` |
| host CLI | not installed |
| extracted `17.4.0` `dist/cli.js` SHA-256 | `1e023799891c51f6efea97b78aaf97dc6623b48b559dfd873caf8364a032f49c` |
| extracted size | 17173733 |
| `docs/rpc.md` blob | identical at `v17.3.8` and `v17.4.0` (`500caf0b4876868c19c5f9ca400d02650ef358e6`) |

Mapped RPC sources identical at both tags:

- `packages/coding-agent/src/modes/rpc/rpc-types.ts`
- `packages/coding-agent/src/modes/rpc/rpc-mode.ts`
- `packages/coding-agent/src/modes/rpc/rpc-messages.ts`
- `packages/coding-agent/src/modes/rpc/rpc-frame.ts`
- `packages/coding-agent/src/modes/rpc/rpc-client.ts`
- `packages/coding-agent/src/jsonrpc/message-framing.ts`

Published stables after previous ceiling `17.3.8`: `17.4.0` only. npm
still has no `17.3.6`, no `17.3.9`, and no `17.4.1`. Not
`@earendil-works/pi-coding-agent`.

Extracted `--version` was not executed: the unpacked tarball fails
without `@oh-my-pi/pi-natives`. Package identity is npm metadata plus
`package.json` `17.4.0` and selected-flag strings inside `dist/cli.js`.

## Selected protocol

Selected Swallowtail argv flags are present as strings in extracted
`17.4.0` `dist/cli.js`:

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

Changelog extras at `17.4.0` stay unmapped: `pi-agent-core` Tokenizer JS
API, `omp ps`, `/cleanse`, `extendedContext` / `/extended-context`,
`compaction.methodOrder`, speculative compaction, composer layouts, and
context-line gauges.

`cli-commands.ts` added unmapped `ps`. `launch-help.ts` changed only an
unmapped External Thinking description. Ready-frame v1/v2 advertisement,
1 MiB physical cap, and 64 MiB reassembly cap remain the frozen `17.2.9`
decoder contract. This card did not send a provider prompt.

## Segment decision for card 083

Compatible extension. Private-milestone checked: adapter-private mapping
unchanged. Same axis `oh-my-pi.package`. Same behavior
`oh-my-pi.rpc-v2-v17.2.9`. Keep AllowUnverified.

Raise latest qualified from `17.3.8` to exact `17.4.0` on the existing
segment, so `17.2.9..=17.4.0` is Maintained. Keep unpublished `17.3.6`
and unpublished `17.3.9`. After qualification, synthetic later-stable
UnverifiedNewer is `17.4.1`.

Do not mix `pi.package` into this card. No new public operation. No
provider prompt. No host install or update.

## Sources

- npm `@oh-my-pi/pi-coding-agent@17.4.0`
- [GitHub `v17.4.0`](https://github.com/can1357/oh-my-pi/releases/tag/v17.4.0)
- frozen `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-17.3.8/`
- frozen `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-rpc-17.2.9/`
