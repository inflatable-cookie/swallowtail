# 2026-09-25 Claude Agent ACP 0.81.2 Identity

## Result

Research 352 froze official npm/GitHub/ACP-registry
`@agentclientprotocol/claude-agent-acp` `0.81.2` against the `0.79.0`
claim. Host `claude-agent-acp` was not on `PATH` and was not installed.
npm `gitHead` matches GitHub tags `v0.80.0`
(`16c9d1a3af6bd81e5a8b07420b4bb17bfda22ffb`), `v0.81.0`
(`d571358e267ed21ed0ec9ec2622fda8935535d57`), `v0.81.1`
(`b264b52bee80e49f20caf1941f7d7cb89edb80c4`), and `v0.81.2`
(`5dbb453c63a89746627799b2b06b31ba01a1b674`). Official `0.81.2` published
2026-09-24T10:18:05.741Z. Published stables after `0.79.0` are exactly
`0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2`. The `0.79.0` tarball reproduces
Research 335. `0.58.0` stays unpublished. `0.79.1` and `0.80.1` are
preview-only and are not new exclusions. First later unpublished stable is
`0.81.3`.

The ACP SDK pin moves `1.4.0` → `1.5.0` at `0.80.0`. Published SDK
`dist/acp.js` is byte-identical across that pin and `PROTOCOL_VERSION`
stays `1`. The Agent SDK pin moves `0.3.274` → `0.3.278` → `0.3.280` and
stays unmapped.

Complete dist inventory `0.79.0` (126 files) → `0.80.0` (126) → `0.81.0`
(129) → `0.81.1` (132) → `0.81.2` (132) is frozen; no file was removed at
any hop. Ninety-nine files, including elicitation, lib, settings, utils,
session-config ids, and permission options/presentation, are byte-identical
across every hop. `dist/index.js` changes only at `0.81.1`. Initialize
session capabilities, client methods, mode and effort ids, permission
kinds, prompt usage, and `stopReason` stay. `notice` and
`compaction_update` stay capability-gated and unadvertised. Usage-model and
informational metadata ride in `_meta` and stay ignored. Claude Code
stream-JSON and the Claude Agent SDK sidecar stay untouched. Decision:
compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7` through `0.81.2`.

## Next

Raise the qualified ceiling to `0.81.2`. Keep exclusion `0.58.0`,
`AllowUnverified`, and a synthetic later `UnverifiedNewer` of `0.81.3`.
