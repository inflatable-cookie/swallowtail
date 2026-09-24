# 2026-09-24 Claude Code 2.1.281 Claim

## Result

Raised `claude-code.headless-stream-json` from `2.1.220..=2.1.278` to
`2.1.220..=2.1.281` as a compatible extension of
`claude-code.headless.stream-json.v1`. Split
`claude-code.response-only-stream-json` into v1 `2.1.227..=2.1.278` and v2
`2.1.280..=2.1.281` under the Contract 039 narrowed built-in-hook guarantee.
Unpublished `2.1.279` is an explicit exclusion on both axes. Synthetic later
stable `2.1.282` is `UnverifiedNewer`.

Official latest at the claim was still npm/GitHub `2.1.281`. Selected
response-only argv is unchanged across segments. v2 without a project
location launches in an adapter-owned empty temporary directory.
`instructionFiles` and telemetry env-var candidates were tested against the
frozen parsers and not pinned. Watcher stays exact `2.1.251`. Every
feature-specific exact set stays on its probed points.

Research 342. g06.022.

## Validation

`cargo fmt -p swallowtail-adapter-claude-agent -- --check`,
`effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy qa:routes`, `effigy qa:docs`, and `git diff --check`.
