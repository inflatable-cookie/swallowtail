# 2026-09-01 Claude Code 2.1.257 Claim

## Result

Raised `claude-code.headless-stream-json` through exact `2.1.257`
(`2.1.220..=2.1.257`) and `claude-code.response-only-stream-json` through
exact `2.1.257` (`2.1.227..=2.1.257`) as compatible extensions of the
existing stream-JSON behaviors. No published intermediate sits between
`2.1.252` and `2.1.257`. Unpublished `2.1.244` and `2.1.249` stay
incompatible. Hop-skipped unpublished `2.1.253` through `2.1.256` are
now gaps. No new milestone. Later stables stay AllowUnverified; the
synthetic later point is unpublished `2.1.258`. Unused help and changelog
surfaces stay unmapped. Maximum-turn and other feature-specific exact
version sets stay on the `2.1.220..=2.1.241` probed points. Claude Agent
ACP remains a separate axis. Watcher stays exact `2.1.251` and is not
live-ready. Official `2.1.257` is rejected at both watcher admission
seams.

Research 273 was recorded as identity-only commit `de3b94a9` before these
claim edits. The current host remains exact `2.1.257` and matches official
darwin-arm64. No host change was made.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check` passed
- `effigy validate:focused swallowtail-adapter-claude-agent` passed: 211 tests
- `effigy package:verify-affected swallowtail-adapter-claude-agent` passed
- `effigy package:api` passed: 40 packages at v0.3.3
- `effigy qa:routes` passed: 48 production routes
- `effigy qa:northstar` passed
- research, logs, roadmaps, g05, batch-card, and next-action indexes passed
- `effigy --json scan god-files` passed: 379 findings held
- `git diff --check` passed
- official npm `latest` remained `2.1.257` through claim closeout
  (published 2026-09-01T17:15:33.223Z); `2.1.253`–`2.1.256` and `2.1.258`
  stayed unpublished

No provider prompt, live session, authentication, install, or host update.

## Next

Reassess remaining AllowUnverified currentness families. Do not start a
second family from this PR. g05.009 remains queued behind the
provider-operation observation decision.
