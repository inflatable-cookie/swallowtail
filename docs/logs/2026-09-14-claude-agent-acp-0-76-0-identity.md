# 2026-09-14 Claude Agent ACP 0.76.0 Identity

## Result

Research 309 froze official npm/GitHub/ACP-registry
`@agentclientprotocol/claude-agent-acp` `0.76.0` against the `0.73.0`
claim as the first family of operator-authorized Research 308. Host remains
exact `0.63.0`
(`260aac90bf75f197b93640087c1de66441761d43c2784efa035fdcee60b5dacd`),
matching the frozen `0.70.0` host digest. npm `gitHead` matches GitHub tags
`v0.74.0` (`847a3b642929654b5bf32b143b3e1efd5884d331`), `v0.75.0`
(`a0122dae4c6f0d19a6716561683607dac4f50215`), `v0.75.1`
(`3e23c5b960b66a6d2c892e7524c952e731c076a7`), and `v0.76.0`
(`c2e4815029ef3962787ecaefe208b0b6f8b81302`). Official `0.76.0` published
2026-09-09T21:16:32.982Z. Published stables after `0.73.0` are exactly
`0.74.0`, `0.75.0`, `0.75.1`, and `0.76.0`; `0.73.1`, `0.74.1`, `0.75.2`,
`0.76.1`, and `0.77.0` stay unpublished stables. The ACP SDK pin stays
`1.4.0` and the Agent SDK pin stays `0.3.257` at every hop.

Complete dist inventory `0.73.0` (96 files) → `0.74.0` (99) → `0.75.0`
(111) → `0.75.1` (117) → `0.76.0` (123) is frozen; no file was removed at
any hop. `dist/index.js`, `dist/elicitation.js`, `dist/settings.js`,
`dist/utils.js`, `dist/tools.js`, `dist/session-mode.js`,
`dist/session-config-ids.js`, and the whole `dist/permissions/**` tree are
byte-identical across every hop, so mode ids/categories,
`plan`/`acceptEdits`, permission option kinds, and the effort config id are
unchanged without inference. `dist/acp-agent.js` changes on every hop; each
change is classified mapped-adjacent, provider-internal, or unmapped with a
reason. The `--hide-claude-auth` guard, the `authStatus` push extension,
the synthetic context-compaction tool call, usage Markdown, AIR fork
metadata, clear-context coordination, and capability-gated recommended
config values stay unmapped. Mapped usage invariant, `stopReason`
domain/catch-all, and `session/cancel` stay selected-compatible. Wire
`protocolVersion` stays `1`; no new `sessionUpdate` kind is emitted.
Unpublished `0.58.0` stays a gap. First later unpublished stable is
`0.77.0`. Claude Code and the watcher stay untouched. Production claims
stayed at `0.73.0` in this commit. Decision for the claim commit: compatible
extension of `claude-agent.acp.initialize-meta-extensions-v7` through
`0.76.0`.

## Next

Raise the qualified ceiling to `0.76.0`. Keep exclusion `0.58.0`,
`AllowUnverified`, and a synthetic later `UnverifiedNewer` of `0.77.0`.
