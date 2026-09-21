# 2026-09-21 Claude Code 2.1.278 Claim

## Result

Raised `claude-code.headless-stream-json` from `2.1.220..=2.1.270` to
`2.1.220..=2.1.278` and `claude-code.response-only-stream-json` from
`2.1.227..=2.1.270` to `2.1.227..=2.1.278` as compatible extensions of
`claude-code.headless.stream-json.v1` and
`claude-code.response-only.stream-json.v1`. All eight published hops after
the previous ceiling are qualified: `2.1.271`, `2.1.272`, `2.1.273`,
`2.1.274`, `2.1.275`, `2.1.276`, `2.1.277`, and `2.1.278`.

Baselines `2.1.220` and `2.1.227`, claim ids
`claude-code.headless.window-1` and `claude-code.response-only.window-1`,
behavior revisions, `AllowUnverified`, and every historical segment and
boundary stay unchanged. Unpublished `2.1.244`, `2.1.249`, `2.1.253` through
`2.1.256`, `2.1.262`, and `2.1.264` stay incompatible. Synthetic later
stable `2.1.279` is the visible `UnverifiedNewer` point.

Wrapper files except `package.json` and `sdk-tools.d.ts` are byte-identical
across all nine compared versions, and every `sdk-tools.d.ts` delta is SDK
tool declaration content neither route consumes. Selected mapped stream-JSON
flags, format, effort, and permission enumerations, the wire permission
spelling, required init keys, and stream-event/hook/result/thinking-token
presence are unchanged on both platform builds. `--verbose`/`--model`/
`--effort` hop-local fingerprints remangle bunfs chunk paths or minified
template ids only. Watcher stays exact `2.1.251` and every feature-specific
exact-version set stays on the `2.1.220..=2.1.241` probed points. Host
`claude` was not on `PATH`; missing install is not a gap.

No public API, provider contact, login, install, host update, release, tag,
publication, or consumer change entered the claim. Claude Agent SDK native
`2.1.270` stays a separate family. Research 331. g06.009.

## Validation

`cargo fmt -p swallowtail-adapter-claude-agent -- --check`,
`effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`, and the
named research, logs, roadmaps, and roadmap-number docs gates. No workspace
`qa`.
