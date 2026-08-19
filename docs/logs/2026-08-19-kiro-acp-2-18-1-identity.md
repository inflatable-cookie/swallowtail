# 2026-08-19 Kiro ACP 2.18.1 Identity

## Result

Card 291 froze official Kiro ACP identity at installer-manifest
`2.18.1` without installing, logging in, or sending `initialize`. The
selected wire is `kiro-cli acp` stdio ACP: initialize, `session/new`,
one bounded `session/prompt` with field `prompt`, cancel, and joined
cleanup. Changelog still headlines `2.18.0`. Docs example field
`content` is rejected. `kiro.headless`, `--cloud`, `--agent`,
`_kiro.dev/*`, advertised `session/load`, TUI `kiro-cli-chat`, and npm
`kiro-cli` stay out. Named fixtures live under the future adapter tree.
No production claim. Current source stays 38 packages and 45 routes.

## Validation

- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`

## Next

Implement the Kiro ACP driver core (card 292).
