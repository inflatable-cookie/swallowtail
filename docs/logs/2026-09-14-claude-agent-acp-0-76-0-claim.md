# 2026-09-14 Claude Agent ACP 0.76.0 Claim

## Result

Raised `claude-agent.acp-adapter` through exact official `0.76.0`
(`0.66.0..=0.76.0`) as a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`. First family of Tom's
authorized Research 308 campaign. No new milestone. Published hops `0.74.0`,
`0.75.0`, `0.75.1`, and `0.76.0` are qualified. Unpublished `0.58.0`,
`0.73.1`, `0.74.1`, `0.75.2`, and `0.76.1` stay incompatible;
unpublished `0.77.0` is the synthetic later `UnverifiedNewer` point. The ACP
SDK pin stays `1.4.0` and the Agent SDK pin stays `0.3.257` at every hop, and
`dist/index.js`, `dist/elicitation.js`, `dist/settings.js`, `dist/utils.js`,
`dist/tools.js`, `dist/session-mode.js`, `dist/session-config-ids.js`, and
the complete `dist/permissions/**` tree are byte-identical across every hop,
so mode ids/categories, `plan`/`acceptEdits`, permission option kinds, and
the effort config id are unchanged. The `--hide-claude-auth` guard, the
`authStatus` push extension, the synthetic context-compaction tool call,
usage Markdown, AIR fork metadata, clear-context coordination, and
capability-gated recommended config values stay unmapped with reasons.
Baseline `0.53.0`, claim id `claude-agent.acp.window-2`, exclusion `0.58.0`,
every historical segment, and `AllowUnverified` survive. Host `0.63.0` stays
observation-only Qualified Deprecated. Claude Code, the Claude Agent SDK
sidecar, and the watcher are untouched. Research 309 was recorded as
identity-only commit `8a9f367b` before these claim edits.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check` passed
- `effigy validate:focused swallowtail-adapter-claude-agent` passed: 535 tests
- `effigy package:verify-affected swallowtail-adapter-claude-agent` passed
- `effigy check:examples` passed
- `effigy package:api` passed: 40 packages at v0.5.1
- `effigy qa:routes` passed: 50 production routes
- `effigy qa:northstar` passed
- research, logs, roadmaps, g05, and next-action indexes passed
- `effigy qa:docs:roadmaps:numbers` passed against canonical `main` `54d110f0`
- `git diff --check` passed
- official latest was rechecked immediately before the identity commit and
  again before push: npm `latest` `0.76.0` (preview `0.76.1-preview.2`),
  GitHub `v0.76.0` target `c2e4815029ef3962787ecaefe208b0b6f8b81302`, ACP
  registry `claude-acp` `0.76.0`. No in-run movement

One material observation: `claude_agent_sdk_driver::readiness::wrapper_death_preserves_partial_capture_journal`
failed once when the whole crate ran under full parallelism and passed on
five isolated reruns. It is the already-recorded cold-courier capture-budget
flake in `PAPERCUTS.md`, unrelated to this Claude Agent ACP change; the
focused package gate passed on its run.

No provider prompt, live ACP initialize, authentication, install, host
update, downloaded-artifact execution, release, tag, publication, or
consumer mutation.

## Next

The Research 308 campaign continues serially. Chatterbox compiles Qwen Code
headless as the next family from fresh canonical main without repeating the
operator authorization.
