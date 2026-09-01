# 2026-09-01 Claude Code 2.1.252 Claim

## Result

Raised `claude-code.headless-stream-json` through exact `2.1.252`
(`2.1.220..=2.1.252`) and `claude-code.response-only-stream-json` through
exact `2.1.252` (`2.1.227..=2.1.252`) as compatible extensions of the
existing stream-JSON behaviors. No published intermediate sits between
`2.1.251` and `2.1.252`. Unpublished `2.1.244` and `2.1.249` stay
incompatible. No new milestone. Later stables stay AllowUnverified; the
synthetic later point is unpublished `2.1.253`. Unused help and changelog
surfaces stay unmapped. Maximum-turn and other feature-specific exact
version sets stay on the `2.1.220..=2.1.241` probed points. Claude Agent
ACP remains a separate axis. Watcher stays exact `2.1.251` and is not
live-ready.

Research 266 was recorded as identity-only commit `967cd0f3` before these
claim edits. The current host remains exact `2.1.251`. No host change was
made.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check` passed
- `effigy validate:focused swallowtail-adapter-claude-agent` passed: 189 tests
- `effigy package:verify-affected swallowtail-adapter-claude-agent` passed
- `effigy qa:routes` passed: 48 production routes
- `effigy qa:northstar` passed
- research, logs, roadmaps, g05, batch-card, and next-action indexes passed
- `git diff --check` passed
- official npm `latest` remained `2.1.252` through closeout; `2.1.253` stayed unpublished

No provider prompt, live session, authentication, install, or host update.

## Next

Reassess remaining AllowUnverified currentness families. Do not start a
second family from this PR. g05.009 remains queued behind the
provider-operation observation decision.
