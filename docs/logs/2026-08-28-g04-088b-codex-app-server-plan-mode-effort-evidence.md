# 2026-08-28 g04.088b Codex App-Server Plan-Mode Effort Evidence

Status: complete
Card: 249
Research: 246

## Boundary

Evidence only. This lane owns card 249, Research 246, this log, and
`crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-plan-mode-effort-range.json`.
Shared planning and production stay unchanged.

## Target

Close exact version, model, Plan mode, effort value, request, confirmation,
persistence, restoration, lifecycle, and omission truth for
`plan_mode_reasoning_effort`.

## Finding

Honest empty deliver-now set. Exact tags `0.147.0`–`0.149.1` expose the control
only as ambient `config.toml` / session-static `config/batchWrite`. The v2
protocol never names the key. Plan selection remains
`collaborationMode.mode = plan` with ordinary
`settings.reasoning_effort` / turn `effort`. App-server never reads
`plan_mode_reasoning_effort` into Plan; TUI applies it client-side only. Cold
resume restores ordinary `model_reasoning_effort`, not the Plan-mode key.
Ordinary turn reasoning and exec config were not promoted onto this control.

## Output Contract

Promoted Research 246 with a closed empty table. Frozen corpus:
`crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-plan-mode-effort-range.json`.

## Validation

```sh
effigy validate:focused swallowtail-adapter-codex
effigy qa:northstar
git diff --check
```

## Next

Worker PR against current pushed `main`. Do not merge or begin production
binding. Shared g04.088 / Next Task closeout stays with the orchestrator.
