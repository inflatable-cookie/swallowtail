# 2026-09-25 Claude Agent ACP 0.81.2 Claim

## Result

Raised `claude-agent.acp-adapter` through exact official `0.81.2`
(`0.66.0..=0.81.2`) as a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`. No new milestone.
Published hops `0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2` are qualified.
Unpublished `0.58.0` stays the standing exclusion. Preview-only `0.79.1`
and `0.80.1` are not new exclusions. Unpublished `0.81.3` is the synthetic
later `UnverifiedNewer` point. The ACP SDK pin moves `1.4.0` → `1.5.0`
with byte-identical `dist/acp.js` and protocol version `1`. The Agent SDK
pin (`0.3.274` → `0.3.278` → `0.3.280`) stays unmapped. Capability-gated
`notice` and `compaction_update`, managed policy, usage-model `_meta`,
informational chunk `_meta`, exit-plan `allow_always` extras, and the
`0.81.2` subagent parent diagnostic stay unmapped with reasons. Baseline
`0.53.0`, claim id `claude-agent.acp.window-2`, exclusion `0.58.0`, every
historical segment, and `AllowUnverified` survive. Claude Code headless and
response-only stay at their current ceilings. Research 352 froze the
identity evidence in the same batch. g06.038 is the milestone record.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent` passed
- `effigy validate:focused swallowtail-adapter-claude-agent` passed: 580 tests and `-D warnings` clippy, on Node `22.23.2`
- `effigy package:verify-affected swallowtail-adapter-claude-agent` passed from `/tmp/st-qa`. The `/workspace` checkout fails the host-path audit because pre-existing Claude Agent SDK text contains the substring `rule/workspace`, which matches that checkout path. That failure is not this ACP change
- `effigy qa:routes` passed
- `effigy qa:northstar` passed
- `effigy qa:docs:index:research`, `qa:docs:index:logs`, `qa:docs:index:roadmaps`, and `qa:docs:index:roadmaps:g06` passed
- `effigy qa:docs:index:roadmaps:batch-cards` is not defined. `effigy qa:docs:roadmaps:status` passed and rejects a `batch-cards/` level
- `effigy qa:docs:roadmaps:numbers` passed against canonical main `49b0d308c6d6`
- official latest was rechecked immediately before the validation push: npm `latest` `0.81.2` (`gitHead` `5dbb453c63a89746627799b2b06b31ba01a1b674`), GitHub `v0.81.2`, ACP registry `claude-acp` `0.81.2`. No in-run movement

No workspace `qa`. No provider prompt, live ACP initialize, authentication, install, host update, downloaded-artifact execution, release, tag, publication, or consumer mutation.

## Next

Family claim is done. No Next Task, roadmap generation, or handoff
mutation.
