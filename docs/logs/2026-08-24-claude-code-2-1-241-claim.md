# 2026-08-24 Claude Code 2.1.241 Claim

## Result

Card 154 raised `claude-code.headless-stream-json` through exact
`2.1.241` (`2.1.220..=2.1.241`) and `claude-code.response-only-stream-json`
through exact `2.1.241` (`2.1.227..=2.1.241`) as compatible extensions of
the existing stream-JSON behaviors. Published intermediates `2.1.239`
and `2.1.240` are qualified. No new milestone. Later stables stay
AllowUnverified; the synthetic later point is now `2.1.242`. Deny-list
stays empty. Unused changelog surfaces stay unmapped. Claude Agent ACP
remains a separate axis.

g04.055 is standing currentness, completed. Next Task pointer and g04
generation status were left alone.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent` passed
- `scripts/validate-focused-packages.sh swallowtail-adapter-claude-agent` passed (102 tests, clippy `-D warnings`)
- `scripts/verify-affected-packages.sh swallowtail-adapter-claude-agent` passed

No `effigy` binary on this host; the named scripts are the task equivalents.

## Next

Resume g04.053 Qoder max turns.
