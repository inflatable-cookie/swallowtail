# 2026-09-01 Claude Agent ACP 0.73.0 Identity

## Result

Operator restart of the unmerged `0.72.0` family after official latest
moved. Card 044 froze official npm `@agentclientprotocol/claude-agent-acp`
`0.73.0` against the `0.70.0` claim. Host remains exact `0.63.0`
(`260aac90bf75f197b93640087c1de66441761d43c2784efa035fdcee60b5dacd`).
npm gitHead matches GitHub tags `v0.71.0`
(`889346fcf5ff546f7c07e546dbc42de37ce0992d`), `v0.72.0`
(`d3eff191576abcaa7592bb3ac55ff7534e4fe35d`), and `v0.73.0`
(`ea7076c0bc324603e65d8c124b7573f158749969`). Official `0.73.0` published
2026-09-01T20:27:53.428Z. Published stables after `0.70.0` are exactly
`0.71.0`, `0.72.0`, and `0.73.0`. Mapped `dist/index.js`,
`dist/elicitation.js`, `dist/lib.js`, `dist/settings.js`, and
`dist/utils.js` are byte-identical to `0.70.0`. Complete dist inventory
`0.70.0` (33 files) → `0.71.0` (96, +63 / −0 / 13 changed) → `0.72.0`
(96, +0 / −0 / 6 changed) → `0.73.0` (96, +0 / −0 / 1 changed:
`package.json` only) is frozen; every `dist/**` file is byte-identical
`0.72.0` to `0.73.0`. Remaining named files stay unmapped with reason.
`0.71.0`/`0.72.0` stay intermediate supporting evidence, not a standalone
ceiling. Usage invariant, `stopReason` domain/catch-all, and
`session/cancel` stay selected-compatible. Five new emitted update kinds
stay unmapped. Wire `protocolVersion` stays `1`. Unpublished `0.58.0`
stays a gap. Unpublished `0.74.0` is the first later stable. Claude Code
and the watcher stay untouched. Production claims stayed at `0.70.0` in
this card. Decision for card 045: compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7` through `0.73.0`.

## Next

Raise the qualified ceiling on card 045. Keep exclusion `0.58.0` and
`AllowUnverified`.
