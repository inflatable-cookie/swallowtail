# 2026-08-27 g04.083c Gemini CLI ACP Thinking Evidence

Status: complete
Card: 234
Research: 235

## Boundary

Evidence only. The worker updated this file, card 234, Research 235, and new
Gemini-local frozen evidence. Shared planning and production code stayed unchanged.

## Worktree

- path: `/Users/tom/.t3/worktrees/swallowtail/t3code-8b6e0278`
- branch: `t3code/review-gemini-cli-acp-evidence`
- base: `0808a6cf` (`origin/main` at dispatch)

## Outcome

Empty deliver-now set for `gemini-cli.acp` thinking configuration through
qualified `0.51.0..=0.56.0`.

ACP `initialize` and `session/new` expose auth, modes, and negotiated model ids.
They expose no `configOptions`, no `session/set_config_option`, and no thinking
vocabulary. Thinking applies only through settings-backed
`generateContentConfig.thinkingConfig` on the interactive `sendMessageStream`
path with `isChatModel: true`. `agent_thought_chunk` is observation during prompt
or history replay, not selected-value confirmation before prompt effects.

Swallowtail's prepared ACP route rejects portable `reasoning_mode`, keeps
ambient harness configuration, and injects no isolated settings env keys.

## Validation

```sh
effigy validate:focused swallowtail-adapter-gemini  # passed
effigy qa:northstar                                 # passed
git diff --check                                    # passed
```

## Evidence

- [Research 235](../research/235-gemini-cli-acp-thinking-evidence.md)
- [thinking-evidence.json](../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-acp-0.56.0-thinking/thinking-evidence.json)

## PR

https://github.com/inflatable-cookie/swallowtail/pull/87
