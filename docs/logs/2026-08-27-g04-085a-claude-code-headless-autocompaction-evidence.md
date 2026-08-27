# 2026-08-27 g04.085a Claude Code Headless Autocompaction Evidence

Status: complete
Card: 238
Research: 237

## Boundary

Evidence only. The worker may update this file, card 238, Research 237, and new
Claude-local frozen evidence. Shared planning and production code stay unchanged.

## Outcome

Honest empty deliver-now set.

`--autocompact <auto|tokens>` is absent at `2.1.220` and present at every
published `2.1.221..=2.1.241` point. It selects an auto-compact window, not
context size, output limit, or enablement. Invalid values reject before doctor
or print work. Explicit argv overrides saved `autoCompactWindow` for one launch
and does not write settings, but `CLAUDE_CODE_AUTO_COMPACT_WINDOW` overrides the
flag in exact source, and `DISABLE_AUTO_COMPACT` / `DISABLE_COMPACT` can nullify
compaction. The headless approved environment is opaque, so caller selection is
not operation-private. No prompt-free effective-window or compaction
confirmation exists.

Frozen evidence:
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-autocompaction.json`
plus `claude_code_headless_autocompaction_identity.rs`.

No production binding, shared index, matrix, or Next Task edit.
