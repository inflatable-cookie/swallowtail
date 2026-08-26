# 217 Oh My Pi 18 Identity

Status: promoted
Owner: Tom
Date: 2026-08-26
Card: g04 batch 194

## Question

Is official npm `@oh-my-pi/pi-coding-agent` `18.0.5` a compatible
extension of `oh-my-pi.package` through `17.4.0`, a private-milestone
that can compile without a new public operation, a new-driver-or-facade,
or a stop? This is a 17→18 major-line reset, not a default
UnverifiedNewer bump.

## Remaining AllowUnverified rank

Assigned family only. Standing currentness; not a generation rank
rewrite.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Oh My Pi RPC | not installed | through `17.4.0` | named family; assigned official is `18.0.5`; observed npm `latest` is `18.0.6` |

Gemini stays deferred. Do not flatten `oh-my-pi.package` onto
`pi.package` (`@earendil-works/pi-coding-agent` remains `0.84.3`).

## Method

Compared npm `@oh-my-pi/pi-coding-agent@18.0.5`, extracted `18.0.0` /
`18.0.5` / `18.0.6` `dist/cli.js` selected-flag and RPC command strings,
GitHub `docs/rpc.md` and mapped RPC sources at tags `v17.4.0`, `v18.0.0`,
`v18.0.5`, and `v18.0.6`, the frozen `17.2.9` RPC corpus, the frozen
`17.4.0` identity corpus, and the production `oh-my-pi.package` claim.

Private-milestone was checked before compatible-extension: adapter
mapping would change only if selected argv, RPC commands, framing, or
decoded response fields changed. Selected flags and commands remain.
Framing is unchanged. The 18.0.0 extras are unused fields the decoder
already ignores.

No provider prompt, host install, update, or claim edit.

## Identity

| Fact | Value |
| --- | --- |
| assigned official | `18.0.5` (published 2026-08-25T16:45:58.711Z) |
| observed npm latest | `18.0.6` (published 2026-08-26T08:28:41.651Z) |
| npm `18.0.5` integrity | `sha512-4bDndTceC6R5gFLS+FnkSiDBrlVbAt2EjL9ca4K29Qd5R+fpxOaad3dOQSenKXd1y3Ot/MfoNGrfH2dXr5hpSA==` |
| npm `18.0.5` shasum | `415d8e183449fea482382268fcc4b8063bfd04e8` |
| GitHub tag | `v18.0.5` at `eab72e88e447a4be45bea2bc302995844c0c51a2` |
| host CLI | not installed |
| extracted `18.0.5` `dist/cli.js` SHA-256 | `3edd2768e2ace4fdc034c8c2f8579d8e21c97954e2605ace51e86283a8be9651` |
| extracted size | 19316793 |
| extracted tarball SHA-256 | `7d9745c2a3cfa4cec84363b952e8a7a649f9906021d22aa5b415c4bc6fbaf6f0` |
| `docs/rpc.md` blob | `v17.4.0` `500caf0b4876868c19c5f9ca400d02650ef358e6`; `v18.0.0` / `v18.0.5` / `v18.0.6` `310b44702de66b4eecd6da37660f9fc40075d973` |

Mapped RPC sources identical at `v17.4.0` and `v18.0.5`:

- `packages/coding-agent/src/modes/rpc/rpc-client.ts`
- `packages/coding-agent/src/modes/rpc/rpc-frame.ts`
- `packages/coding-agent/src/modes/rpc/rpc-messages.ts`
- `packages/coding-agent/src/jsonrpc/message-framing.ts` (extracted npm copies)

Changed at `18.0.0`, then identical through extracted `18.0.5` and
`18.0.6`:

- `rpc-types.ts` — optional select `optionDetails`
- `rpc-mode.ts` — `requestRpcSelect`, builtin `agentInvoked` accuracy,
  unused `runCommandInBackground`

Published stables after previous ceiling `17.4.0`: `17.4.1`, `17.4.2`,
`18.0.0`, `18.0.1`, `18.0.3`, `18.0.4`, `18.0.5`. Observed later official
`18.0.6`. npm still has no `18.0.2` (GitHub tag `v18.0.2` exists). No
npm `17.4.3` / `17.4.4`. Not `@earendil-works/pi-coding-agent`.

Extracted `--version` was not executed: the unpacked tarball fails
without `@oh-my-pi/pi-natives`. Package identity is npm metadata plus
`package.json` `18.0.5` and selected-flag strings inside `dist/cli.js`.

## Selected protocol

Selected Swallowtail argv flags are present as strings in extracted
`18.0.5` `dist/cli.js`:

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

Selected RPC commands remain documented and present: `negotiate_protocol`
v2, `set_model` / `get_state` / `get_available_models`, `set_auto_retry`
/ `set_auto_compaction`, `set_steering_mode` / `set_follow_up_mode` /
`set_interrupt_mode`, `set_thinking_level`, `prompt` / `steer` /
`follow_up` / `abort`.

`docs/rpc.md` at `18.x` documents optional select `optionDetails`. Hosts
that do not render descriptions keep using `options` alone. Swallowtail's
UI decoder already ignores unknown select fields. `prompt` success still
only reads `success`; it does not consume `agentInvoked`.

18.0.0 changelog extras stay unmapped: `omp render`, typo detection,
`/shake thinking`, TUI autocomplete. Later `18.0.5` extras stay
unmapped: `omp if-bench`, Yolo-Auto login, OpenRouter browser sign-in.

Ready-frame v1/v2 advertisement, 1 MiB physical cap, and 64 MiB
reassembly cap remain the frozen `17.2.9` decoder contract. This card
did not send a provider prompt.

## Segment decision for card 194

Stop for this run. Official latest moved: assigned `18.0.5`, observed
npm `latest` `18.0.6`. `18.0.5` stays unqualified. No silent inheritance
of `17.2.9..=17.4.0` onto that assigned point. No claim card.

This freeze does not name the future 18.x segment. Contract 029 says a
retained product's major-line reset normally becomes a same-axis
milestone after corpus evidence; the skill requires operator input on
that reset. Exact-current `18.0.6` needs a fresh identity investigation
and that operator decision.

Selected extras on the assigned `18.0.5` extract are unused. They are
not a new public operation and not a flatten onto Pi RPC. That corpus
note is evidence for the later card, not a segment close.

Same-line `17.4.1` and `17.4.2` can be a later 17.x useful-newer card,
not this one. GitHub `v17.4.1` `rpc-types.ts` still matches `v17.4.0`.
GitHub `v17.4.2` already carries the `18.0.0` `optionDetails` type;
that later card must extract the npm tarballs, not assume the tags.

Keep unpublished `18.0.2`. Keep decoder specimen `oh-my-pi-rpc-17.2.9`.
Do not mix `pi.package`. No provider prompt. No host install or update.

## Sources

- npm `@oh-my-pi/pi-coding-agent@18.0.5` and observed `latest` `18.0.6`
- [GitHub `v18.0.5`](https://github.com/can1357/oh-my-pi/releases/tag/v18.0.5)
- frozen `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-17.4.0/`
- frozen `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-rpc-17.2.9/`
