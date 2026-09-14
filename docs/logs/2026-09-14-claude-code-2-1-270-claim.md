# 2026-09-14 Claude Code 2.1.270 Claim

## Result

Raised `claude-code.headless-stream-json` from `2.1.220..=2.1.257` to
`2.1.220..=2.1.270` and `claude-code.response-only-stream-json` from
`2.1.227..=2.1.257` to `2.1.227..=2.1.270` as compatible extensions of
`claude-code.headless.stream-json.v1` and
`claude-code.response-only.stream-json.v1`. All eleven published hops after
the previous ceiling are qualified: `2.1.258`, `2.1.259`, `2.1.260`,
`2.1.261`, `2.1.263`, `2.1.265`, `2.1.266`, `2.1.267`, `2.1.268`, `2.1.269`,
and `2.1.270`.

Baselines `2.1.220` and `2.1.227`, claim ids
`claude-code.headless.window-1` and `claude-code.response-only.window-1`,
behavior revisions, `AllowUnverified`, and every historical segment and
boundary stay unchanged. Unpublished `2.1.244`, `2.1.249`, hop-skipped
`2.1.253` through `2.1.256`, and the newly observed unpublished `2.1.262`
and `2.1.264` stay incompatible. Synthetic later stable `2.1.271` is the
visible `UnverifiedNewer` point.

Wrapper files except `package.json` and `sdk-tools.d.ts` are byte-identical
across all twelve compared versions, and every `sdk-tools.d.ts` delta is SDK
tool declaration content neither route consumes. Selected mapped stream-JSON
flags, format, effort, and permission enumerations, the wire permission
spelling, and the normalized `init`, `stream_event`, `hook_started`, `result`,
and `thinking_tokens` constructions are unchanged on both platform builds.
Watcher stays exact `2.1.251` and every feature-specific exact-version set
stays on the `2.1.220..=2.1.241` probed points. Host `2.1.258` matches the
official darwin-arm64 binary and its help equals the frozen `2.1.257` digest;
it stayed observation-only.

No public API, provider contact, login, install, host update, release, tag,
publication, or consumer change entered the claim. Research 307. g05.058.

## Validation

`cargo fmt -p swallowtail-adapter-claude-agent -- --check`,
`effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy check:examples`, `effigy package:api`, `effigy qa:routes`,
`effigy qa:northstar`, the research, logs, roadmaps, g05, roadmap-number,
lifecycle, and next-action checks, and `git diff --check`.

## Review And Merge

Independent exact-head review and the merge gate are queue-owned.
