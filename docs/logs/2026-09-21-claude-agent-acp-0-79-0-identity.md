# 2026-09-21 Claude Agent ACP 0.79.0 Identity

## Result

Research 335 froze official npm/GitHub/ACP-registry
`@agentclientprotocol/claude-agent-acp` `0.79.0` against the `0.76.0`
claim. Host `claude-agent-acp` was not on `PATH` and was not installed.
npm `gitHead` matches GitHub tags `v0.77.0`
(`dfe823b9581979cd22db40272d4469cc7e42b77e`), `v0.78.0`
(`becd854dd813af769475cf66e081d48c3f25f081`), and `v0.79.0`
(`d421f56a6c43cde16d9a7531d08a750a5ef2f04a`). Official `0.79.0` published
2026-09-17T14:55:56.043Z. Published stables after `0.76.0` are exactly
`0.77.0`, `0.78.0`, and `0.79.0`; `0.73.1`, `0.74.1`, `0.75.2`, `0.76.1`,
`0.77.1`, `0.78.1`, and `0.80.0` stay unpublished stables. The ACP SDK pin
stays `1.4.0`. The Agent SDK pin moves `0.3.257` → `0.3.270` → `0.3.274`
and stays unmapped.

Complete dist inventory `0.76.0` (123 files) → `0.77.0` (123) → `0.78.0`
(126) → `0.79.0` (126) is frozen; no file was removed at any hop.
`dist/index.js`, `dist/settings.js`, `dist/utils.js`, and `dist/lib.js` are
byte-identical across every hop. Mode ids/categories, `plan`/`acceptEdits`,
permission option kinds, and the effort config id stay by classified
source. `dist/acp-agent.js` and `dist/elicitation.js` change; each change
is classified mapped-adjacent, provider-internal, or unmapped with a
reason. The removed `agent` config option, capability-gated
`compaction_update`, AIR file-change / diffStats, defaultToNo option order,
and shell permission titles stay unmapped. Already-mapped form elicitation
keeps the same field keys and `Other` title; the two new Other description
strings are accepted so those forms still map. Mapped usage invariant,
`stopReason` domain/catch-all, and `session/cancel` stay
selected-compatible. Wire `protocolVersion` stays `1`; no new
`sessionUpdate` kind is emitted on the selected route. Unpublished `0.58.0`
stays a gap. First later unpublished stable is `0.80.0`. Claude Code
stream-JSON and the Claude Agent SDK sidecar stay untouched. Decision:
compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7` through `0.79.0`.

## Next

Raise the qualified ceiling to `0.79.0`. Keep exclusion `0.58.0`,
`AllowUnverified`, and a synthetic later `UnverifiedNewer` of `0.80.0`.
Widen already-mapped elicitation Other-description matching.
