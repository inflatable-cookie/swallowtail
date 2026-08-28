# 2026-08-28 g04.089a Claude Code Headless Permission-Mode Evidence

Status: complete
Card: 252
Research: 249

## Boundary

Evidence only. This lane owns card 252, Research 249, this log, and optional
new Claude-local frozen evidence. Shared planning and production stay unchanged.

## Target

Close exact version, permission-mode membership, resource/tool authority,
application, terminal, lifecycle, cleanup, and omission truth.

## What Changed

- Probed every published `@anthropic-ai/claude-code-darwin-arm64` point in
  `2.1.220..=2.1.241` under disposable homes. No host install, login, or
  provider prompt.
- Froze official permission-modes / CLI / settings docs plus package digests
  into
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-permission-mode.json`.
- Promoted Research 249 with an honest empty deliver-now set.
- Completed card 252.

## Evidence Verdict

`--permission-mode` is advertised and parse-accepted across the whole
qualified window. Help choices are
`acceptEdits|auto|bypassPermissions|manual|dontAsk|plan`; `default` is
accepted as Manual and `manual` aliases to it. Unsupported tokens reject
before prompt effects. `acceptEdits` and `auto` widen writes/approvals
relative to Plan. `default`/`dontAsk` remain parse-only candidates: ambient
`permissions.allow` / `defaultMode` via `--setting-sources` and non-Plan
lifecycle prevent a closed Plan replacement. `bypassPermissions` stays
excluded. Deliver-now rows: none. Omission unchanged; keep
`--permission-mode plan`. Card acceptance line for non-empty-row closure is
annotated N/A.

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent` passed
- `effigy qa:northstar` passed
- `git diff --check` passed

## Next

PR against current pushed `main`. Merge remains operator-owned. No production
binding.
