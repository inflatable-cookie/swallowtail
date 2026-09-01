# 2026-09-01 Claude Agent ACP 0.73.0 Claim

## Result

Raised `claude-agent.acp-adapter` through exact official `0.73.0`
(`0.66.0..=0.73.0`) as a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`. Operator restart after
official latest moved during the unmerged `0.72.0` family. No new
milestone. Published intermediates `0.71.0`, `0.72.0`, and `0.73.0` are
qualified. Unpublished `0.58.0` stays incompatible. Later stables stay
AllowUnverified; the synthetic later point is unpublished `0.74.0`.
`#1004`/`#1045` keep mapped mode ids/categories, `plan`/`acceptEdits`,
steering unmapped, and the permission callback observable contract.
`0.72.0` effort, result attribution, PostModelSwitch, and PreModelSwitch
stay classified and selected-compatible. `0.73.0` changes only
`package.json` (version plus Agent SDK pin `0.3.252`→`0.3.257`); every
`dist/**` file is byte-identical to `0.72.0`. Additive
`sessionCapabilities.subagents`, native subagent/async-task updates,
session titles, Providers API, ACP SDK `1.4.0` elicitation rename, and the
Agent SDK pin stay unmapped. Claude Code and the watcher stay separate.

Research 272 was recorded as identity-only commit `af9ddfd4` before these
claim edits. The current host remains exact `0.63.0`. No host change was
made.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check` passed
- `effigy validate:focused swallowtail-adapter-claude-agent` passed: 200 tests
- `effigy package:verify-affected swallowtail-adapter-claude-agent` passed
- `effigy package:api` passed: 40 packages at v0.3.3
- `effigy qa:routes` passed: 48 production routes
- `effigy qa:northstar` passed
- research, logs, roadmaps, g05, batch-card, and next-action indexes passed
- `effigy --json scan god-files` passed: 381 findings held
- `git diff --check` passed
- identity freeze and claim recheck recorded official npm `latest` = `0.73.0`
  published 2026-09-01T20:27:53.428Z; GitHub `v0.73.0` target
  `ea7076c0bc324603e65d8c124b7573f158749969`. `0.58.0` stayed unpublished.
  First later unpublished stable is `0.74.0`

No provider prompt, live session, authentication, install, or host update.

## Next

Implement Claude Code useful-newer qualification for official `2.1.257`.
Watcher stays exact `2.1.251`. Do not start a second family from this PR.
Do not reopen `kimi-code.acp`.
