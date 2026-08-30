# 2026-08-30 Claude Code 2.1.251 Claim

## Result

Card 018 raised `claude-code.headless-stream-json` through exact
`2.1.251` (`2.1.220..=2.1.251`) and `claude-code.response-only-stream-json`
through exact `2.1.251` (`2.1.227..=2.1.251`) as compatible extensions of
the existing stream-JSON behaviors. Published intermediates `2.1.242`,
`2.1.243`, `2.1.245`, `2.1.246`, `2.1.247`, `2.1.248`, and `2.1.250` are
qualified. Unpublished `2.1.244` and `2.1.249` stay incompatible. No new
milestone. Later stables stay AllowUnverified; the synthetic later point
is now `2.1.252`. Unused help and changelog surfaces stay unmapped.
Maximum-turn and other feature-specific exact version sets stay on the
`2.1.220..=2.1.241` probed points. Claude Agent ACP remains a separate
axis. Watcher support is not advertised.

g05.005 is standing currentness, completed. Next Task pointer stays on
the post-currentness watcher card 010 reassessment.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent` passed
- `effigy validate:focused swallowtail-adapter-claude-agent` passed (118 tests)
- `effigy package:verify-affected swallowtail-adapter-claude-agent` passed
- `effigy qa:routes` passed
- `effigy qa:northstar` passed
- research, logs, roadmaps, g05, batch-card, and next-action indexes passed
- `git diff --check` passed
- official npm `latest` remained `2.1.251` through closeout

## Next

Reassess watcher card 010 against the landed `2.1.251` route
qualification. Do not start watcher wiring.
