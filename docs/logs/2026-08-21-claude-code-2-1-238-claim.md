# 2026-08-21 Claude Code 2.1.238 Claim And Acceptance

## Result

Card 080 raised `claude-code.headless-stream-json` through exact
`2.1.238` (`2.1.220..=2.1.238`) and `claude-code.response-only-stream-json`
through exact `2.1.238` (`2.1.227..=2.1.238`) as compatible extensions of
the existing stream-JSON behaviors. Published intermediates `2.1.236`
and `2.1.237` are qualified. No new milestone. Later stables stay
AllowUnverified; the synthetic later point is now `2.1.239`. Deny-list
stays empty. Unused changelog surfaces stay unmapped. Claude Agent ACP
remains a separate axis.

g04.028 is complete as standing currentness compiled into the active
generation. The Next Task stays on g04.024; Kimi Platform implementation has
not started.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent`
- `effigy validate:focused swallowtail-adapter-claude-agent` — 101 passed
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- route, Northstar, research/log/roadmap index, and next-action gates

## Next

Review and land the remaining operator-named currentness families without
changing the g04.024 pointer.
